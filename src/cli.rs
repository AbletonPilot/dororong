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
    Say {
        text: String,
    },
    /// Bokbok animation
    #[command(name = "bokbok")]
    Bokbok {
        text: Option<String>,
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Pangpang animation  
    #[command(name = "pangpang")]
    Pangpang {
        text: Option<String>,
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Run animation
    #[command(name = "run")]
    Run {
        text: Option<String>,
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Dance animation
    #[command(name = "dance")]
    Dance {
        text: Option<String>,
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Frontback animation
    #[command(name = "frontback")]
    Frontback {
        text: Option<String>,
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
    /// Updown animation
    #[command(name = "updown")]
    Updown {
        text: Option<String>,
        /// Fast version of animation
        #[arg(short, long)]
        fast: bool,
    },
}
