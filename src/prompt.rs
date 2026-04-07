use dialoguer::{theme::ColorfulTheme, Input, Select};

pub enum SoundType {
    BuiltIn(String),
    Youtube(String),
    None,
}

pub struct AppConfig {
    pub work_duration: u64,
    pub break_duration: u64,
    pub cycles: Option<u32>,
    pub sound: SoundType,
}

pub fn run_prompts() -> AppConfig {
    let theme = ColorfulTheme::default();

    println!("🧠 Welcome to pomocli!");

    let work_duration: u64 = Input::with_theme(&theme)
        .with_prompt("Work duration (minutes)")
        .default(25)
        .interact_text()
        .unwrap_or(25);

    let break_duration: u64 = Input::with_theme(&theme)
        .with_prompt("Break duration (minutes)")
        .default(5)
        .interact_text()
        .unwrap_or(5);

    let cycles_input: String = Input::with_theme(&theme)
        .with_prompt("Number of cycles (leave empty for infinite)")
        .allow_empty(true)
        .interact_text()
        .unwrap_or_default();

    let cycles = if cycles_input.is_empty() {
        None
    } else {
        cycles_input.parse::<u32>().ok()
    };

    let sound_options = vec!["None", "Rain (Built-in)", "Fireplace (Built-in)", "Cafe (Built-in)", "YouTube Link"];
    let selection = Select::with_theme(&theme)
        .with_prompt("Select background sound")
        .items(&sound_options)
        .default(0)
        .interact()
        .unwrap_or(0);

    let sound = match selection {
        0 => SoundType::None,
        1 => SoundType::BuiltIn("rain.mp3".to_string()),
        2 => SoundType::BuiltIn("fireplace.mp3".to_string()),
        3 => SoundType::BuiltIn("cafe.mp3".to_string()),
        4 => {
            let link: String = Input::with_theme(&theme)
                .with_prompt("Enter YouTube link (e.g., https://www.youtube.com/watch?v=...)")
                .validate_with(|input: &String| {
                    if input.starts_with("https://") {
                        Ok(())
                    } else {
                        Err("Link must start with https://")
                    }
                })
                .interact_text()
                .unwrap_or_default();
            if link.is_empty() { SoundType::None } else { SoundType::Youtube(link) }
        },
        _ => SoundType::None,
    };

    AppConfig {
        work_duration,
        break_duration,
        cycles,
        sound,
    }
}
