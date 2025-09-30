use crate::frames::{AnimatedFrames, Frame};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout, Write};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration as TokioDuration};

const MIN_TERMINAL_WIDTH: u16 = 60;
const MIN_TERMINAL_HEIGHT: u16 = 30;

fn get_animation_dimensions(frame: &Frame) -> (u16, u16) {
    let width = frame.lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
    let height = frame.lines.len() as u16;
    (width, height)
}

pub fn create_speech_bubble_with_tail(text: &str, max_width: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + word.len() < max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    let bubble_width = lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0)
        .max(1);
    let mut bubble = Vec::new();

    // Top border
    bubble.push(format!("┌{}┐", "─".repeat(bubble_width + 2)));

    // Content lines
    for line in &lines {
        bubble.push(format!("│ {line:<bubble_width$} │"));
    }

    // Bottom border
    bubble.push(format!("└{}┘", "─".repeat(bubble_width + 2)));

    // Add tail pointing to dororong (left side) using / characters
    // Add connecting lines with / pointing toward dororong
    bubble.push("   /".to_string());
    bubble.push("  /".to_string());
    bubble.push(" /".to_string());

    bubble
}

pub fn display_say_command(frame: &Frame, text: &str) {
    let bubble = create_speech_bubble_with_tail(text, 30);
    let frame_lines = &frame.lines;

    let max_frame_height = frame_lines.len();
    let max_bubble_height = bubble.len();
    let max_height = max_frame_height.max(max_bubble_height);

    for i in 0..max_height {
        let frame_line = if i < frame_lines.len() {
            frame_lines[i]
        } else {
            ""
        };

        let bubble_line = if i < bubble.len() { &bubble[i] } else { "" };

        println!("{frame_line} {bubble_line}");
    }
}

pub fn check_terminal_size() -> io::Result<bool> {
    let (width, height) = terminal::size()?;
    Ok(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT)
}

pub fn setup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    terminal::disable_raw_mode()?;
    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}

pub fn spawn_exit_listener(exit_tx: broadcast::Sender<()>) {
    tokio::spawn(async move {
        loop {
            if let Ok(true) = tokio::task::spawn_blocking(|| {
                if event::poll(Duration::from_millis(10)).unwrap_or(false) {
                    if let Ok(Event::Key(key_event)) = event::read() {
                        match key_event.code {
                            KeyCode::Char('q') => return true,
                            KeyCode::Esc => return true,
                            KeyCode::Char('c')
                                if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                            {
                                return true
                            }
                            _ => {}
                        }
                    }
                }
                false
            })
            .await
            {
                let _ = exit_tx.send(());
                break;
            }
            sleep(TokioDuration::from_millis(10)).await;
        }
    });
}

pub async fn display_animation_once(
    frames: &AnimatedFrames,
    mut exit_rx: broadcast::Receiver<()>,
) -> io::Result<bool> {
    let (term_width, term_height) = terminal::size()?;

    // Get animation dimensions from the first frame
    let (animation_width, animation_height) = if let Some(first_frame) = frames.frames.first() {
        get_animation_dimensions(first_frame)
    } else {
        (0, 0)
    };

    for (current_frame, interval) in frames.iter() {
        // Check for exit signal at the start of each frame
        if exit_rx.try_recv().is_ok() {
            return Ok(true); // Exit requested
        }

        // Clear screen
        execute!(stdout(), Clear(ClearType::All))?;

        // Calculate display area
        let total_width = animation_width;
        let total_height = animation_height;

        let start_x = (term_width.saturating_sub(total_width)) / 2;
        let start_y = (term_height.saturating_sub(total_height)) / 2;

        // Display current frame
        for (i, line) in current_frame.lines.iter().enumerate() {
            execute!(stdout(), MoveTo(start_x, start_y + i as u16), Print(line))?;
        }

        stdout().flush()?;

        // Use tokio::select! to wait for either frame duration or exit signal
        let frame_duration = TokioDuration::from_millis(interval);

        tokio::select! {
            _ = sleep(frame_duration) => {
                // Frame duration completed, continue to next frame
            }
            _ = exit_rx.recv() => {
                return Ok(true); // Exit requested
            }
        }
    }

    Ok(false) // No exit requested
}
