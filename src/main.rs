mod cli;
mod prompt;
mod timer;
mod sound;
mod ui;
mod app;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Check if mpv is available
    if !utils::is_mpv_available() {
        eprintln!("Error: mpv is not installed or not in PATH.");
        eprintln!("Please install it: sudo apt install mpv (or equivalent).");
        std::process::exit(1);
    }

    let config = prompt::run_prompts();
    
    if let Err(e) = app::run(config).await {
        eprintln!("Application error: {}", e);
    }

    Ok(())
}
