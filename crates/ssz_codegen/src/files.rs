//! File reading module for reading SSZ files.

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

/// Reads specified SSZ entry point files and returns a mapping of file paths to their contents.
///
/// # Arguments
///
/// * `entry_points` - A slice of strings representing the entry point file paths
/// * `base_dir` - The base directory where the entry point files are located
///
/// # Returns
///
/// A `Result` containing a `HashMap` where keys are file paths as strings and values are file
/// contents as strings. Returns an `io::Error` if reading any file fails.
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
/// let files = ssz_codegen::files::read_entrypoint_ssz(&["schema1.ssz", "schema2.ssz"], "./specs").unwrap();
/// for (path, content) in files {
///     println!("File: {path:?}, Content: {content}");
/// }
/// ```
pub fn read_entrypoint_ssz(
    entry_points: &[&str],
    base_dir: &str,
) -> io::Result<HashMap<PathBuf, String>> {
    let mut file_map = HashMap::new();
    for entry_point in entry_points {
        let path = Path::new(base_dir).join(entry_point);
        let content = fs::read_to_string(&path)?;
        file_map.insert(path.with_extension(""), content);
    }
    Ok(file_map)
}
