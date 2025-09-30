use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dororong")]
#[command(about = "Dororong will be dancing for you!")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Static display with text
    #[command(name = "say")]
    Say { text: String },
    /// Bokbok animation
    #[command(name = "bokbok")]
    Bokbok {
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Pangpang animation  
    #[command(name = "pangpang")]
    Pangpang {
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Run animation
    #[command(name = "run")]
    Run {
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Dance animation
    #[command(name = "dance")]
    Dance {
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Frontback animation
    #[command(name = "frontback")]
    Frontback {
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Updown animation
    #[command(name = "updown")]
    Updown {
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
}
