extern crate clap;
extern crate image;

use std::path::Path;

use thumbnailer::{GenericThumbnail, Thumbnail};

use crate::cli::{get_matches, NAME_FILE_IN, NAME_FILE_OUT, read_commands};
use crate::commands::CommandTrait;

pub mod cli;
pub mod commands;

pub struct Commands {
    commands: Vec<Box<dyn CommandTrait>>,
}

//  .\target\debug\thumbnailer_cli.exe -h
//  v
//  .\target\debug\thumbnailer_cli.exe in.png out.png --flip_vertical  --brighten 15 --crop_ratio 4 3 --blur 34.151545 --unsharpen 56 14 --resize 89 4 true
//
//  .\target\debug\thumbnailer_cli.exe in.png out.png -c settings.conf -mmm watermark icon.png colored
//  .\target\debug\thumbnailer_cli.exe in.png out.png -c settings.conf -mmm --blur 6 --brighten 15
//  .\target\debug\thumbnailer_cli.exe in.png out.png --blur 6 --brighten 15

fn main() {
    let matches = get_matches();

    let file_in = String::from(matches.value_of(NAME_FILE_IN).unwrap());
    let file_out = String::from(matches.value_of(NAME_FILE_OUT).unwrap());
    let cmd_list = read_commands(matches);

    println!("Input file: {}", file_in);
    let path_buf = Path::new(&file_in).to_path_buf();
    let thumbnail = Thumbnail::load(path_buf);
    let mut image: &mut dyn GenericThumbnail = &mut thumbnail.unwrap();

    for i in 0..cmd_list.commands.len() {
        println!("{}", cmd_list.commands.get(i).unwrap().print());
        image = cmd_list.commands.get(i).unwrap().execute(image);
    }


    println!("Output file: {}", file_out);

    // TODO more program logic goes here...
}

