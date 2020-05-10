extern crate clap;
extern crate image;

use std::ffi::OsStr;
use std::path::Path;

use thumbnailer::{GenericThumbnail, Target, Thumbnail};
use thumbnailer::target::TargetFormat;

use crate::cli::{get_matches, NAME_FILE_IN, NAME_FILE_OUT, read_commands};
use crate::commands::Command;

pub mod cli;
pub mod commands;

/// Representation of the command-list as a struct
pub struct Commands {
    /// Contains the implementors of `Command` to apply a list of operations, which are provided by `thumbnailer`, on the supplied image(s)
    commands: Vec<Box<dyn Command>>,
}

/// Main logic of the thumbnailer command line interface (`thumbnailer_cli`)
///
/// Run `thumbnailer_cli.exe -h` to view the help and learn about its functionality.
/// Run `thumbnailer_cli.exe -V` to print the version information.
//  .\target\debug\thumbnailer_cli.exe -h
//  .\target\debug\thumbnailer_cli.exe C:\Users\p372094\IdeaProjects\thumbnailer_cli\img\test.JPG C:\Users\p372094\IdeaProjects\thumbnailer_cli\img\result.png --blur 6 --brighten 15
fn main() {
    let matches = get_matches();

    let file_in = String::from(matches.value_of(NAME_FILE_IN).unwrap());
    let file_out = String::from(matches.value_of(NAME_FILE_OUT).unwrap());
    let cmd_list = read_commands(matches);

    println!("Input file: {}", file_in);
    let mut image = Thumbnail::load(Path::new(&file_in).to_path_buf()).unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: failed to load the image with the supplied path ←‼",
            NAME_FILE_IN
        )
    });

    for i in 0..cmd_list.commands.len() {
        println!("{}", cmd_list.commands.get(i).unwrap().print());
        cmd_list.commands.get(i).unwrap().execute(&mut image);
    }

    println!("Output file: {}", file_out);
    let path = Path::new(&file_out);
    let format = match path.extension().and_then(OsStr::to_str) {
        Some("png") => TargetFormat::Png,
        Some("tiff") => TargetFormat::Tiff,
        Some("bmp") => TargetFormat::Bmp,
        Some("gif") => TargetFormat::Gif,
        _ => TargetFormat::Jpeg,
    };
    let target = Target::new(format, Path::new(&file_out).to_path_buf());
    image.apply_store(&target);
}
