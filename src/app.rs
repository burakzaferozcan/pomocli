use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::time::Duration;
use tokio::time::interval;

use crate::prompt::AppConfig;
use crate::sound::SoundController;
use crate::timer::{PomodoroTimer, TimerMode};
use crate::ui::UI;

pub async fn run(config: AppConfig) -> io::Result<()> {
    enable_raw_mode()?;

    // Clear terminal before starting the timer UI
    execute!(
        io::stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let mut current_cycle = 1;
    let mut timer = PomodoroTimer::new(TimerMode::Work, config.work_duration);
    let ui = UI::new(timer.initial_seconds);
    let mut sound_ctrl = SoundController::new();

    // Start sound for the first WORK session
    sound_ctrl.start(&config.sound);

    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                timer.tick();
                ui.update(timer.current_seconds, timer.mode, timer.status, current_cycle);

                if timer.is_finished() {
                    match timer.mode {
                        TimerMode::Work => {
                            // Work finished
                            sound_ctrl.stop();
                            ui.finish();

                            // Check cycles
                            if config.cycles.is_some_and(|max| current_cycle >= max) {
                                println!("\nPomodoro finished! Great job.");
                                break;
                            }

                            println!("\n✔ Work finished! ☕ Break time");

                            // Start Break
                            timer = PomodoroTimer::new(TimerMode::Break, config.break_duration);
                            ui.reset(timer.initial_seconds);
                        }
                        TimerMode::Break => {
                            // Break finished
                            sound_ctrl.stop();
                            ui.finish();
                            current_cycle += 1;
                            println!("\n☕ Break finished! 🧠 Back to work");

                            // Start Work
                            timer = PomodoroTimer::new(TimerMode::Work, config.work_duration);
                            ui.reset(timer.initial_seconds);
                            sound_ctrl.start(&config.sound);
                        }
                    }
                }
            }
            res = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(50))) => {
                if let Ok(Ok(true)) = res {
                    let Ok(Event::Key(key)) = event::read() else {
                        continue;
                    };
                    match key.code {
                        KeyCode::Char('q') => {
                            println!("\nQuitting...");
                            break;
                        }
                        KeyCode::Char('p') => {
                            timer.pause();
                            sound_ctrl.pause();
                        }
                        KeyCode::Char('r') => {
                            timer.resume();
                            sound_ctrl.resume();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
