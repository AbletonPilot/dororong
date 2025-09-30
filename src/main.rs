mod cli;
mod frames;
mod display;

use crate::cli::{Cli, Commands};
use crate::frames::{STATIC_FRAME, BOKBOK_FRAMES, BOKBOK_FRAMES_FAST, 
                   PANGPANG_FRAMES, PANGPANG_FRAMES_FAST, 
                   RUN_FRAMES, RUN_FRAMES_FAST};
use crate::display::{display_say_command, display_animation_once, check_terminal_size, setup_terminal, cleanup_terminal, spawn_exit_listener};
use clap::Parser;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say { text } => {
            display_say_command(&STATIC_FRAME, &text);
        }
        Commands::Bokbok { text, fast } => {
            let frames = if fast {
                &*BOKBOK_FRAMES_FAST
            } else {
                &*BOKBOK_FRAMES
            };
            
            run_animation(frames, text.as_deref()).await;
        }
        Commands::Pangpang { text, fast } => {
            let frames = if fast {
                &*PANGPANG_FRAMES_FAST
            } else {
                &*PANGPANG_FRAMES
            };
            
            run_animation(frames, text.as_deref()).await;
        }
        Commands::Run { text, fast } => {
            let frames = if fast {
                &*RUN_FRAMES_FAST
            } else {
                &*RUN_FRAMES
            };
            
            run_animation(frames, text.as_deref()).await;
        }
    }
}

async fn run_animation(frames: &crate::frames::AnimatedFrames, text: Option<&str>) {
    if !check_terminal_size().unwrap_or(false) {
        println!("your terminal is too small for dororong");
        return;
    }

    if let Err(e) = setup_terminal() {
        eprintln!("Error setting up terminal: {e}");
        std::process::exit(1);
    }

    // Create broadcast channel for exit signals
    let (exit_tx, _) = broadcast::channel::<()>(1);

    // Spawn the exit listener
    spawn_exit_listener(exit_tx.clone());

    loop {
        let exit_rx = exit_tx.subscribe();
        match display_animation_once(frames, text, exit_rx).await {
            Ok(should_exit) => {
                if should_exit {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error during animation: {e}");
                break;
            }
        }
    }

    if let Err(e) = cleanup_terminal() {
        eprintln!("Error cleaning up terminal: {e}");
        std::process::exit(1);
    }
}

