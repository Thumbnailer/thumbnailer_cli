#[macro_use]
extern crate assert_cli;
extern crate clap;
extern crate image;

//mod generic_thumbnail;

use clap::{App, Arg};

use crate::generic::{CropConfig, FlipOrientation, ResampleFilter, ResizeSize};
pub use crate::generic::GenericThumbnail;
pub use crate::thumbnail::SingleThumbnail;
pub use crate::thumbnail::StaticThumbnail;

mod generic;
mod thumbnail;

const FUNC_BLUR: &str = "blur";
const FUNC_BRIGHTEN: &str = "brighten";
const FUNC_HUEROTATE: &str = "huerotate";
const FUNC_ROTATE90: &str = "rotate90";
const FUNC_ROTATE180: &str = "rotate180";
const FUNC_ROTATE270: &str = "rotate270";
const FUNC_CONTRAST: &str = "Contrast";
const FUNC_CROP_BOX: &str = "crop_box";
const FUNC_CROP_RATIO: &str = "crop_ratio";
const FUNC_FLIP_HORIZONTAL: &str = "flip_horizontal";
const FUNC_FLIP_VERTICAL: &str = "flip_vertical";
const FUNC_INVERT: &str = "invert";
const FUNC_RESIZE: &str = "resize";
const FUNC_RESIZE_F: &str = "resize_f";
const FUNC_UNSHARPEN: &str = "unsharpen";

const BOOLS: [&str; 2] = ["true", "false"];
const FILTERS: [&str; 5] = ["nearest", "triangle", "catmull_rom", "gaussian", "lanczos3"];

//  .\target\debug\thumbnailer_cli.exe -h
//  .\target\debug\thumbnailer_cli.exe in.png out.png --flip_vertical  --brighten 15 --blur 34.151545 --unsharpen 56 14
//  .\target\debug\thumbnailer_cli.exe in.png out.png --flip_vertical  --brighten 15 --crop_ratio 4 3 --blur 34.151545 --unsharpen 56 14 --resize 89 4 true
//
//  .\target\debug\thumbnailer_cli.exe in.png out.png -c settings.conf -mmm watermark icon.png colored
//  .\target\debug\thumbnailer_cli.exe in.png out.png -c settings.conf -mmm --blur 6 --brighten 15
//  .\target\debug\thumbnailer_cli.exe in.png out.png --blur 6 --brighten 15

/// blur
struct Blur {
    index: u32,
    sigma: f32,
}

impl CommandTrait for Blur {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::blur(image, self.sigma);
    }
}

/// brighten
struct Brighten {
    index: u32,
    value: i32,
}

impl CommandTrait for Brighten {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::brighten(image, self.value);
    }
}

/// huerotate
struct Huerotate {
    index: u32,
    degree: i32,
}

impl CommandTrait for Huerotate {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::huerotate(image, self.degree);
    }
}

/// Contrast
struct Contrast {
    index: u32,
    value: f32,
}

impl CommandTrait for Contrast {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::contrast(image, self.value);
    }
}

/// crop

struct Crop {
    index: u32,
    config: CropConfig,
}

impl CommandTrait for Crop {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::crop(image, self.config);
    }
}

/// flip

struct Flip {
    index: u32,
    orientation: FlipOrientation,
}

impl CommandTrait for Flip {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::flip(image, self.orientation);
    }
}

/// invert
struct Invert {
    index: u32,
}

impl CommandTrait for Invert {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::invert(image);
    }
}

/// Resize
struct Resize {
    index: u32,
    size: ResizeSize,
}

impl CommandTrait for Resize {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::resize(image, self.size);
    }
}

/// ResizeFilter
struct ResizeFilter {
    index: u32,
    size: ResizeSize,
    filter: ResampleFilter,
}

impl CommandTrait for ResizeFilter {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::resize_filter(image, self.size, self.filter);
    }
}

/// unsharpen
struct Unsharpen {
    index: u32,
    sigma: f32,
    threshold: u32,
}

impl CommandTrait for Unsharpen {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::unsharpen(image, self.sigma, self.threshold);
    }
}

/// Commands
struct Commands {
    commands: Vec<Box<dyn CommandTrait>>,
}

trait CommandTrait {
    fn execute(&self, image: &mut dyn GenericThumbnail);
}

fn main() {
    let matches = App::new("Thumbnail-Generator")
        .version("1.0")
        .author("DHBW Stuttgart, ITA18 <info@dhbw-stuttgart.de>")
        .about("Does awesome things")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to save")
            .required(true)
            .index(2))
        .arg(Arg::with_name(FUNC_BLUR)
            .long(FUNC_BLUR)
            .value_name("sigma")
            .help("Performs a Gaussian blur on the supplied image. sigma is a measure of how much to blur by.")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_BRIGHTEN)
            .long(FUNC_BRIGHTEN)
            .value_name("value")
            .help("Brighten the supplied image. value is the amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it.")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_HUEROTATE)
            .long(FUNC_HUEROTATE)
            .value_name("value")
            .help("Hue rotate the supplied image. value is the degrees to rotate each pixel by. 0 and 360 do nothing, the rest rotates by the given degree value.")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_ROTATE90)
            .short("r")
            .long(FUNC_ROTATE90)
            .help("Rotate an image 90 degrees clockwise.")
            .multiple(true))
        .arg(Arg::with_name(FUNC_ROTATE180)
            .long(FUNC_ROTATE180)
            .help("Rotate an image 180 degrees clockwise."))
        .arg(Arg::with_name(FUNC_ROTATE270)
            .long(FUNC_ROTATE270)
            .help("Rotate an image 270 degrees clockwise."))
        .arg(Arg::with_name(FUNC_CONTRAST)
            .long(FUNC_CONTRAST)
            .value_name("contrast")
            .help("Adjust the contrast of the supplied image. contrast is the amount to adjust the contrast by. Negative values decrease the contrast and positive values increase the contrast.")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_CROP_BOX)
            .long(FUNC_CROP_BOX)
            .value_name("x")
            .value_name("y")
            .value_name("width")
            .value_name("height")
            .help("Crop the supplied image. x is the horizontal and y the vertical offset. The image will be cropped to the given width and height.")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_CROP_RATIO)
            .long(FUNC_CROP_RATIO)
            .value_name("x_ratio")
            .value_name("y_ratio")
            .help("Crop the supplied image. The image will be cropped to the given ratio x_ratio:y_ratio.")
            .takes_value(true))
        .arg(Arg::with_name("filter3x3")
            .long("filter3x3")
            .value_name("kernel")
            .help("Perform a 3x3 box filter on the supplied image. kernel is an array of the filter weights of length 9.")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_FLIP_HORIZONTAL)
            .long(FUNC_FLIP_HORIZONTAL)
            .help("Flip an image horizontally."))
        .arg(Arg::with_name(FUNC_FLIP_VERTICAL)
            .long(FUNC_FLIP_VERTICAL)
            .help("Flip an image vertically."))
        .arg(Arg::with_name("grayscale")
            .long("grayscale")
            .help("Convert the supplied image to grayscale."))
        .arg(Arg::with_name(FUNC_INVERT)
            .long(FUNC_INVERT)
            .help("Invert each pixel within the supplied image. This function operates in place."))
        .arg(Arg::with_name(FUNC_RESIZE)
            .long(FUNC_RESIZE)
            .value_name("nwidth")
            .value_name("nheight")
            .value_name("exact")
            .help("Resize the supplied image to the specified dimensions. nwidth and nheight are the new dimensions.")
            .takes_value(true))
        //TODO predefine filter
        .arg(Arg::with_name(FUNC_RESIZE_F)
            .long(FUNC_RESIZE_F)
            .value_name("nwidth")
            .value_name("nheight")
            .value_name("exact")
            .value_name("filter")
            .help("Resize the supplied image to the specified dimensions. nwidth and nheight are the new dimensions. filter is the sampling filter to use. available filters: nearest, triangle, catmull_rom, gaussian, lanczos3")
            .takes_value(true))
        .arg(Arg::with_name(FUNC_UNSHARPEN)
            .long(FUNC_UNSHARPEN)
            .value_name("sigma")
            .value_name("threshold")
            .help("Performs an unsharpen mask on the supplied image. sigma is the amount to blur the image by. threshold is the threshold for the difference between.")
            .takes_value(true))
        .get_matches();

    let file_in = String::from(matches.value_of("INPUT").unwrap());
    let file_out = String::from(matches.value_of("OUTPUT").unwrap());

    println!("Input file: {}", file_in);
    println!("Output file: {}", file_out);

    let mut cmd_list = Commands { commands: vec![] };

    if matches.is_present(FUNC_BLUR) {
        let index = matches.index_of(FUNC_BLUR).unwrap() as u32;
        let sigma = String::from(matches.value_of(FUNC_BLUR).unwrap()).parse::<f32>().unwrap_or(0.0);

        let blur = Blur { index, sigma };
        println!("► {:02}. blur:\t\tsigma = {}", blur.index, blur.sigma);
        cmd_list.commands.push(Box::new(blur));
    }

    if matches.is_present(FUNC_BRIGHTEN) {
        let index = matches.index_of(FUNC_BRIGHTEN).unwrap() as u32;
        let value = String::from(matches.value_of(FUNC_BRIGHTEN).unwrap()).parse::<i32>().unwrap_or(0);

        let brighten = Brighten { index, value };
        println!("► {:02}. brighten:\t\tvalue = {}", brighten.index, brighten.value);
        cmd_list.commands.push(Box::new(brighten));
    }

    if matches.is_present(FUNC_HUEROTATE) {
        let index = matches.index_of(FUNC_HUEROTATE).unwrap() as u32;
        let degree = String::from(matches.value_of(FUNC_HUEROTATE).unwrap()).parse::<i32>().unwrap_or(0);

        let huerotate = Huerotate { index, degree };
        println!("► {:02}. huerotate:\tdegree = {}", huerotate.index, huerotate.degree);
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_ROTATE90) {
        let index = matches.index_of(FUNC_ROTATE90).unwrap() as u32;
        let degree = 90 * matches.occurrences_of(FUNC_ROTATE90) as i32;

        let huerotate = Huerotate { index, degree };
        println!("► {:02}. huerotate:\tdegree = {}", huerotate.index, huerotate.degree);
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_ROTATE180) {
        let index = matches.index_of(FUNC_ROTATE180).unwrap() as u32;

        let huerotate = Huerotate { index, degree: 180 };
        println!("► {:02}. huerotate:\tdegree = {}", huerotate.index, huerotate.degree);
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_ROTATE270) {
        let index = matches.index_of(FUNC_ROTATE270).unwrap() as u32;

        let huerotate = Huerotate { index, degree: 270 };
        println!("► {:02}. huerotate:\tdegree = {}", huerotate.index, huerotate.degree);
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_CONTRAST) {
        let index = matches.index_of(FUNC_CONTRAST).unwrap() as u32;
        let value = String::from(matches.value_of(FUNC_CONTRAST).unwrap()).parse::<f32>().unwrap_or(0.0);

        let contrast = Contrast { index, value };
        println!("► {:02}. contrast:\tvalue = {}", contrast.index, contrast.value);
        cmd_list.commands.push(Box::new(contrast));
    }

    //TODO rework combine both crop functions?
    if matches.is_present(FUNC_CROP_BOX) {
        let index = matches.index_of(FUNC_CROP_BOX).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_CROP_BOX).unwrap().collect();

        let x = values[0].parse::<u32>().unwrap_or(0);
        let y = values[1].parse::<u32>().unwrap_or(0);
        let width = values[2].parse::<u32>().unwrap_or(0);
        let height = values[3].parse::<u32>().unwrap_or(0);

        let crop = Crop { index, config: CropConfig::Box(x, y, width, height) };
        println!("► {:02}. crop_box:\t{:?}", crop.index, crop.config);
        cmd_list.commands.push(Box::new(crop));
    }

    if matches.is_present(FUNC_CROP_RATIO) {
        let index = matches.index_of(FUNC_CROP_RATIO).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_CROP_RATIO).unwrap().collect();

        let x_ratio = values[0].parse::<f32>().unwrap_or(0.0);
        let y_ratio = values[1].parse::<f32>().unwrap_or(0.0);

        let crop = Crop { index, config: CropConfig::Ratio(x_ratio, y_ratio) };
        println!("► {:02}. crop_ratio:\t{:?}", crop.index, crop.config);
        cmd_list.commands.push(Box::new(crop));
    }

    if matches.is_present(FUNC_FLIP_HORIZONTAL) {
        let index = matches.index_of(FUNC_FLIP_HORIZONTAL).unwrap() as u32;

        let flip = Flip { index, orientation: FlipOrientation::Horizontal };
        println!("► {:02}. flip:\t\torientation = {:?}", flip.index, flip.orientation);
        cmd_list.commands.push(Box::new(flip));
    }

    if matches.is_present(FUNC_FLIP_VERTICAL) {
        let index = matches.index_of(FUNC_FLIP_VERTICAL).unwrap() as u32;

        let flip = Flip { index, orientation: FlipOrientation::Vertical };
        println!("► {:02}. flip:\t\torientation = {:?}", flip.index, flip.orientation);
        cmd_list.commands.push(Box::new(flip));
    }

    if matches.is_present(FUNC_INVERT) {
        let index = matches.index_of(FUNC_INVERT).unwrap() as u32;

        let invert = Invert { index };
        println!("► {:02}. invert", invert.index);
        cmd_list.commands.push(Box::new(invert));
    }

    //TODO rework (currently needing all 3 arguments)?
    if matches.is_present(FUNC_RESIZE) {
        let index = matches.index_of(FUNC_RESIZE).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_RESIZE).unwrap().collect();

        let width = values[0].parse::<u32>().unwrap_or(0);
        let height = values[1].parse::<u32>().unwrap_or(0);
        let exact = values[2].parse::<bool>().unwrap_or(false);

        let resize;
        if height == 0 {
            resize = Resize { index, size: ResizeSize::Width(width) };
        } else if width == 0 {
            resize = Resize { index, size: ResizeSize::Height(height) };
        } else if !exact {
            resize = Resize { index, size: ResizeSize::BoundingBox(width, height) };
        } else {
            resize = Resize { index, size: ResizeSize::ExactBox(width, height) };
        }
        println!("► {:02}. resize:\t\t{:?}", resize.index, resize.size);
        cmd_list.commands.push(Box::new(resize));
    }

    if matches.is_present(FUNC_RESIZE_F) {
        let index = matches.index_of(FUNC_RESIZE_F).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_RESIZE_F).unwrap().collect();

        let width = values[0].parse::<u32>().unwrap_or(0);
        let height = values[1].parse::<u32>().unwrap_or(0);
        let exact = values[2].parse::<bool>().unwrap_or(false);

        let filter = match values[3] {
            _ if values[3] == FILTERS[0] => ResampleFilter::Nearest,
            _ if values[3] == FILTERS[1] => ResampleFilter::Triangle,
            _ if values[3] == FILTERS[2] => ResampleFilter::CatmullRom,
            _ if values[3] == FILTERS[3] => ResampleFilter::Gaussian,
            _ if values[3] == FILTERS[4] => ResampleFilter::Lanczos3,
            _ => ResampleFilter::Nearest,
        };

        let resize_filter;
        if height == 0 {
            resize_filter = ResizeFilter { index, size: ResizeSize::Width(width), filter };
        } else if width == 0 {
            resize_filter = ResizeFilter { index, size: ResizeSize::Height(height), filter };
        } else if !exact {
            resize_filter = ResizeFilter { index, size: ResizeSize::BoundingBox(width, height), filter };
        } else {
            resize_filter = ResizeFilter { index, size: ResizeSize::ExactBox(width, height), filter };
        }
        println!("► {:02}. resize_f:\t\t{:?}\tfilter: {:?}", resize_filter.index, resize_filter.size, resize_filter.filter);
        cmd_list.commands.push(Box::new(resize_filter));
    }

    if matches.is_present(FUNC_UNSHARPEN) {
        let index = matches.index_of(FUNC_UNSHARPEN).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_UNSHARPEN).unwrap().collect();

        let sigma = values[0].parse::<f32>().unwrap_or(0.0);
        let threshold = values[1].parse::<u32>().unwrap_or(0);

        let unsharpen = Unsharpen { index, sigma, threshold };
        println!("► {:02}. unsharpen:\tsigma = {}\t\tthreshold = {}", unsharpen.index, unsharpen.sigma, unsharpen.threshold);
        cmd_list.commands.push(Box::new(unsharpen));
    }


    // more program logic goes here...

    // println!("{:?}", cmd_list);
}

#[cfg(test)]
mod test {
    #[test]
    fn cli_default_mmm() {
        let t = assert_cmd!(echo "target\\debug\\thumbnailer_cli.exe in.png out.png --flip_vertical  --brighten 15 --blur 34.151545 --unsharpen 56 14")
            .stdout().contains("Input file: in.txt")
            .stdout().contains("Output file: out.txt")
            .stdout().contains("► 07. blur:             sigma = 34.151546")
            .stdout().contains("► 05. brighten:         value = 15")
            .stdout().contains("► 03. flip:             orientation = Vertical")
            .stdout().contains("► 09. unsharpen:        sigma = 56              threshold = 14")
            .stderr().is("")
            .execute();
        assert!(t.is_err());
    }
}