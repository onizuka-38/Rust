mod collector;
mod model;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, ExecutableCommand};
use model::MetricsSnapshot;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::time;
use ui::UiState;

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    let (metrics_tx, metrics_rx) = watch::channel(MetricsSnapshot::default());
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let (key_tx, mut key_rx) = mpsc::unbounded_channel::<Event>();

    tokio::spawn(collector::run_collector(metrics_tx, shutdown_rx, 700));
    spawn_input_reader(key_tx);

    let mut app = UiState::default();
    let mut metrics_rx = metrics_rx;
    let mut draw_tick = time::interval(Duration::from_millis(80));

    let run_result = loop {
        tokio::select! {
            _ = draw_tick.tick() => {
                app.tick();
                let snapshot = metrics_rx.borrow().clone();
                if let Err(err) = terminal.draw(|f| ui::draw(f, &app, &snapshot)) {
                    break Err(err.into());
                }
            }
            Some(ev) = key_rx.recv() => {
                if let Event::Key(k) = ev {
                    if k.kind == KeyEventKind::Press {
                        if matches!(k.code, KeyCode::Char('q') | KeyCode::Esc) {
                            break Ok(());
                        }
                    }
                }
            }
            changed = metrics_rx.changed() => {
                if changed.is_err() {
                    break Ok(());
                }
            }
        }
    };

    let _ = shutdown_tx.send(());
    restore_terminal(&mut terminal)?;
    run_result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    execute!(stdout, crossterm::cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, crossterm::cursor::Show)?;
    terminal.show_cursor()?;
    Ok(())
}

fn spawn_input_reader(tx: mpsc::UnboundedSender<Event>) {
    std::thread::spawn(move || {
        loop {
            match event::poll(Duration::from_millis(100)) {
                Ok(true) => match event::read() {
                    Ok(ev) => {
                        if tx.send(ev).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                },
                Ok(false) => {}
                Err(_) => break,
            }
        }
    });
}
