//! Contains additional file system routines.

use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::path::absolute;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Recursively reads a directory and returns a list of all files.
/// Returned paths are relative to the given directory.
pub fn read_dir_all<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    // FIXME: One circular symbolic link and this blows up
    // TODO: Maybe turns this into an iterator?
    let mut frontier = VecDeque::new();
    let mut visited = Vec::new();
    frontier.push_back(dir.as_ref().to_path_buf());
    while let Some(dir) = frontier.pop_front() {
        for item in fs::read_dir(dir)? {
            let path = item?.path();
            let stat = fs::metadata(&path)?;
            if stat.is_dir() {
                frontier.push_back(path);
            } else if stat.is_file() {
                visited.push(path);
            } else {
                // only visited if we symlink_metadata
            }
        }
    }
    Ok(visited)
}

#[cfg(target_os = "windows")]
/// Opens Windows Explorer and highlights the given file
pub fn open_explorer(path: impl AsRef<Path>) -> io::Result<()> {
    use std::os::windows::process::CommandExt;
    let path = absolute(path)?;
    let arg = format!("/select,{}", path.display());
    // Note the "raw_arg"; without that, paths with space wouldn't work
    Command::new("explorer.exe").raw_arg(arg).spawn()?;
    sleep(Duration::from_millis(50)); // Delay to allow the window to open
    Ok(())
}
