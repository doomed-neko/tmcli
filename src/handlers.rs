use std::process::exit;

use chrono::{Local, TimeZone};
use color_eyre::eyre::Result;
use colored::Colorize;
use tmapi::Email;

fn check_email(email: &str) {
    if !email_address::EmailAddress::is_valid(email) {
        eprintln!(
            "{}: Invalid email format: {}",
            "ERROR".red().bold(),
            email.cyan()
        );
        exit(1);
    };
}

pub async fn list(email: String, limit: u8, offset: u32, json: bool) -> Result<()> {
    check_email(&email);

    if limit > 100 {
        eprintln!("{}: maximum limit is 100", "ERROR".red().bold());
        exit(1);
    }

    let client = tmapi::Client::new(email).unwrap();
    let msgs = client.get_emails(limit, offset).await;
    match msgs {
        Ok(mut msgs) => {
            msgs.sort_by(|x, y| x.received_at.cmp(&y.received_at));
            if json {
                println!("{}", serde_json::to_string(&msgs).unwrap());
                return color_eyre::eyre::Ok(());
            }
            for Email {
                id,
                from_address,
                to_address,
                subject,
                received_at,
                ..
            } in msgs
            {
                let date = Local.timestamp_opt(received_at, 0).unwrap().to_string();
                println!("{:10} {}", "ID".bold(), id.red());
                println!("{:10} {}", "DATE".bold(), date.blue());
                println!("{:10} {}", "FROM".bold(), from_address.cyan());
                println!("{:10} {}", "TO".bold(), to_address.cyan());
                println!("{:10} {}", "SUBJECT".bold(), subject);
                println!("{:-<50}", "");
            }
        }
        Err(t) => eprintln!("{}: {t}", "ERROR".red().bold()),
    };
    color_eyre::eyre::Ok(())
}

pub async fn open(id: String, json: bool) -> Result<()> {
    let client = tmapi::Client::new("example@example.com").unwrap();
    let msg = client.get_inbox(id).await;
    match msg {
        Ok(msg) => {
            if json {
                println!("{}", serde_json::to_string(&msg).unwrap());
                return color_eyre::eyre::Ok(());
            }

            let Email {
                id,
                from_address,
                to_address,
                subject,
                received_at,
                text_content,
                ..
            } = msg;
            let date = Local.timestamp_opt(received_at, 0).unwrap().to_string();
            let text_content = text_content.unwrap_or_default();
            println!("{:10} {}", "ID".bold(), id.red());
            println!("{:10} {}", "DATE".bold(), date.blue());
            println!("{:10} {}", "FROM".bold(), from_address.cyan());
            println!("{:10} {}", "TO".bold(), to_address.cyan());
            println!("{:10} {}", "SUBJECT".bold(), subject);
            println!("{:-<50}", "");
            println!("{text_content}");
        }
        Err(_) => todo!(),
    }
    color_eyre::eyre::Ok(())
}

pub async fn delete_all(email: String, json: bool) -> Result<()> {
    check_email(&email);
    let client = tmapi::Client::new(email).unwrap();
    let result = client.delete_all_emails().await;
    match result {
        Ok(t) => {
            if json {
                println!("{t}");
                return color_eyre::eyre::Ok(());
            }
            println!(
                "{}: Deleted {} emails",
                "SUCCESS".green().bold(),
                t.to_string().bold().red()
            );
        }
        Err(e) => {
            eprintln!("{}: Failed to delete emails: {e}", "ERROR".red().bold());
            exit(1);
        }
    }
    color_eyre::eyre::Ok(())
}

pub async fn delete(id: String, json: bool) -> Result<()> {
    let client = tmapi::Client::new("example@example.com").unwrap();
    let result = client.delete_inbox(id).await;
    match result {
        Ok(()) => {
            if json {
                println!("null");
                return color_eyre::eyre::Ok(());
            }
            println!("{}: Email {}", "SUCCESS".green().bold(), "deleted".red());
        }
        Err(e) => {
            eprintln!("{}: Failed to delete email: {e}", "ERROR".red().bold());
            exit(1);
        }
    }
    color_eyre::eyre::Ok(())
}
