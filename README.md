# 🧠 pomocli

A sleek, interactive Pomodoro CLI with ambient sound support and YouTube integration. Stay focused and productive with a beautiful terminal interface.

## ✨ Features

- ⏱️ **Interactive Pomodoro Timer**: Easily set work and break durations.
- 🔄 **Customizable Cycles**: Run a specific number of cycles or go infinite.
- 🎵 **Ambient Sounds**: Built-in support for Rain, Fireplace, and Cafe sounds.
- 📺 **YouTube Integration**: Play any YouTube audio as your background focus music.
- 🎨 **Beautiful UI**: Colored progress bars, spinners, and clear status indicators.
- ⏸️ **Pause/Resume**: Full control over your timer and music.

## 🛠️ Requirements

`pomocli` uses `mpv` for audio playback. Ensure the following are installed:

- **mpv**: Required for all audio playback.
- **yt-dlp** (or `youtube-dl`): Required if you want to use YouTube links.
- **kill**: Used for pausing/resuming the audio process (standard on most Linux systems).

### Install Requirements (Linux/Arch)
```bash
sudo pacman -S mpv yt-dlp
```

### Install Requirements (macOS)
```bash
brew install mpv yt-dlp
```

## 🚀 Installation

Install `pomocli` directly from [crates.io](https://crates.io/crates/pomocli) (once published):

```bash
cargo install pomocli
```

Or build from source:

```bash
git clone https://github.com/burakzaferozcan/pomocli.git
cd pomocli
cargo install --path .
```

## 📖 Usage

Simply run:

```bash
pomocli
```

Follow the interactive prompts to:
1. Set your **Work duration** (default: 25 min).
2. Set your **Break duration** (default: 5 min).
3. Set the **Number of cycles** (leave empty for infinite).
4. Select a **Background sound** (None, Rain, Fireplace, Cafe, or YouTube Link).

### Controls during session:
- `p`: Pause timer and sound.
- `r`: Resume timer and sound.
- `q`: Quit application.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
