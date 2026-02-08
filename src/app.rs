use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Attendance CLI", version = "1.0", about = "Logging attendance")]
pub struct App {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  #[command(
    about = "Add a new Entry",
    long_about = "date: YYYY-MM-DD. subjects: comma separated subjects within a string"
  )]
  Add {
    #[arg(short, long)]
    date: String,
    #[arg(short, long)]
    subjects: String,
  },

  #[command(about = "Get the existing entries")]
  List {
    #[command(subcommand)]
    mode: ListMode,
  },
}

#[derive(Subcommand)]
pub enum ListMode {
  #[command(about = "Get all entries")]
  All,
  #[command(about = "Get the entry on a specific date")]
  Date {
    #[arg(short, long)]
    date: String, // "2026-01-01"
  },
  #[command(about = "Get the entry on a range of dates (in order)")]
  Range {
    #[arg(short, long)]
    range: String, // "2026-01-01..2026-01-07"
  },
  Subject {
    #[arg(short, long)]
    subject: String,
  },
}
