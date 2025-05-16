use rdev::{listen, Event, EventType, Key};
use chrono::Local;
use std::fs::{create_dir_all, File};
use std::io::{Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use dirs::home_dir;
use copypasta::{ClipboardContext, ClipboardProvider};

fn get_base_path() -> std::path::PathBuf {
    let date = Local::now().format("%Y/%m-%d").to_string();
    let mut path = home_dir().unwrap();
    path.push("SnapNote");
    path.push(date);
    path
}

fn save_text_note(content: &str, prefix: &str) {
    let now = Local::now();
    let filename = format!("{}_{}.txt", prefix, now.format("%H-%M-%S"));
    let dir = get_base_path();
    create_dir_all(&dir).unwrap();
    let filepath = dir.join(filename);
    let mut file = File::create(filepath).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    println!("Saved {} successfully!", prefix);
}

fn capture_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    if let Ok(contents) = ctx.get_contents() {
        save_text_note(&contents, "code");
    }
}

fn take_screenshot() {
    let dir = get_base_path();
    create_dir_all(&dir).unwrap();
    let now = Local::now();
    let filename = format!("screenshot_{}.png", now.format("%H-%M-%S"));
    let filepath = dir.join(filename);

    #[cfg(target_os = "macos")]
    {
        Command::new("screencapture")
            .arg(filepath.to_str().unwrap())
            .output()
            .expect("Failed to take screenshot");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("scrot")
            .arg(filepath.to_str().unwrap())
            .output()
            .expect("Failed to take screenshot");
    }

    println!("Screenshot saved!");
}

fn listen_hotkeys() {
    let mut keys_pressed = vec![];

    if let Err(error) = listen(move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                keys_pressed.push(key);

                if keys_pressed.contains(&Key::ControlLeft) && keys_pressed.contains(&Key::ShiftLeft) {
                    match key {
                        Key::KeyS => {
                            println!("Hotkey: Screenshot");
                            take_screenshot();
                            keys_pressed.clear();
                        }
                        Key::KeyC => {
                            println!("Hotkey: Clipboard Code");
                            capture_clipboard();
                            keys_pressed.clear();
                        }
                        Key::KeyN => {
                            println!("Hotkey: Text Note");
                            save_text_note("Quick note placeholder text.", "note");
                            keys_pressed.clear();
                        }
                        _ => {}
                    }
                }
            }
            EventType::KeyRelease(key) => {
                keys_pressed.retain(|&k| k != key);
            }
            _ => {}
        }
    }) {
        println!("Error: {:?}", error)
    }
}

#[tokio::main]
async fn main() {
    println!("SnapNote daemon is running...");
    thread::sleep(Duration::from_millis(500));
    listen_hotkeys();
}
