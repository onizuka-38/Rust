use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, NaiveDate, Utc};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_DATA_FILE: &str = "transactions.json";

#[derive(Parser, Debug)]
#[command(name = "budget-tracker")]
#[command(about = "Track personal income and expenses from the command line")]
struct Cli {
    #[arg(long, default_value = DEFAULT_DATA_FILE, global = true)]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(value_enum)]
        kind: KindArg,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        category: String,
        #[arg(long, help = "Date in YYYY-MM-DD. Defaults to today")]
        date: Option<String>,
        #[arg(long)]
        memo: Option<String>,
    },
    List {
        #[arg(long, help = "Filter by month: YYYY-MM")]
        month: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    Summary {
        #[arg(long, help = "Filter by month: YYYY-MM")]
        month: Option<String>,
    },
    Delete {
        id: u64,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum KindArg {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Transaction {
    id: u64,
    kind: TransactionType,
    date: NaiveDate,
    category: String,
    amount: f64,
    memo: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BudgetStore {
    next_id: u64,
    transactions: Vec<Transaction>,
}

impl Default for BudgetStore {
    fn default() -> Self {
        Self {
            next_id: 1,
            transactions: Vec::new(),
        }
    }
}

impl BudgetStore {
    fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = fs::read_to_string(path)
            .with_context(|| format!("Failed to read data file: {}", path.display()))?;
        let store: Self = serde_json::from_str(&raw)
            .with_context(|| format!("Failed to parse JSON from {}", path.display()))?;
        Ok(store)
    }

    fn save(&self, path: &Path) -> Result<()> {
        let pretty = serde_json::to_string_pretty(self).context("Failed to serialize data")?;
        fs::write(path, pretty)
            .with_context(|| format!("Failed to write data file: {}", path.display()))?;
        Ok(())
    }

    fn add_transaction(&mut self, input: AddInput) -> &Transaction {
        let tx = Transaction {
            id: self.next_id,
            kind: input.kind,
            date: input.date,
            category: input.category,
            amount: input.amount,
            memo: input.memo,
            created_at: Utc::now(),
        };
        self.transactions.push(tx);
        self.next_id += 1;
        self.transactions.last().expect("pushed transaction")
    }

    fn delete_transaction(&mut self, id: u64) -> bool {
        let before = self.transactions.len();
        self.transactions.retain(|tx| tx.id != id);
        before != self.transactions.len()
    }
}

struct AddInput {
    kind: TransactionType,
    date: NaiveDate,
    category: String,
    amount: f64,
    memo: Option<String>,
}

#[derive(Clone, Copy)]
struct MonthFilter {
    year: i32,
    month: u32,
}

impl MonthFilter {
    fn matches(&self, date: NaiveDate) -> bool {
        date.year() == self.year && date.month() == self.month
    }
}

fn parse_month_filter(input: Option<String>) -> Result<Option<MonthFilter>> {
    let Some(raw) = input else {
        return Ok(None);
    };

    let mut split = raw.split('-');
    let year: i32 = split
        .next()
        .context("Month must be in YYYY-MM")?
        .parse()
        .context("Invalid year in month filter")?;
    let month: u32 = split
        .next()
        .context("Month must be in YYYY-MM")?
        .parse()
        .context("Invalid month in month filter")?;

    if split.next().is_some() || !(1..=12).contains(&month) {
        anyhow::bail!("Month must be in YYYY-MM with month 01-12");
    }

    Ok(Some(MonthFilter { year, month }))
}

fn parse_date(input: Option<String>) -> Result<NaiveDate> {
    match input {
        Some(raw) => Ok(NaiveDate::parse_from_str(&raw, "%Y-%m-%d")
            .with_context(|| format!("Invalid date '{raw}'. Use YYYY-MM-DD"))?),
        None => Ok(Local::now().date_naive()),
    }
}

fn to_kind(kind: KindArg) -> TransactionType {
    match kind {
        KindArg::Income => TransactionType::Income,
        KindArg::Expense => TransactionType::Expense,
    }
}

fn fmt_amount(amount: f64) -> String {
    format!("{amount:.2}")
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = BudgetStore::load(&cli.file)?;

    match cli.command {
        Commands::Add {
            kind,
            amount,
            category,
            date,
            memo,
        } => {
            if amount <= 0.0 {
                anyhow::bail!("Amount must be greater than 0");
            }

            let input = AddInput {
                kind: to_kind(kind),
                amount,
                category,
                date: parse_date(date)?,
                memo,
            };

            let added = store.add_transaction(input);
            store.save(&cli.file)?;

            println!("Added transaction #{}", added.id);
            println!(
                "  {} | {} | {} | {}",
                added.date,
                match added.kind {
                    TransactionType::Income => "income",
                    TransactionType::Expense => "expense",
                },
                added.category,
                fmt_amount(added.amount)
            );
            if let Some(memo) = &added.memo {
                println!("  memo: {memo}");
            }
        }
        Commands::List {
            month,
            category,
            limit,
        } => {
            let month = parse_month_filter(month)?;
            let category = category.map(|c| c.to_lowercase());

            let mut rows: Vec<&Transaction> = store
                .transactions
                .iter()
                .filter(|tx| {
                    let month_ok = month.map(|m| m.matches(tx.date)).unwrap_or(true);
                    let category_ok = category
                        .as_ref()
                        .map(|c| tx.category.to_lowercase() == *c)
                        .unwrap_or(true);
                    month_ok && category_ok
                })
                .collect();

            rows.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| b.id.cmp(&a.id)));

            if rows.is_empty() {
                println!("No transactions found.");
                return Ok(());
            }

            println!("id | date       | kind    | category           | amount   | memo");
            println!("---+------------+---------+--------------------+----------+----------------");

            for tx in rows.into_iter().take(limit) {
                let memo = tx.memo.as_deref().unwrap_or("");
                let kind = match tx.kind {
                    TransactionType::Income => "income",
                    TransactionType::Expense => "expense",
                };
                println!(
                    "{:<2} | {} | {:<7} | {:<18} | {:>8} | {}",
                    tx.id,
                    tx.date,
                    kind,
                    tx.category,
                    fmt_amount(tx.amount),
                    memo
                );
            }
        }
        Commands::Summary { month } => {
            let month = parse_month_filter(month)?;

            let filtered: Vec<&Transaction> = store
                .transactions
                .iter()
                .filter(|tx| month.map(|m| m.matches(tx.date)).unwrap_or(true))
                .collect();

            let income: f64 = filtered
                .iter()
                .filter(|tx| tx.kind == TransactionType::Income)
                .map(|tx| tx.amount)
                .sum();
            let expense: f64 = filtered
                .iter()
                .filter(|tx| tx.kind == TransactionType::Expense)
                .map(|tx| tx.amount)
                .sum();

            println!("Transactions: {}", filtered.len());
            println!("Income : {}", fmt_amount(income));
            println!("Expense: {}", fmt_amount(expense));
            println!("Net    : {}", fmt_amount(income - expense));

            let mut expense_by_category: Vec<(String, f64)> = Vec::new();
            for tx in filtered
                .iter()
                .copied()
                .filter(|tx| tx.kind == TransactionType::Expense)
            {
                if let Some((_, total)) = expense_by_category
                    .iter_mut()
                    .find(|(category, _)| *category == tx.category)
                {
                    *total += tx.amount;
                } else {
                    expense_by_category.push((tx.category.clone(), tx.amount));
                }
            }
            expense_by_category.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            if !expense_by_category.is_empty() {
                println!("\nTop expense categories:");
                for (category, total) in expense_by_category.into_iter().take(5) {
                    println!("  {:<18} {}", category, fmt_amount(total));
                }
            }
        }
        Commands::Delete { id } => {
            if store.delete_transaction(id) {
                store.save(&cli.file)?;
                println!("Deleted transaction #{}", id);
            } else {
                println!("Transaction #{} not found", id);
            }
        }
    }

    Ok(())
}
