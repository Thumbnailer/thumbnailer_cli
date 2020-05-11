use std::path::Path;

use clap::{App, Arg, ArgMatches};
use thumbnailer::{
    BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize, Rotation, Thumbnail,
};

use crate::commands::{
    CmdBlur, CmdBrighten, CmdCombine, CmdContrast, CmdCrop, CmdExif, CmdFlip, CmdHuerotate,
    CmdInvert, CmdResize, CmdResizeFilter, CmdRotate, CmdText, CmdUnsharpen, Command,
};
use crate::Commands;

pub const NAME_FILE_IN: &str = "INPUT_PATH";
pub const NAME_FILE_OUT: &str = "OUTPUT_PATH";

const ARG_BLUR: &str = "blur";
const ARG_BRIGHTEN: &str = "brighten";
const ARG_CONTRAST: &str = "contrast";
const ARG_COMBINE_TL: &str = "combine_tl";
const ARG_COMBINE_TR: &str = "combine_tr";
const ARG_COMBINE_BL: &str = "combine_bl";
const ARG_COMBINE_BR: &str = "combine_br";
const ARG_CROP_BOX: &str = "crop_box";
const ARG_CROP_RATIO: &str = "crop_ratio";
const ARG_EXIF: &str = "exif";
const ARG_FLIP_HORIZONTAL: &str = "flip_horizontal";
const ARG_FLIP_VERTICAL: &str = "flip_vertical";
const ARG_HUEROTATE: &str = "huerotate";
const ARG_INVERT: &str = "invert";
const ARG_RESIZE: &str = "resize";
const ARG_RESIZE_N: &str = "resize_n";
const ARG_RESIZE_T: &str = "resize_t";
const ARG_RESIZE_C: &str = "resize_c";
const ARG_RESIZE_G: &str = "resize_g";
const ARG_RESIZE_L: &str = "resize_l";
const ARG_ROTATE90: &str = "rotate90";
const ARG_ROTATE180: &str = "rotate180";
const ARG_ROTATE270: &str = "rotate270";
const ARG_TEXT_TL: &str = "text_tl";
const ARG_TEXT_TR: &str = "text_tr";
const ARG_TEXT_BL: &str = "text_bl";
const ARG_TEXT_BR: &str = "text_br";
const ARG_UNSHARPEN: &str = "unsharpen";
const ARG_PRESET: &str = "preset";

const GROUP_COMBINE: &str = "Combine";
const GROUP_CROP: &str = "Crop";
const GROUP_FLIP: &str = "Flip";
const GROUP_RESIZE: &str = "Resize";
const GROUP_ROTATE: &str = "Rotate";
const GROUP_TEXT: &str = "Text";

const VAL_COMBINE: [&str; 3] = ["IMAGE_PATH", "x_offset", "y_offset"];
const VAL_RESIZE: [&str; 3] = ["nwidth", "nheight", "exact"];
const VAL_TEXT: [&str; 3] = ["text", "x_offset", "y_offset"];

const PRESETS: [&str; 3] = ["app_copyright", "full_hd", "background"];

/// This function bundles the definition of the command line arguments as provided by clap
///
/// Extract from the documentation of `clap::App`:
/// "Used to create a representation of a command line program and all possible command line arguments.
/// Application settings are set using the "builder pattern" with the App::get_matches family of methods being the terminal methods that starts the runtime-parsing process.
/// These methods then return information about the user supplied arguments (or lack there of)."
/// (c) https://docs.rs/clap/2.33.0/clap/
///
/// Returns a new `clap::ArgMatches` struct
pub fn get_matches() -> clap::ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name(NAME_FILE_IN)
            .index(1)
            .help("Sets the input file as path to use.")
            .required(true))

        .arg(Arg::with_name(NAME_FILE_OUT)
            .index(2)
            .default_value("thumbnail.png")
            .help("Sets the output file as path to save.")
            .required(true))

        .arg(Arg::with_name(ARG_BLUR)
            .long(ARG_BLUR)
            .value_name("sigma")
            .help("Performs a Gaussian blur on the supplied image(s). sigma as f32 is a measure of how much to blur by.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_BRIGHTEN)
            .long(ARG_BRIGHTEN)
            .value_name("value")
            .help("Brightens the supplied image(s). value as i32 is the amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_CONTRAST)
            .long(ARG_CONTRAST)
            .value_name("contrast")
            .help("Adjusts the contrast of the supplied image(s). contrast as f32 is the amount to adjust the contrast by. Negative values decrease the contrast and positive values increase the contrast.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_COMBINE_TL)
            .long(ARG_COMBINE_TL)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_TR)
            .long(ARG_COMBINE_TR)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP RIGHT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_BL)
            .long(ARG_COMBINE_BL)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_BR)
            .long(ARG_COMBINE_BR)
            .value_names(&VAL_COMBINE)
            .group(GROUP_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM RIGHT corner of the photo.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_CROP_BOX)
            .long(ARG_CROP_BOX)
            .value_name("x")
            .value_name("y")
            .value_name("width")
            .value_name("height")
            .group(GROUP_CROP)
            .help("Crops the supplied image(s) to the given width as u32 and height as u32. x as u32 is the horizontal and y as u32 the vertical offset.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_CROP_RATIO)
            .long(ARG_CROP_RATIO)
            .value_name("x_ratio")
            .value_name("y_ratio")
            .group(GROUP_CROP)
            .help("Crops the supplied image(s) to the given ratio 'x_ratio:y_ratio'. x_ratio as f32 is representing the horizontal and y_ratio as f32 the vertical.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_EXIF)
            .long(ARG_EXIF)
            .help("The metadata of the target file is taken from the source file and not removed as usual."))

        .arg(Arg::with_name(ARG_FLIP_HORIZONTAL)
            .long(ARG_FLIP_HORIZONTAL)
            .group(GROUP_FLIP)
            .help("Flip the supplied image(s) horizontally."))
        .arg(Arg::with_name(ARG_FLIP_VERTICAL)
            .long(ARG_FLIP_VERTICAL)
            .group(GROUP_FLIP)
            .help("Flip the supplied image(s) vertically."))

        .arg(Arg::with_name(ARG_HUEROTATE)
            .long(ARG_HUEROTATE)
            .value_name("value")
            .group(GROUP_ROTATE)
            .help("Hue rotate the supplied image(s). value as i32 is the degrees to rotate each pixel by. 0 and 360 do nothing, the rest rotates by the given degree value.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_INVERT)
            .long(ARG_INVERT)
            .help("Invert each pixel within the supplied image(s)."))

        .arg(Arg::with_name(ARG_RESIZE)
            .long(ARG_RESIZE)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_N)
            .long(ARG_RESIZE_N)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Nearest is the used filter (Nearest Neighbor Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_T)
            .long(ARG_RESIZE_T)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Triangle is the used filter (Linear Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_C)
            .long(ARG_RESIZE_C)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. CatmullRom is the used filter (Cubic Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_G)
            .long(ARG_RESIZE_G)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Gaussian is the used filter (Gaussian Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_L)
            .long(ARG_RESIZE_L)
            .value_names(&VAL_RESIZE)
            .group(GROUP_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Lanczos3 is the used filter (Lanczos with window 3). To resize only by one dimension, set the other to 0.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_ROTATE90)
            .short("r")
            .long(ARG_ROTATE90)
            .group(GROUP_ROTATE)
            .help("Rotate the supplied image(s) 90 degrees clockwise.")
            .multiple(true))
        .arg(Arg::with_name(ARG_ROTATE180)
            .long(ARG_ROTATE180)
            .group(GROUP_ROTATE)
            .help("Rotate the supplied image(s) 180 degrees clockwise."))
        .arg(Arg::with_name(ARG_ROTATE270)
            .long(ARG_ROTATE270)
            .group(GROUP_ROTATE)
            .help("Rotate the supplied image(s) 270 degrees clockwise."))

        .arg(Arg::with_name(ARG_TEXT_TL)
            .long(ARG_TEXT_TL)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_TR)
            .long(ARG_TEXT_TR)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP RIGHT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_BL)
            .long(ARG_TEXT_BL)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_BR)
            .long(ARG_TEXT_BR)
            .value_names(&VAL_TEXT)
            .group(GROUP_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM RIGHT corner of the photo.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_UNSHARPEN)
            .long(ARG_UNSHARPEN)
            .value_name("sigma")
            .value_name("threshold")
            .help("Performs an unsharpen mask on the supplied image(s). sigma as f32 is the amount to unsharpen the image by. threshold as i32 controls the minimal brightness change or how far apart adjacent tonal values have to be before the filter does anything.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_PRESET)
            .long(ARG_PRESET)
            .value_name("name")
            .possible_values(&PRESETS)
            .help("Performs predefined commands in a given order, based on the preset, which was chosen.")
            .takes_value(true))

        .get_matches()
    //.get_matches_from(vec![env!("CARGO_PKG_NAME"), "in.png" ,"--combine_tl", "C:\\Users\\p372094\\IdeaProjects\\thumbnailer_cli\\img\\test.JPG", "40", "30"])
}

/// This function is parsing the given values for all supplied arguments
///
/// Returns a new `Commands` struct
///
/// # Arguments
///
/// * `matches` - The `ArgMatches` struct from clap, containing the provided arguments
///
/// # Examples
/// ```
/// use clap::{App, Arg, ArgMatches};
/// pub use thumbnailer::{BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize};
///
/// use crate::commands::{CmdBlur, CmdBrighten, CmdCombine, CmdContrast, CmdCrop, CmdExif, CmdFlip, CmdHuerotate, CmdInvert, CmdResize, CmdResizeFilter, CmdText, CmdUnsharpen};
/// use crate::Commands;
///
/// pub const NAME_FILE_IN: &str = "INPUT_PATH";
/// pub const NAME_FILE_OUT: &str = "OUTPUT_PATH";
///
/// const ARG_BLUR: &str = "blur";
/// const ARG_BRIGHTEN: &str = "brighten";
/// const ARG_CONTRAST: &str = "contrast";
/// const ARG_COMBINE_TL: &str = "combine_tl";
/// const ARG_COMBINE_TR: &str = "combine_tr";
/// const ARG_COMBINE_BL: &str = "combine_bl";
/// const ARG_COMBINE_BR: &str = "combine_br";
/// const ARG_CROP_BOX: &str = "crop_box";
/// const ARG_CROP_RATIO: &str = "crop_ratio";
/// const ARG_EXIF: &str = "exif";
/// const ARG_FLIP_HORIZONTAL: &str = "flip_horizontal";
/// const ARG_FLIP_VERTICAL: &str = "flip_vertical";
/// const ARG_HUEROTATE: &str = "huerotate";
/// const ARG_INVERT: &str = "invert";
/// const ARG_RESIZE: &str = "resize";
/// const ARG_RESIZE_N: &str = "resize_n";
/// const ARG_RESIZE_T: &str = "resize_t";
/// const ARG_RESIZE_C: &str = "resize_c";
/// const ARG_RESIZE_G: &str = "resize_g";
/// const ARG_RESIZE_L: &str = "resize_l";
/// const ARG_ROTATE90: &str = "rotate90";
/// const ARG_ROTATE180: &str = "rotate180";
/// const ARG_ROTATE270: &str = "rotate270";
/// const ARG_TEXT_TL: &str = "text_tl";
/// const ARG_TEXT_TR: &str = "text_tr";
/// const ARG_TEXT_BL: &str = "text_bl";
/// const ARG_TEXT_BR: &str = "text_br";
/// const ARG_UNSHARPEN: &str = "unsharpen";
///
/// let matches = App::new(env!("CARGO_PKG_NAME"))
///     .arg(Arg::with_name(ARG_BLUR)
///         .long(ARG_BLUR)
///         .value_name("sigma")
///         .help("Performs a Gaussian blur on the supplied image(s). sigma as f32 is a measure of how much to blur by.")
///         .takes_value(true))
///     .get_matches_from(vec![env!("CARGO_PKG_NAME"), "--blur", "5.8"]);
///
/// let cmd_list = read_commands(matches);
/// for i in 0..cmd_list.commands.len() {
///     println!("{}", cmd_list.commands.get(i).unwrap().print());
/// }
/// ```
pub fn read_commands(matches: ArgMatches<'static>) -> Commands {
    let mut cmd_list = Commands { commands: vec![] };

    if matches.is_present(ARG_BLUR) {
        let index = matches.index_of(ARG_BLUR).unwrap() as u32;
        let sigma = String::from(matches.value_of(ARG_BLUR).unwrap())
            .parse::<f32>()
            .unwrap_or_else(|_| {
                panic!(
                    "‼→ ERROR in {}: sigma expects f32, got {} ←‼",
                    ARG_BLUR,
                    matches.value_of(ARG_BLUR).unwrap()
                )
            });

        cmd_list.commands.push(Box::new(CmdBlur::new(index, sigma)));
    }

    if matches.is_present(ARG_BRIGHTEN) {
        let index = matches.index_of(ARG_BRIGHTEN).unwrap() as u32;
        let value = String::from(matches.value_of(ARG_BRIGHTEN).unwrap())
            .parse::<i32>()
            .unwrap_or_else(|_| {
                panic!(
                    "‼→ ERROR in {}: value expects i32, got {} ←‼",
                    ARG_BRIGHTEN,
                    matches.value_of(ARG_BRIGHTEN).unwrap()
                )
            });

        cmd_list
            .commands
            .push(Box::new(CmdBrighten::new(index, value)));
    }

    if matches.is_present(ARG_CONTRAST) {
        let index = matches.index_of(ARG_CONTRAST).unwrap() as u32;
        let value = String::from(matches.value_of(ARG_CONTRAST).unwrap())
            .parse::<f32>()
            .unwrap_or_else(|_| {
                panic!(
                    "‼→ ERROR in {}: value expects f32, got {} ←‼",
                    ARG_CONTRAST,
                    matches.value_of(ARG_CONTRAST).unwrap()
                )
            });

        cmd_list
            .commands
            .push(Box::new(CmdContrast::new(index, value)));
    }

    if matches.is_present(ARG_COMBINE_TL) {
        cmd_list.commands.push(Box::new(create_cmd_combine(
            matches.clone(),
            ARG_COMBINE_TL,
        )));
    }
    if matches.is_present(ARG_COMBINE_TR) {
        cmd_list.commands.push(Box::new(create_cmd_combine(
            matches.clone(),
            ARG_COMBINE_TR,
        )));
    }
    if matches.is_present(ARG_COMBINE_BL) {
        cmd_list.commands.push(Box::new(create_cmd_combine(
            matches.clone(),
            ARG_COMBINE_BL,
        )));
    }
    if matches.is_present(ARG_COMBINE_BR) {
        cmd_list.commands.push(Box::new(create_cmd_combine(
            matches.clone(),
            ARG_COMBINE_BR,
        )));
    }

    if matches.is_present(ARG_CROP_BOX) {
        let index = matches.index_of(ARG_CROP_BOX).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_CROP_BOX).unwrap().collect();

        let x = values[0].parse::<u32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: x expects u32, got {} ←‼",
                ARG_CROP_BOX, values[0]
            )
        });
        let y = values[1].parse::<u32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: y expects u32, got {} ←‼",
                ARG_CROP_BOX, values[1]
            )
        });
        let width = values[2].parse::<u32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: width expects u32, got {} ←‼",
                ARG_CROP_BOX, values[2]
            )
        });
        let height = values[3].parse::<u32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: height expects u32, got {} ←‼",
                ARG_CROP_BOX, values[3]
            )
        });

        cmd_list.commands.push(Box::new(CmdCrop::new(
            index,
            Crop::Box(x, y, width, height),
        )));
    }

    if matches.is_present(ARG_CROP_RATIO) {
        let index = matches.index_of(ARG_CROP_RATIO).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_CROP_RATIO).unwrap().collect();

        let x_ratio = values[0].parse::<f32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: x_ratio expects f32, got {} ←‼",
                ARG_CROP_RATIO, values[0]
            )
        });
        let y_ratio = values[1].parse::<f32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: y_ratio expects f32, got {} ←‼",
                ARG_CROP_RATIO, values[1]
            )
        });

        cmd_list
            .commands
            .push(Box::new(CmdCrop::new(index, Crop::Ratio(x_ratio, y_ratio))));
    }

    if matches.is_present(ARG_EXIF) {
        let index = matches.index_of(ARG_EXIF).unwrap() as u32;

        cmd_list
            .commands
            .push(Box::new(CmdExif::new(index, Exif::Keep)));
    }

    if matches.is_present(ARG_FLIP_HORIZONTAL) {
        let index = matches.index_of(ARG_FLIP_HORIZONTAL).unwrap() as u32;

        cmd_list
            .commands
            .push(Box::new(CmdFlip::new(index, Orientation::Horizontal)));
    }

    if matches.is_present(ARG_FLIP_VERTICAL) {
        let index = matches.index_of(ARG_FLIP_VERTICAL).unwrap() as u32;

        cmd_list
            .commands
            .push(Box::new(CmdFlip::new(index, Orientation::Vertical)));
    }

    if matches.is_present(ARG_HUEROTATE) {
        let index = matches.index_of(ARG_HUEROTATE).unwrap() as u32;
        let degree = String::from(matches.value_of(ARG_HUEROTATE).unwrap())
            .parse::<i32>()
            .unwrap_or_else(|_| {
                panic!(
                    "‼→ ERROR in {}: degree expects i32, got {} ←‼",
                    ARG_HUEROTATE,
                    matches.value_of(ARG_HUEROTATE).unwrap()
                )
            });

        cmd_list
            .commands
            .push(Box::new(CmdHuerotate::new(index, degree)));
    }

    if matches.is_present(ARG_INVERT) {
        let index = matches.index_of(ARG_INVERT).unwrap() as u32;

        cmd_list.commands.push(Box::new(CmdInvert::new(index)));
    }

    if matches.is_present(ARG_RESIZE) {
        let index = matches.index_of(ARG_RESIZE).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_RESIZE).unwrap().collect();

        let width = values[0].parse::<u32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: width expects u32, got {} ←‼",
                ARG_RESIZE, values[0]
            )
        });
        let height = values[1].parse::<u32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: height expects u32, got {} ←‼",
                ARG_RESIZE, values[1]
            )
        });
        let exact = values[2].parse::<bool>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: exact expects bool, got {} ←‼",
                ARG_RESIZE, values[2]
            )
        });

        let size;
        if height == 0 {
            size = Resize::Width(width);
        } else if width == 0 {
            size = Resize::Height(height);
        } else if !exact {
            size = Resize::BoundingBox(width, height);
        } else {
            size = Resize::ExactBox(width, height);
        }
        cmd_list
            .commands
            .push(Box::new(CmdResize::new(index, size)));
    }

    if matches.is_present(ARG_RESIZE_N) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(
            matches.clone(),
            ARG_RESIZE_N,
        )));
    }
    if matches.is_present(ARG_RESIZE_T) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(
            matches.clone(),
            ARG_RESIZE_T,
        )));
    }
    if matches.is_present(ARG_RESIZE_C) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(
            matches.clone(),
            ARG_RESIZE_C,
        )));
    }
    if matches.is_present(ARG_RESIZE_G) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(
            matches.clone(),
            ARG_RESIZE_G,
        )));
    }
    if matches.is_present(ARG_RESIZE_L) {
        cmd_list.commands.push(Box::new(create_cmd_resize_filter(
            matches.clone(),
            ARG_RESIZE_L,
        )));
    }

    if matches.is_present(ARG_ROTATE90) {
        let index = matches.index_of(ARG_ROTATE90).unwrap() as u32;
        let degree = (90 * (matches.occurrences_of(ARG_ROTATE90) as i32)) % 360;

        match degree {
            90 => {
                cmd_list
                    .commands
                    .push(Box::new(CmdRotate::new(index, Rotation::Rotate90)));
            }
            180 => {
                cmd_list
                    .commands
                    .push(Box::new(CmdRotate::new(index, Rotation::Rotate180)));
            }
            270 => {
                cmd_list
                    .commands
                    .push(Box::new(CmdRotate::new(index, Rotation::Rotate270)));
            }
            _ => {}
        }
    }

    if matches.is_present(ARG_ROTATE180) {
        let index = matches.index_of(ARG_ROTATE180).unwrap() as u32;

        cmd_list
            .commands
            .push(Box::new(CmdRotate::new(index, Rotation::Rotate180)));
    }

    if matches.is_present(ARG_ROTATE270) {
        let index = matches.index_of(ARG_ROTATE270).unwrap() as u32;

        cmd_list
            .commands
            .push(Box::new(CmdRotate::new(index, Rotation::Rotate270)));
    }

    if matches.is_present(ARG_TEXT_TL) {
        cmd_list
            .commands
            .push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_TL)));
    }
    if matches.is_present(ARG_TEXT_TR) {
        cmd_list
            .commands
            .push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_TR)));
    }
    if matches.is_present(ARG_TEXT_BL) {
        cmd_list
            .commands
            .push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_BL)));
    }
    if matches.is_present(ARG_TEXT_BR) {
        cmd_list
            .commands
            .push(Box::new(create_cmd_text(matches.clone(), ARG_TEXT_BR)));
    }

    if matches.is_present(ARG_UNSHARPEN) {
        let index = matches.index_of(ARG_UNSHARPEN).unwrap() as u32;
        let values: Vec<_> = matches.values_of(ARG_UNSHARPEN).unwrap().collect();

        let sigma = values[0].parse::<f32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: sigma expects f32, got {} ←‼",
                ARG_UNSHARPEN, values[0]
            )
        });
        let threshold = values[1].parse::<i32>().unwrap_or_else(|_| {
            panic!(
                "‼→ ERROR in {}: threshold expects i32, got {} ←‼",
                ARG_UNSHARPEN, values[1]
            )
        });

        let unsharpen = CmdUnsharpen::new(index, sigma, threshold);
        cmd_list.commands.push(Box::new(unsharpen));
    }

    if matches.is_present(ARG_PRESET) {
        cmd_list
            .commands
            .append(&mut create_cmd_list_preset(matches.clone()));
    }

    cmd_list.commands.sort();
    cmd_list
}

/// This function is parsing the given values for the argument of the combine-command
///
/// Returns a new `CmdCombine` struct
///
/// # Arguments
///
/// * `matches` - The `ArgMatches` struct from clap, containing the provided arguments
/// * `arg` - The argument name of the combine-command to derive the correct position
///
/// # Examples
/// ```
/// use clap::{App, Arg, ArgMatches};
/// use crate::commands::CmdCombine;
///
/// use thumbnailer::BoxPosition;
///
/// const ARG_COMBINE_TL: &str = "combine_tl";
/// const ARG_COMBINE_TR: &str = "combine_tr";
/// const ARG_COMBINE_BL: &str = "combine_bl";
/// const ARG_COMBINE_BR: &str = "combine_br";
///
/// let matches = App::new(env!("CARGO_PKG_NAME"))
///     .arg(Arg::with_name(ARG_COMBINE_TL)
///         .long(ARG_COMBINE_TL)
///         .value_name("IMAGE_PATH")
///         .value_name("x_offset")
///         .value_name("y_offset")
///         .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP LEFT corner of the photo.")
///         .takes_value(true))
///     .get_matches_from(vec![env!("CARGO_PKG_NAME"), "--combine_tl", "img\test.JPG", "5", "10"]);
///
/// let combine = create_cmd_combine(matches, "combine_tl");
/// combine.print();
/// ```
fn create_cmd_combine(matches: ArgMatches<'static>, arg: &str) -> CmdCombine {
    let index = matches.index_of(arg).unwrap() as u32;
    let values: Vec<_> = matches.values_of(arg).unwrap().collect();

    let image = values[0];
    let x_offset = String::from(values[1]).parse::<u32>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: x_offset expects u32, got {} ←‼",
            arg, values[1]
        )
    });
    let y_offset = String::from(values[2]).parse::<u32>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: y_offset expects u32, got {} ←‼",
            arg, values[2]
        )
    });

    let position = match arg {
        _ if arg == ARG_COMBINE_TL => BoxPosition::TopLeft(x_offset, y_offset),
        _ if arg == ARG_COMBINE_TR => BoxPosition::TopRight(x_offset, y_offset),
        _ if arg == ARG_COMBINE_BL => BoxPosition::BottomLeft(x_offset, y_offset),
        _ if arg == ARG_COMBINE_BR => BoxPosition::BottomRight(x_offset, y_offset),
        _ => BoxPosition::TopLeft(x_offset, y_offset),
    };

    let mut thumbnail = Thumbnail::load(Path::new(image).to_path_buf()).unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: failed to load the photo with the supplied path ←‼",
            arg
        )
    });

    let static_thumbnail = thumbnail.clone_static_copy().unwrap_or_else(|| {
        panic!(
            "‼→ ERROR in {}: failed to convert Thumbnail to StaticThumbnail ←‼",
            arg
        )
    });

    CmdCombine::new(index, static_thumbnail, position)
}

/// This function is parsing the given values for the argument of the resize_filter-command
///
/// Returns a new `CmdResizeFilter` struct
///
/// # Arguments
///
/// * `matches` - The `ArgMatches` struct from clap, containing the provided arguments
/// * `arg` - The argument name of the resize_filter-command to derive the correct filter
///
/// # Examples
/// ```
/// use clap::{App, Arg, ArgMatches};
/// use crate::commands::CmdResizeFilter;
///
/// use thumbnailer::{ResampleFilter, Resize};
///
/// const ARG_RESIZE_N: &str = "resize_n";
/// const ARG_RESIZE_T: &str = "resize_t";
/// const ARG_RESIZE_C: &str = "resize_c";
/// const ARG_RESIZE_G: &str = "resize_g";
/// const ARG_RESIZE_L: &str = "resize_l";
///
/// let matches = App::new(env!("CARGO_PKG_NAME"))
///     .arg(Arg::with_name(ARG_RESIZE_N)
///         .long(ARG_RESIZE_N)
///         .value_name("width")
///         .value_name("height")
///         .value_name("exact")
///         .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Nearest is the used filter (Nearest Neighbor Filter). To resize only by one dimension, set the other to 0.")
///         .takes_value(true))
///     .get_matches_from(vec![env!("CARGO_PKG_NAME"), "--resize_n", "400", "300", "false"]);
///
/// let resize_filter = create_cmd_resize_filter(matches, "resize_n");
/// resize_filter.print();
/// ```
fn create_cmd_resize_filter(matches: ArgMatches<'static>, arg: &str) -> CmdResizeFilter {
    let index = matches.index_of(arg).unwrap() as u32;
    let values: Vec<_> = matches.values_of(arg).unwrap().collect();

    let width = values[0].parse::<u32>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: width expects u32, got {} ←‼",
            arg, values[0]
        )
    });
    let height = values[1].parse::<u32>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: height expects u32, got {} ←‼",
            arg, values[1]
        )
    });
    let exact = values[2].parse::<bool>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: exact expects bool, got {} ←‼",
            arg, values[2]
        )
    });

    let size;
    if height == 0 {
        size = Resize::Width(width);
    } else if width == 0 {
        size = Resize::Height(height);
    } else if !exact {
        size = Resize::BoundingBox(width, height);
    } else {
        size = Resize::ExactBox(width, height);
    }

    let filter = match arg {
        _ if arg == ARG_RESIZE_N => ResampleFilter::Nearest,
        _ if arg == ARG_RESIZE_T => ResampleFilter::Triangle,
        _ if arg == ARG_RESIZE_C => ResampleFilter::CatmullRom,
        _ if arg == ARG_RESIZE_G => ResampleFilter::Gaussian,
        _ if arg == ARG_RESIZE_L => ResampleFilter::Lanczos3,
        _ => ResampleFilter::Nearest,
    };

    CmdResizeFilter::new(index, size, filter)
}

/// This function is parsing the given values for the argument of the text-command
///
/// Returns a new `CmdText` struct
///
/// # Arguments
///
/// * `matches` - The `ArgMatches` struct from clap, containing the provided arguments
/// * `arg` - The argument name of the text-command to derive the correct position
///
/// # Examples
/// ```
/// use clap::{App, Arg, ArgMatches};
/// use crate::commands::CmdText;
///
/// use thumbnailer::BoxPosition;
///
/// const ARG_TEXT_TL: &str = "text_tl";
/// const ARG_TEXT_TR: &str = "text_tr";
/// const ARG_TEXT_BL: &str = "text_bl";
/// const ARG_TEXT_BR: &str = "text_br";
///
/// let matches = App::new(env!("CARGO_PKG_NAME"))
///     .arg(Arg::with_name(ARG_TEXT_TL)
///         .long(ARG_TEXT_TL)
///         .value_name("text")
///         .value_name("x_offset")
///         .value_name("y_offset")
///         .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP LEFT corner of the photo.")
///         .takes_value(true))
///     .get_matches_from(vec![env!("CARGO_PKG_NAME"), "--text_tl", "test", "5", "10"]);
///
/// let text = create_cmd_text(matches, "text_tl");
/// text.print();
/// ```
fn create_cmd_text(matches: ArgMatches<'static>, arg: &str) -> CmdText {
    let index = matches.index_of(arg).unwrap() as u32;
    let values: Vec<_> = matches.values_of(arg).unwrap().collect();

    let text = String::from(values[0]);
    let x_offset = String::from(values[1]).parse::<u32>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: x_offset expects u32, got {} ←‼",
            arg, values[1]
        )
    });
    let y_offset = String::from(values[2]).parse::<u32>().unwrap_or_else(|_| {
        panic!(
            "‼→ ERROR in {}: y_offset expects u32, got {} ←‼",
            arg, values[2]
        )
    });

    let position = match arg {
        _ if arg == ARG_TEXT_TL => BoxPosition::TopLeft(x_offset, y_offset),
        _ if arg == ARG_TEXT_TR => BoxPosition::TopRight(x_offset, y_offset),
        _ if arg == ARG_TEXT_BL => BoxPosition::BottomLeft(x_offset, y_offset),
        _ if arg == ARG_TEXT_BR => BoxPosition::BottomRight(x_offset, y_offset),
        _ => BoxPosition::TopLeft(x_offset, y_offset),
    };

    CmdText::new(index, text, position)
}

/// This function is parsing the given preset
///
/// Returns a new `Vec<Box<dyn Command>>` list
///
/// # Arguments
///
/// * `matches` - The `ArgMatches` struct from clap, containing the provided arguments
///
/// # Examples
/// ```
/// use clap::{App, Arg, ArgMatches};
/// use crate::commands::{
///     CmdBlur, CmdBrighten, CmdCombine, CmdContrast, CmdCrop, CmdExif, CmdFlip, CmdHuerotate,
///     CmdInvert, CmdResize, CmdResizeFilter, CmdRotate, CmdText, CmdUnsharpen, Command,
/// };
///
/// use thumbnailer::{
///     BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize, Rotation, Thumbnail,
/// };
///
/// const ARG_PRESET: &str = "preset";
/// const PRESETS: [&str; 3] = ["app_copyright", "web_copyright", "background"];
///
/// let matches = App::new(env!("CARGO_PKG_NAME"))
///     .arg(Arg::with_name(ARG_PRESET)
///         .long(ARG_PRESET)
///         .value_name("name")
///         .possible_values(&PRESETS)
///         .help("Performs predefined commands in a given order, based on the preset, which was chosen.")
///         .takes_value(true))
///     .get_matches_from(vec![env!("CARGO_PKG_NAME"), "--preset", "app_copyright"]);
///
/// let mut cmd_list = Commands { commands: vec![] };
/// cmd_list
///     .commands
///     .append(&mut create_cmd_list_preset(matches.clone()));
/// ```
fn create_cmd_list_preset(matches: ArgMatches<'static>) -> Vec<Box<dyn Command>> {
    let mut cmd_list = Commands { commands: vec![] };

    let index = matches.index_of(ARG_PRESET).unwrap() as u32;
    let name = matches.value_of(ARG_PRESET).unwrap();
    match name {
        _ if name == PRESETS[0] => {
            cmd_list
                .commands
                .push(Box::new(CmdCrop::new(index, Crop::Ratio(4.0, 3.0))));
            cmd_list
                .commands
                .push(Box::new(CmdResize::new(index, Resize::ExactBox(600, 450))));
            cmd_list.commands.push(Box::new(CmdText::new(
                index,
                String::from("(c) thumbnailer"),
                BoxPosition::BottomRight(580, 435),
            )));
        }
        _ if name == PRESETS[1] => {
            cmd_list
                .commands
                .push(Box::new(CmdCrop::new(index, Crop::Ratio(16.0, 9.0))));
            cmd_list
                .commands
                .push(Box::new(CmdResize::new(index, Resize::Height(1080))));
        }
        _ if name == PRESETS[2] => {
            cmd_list
                .commands
                .push(Box::new(CmdBrighten::new(index, -50)));
            cmd_list.commands.push(Box::new(CmdBlur::new(index, 10.0)));
        }
        _ => {}
    }
    cmd_list.commands
}
