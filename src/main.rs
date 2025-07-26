use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::Result;

use crate::handlers::{delete, delete_all, list, open};

mod handlers;
mod opts {
    use colored::control::SHOULD_COLORIZE;
    use std::io::{IsTerminal, stdout};

    use crate::ColorOptions;
    pub fn colors(opt: ColorOptions) {
        match opt {
            ColorOptions::Always => SHOULD_COLORIZE.set_override(true),
            ColorOptions::Auto => SHOULD_COLORIZE.set_override(stdout().is_terminal()),
            ColorOptions::Never => SHOULD_COLORIZE.set_override(false),
        }
    }
}

#[derive(Parser, Clone)]
#[command(version)]
struct Cli {
    /// Use json output
    #[arg(short, long)]
    json: bool,

    /// Specify when to use colors
    #[arg(short, long, value_enum, default_value_t)]
    color: ColorOptions,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, ValueEnum, Default)]
enum ColorOptions {
    Always,
    #[default]
    Auto,
    Never,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// List incoming emails
    List {
        /// Email to be used
        #[arg(short, long)]
        email: String,

        /// Only return this much emails, max is 100
        #[arg(short, long, default_value_t = 10)]
        limit: u8,

        /// Skip this many emails and return the rest
        #[arg(short, long, default_value_t = 0)]
        offset: u32,
    },

    /// Open a specific email
    Open {
        #[arg(short, long)]
        id: String,
    },

    /// Delete all emails
    DeleteAll {
        #[arg(short, long)]
        email: String,
    },

    /// Delete a specific email
    Delete {
        #[arg(short, long)]
        id: String,
    },
}

#[tokio::main]
#[allow(dead_code, unused_variables)]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    opts::colors(cli.color);
    match cli.command {
        Commands::List {
            email,
            limit,
            offset,
        } => list(email, limit, offset, cli.json).await?,
        Commands::Open { id } => open(id, cli.json).await?,
        Commands::DeleteAll { email } => delete_all(email, cli.json).await?,
        Commands::Delete { id } => delete(id, cli.json).await?,
    };
    Ok(())
}
