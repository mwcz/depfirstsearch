use regex::Regex;
use std::env;
use std::process::Command;
use termion::{color, style};

const USAGE: &str = "USAGE: depfirstsearch REGEX";

fn main() {
    let Some(search_term) = env::args().nth(1) else {
        eprintln!("Missing REGEX\n{USAGE}");
        std::process::exit(1);
    };

    let is_stdout_tty = termion::is_tty(&std::fs::File::create("/dev/stdout").unwrap());

    let output = Command::new("cargo")
        .args(["metadata"])
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata output");

    let workspace_members = metadata["packages"]
        .as_array()
        .expect("Failed to extract workspace members");

    let mut all_crates = Vec::new();
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

    let keyword_re = Regex::new(&search_term).unwrap();

    let matching_crates = all_crates
        .iter()
        .filter(|&cr| keyword_re.is_match(cr))
        .map(|cr| cr.trim())
        .collect::<Vec<&str>>();

    // Search for the user's term in the collected output.
    if matching_crates.is_empty() {
        println!("No crates found matching {search_term}");
    } else {
        println!("{}", matching_crates.join("\n"));
    }
}
