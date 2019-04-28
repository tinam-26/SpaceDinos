extern crate nfd;
extern crate serde;
extern crate structopt;
extern crate serde_json;

extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate touch_visualizer;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

mod level;
mod editor;

use level::Level;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    in_file: Option<PathBuf>,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    editor::editor(
        Level::open(opt.in_file.unwrap_or_default())
                .unwrap_or_default()
    );
    Ok(())
}
