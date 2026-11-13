use std::fs;
use std::path::Path;

/// Returns current process open file descriptor count.
#[allow(dead_code)]
pub fn get_open_fd_count() -> usize {
    if let Ok(entries) = fs::read_dir("/proc/self/fd") {
        entries.filter_map(|e| e.ok()).count()
    } else {
        0
    }
}

/// Returns current process Resident Set Size (RSS) in bytes.
#[allow(dead_code)]
pub fn get_rss_bytes() -> usize {
    if let Ok(statm) = fs::read_to_string("/proc/self/statm") {
        let parts: Vec<&str> = statm.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(pages) = parts[1].parse::<usize>() {
                return pages * 4096; // 4KB page size
            }
        }
    }
    0
}

/// Recursively copies directory contents from `src` to `dst`.
#[allow(dead_code)]
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}

/// Truncates the specified WAL file at `offset` bytes.
#[allow(dead_code)]
pub fn truncate_wal_file(wal_path: &Path, offset: usize) -> std::io::Result<()> {
    if !wal_path.exists() {
        return Ok(());
    }
    let data = fs::read(wal_path)?;
    let cutoff = offset.min(data.len());
    fs::write(wal_path, &data[..cutoff])?;
    Ok(())
}
