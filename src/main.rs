use std::collections::HashSet;
use clap::{Parser, Args, Subcommand};
use anyhow::Result;

mod provider;
use provider::Provider;

// The top-level application
#[derive(Parser, Debug)]
#[clap(name = "omnichromatic", version = "0.1.0", author = "JP Verkamp")]
struct App {
    #[clap(flatten)]
    globals: GlobalArgs,

    #[clap(subcommand)]
    mode: Mode,
}

// Global arguments that apply to all subcommands
#[derive(Args, Debug)]
struct GlobalArgs {
    #[clap(long, short = 'd', name = "debug")]
    debug: bool,

    #[clap(long, short = 'x', name = "width", help = "Width of the generated image")]
    width: u32,

    #[clap(long, short = 'y', name = "height", help = "Height of the generated image")]
    height: u32,

    #[clap(long, short = 'c', name = "color-provider", help = "Provide the order of RGB colors")]
    color_provider: String,

    #[clap(long, short = 'p', name = "point-provider", help = "Provide the order of XY points")]
    point_provider: String,    
}

/// The specific subcommands that can be run
#[derive(Subcommand, Debug)]
enum Mode {
    #[clap(name = "render", about = "Render a single image")]
    Render,

    #[clap(name = "animate", about = "Render an animation of the image being built")]
    Animate,
}


fn main() -> Result<()> {
    env_logger::init();

    let args = App::parse();
    let width = args.globals.width;
    let height = args.globals.height;
    let size = width * height;

    let mut image = image::RgbImage::new(width, height);

    let mut color_provider = Provider::new(args.globals.color_provider)?;
    let mut point_provider = Provider::new(args.globals.point_provider)?;
    
    // Send initialization to providers, don't expect a response
    color_provider.send(format!("set-size {width} {height}"))?;
    point_provider.send(format!("set-size {width} {height}"))?;

    // Track points and colors used in case the providers mess up
    let mut colors_used = HashSet::new();
    let mut points_used = HashSet::new();

    // Generate enough points, this will assume that the providers behave
    for i in 0..size {
        let point = point_provider.ask(format!("get-point"), "point")?;
        assert!(point.len() == 2);
        let x = point[0].parse::<u32>()?;
        let y = point[1].parse::<u32>()?;
        assert!(points_used.contains(&(x, y)) == false);
        
        let color = color_provider.ask(format!("get-color"), "color")?;
        assert!(color.len() == 3);
        let r = color[0].parse::<u8>()?;
        let g = color[1].parse::<u8>()?;
        let b = color[2].parse::<u8>()?;
        assert!(colors_used.contains(&(r, g, b)) == false);

        if i % 1000 == 0 {
            log::info!("[{i} of {size}] {r},{g},{b} @ {x},{y}");
        } else {
            log::debug!("[{i} of {size}] {r},{g},{b} @ {x},{y}");
        }

        let pixel = image.get_pixel_mut(x as u32, y as u32);
        *pixel = image::Rgb::from([r, g, b]);
        
        points_used.insert((x, y));
        colors_used.insert((r, g, b));
    }

    // TODO: arg this
    let filename = "output.png";
    image.save(filename)?;

    Ok(())
}
