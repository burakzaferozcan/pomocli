use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    execute, cursor,
};
use tokio::time::interval;

use crate::prompt::AppConfig;
use crate::timer::{PomodoroTimer, TimerMode};
use crate::ui::UI;
use crate::sound::SoundController;

pub async fn run(config: AppConfig) -> io::Result<()> {
    enable_raw_mode()?;

    // Clear terminal before starting the timer UI
    execute!(io::stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

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
                            if let Some(max_cycles) = config.cycles {
                                if current_cycle >= max_cycles {
                                    println!("\nPomodoro finished! Great job.");
                                    break;
                                }
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
                    if let Ok(Event::Key(key)) = event::read() {
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
    }

    disable_raw_mode()?;
    Ok(())
}
