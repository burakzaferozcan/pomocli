use std::process::Command;

/// Check if mpv is installed on the system.
pub fn is_mpv_available() -> bool {
    Command::new("mpv")
        .arg("--version")
        .output()
        .is_ok()
}


/// Format seconds into mm:ss
pub fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}
