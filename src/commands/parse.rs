use std::path::Path;

use crate::policy;

pub(crate) fn run(path: &Path) -> Result<(), String> {
    let parsed = policy::parse_file(path)?;
    policy::print(&parsed);
    Ok(())
}
