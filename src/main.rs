use std::env;
use std::process::Command;

use regex::Regex;

fn main() {
    let search_term = env::args().nth(1).expect("Missing search term");

    let output = Command::new("cargo")
        .args(["metadata"])
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata output");

    let workspace_members = metadata["packages"]
        .as_array()
        .expect("Failed to extract workspace members");

    let mut all_info = Vec::new();
    for member in workspace_members {
        let name = member["name"].as_str().unwrap_or("");
        let version = member["version"].as_str().unwrap_or("");
        let description = member["description"]
            .as_str()
            .unwrap_or("")
            .trim()
            .replace("\n", "\n\t");
        let keywords: Vec<String> = member["keywords"]
            .as_array()
            .map(|kws| {
                kws.iter()
                    .map(|kw| format!("#{}", kw.as_str().unwrap_or_default()))
                    .collect()
            })
            .unwrap_or_default();

        all_info.push(format!(
            "{name}@{version}\n\t{description}\n\t{keywords}",
            keywords = keywords.join(" ")
        ));
    }

    let keyword_re = Regex::new(&search_term).unwrap();

    // Search for the user's term in the collected output.
    let out = all_info
        .iter()
        .filter(|&package| keyword_re.is_match(package))
        .map(|package| package.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    println!("{}", out);
}
