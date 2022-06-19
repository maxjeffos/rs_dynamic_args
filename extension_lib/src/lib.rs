use std::path::PathBuf;

use anyhow::Context;

pub mod launch_codes_orig;
pub mod launch_codes_new;
pub mod extension_metadata;

// Returns the directory where the current executable is located.
pub fn extension_root() -> anyhow::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .with_context(|| format!("Could not get parent directory of {}", exe_path.display()))?;
    Ok(exe_dir.to_path_buf())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
