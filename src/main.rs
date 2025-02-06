use argh::FromArgs;
use regex::Regex;
use std::io;
use std::process::{Command, Output};
use termion::{color, style};

#[derive(Debug, FromArgs)]
/// Keep your dependency tree small by searching for crates you may already have.
struct Args {
    /// the term to search for (supports regex)
    #[argh(positional)]
    term: String,
}

fn main() {
    let args: Args = argh::from_env();

    let Ok(output): io::Result<Output> = Command::new("cargo").args(["metadata"]).output() else {
        eprintln!("Failed to execute `cargo metadata`; are you in a cargo workspace?");
        std::process::exit(1);
    };

    if !output.status.success() {
        eprintln!("Failed to execute `cargo metadata`; are you in a cargo workspace?");
        std::process::exit(1);
    }

    let Ok(metadata): Result<serde_json::Value, _> = serde_json::from_slice(&output.stdout) else {
        eprintln!("Could not parse `cargo metadata` output.");
        std::process::exit(1);
    };

    let is_stdout_tty = termion::is_tty(&std::fs::File::create("/dev/stdout").unwrap());

    let workspace_members = metadata["packages"]
        .as_array()
        .expect("Failed to extract workspace members");

    let keyword_re = Regex::new(&args.term).unwrap();

    let size_estimate = workspace_members.len() / 10;

    let mut all_crates = Vec::with_capacity(size_estimate);

    for member in workspace_members {
        let name = member["name"].as_str().unwrap_or("");
        let version = member["version"].as_str().unwrap_or("");
        let description = member["description"]
            .as_str()
            .unwrap_or("")
            .trim()
            .replace("\n", "\n\t");
        let keywords = member["keywords"]
            .as_array()
            .map(|kws| {
                kws.iter()
                    .map(|kw| format!("#{}", kw.as_str().unwrap_or_default()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
            .join(" ");

        if keyword_re.is_match(name)
            || keyword_re.is_match(version)
            || keyword_re.is_match(&keywords)
            || keyword_re.is_match(&description)
        {
            let crate_msg = if is_stdout_tty {
                format!(
                    "{bold}{name}{reset_style}@{version}  {kw_col}{keywords}\n\t{desc_col}{description}{reset_col}",
                    bold = style::Bold,
                    kw_col = color::Fg(color::Cyan),
                    desc_col = color::Fg(color::Green),
                    reset_col = color::Reset.fg_str(),
                    reset_style = style::Reset,
                )
            } else {
                format!("{name}@{version}  {keywords}\n\t{description}")
            };

            all_crates.push(crate_msg);
        }
    }

    // Search for the user's term in the collected output.
    if all_crates.is_empty() {
        println!("No crates found matching {}", args.term);
    } else {
        println!("{}", all_crates.join("\n"));
    }
}
