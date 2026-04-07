#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerMode {
    Work,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerStatus {
    Running,
    Paused,
    Finished,
}

pub struct PomodoroTimer {
    pub mode: TimerMode,
    pub status: TimerStatus,
    pub current_seconds: u64,
    pub initial_seconds: u64,
}

impl PomodoroTimer {
    pub fn new(mode: TimerMode, duration_minutes: u64) -> Self {
        let seconds = duration_minutes * 60;
        Self {
            mode,
            status: TimerStatus::Running,
            current_seconds: seconds,
            initial_seconds: seconds,
        }
    }

    pub fn tick(&mut self) {
        if self.status == TimerStatus::Running {
            if self.current_seconds > 0 {
                self.current_seconds -= 1;
            } else {
                self.status = TimerStatus::Finished;
            }
        }
    }

    pub fn pause(&mut self) {
        if self.status == TimerStatus::Running {
            self.status = TimerStatus::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == TimerStatus::Paused {
            self.status = TimerStatus::Running;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.status == TimerStatus::Finished
    }
}
