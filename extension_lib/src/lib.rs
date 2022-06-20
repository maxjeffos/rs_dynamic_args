use std::path::PathBuf;

use anyhow::Context;

pub mod extension_metadata;
pub mod launch_codes_new;
pub mod launch_codes_orig;

// Returns the directory where the current executable is located.
pub fn extension_root() -> anyhow::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .with_context(|| format!("Could not get parent directory of {}", exe_path.display()))?;
    Ok(exe_dir.to_path_buf())
}

pub fn read_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    let mut consecutive_newlines = 0;
    loop {
        let mut s = String::new();
        let bytes_read = std::io::stdin().read_line(&mut s)?;
        if bytes_read == 0 {
            break;
        }
        if s.trim().is_empty() {
            consecutive_newlines += 1;
            if consecutive_newlines >= 2 {
                break;
            }
        } else {
            consecutive_newlines = 0;
            input.push_str(&s);
        }
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
