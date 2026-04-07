use crate::timer::{TimerMode, TimerStatus};
use crate::utils::format_duration;
use indicatif::{ProgressBar, ProgressStyle};

pub struct UI {
    pb: ProgressBar,
}

impl UI {
    pub fn new(total_seconds: u64) -> Self {
        let pb = ProgressBar::new(total_seconds);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {percent}%")
                .unwrap()
                .progress_chars("━╾─"),
        );
        Self { pb }
    }

    pub fn update(&self, current_seconds: u64, mode: TimerMode, status: TimerStatus, cycle: u32) {
        let (mode_icon, mode_label, color) = match mode {
            TimerMode::Work => ("⏳", "WORK", "green"),
            TimerMode::Break => ("☕", "BREAK", "blue"),
        };

        let status_part = if status == TimerStatus::Paused {
            "⏸️ PAUSED"
        } else {
            "▶ RUNNING"
        };

        let bar_color = if status == TimerStatus::Paused {
            "yellow"
        } else {
            color
        };

        let time_str = format_duration(current_seconds);

        let msg = format!(
            "{} [{}] | {} | {} | 🔄 Cycle: {} | [p: pause | r: resume | q: quit]",
            mode_icon, mode_label, status_part, time_str, cycle
        );

        // Update style with correct colors and chars
        self.pb.set_style(
            ProgressStyle::with_template(&format!(
                "{{spinner:.{}}} {{msg}} [{{bar:40.{}.{bar_color}}}] {{percent}}%",
                color, bar_color
            ))
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .progress_chars("━╾─"),
        );

        self.pb
            .set_position(self.pb.length().unwrap() - current_seconds);
        self.pb.set_message(msg);
        self.pb.tick();
    }

    pub fn reset(&self, total_seconds: u64) {
        self.pb.set_length(total_seconds);
        self.pb.set_position(0);
    }

    pub fn finish(&self) {
        self.pb.finish();
    }
}
