
mod hash_info;
mod hashes;

use clap::Parser;
use colored::*;
use serde::Serialize;
use hash_info::HashInfo;

#[derive(Parser)]
#[command(name = "find_hash")]
#[command(author = "find_hash")]
#[command(version = "1.0")]
#[command(about = "Identify hash types", long_about = None)]
struct Cli {
    /// The hash string to identify
    #[arg(required = true)]
    hash: String,

    /// Output in JSON format
    #[arg(short, long)]
    json: bool,

    /// Show all matches (not just the most likely)
    #[arg(short, long)]
    all: bool,
}

#[derive(Serialize)]
struct JsonOutput {
    input: String,
    hashes: Vec<JsonHashDebug>,
}

#[derive(Serialize)]
struct JsonHashDebug {
    name: String,
    hashcat: Option<i32>,
    john: Option<String>,
    extended: bool,
    description: Option<String>,
}

impl From<&HashInfo> for JsonHashDebug {
    fn from(info: &HashInfo) -> Self {
        Self {
            name: info.name.to_string(),
            hashcat: info.hashcat,
            john: info.john.map(|s| s.to_string()),
            extended: info.extended,
            description: info.description.map(|s| s.to_string()),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let hash_input = cli.hash.trim();

    let mut matches = Vec::new();

    for prototype in hashes::PROTOTYPES.iter() {
        if prototype.regex.is_match(hash_input) {
            for mode in &prototype.modes {
                matches.push(mode);
            }
        }
    }

    if cli.json {
        let json_hashes: Vec<JsonHashDebug> = matches.iter().map(|&h| h.into()).collect();
        let output = JsonOutput {
            input: hash_input.to_string(),
            hashes: json_hashes,
        };
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        if matches.is_empty() {
            println!("{}", "No matches found.".red().bold());
            return;
        }

        println!("{} {}", "Analyzing hash:".bold(), hash_input.cyan());
        println!("{}", "Most Likely:".green().bold().underline());

        for (i, info) in matches.iter().enumerate() {
            println!();
            println!("  {}. {}", i + 1, info.name.yellow().bold());
            if let Some(desc) = info.description {
                println!("     {}", desc);
            }
            if let Some(hc) = info.hashcat {
                println!("     Hashcat Mode: {}", hc.to_string().blue());
            }
            if let Some(john) = info.john {
                println!("     John Format:  {}", john.blue());
            }
        }
    }
}
