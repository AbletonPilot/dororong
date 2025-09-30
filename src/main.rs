mod cli;
mod display;
mod frames;

use crate::cli::{Cli, Commands};
use crate::display::{
    check_terminal_size, cleanup_terminal, display_animation_once, display_say_command,
    setup_terminal, spawn_exit_listener,
};
use crate::frames::{
    BOKBOK_FRAMES, BOKBOK_FRAMES_FAST, DANCE_FRAMES, DANCE_FRAMES_FAST, FRONTBACK_FRAMES,
    FRONTBACK_FRAMES_FAST, PANGPANG_FRAMES, PANGPANG_FRAMES_FAST, RUN_FRAMES, RUN_FRAMES_FAST,
    STATIC_FRAME, UPDOWN_FRAMES, UPDOWN_FRAMES_FAST,
};
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
            run_animation_with_speed(&BOKBOK_FRAMES, &BOKBOK_FRAMES_FAST, fast, text.as_deref())
                .await;
        }
        Commands::Pangpang { text, fast } => {
            run_animation_with_speed(
                &PANGPANG_FRAMES,
                &PANGPANG_FRAMES_FAST,
                fast,
                text.as_deref(),
            )
            .await;
        }
        Commands::Run { text, fast } => {
            run_animation_with_speed(&RUN_FRAMES, &RUN_FRAMES_FAST, fast, text.as_deref()).await;
        }
        Commands::Dance { text, fast } => {
            run_animation_with_speed(&DANCE_FRAMES, &DANCE_FRAMES_FAST, fast, text.as_deref())
                .await;
        }
        Commands::Frontback { text, fast } => {
            run_animation_with_speed(
                &FRONTBACK_FRAMES,
                &FRONTBACK_FRAMES_FAST,
                fast,
                text.as_deref(),
            )
            .await;
        }
        Commands::Updown { text, fast } => {
            run_animation_with_speed(&UPDOWN_FRAMES, &UPDOWN_FRAMES_FAST, fast, text.as_deref())
                .await;
        }
    }
}

async fn run_animation_with_speed(
    normal_frames: &crate::frames::AnimatedFrames,
    fast_frames: &crate::frames::AnimatedFrames,
    fast: bool,
    text: Option<&str>,
) {
    let frames = if fast { fast_frames } else { normal_frames };
    run_animation(frames, text).await;
}

async fn run_animation(frames: &crate::frames::AnimatedFrames, text: Option<&str>) {
    match check_terminal_size() {
        Ok(true) => {
            // Proceed
        }
        Ok(false) => {
            println!("Terminal is too small. Minimum 60x30 size is required.");
            return;
        }
        Err(e) => {
            eprintln!("terminal size check error: {e}");
            return;
        }
    }

    if let Err(e) = setup_terminal() {
        eprintln!("terminal setting error: {e}");
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
                eprintln!("animation error: {e}");
                break;
            }
        }
    }

    if let Err(e) = cleanup_terminal() {
        eprintln!("terminal cleanup error: {e}");
        std::process::exit(1);
    }
}
