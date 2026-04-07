use crate::prompt::SoundType;
use std::process::{Child, Command, Stdio};

pub struct SoundController {
    process: Option<Child>,
}

impl SoundController {
    pub fn new() -> Self {
        Self { process: None }
    }

    pub fn start(&mut self, sound: &SoundType) {
        self.stop();

        let cmd_args = match sound {
            SoundType::BuiltIn(name) => vec![format!("assets/{}", name)],
            SoundType::Youtube(link) => vec![link.clone()],
            SoundType::None => return,
        };

        let mut args = vec![
            "--no-video".to_string(),
            "--loop".to_string(),
            "--ytdl-format=bestaudio/best".to_string(),
        ];
        args.extend(cmd_args);

        // Open a log file for mpv errors
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("pomocli.log")
            .unwrap();

        let child = Command::new("mpv")
            .args(&args)
            .stdout(Stdio::null())
            .stderr(Stdio::from(log_file))
            .spawn();

        match child {
            Ok(c) => self.process = Some(c),
            Err(e) => eprintln!("Failed to start mpv: {}", e),
        }
    }

    pub fn pause(&self) {
        if let Some(child) = &self.process {
            let pid = child.id();
            let _ = Command::new("kill")
                .arg("-STOP")
                .arg(pid.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }

    pub fn resume(&self) {
        if let Some(child) = &self.process {
            let pid = child.id();
            let _ = Command::new("kill")
                .arg("-CONT")
                .arg(pid.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl Drop for SoundController {
    fn drop(&mut self) {
        self.stop();
    }
}
