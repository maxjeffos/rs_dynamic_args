use crate::extension_metadata::{self, ExtensionMetadata};
use anyhow;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct Extension {
    root: PathBuf,
    bin_path: PathBuf,
    pub metadata: ExtensionMetadata,
}

/// Load an extension from the given directory
pub fn try_load(dir_path: &Path) -> anyhow::Result<Extension> {
    let full_dir_path = dir_path.canonicalize()?;
    if !full_dir_path.exists() {
        return Err(anyhow::anyhow!(
            "extension root directory does not exist: {}",
            full_dir_path.display()
        ));
    }

    let dir_meta = fs::metadata(&full_dir_path)?;
    if !dir_meta.is_dir() {
        return Err(anyhow::anyhow!(
            "{} is not a directory",
            full_dir_path.display()
        ));
    }

    let extension_metadata_path = full_dir_path.join("extension.json");

    if !extension_metadata_path.exists() {
        return Err(anyhow::anyhow!(
            "extension metadata file {} does not exist",
            extension_metadata_path.display()
        ));
    }

    let metadata = extension_metadata::deser_extension_metadata(&extension_metadata_path)?;

    let bin_postfix = "_darwin_arm64";
    let bin_filename = format!("{}{}", metadata.name, bin_postfix);
    let bin_path = full_dir_path.join(bin_filename);

    if !bin_path.exists() {
        return Err(anyhow::anyhow!(
            "extension binary does not exist: {}",
            bin_path.display()
        ));
    }

    Ok(Extension {
        root: dir_path.to_path_buf(),
        bin_path,
        metadata,
    })
}

impl Extension {
    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    pub fn bin_path(&self) -> &Path {
        self.bin_path.as_path()
    }
}
