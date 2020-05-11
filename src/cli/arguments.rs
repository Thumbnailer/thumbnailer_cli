use clap::{App, Arg};

use crate::cli::{
    ARG_BLUR, ARG_BRIGHTEN, ARG_COMBINE_BL, ARG_COMBINE_BR, ARG_COMBINE_TL, ARG_COMBINE_TR,
    ARG_CONTRAST, ARG_CROP_BOX, ARG_CROP_RATIO, ARG_EXIF, ARG_FLIP_HORIZONTAL, ARG_FLIP_VERTICAL,
    ARG_HUEROTATE, ARG_INVERT, ARG_PRESET, ARG_RESIZE, ARG_RESIZE_C, ARG_RESIZE_G, ARG_RESIZE_L,
    ARG_RESIZE_N, ARG_RESIZE_T, ARG_ROTATE180, ARG_ROTATE270, ARG_ROTATE90, ARG_TEXT_BL,
    ARG_TEXT_BR, ARG_TEXT_TL, ARG_TEXT_TR, ARG_UNSHARPEN, NAME_FILE_IN, NAME_FILE_OUT, PRESETS,
    VAL_COMBINE, VAL_RESIZE, VAL_TEXT,
};

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
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_TR)
            .long(ARG_COMBINE_TR)
            .value_names(&VAL_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP RIGHT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_BL)
            .long(ARG_COMBINE_BL)
            .value_names(&VAL_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COMBINE_BR)
            .long(ARG_COMBINE_BR)
            .value_names(&VAL_COMBINE)
            .help("Inserts a photo, such as a logo given as path, into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM RIGHT corner of the photo.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_CROP_BOX)
            .long(ARG_CROP_BOX)
            .value_name("x")
            .value_name("y")
            .value_name("width")
            .value_name("height")
            .help("Crops the supplied image(s) to the given width as u32 and height as u32. x as u32 is the horizontal and y as u32 the vertical offset.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_CROP_RATIO)
            .long(ARG_CROP_RATIO)
            .value_name("x_ratio")
            .value_name("y_ratio")
            .help("Crops the supplied image(s) to the given ratio 'x_ratio:y_ratio'. x_ratio as f32 is representing the horizontal and y_ratio as f32 the vertical.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_EXIF)
            .long(ARG_EXIF)
            .help("The metadata of the target file is taken from the source file and not removed as usual."))

        .arg(Arg::with_name(ARG_FLIP_HORIZONTAL)
            .long(ARG_FLIP_HORIZONTAL)
            .help("Flip the supplied image(s) horizontally."))
        .arg(Arg::with_name(ARG_FLIP_VERTICAL)
            .long(ARG_FLIP_VERTICAL)
            .help("Flip the supplied image(s) vertically."))

        .arg(Arg::with_name(ARG_HUEROTATE)
            .long(ARG_HUEROTATE)
            .value_name("value")
            .help("Hue rotate the supplied image(s). value as i32 is the degrees to rotate each pixel by. 0 and 360 do nothing, the rest rotates by the given degree value.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_INVERT)
            .long(ARG_INVERT)
            .help("Invert each pixel within the supplied image(s)."))

        .arg(Arg::with_name(ARG_RESIZE)
            .long(ARG_RESIZE)
            .value_names(&VAL_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_N)
            .long(ARG_RESIZE_N)
            .value_names(&VAL_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Nearest is the used filter (Nearest Neighbor Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_T)
            .long(ARG_RESIZE_T)
            .value_names(&VAL_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Triangle is the used filter (Linear Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_C)
            .long(ARG_RESIZE_C)
            .value_names(&VAL_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. CatmullRom is the used filter (Cubic Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_G)
            .long(ARG_RESIZE_G)
            .value_names(&VAL_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Gaussian is the used filter (Gaussian Filter). To resize only by one dimension, set the other to 0.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_RESIZE_L)
            .long(ARG_RESIZE_L)
            .value_names(&VAL_RESIZE)
            .help("Resize the supplied image(s) to the specified dimensions. nwidth and nheight are the new dimensions. exact as bool forces the exact resizing, but the aspect ratio may change. Lanczos3 is the used filter (Lanczos with window 3). To resize only by one dimension, set the other to 0.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_ROTATE90)
            .short("r")
            .long(ARG_ROTATE90)
            .help("Rotate the supplied image(s) 90 degrees clockwise.")
            .multiple(true))
        .arg(Arg::with_name(ARG_ROTATE180)
            .long(ARG_ROTATE180)
            .help("Rotate the supplied image(s) 180 degrees clockwise."))
        .arg(Arg::with_name(ARG_ROTATE270)
            .long(ARG_ROTATE270)
            .help("Rotate the supplied image(s) 270 degrees clockwise."))

        .arg(Arg::with_name(ARG_TEXT_TL)
            .long(ARG_TEXT_TL)
            .value_names(&VAL_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_TR)
            .long(ARG_TEXT_TR)
            .value_names(&VAL_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the TOP RIGHT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_BL)
            .long(ARG_TEXT_BL)
            .value_names(&VAL_TEXT)
            .help("Inserts a text as String into the supplied image(s). x_offset as u32 is the horizontal and y-offset as u32 vertical offset to the BOTTOM LEFT corner of the photo.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_TEXT_BR)
            .long(ARG_TEXT_BR)
            .value_names(&VAL_TEXT)
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
    //to debug combine, use:
    //.get_matches_from(vec![env!("CARGO_PKG_NAME"), "in.png" ,"--combine_tl", "C:\\Users\\p372094\\IdeaProjects\\thumbnailer_cli\\img\\test.JPG", "40", "30"])
}
