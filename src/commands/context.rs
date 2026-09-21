use std::fs;

use crate::model::Rule;
use crate::policy::parse_file;
use crate::repository::{ensure_initialized, root};
use crate::util::io_error;

pub(crate) fn run() -> Result<(), String> {
    ensure_initialized()?;
    let directory = root()?.join("policies");
    let mut paths = fs::read_dir(directory)
        .map_err(io_error)?
        .filter_map(|entry| entry.ok().map(|value| value.path()))
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("crane"))
        .collect::<Vec<_>>();
    paths.sort();
    println!("# Crane active contracts");
    for path in paths {
        let policy = parse_file(&path)?;
        println!(
            "\n## Policy: {}\nCheckpoint: {}",
            policy.name, policy.checkpoint
        );
        for rule in policy.rules {
            let Rule::PreserveFunction { target } = rule;
            println!("- MUST NOT modify function `{target}`.");
        }
    }
    println!("\nCrane independently verifies these contracts after repository changes.");
    Ok(())
}
