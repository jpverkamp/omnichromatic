use clap::{Parser, Args, Subcommand};

// The top-level application
#[derive(Parser, Debug)]
#[clap(name = "omnichromatic", version = "0.2.0", author = "JP Verkamp")]
pub struct App {
    #[clap(flatten)]
    pub globals: GlobalArgs,

    #[clap(subcommand)]
    pub mode: Mode,
}

// Global arguments that apply to all subcommands
#[derive(Args, Debug)]
pub struct GlobalArgs {
    #[clap(long, short = 'd', name = "debug")]
    pub debug: bool,

    #[clap(long, short = 'x', name = "width", help = "Width of the generated image", default_value = "640")]
    pub width: usize,

    #[clap(long, short = 'y', name = "height", help = "Height of the generated image", default_value = "480")]
    pub height: usize,

    #[clap(long, short = 'c', name = "color-provider", help = "Provide the order of RGB colors")]
    pub color_provider: String,

    #[clap(long, short = 'p', name = "point-provider", help = "Provide the order of XY points")]
    pub point_provider: String,

    #[clap(long, short = 'o', name = "output", help = "Output file name", default_value = "omnichromatic.png")]
    pub output: String,
}

/// The specific subcommands that can be run
#[derive(Subcommand, Debug)]
pub enum Mode {
    #[clap(name = "render", about = "Render a single image")]
    Render,

    #[clap(name = "animate", about = "Render an animation of the image being built")]
    Animate { 
        #[clap(long, short = 'p', name = "ppf", help = "Pixels per frame")]
        ppf: usize,

        #[clap(long, short = 'f', name = "fps", help = "Frames per second")]
        fps: usize,
    },
}
