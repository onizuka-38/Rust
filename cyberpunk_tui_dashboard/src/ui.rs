use crate::model::MetricsSnapshot;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

#[derive(Debug, Clone)]
pub struct UiState {
    pub frame_count: u64,
    pub glitch_seed: u64,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            glitch_seed: 0xC0FFEE,
        }
    }
}

impl UiState {
    pub fn tick(&mut self) {
        self.frame_count = self.frame_count.saturating_add(1);
        self.glitch_seed = self.glitch_seed.wrapping_mul(1103515245).wrapping_add(12345);
    }
}

pub fn draw(frame: &mut Frame<'_>, state: &UiState, snap: &MetricsSnapshot) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(11),
            Constraint::Min(8),
        ])
        .split(frame.size());

    draw_header(frame, root[0], state, snap);

    let mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(root[1]);

    draw_cpu_memory(frame, mid[0], snap);
    draw_network_gpu(frame, mid[1], snap);

    draw_process_panel(frame, root[2], snap);
}

fn draw_header(frame: &mut Frame<'_>, area: Rect, state: &UiState, snap: &MetricsSnapshot) {
    let title = glitch_title(state);
    let line = Line::from(vec![
        Span::styled("[ ", Style::default().fg(Color::Cyan)),
        Span::styled(
            title,
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ]", Style::default().fg(Color::Cyan)),
        Span::raw("   "),
        Span::styled(
            format!("cpu:{:>5.1}%", snap.cpu_avg),
            Style::default().fg(Color::Green),
        ),
        Span::raw("   "),
        Span::styled(
            format!("mem:{}/{} MiB", snap.memory_used_mb, snap.memory_total_mb),
            Style::default().fg(Color::LightCyan),
        ),
        Span::raw("   "),
        Span::styled(
            format!("tick:{}", state.frame_count),
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let p = Paragraph::new(line)
        .block(
            Block::default()
                .title("Neon Ops Grid")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightMagenta)),
        )
        .style(Style::default().bg(Color::Black));
    frame.render_widget(p, area);
}

fn draw_cpu_memory(frame: &mut Frame<'_>, area: Rect, snap: &MetricsSnapshot) {
    let mut lines = Vec::new();
    lines.push(Line::styled(
        "CPU CORES",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));

    for (i, v) in snap.cpu_per_core.iter().take(12).enumerate() {
        let bar = neon_bar(*v as f64, 18);
        lines.push(Line::from(vec![
            Span::styled(format!("C{:02} ", i), Style::default().fg(Color::Magenta)),
            Span::styled(bar, Style::default().fg(Color::LightGreen)),
            Span::raw(" "),
            Span::styled(format!("{:>5.1}%", v), Style::default().fg(Color::Yellow)),
        ]));
    }

    let mem_pct = if snap.memory_total_mb == 0 {
        0.0
    } else {
        (snap.memory_used_mb as f64 / snap.memory_total_mb as f64) * 100.0
    };
    lines.push(Line::raw(""));
    lines.push(Line::styled(
        format!(
            "MEM [{}] {:>5.1}% ({}/{}) MiB",
            neon_bar(mem_pct, 18),
            mem_pct,
            snap.memory_used_mb,
            snap.memory_total_mb
        ),
        Style::default().fg(Color::LightBlue),
    ));

    let p = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("Compute")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    frame.render_widget(p, area);
}

fn draw_network_gpu(frame: &mut Frame<'_>, area: Rect, snap: &MetricsSnapshot) {
    let mut lines = Vec::new();
    lines.push(Line::styled(
        "NET I/O",
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(vec![
        Span::styled("RX ", Style::default().fg(Color::Green)),
        Span::styled(
            format_speed(snap.net_rx_bytes_per_sec),
            Style::default().fg(Color::White),
        ),
        Span::raw("   "),
        Span::styled("TX ", Style::default().fg(Color::Magenta)),
        Span::styled(
            format_speed(snap.net_tx_bytes_per_sec),
            Style::default().fg(Color::White),
        ),
    ]));

    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "GPU",
        Style::default()
            .fg(Color::LightMagenta)
            .add_modifier(Modifier::BOLD),
    ));

    if !snap.nvml_available {
        lines.push(Line::styled(
            "NVML unavailable (NVIDIA driver/library not detected)",
            Style::default().fg(Color::DarkGray),
        ));
    } else if snap.gpus.is_empty() {
        lines.push(Line::styled(
            "No GPUs reported by NVML",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for g in &snap.gpus {
            let pct = if g.vram_total_mb == 0 {
                0.0
            } else {
                (g.vram_used_mb as f64 / g.vram_total_mb as f64) * 100.0
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("#{} {} ", g.index, g.name),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    format!("VRAM {} / {} MiB ({:>4.1}%)", g.vram_used_mb, g.vram_total_mb, pct),
                    Style::default().fg(Color::LightGreen),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(
                    format!("TEMP {}C", g.temperature_c),
                    Style::default().fg(Color::Red),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("GPU-UTIL {:>3}%", g.utilization_gpu),
                    Style::default().fg(Color::Cyan),
                ),
            ]));
        }
    }

    let p = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("Network / GPU")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightGreen)),
        );
    frame.render_widget(p, area);
}

fn draw_process_panel(frame: &mut Frame<'_>, area: Rect, snap: &MetricsSnapshot) {
    let mut lines = Vec::new();
    lines.push(Line::styled(
        "TOP PROCESSES (by CPU)",
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::styled(
        "PID      CPU%      MEM(MiB)   NAME",
        Style::default().fg(Color::DarkGray),
    ));

    if snap.top_processes.is_empty() {
        lines.push(Line::styled(
            "No process data yet...",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for p in &snap.top_processes {
            lines.push(Line::from(vec![
                Span::styled(format!("{:<7}  ", p.pid), Style::default().fg(Color::Magenta)),
                Span::styled(format!("{:>6.1}   ", p.cpu), Style::default().fg(Color::Green)),
                Span::styled(format!("{:>8}   ", p.memory_mb), Style::default().fg(Color::Cyan)),
                Span::styled(truncate_name(&p.name, 42), Style::default().fg(Color::White)),
            ]));
        }
    }

    let p = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("Process Matrix")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        );
    frame.render_widget(p, area);
}

fn truncate_name(name: &str, max: usize) -> String {
    if name.chars().count() <= max {
        return name.to_string();
    }
    let mut out = name
        .chars()
        .take(max.saturating_sub(1))
        .collect::<String>();
    out.push('~');
    out
}

fn glitch_title(state: &UiState) -> String {
    let variants = [
        "CYBERPUNK SYS MONITOR",
        "CYBERPUNK SYS M0NITOR",
        "CYBERPUNK // SYS MONITOR",
        "CYBERPUNK SYS M0N1T0R",
        "CYBERPUNK SYS MON1TOR",
    ];
    let idx = ((state.frame_count + (state.glitch_seed & 0x3)) % variants.len() as u64) as usize;
    variants[idx].to_string()
}

fn neon_bar(pct: f64, width: usize) -> String {
    let clamped = pct.clamp(0.0, 100.0);
    let filled = ((clamped / 100.0) * width as f64).round() as usize;
    let mut out = String::with_capacity(width);
    for i in 0..width {
        if i < filled {
            out.push('#');
        } else {
            out.push('.');
        }
    }
    out
}

fn format_speed(bytes_per_sec: u64) -> String {
    let b = bytes_per_sec as f64;
    if b >= 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} GiB/s", b / 1024.0 / 1024.0 / 1024.0)
    } else if b >= 1024.0 * 1024.0 {
        format!("{:.2} MiB/s", b / 1024.0 / 1024.0)
    } else if b >= 1024.0 {
        format!("{:.2} KiB/s", b / 1024.0)
    } else {
        format!("{} B/s", bytes_per_sec)
    }
}
