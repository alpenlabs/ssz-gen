//! File reading module for reading SSZ files.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

/// Recursively reads all `.ssz` files in a directory and returns a mapping of file paths to their contents.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory to read files from
///
/// # Returns
///
/// A `Result` containing a `HashMap` where keys are relative file paths as strings and values are file contents as strings.
/// Returns an `io::Error` if reading the directory or any file fails.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// let files = ssz_codegen::files::read_directory_ssz_files("./schema").unwrap();
/// for (path, content) in files {
///     println!("File: {path}, Content: {content}");
/// }
/// ```
pub fn read_directory_ssz_files<P: AsRef<Path>>(
    dir_path: P,
) -> io::Result<HashMap<String, String>> {
    let dir_path = dir_path.as_ref();
    let mut file_map = HashMap::new();
    read_directory_ssz_files_recursive(dir_path, dir_path, &mut file_map)?;
    Ok(file_map)
}

fn read_directory_ssz_files_recursive<P: AsRef<Path>>(
    base_dir: &Path,
    current_dir: P,
    file_map: &mut HashMap<String, String>,
) -> io::Result<()> {
    if current_dir.as_ref().is_dir() {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                read_directory_ssz_files_recursive(base_dir, &path, file_map)?;
            } else if path.extension().map(|ext| ext == "ssz").unwrap_or(false) {
                let relative_path = path
                    .strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                let content = fs::read_to_string(&path)?;
                file_map.insert(relative_path, content);
            }
        }
    }

    Ok(())
}
