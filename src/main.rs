#[macro_use]
extern crate assert_cli;
extern crate clap;
extern crate image;

//mod generic_thumbnail;

use std::cmp::Eq;
use std::cmp::Ordering;

use clap::{App, Arg};

use crate::generic::{Crop, Orientation, ResampleFilter, Resize};
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
struct CmdBlur {
    index: u32,
    sigma: f32,
}

impl CommandTrait for CmdBlur {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::blur(image, self.sigma);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. blur:\t\tsigma = {}", self.index, self.sigma)
    }
}

/// brighten
struct CmdBrighten {
    index: u32,
    value: i32,
}

impl CommandTrait for CmdBrighten {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::brighten(image, self.value);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. brighten:\t\tvalue = {}", self.index, self.value)
    }
}

/// huerotate
struct CmdHuerotate {
    index: u32,
    degree: i32,
}

impl CommandTrait for CmdHuerotate {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::huerotate(image, self.degree);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. huerotate:\tdegree = {}", self.index, self.degree)
    }
}

/// Contrast
struct CmdContrast {
    index: u32,
    value: f32,
}

impl CommandTrait for CmdContrast {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::contrast(image, self.value);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. contrast:\tvalue = {}", self.index, self.value)
    }
}

/// crop
struct CmdCrop {
    index: u32,
    config: Crop,
}

impl CommandTrait for CmdCrop {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::crop(image, self.config);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. crop:\t\t{:?}", self.index, self.config)
    }
}

/// flip
struct CmdFlip {
    index: u32,
    orientation: Orientation,
}

impl CommandTrait for CmdFlip {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::flip(image, self.orientation);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. flip:\t\torientation = {:?}", self.index, self.orientation)
    }
}

/// invert
struct CmdInvert {
    index: u32,
}

impl CommandTrait for CmdInvert {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::invert(image);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. invert", self.index)
    }
}

/// Resize
struct CmdResize {
    index: u32,
    size: Resize,
}

impl CommandTrait for CmdResize {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::resize(image, self.size);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. resize:\t\t{:?}", self.index, self.size)
    }
}

/// ResizeFilter
struct CmdResizeFilter {
    index: u32,
    size: Resize,
    filter: ResampleFilter,
}

impl CommandTrait for CmdResizeFilter {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::resize_filter(image, self.size, self.filter);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. resize_f:\t\t{:?}\tfilter: {:?}", self.index, self.size, self.filter)
    }
}

/// unsharpen
struct CmdUnsharpen {
    index: u32,
    sigma: f32,
    threshold: u32,
}

impl CommandTrait for CmdUnsharpen {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::unsharpen(image, self.sigma, self.threshold);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. unsharpen:\tsigma = {}\t\tthreshold = {}", self.index, self.sigma, self.threshold)
    }
}

/// Commands
struct Commands {
    commands: Vec<Box<dyn CommandTrait>>,
}

trait CommandTrait {
    fn execute(&self, image: &mut dyn GenericThumbnail);
    fn get_index(&self) -> u32;
    fn print(&self) -> String;
}

impl Ord for dyn CommandTrait {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_index().cmp(&other.get_index())
    }
}

impl PartialOrd for dyn CommandTrait {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for dyn CommandTrait {
    fn eq(&self, other: &Self) -> bool {
        self.get_index() == other.get_index()
    }
}

impl Eq for dyn CommandTrait {}

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

        let blur = CmdBlur { index, sigma };
        cmd_list.commands.push(Box::new(blur));
    }

    if matches.is_present(FUNC_BRIGHTEN) {
        let index = matches.index_of(FUNC_BRIGHTEN).unwrap() as u32;
        let value = String::from(matches.value_of(FUNC_BRIGHTEN).unwrap()).parse::<i32>().unwrap_or(0);

        let brighten = CmdBrighten { index, value };
        cmd_list.commands.push(Box::new(brighten));
    }

    if matches.is_present(FUNC_HUEROTATE) {
        let index = matches.index_of(FUNC_HUEROTATE).unwrap() as u32;
        let degree = String::from(matches.value_of(FUNC_HUEROTATE).unwrap()).parse::<i32>().unwrap_or(0);

        let huerotate = CmdHuerotate { index, degree };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_ROTATE90) {
        let index = matches.index_of(FUNC_ROTATE90).unwrap() as u32;
        let degree = 90 * matches.occurrences_of(FUNC_ROTATE90) as i32;

        let huerotate = CmdHuerotate { index, degree };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_ROTATE180) {
        let index = matches.index_of(FUNC_ROTATE180).unwrap() as u32;

        let huerotate = CmdHuerotate { index, degree: 180 };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_ROTATE270) {
        let index = matches.index_of(FUNC_ROTATE270).unwrap() as u32;

        let huerotate = CmdHuerotate { index, degree: 270 };
        cmd_list.commands.push(Box::new(huerotate));
    }

    if matches.is_present(FUNC_CONTRAST) {
        let index = matches.index_of(FUNC_CONTRAST).unwrap() as u32;
        let value = String::from(matches.value_of(FUNC_CONTRAST).unwrap()).parse::<f32>().unwrap_or(0.0);

        let contrast = CmdContrast { index, value };
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

        let crop = CmdCrop { index, config: Crop::Box(x, y, width, height) };
        cmd_list.commands.push(Box::new(crop));
    }

    if matches.is_present(FUNC_CROP_RATIO) {
        let index = matches.index_of(FUNC_CROP_RATIO).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_CROP_RATIO).unwrap().collect();

        let x_ratio = values[0].parse::<f32>().unwrap_or(0.0);
        let y_ratio = values[1].parse::<f32>().unwrap_or(0.0);

        let crop = CmdCrop { index, config: Crop::Ratio(x_ratio, y_ratio) };
        cmd_list.commands.push(Box::new(crop));
    }

    if matches.is_present(FUNC_FLIP_HORIZONTAL) {
        let index = matches.index_of(FUNC_FLIP_HORIZONTAL).unwrap() as u32;

        let flip = CmdFlip { index, orientation: Orientation::Horizontal };
        cmd_list.commands.push(Box::new(flip));
    }

    if matches.is_present(FUNC_FLIP_VERTICAL) {
        let index = matches.index_of(FUNC_FLIP_VERTICAL).unwrap() as u32;

        let flip = CmdFlip { index, orientation: Orientation::Vertical };
        cmd_list.commands.push(Box::new(flip));
    }

    if matches.is_present(FUNC_INVERT) {
        let index = matches.index_of(FUNC_INVERT).unwrap() as u32;

        let invert = CmdInvert { index };
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
            resize = CmdResize { index, size: Resize::Width(width) };
        } else if width == 0 {
            resize = CmdResize { index, size: Resize::Height(height) };
        } else if !exact {
            resize = CmdResize { index, size: Resize::BoundingBox(width, height) };
        } else {
            resize = CmdResize { index, size: Resize::ExactBox(width, height) };
        }
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
            resize_filter = CmdResizeFilter { index, size: Resize::Width(width), filter };
        } else if width == 0 {
            resize_filter = CmdResizeFilter { index, size: Resize::Height(height), filter };
        } else if !exact {
            resize_filter = CmdResizeFilter { index, size: Resize::BoundingBox(width, height), filter };
        } else {
            resize_filter = CmdResizeFilter { index, size: Resize::ExactBox(width, height), filter };
        }
        cmd_list.commands.push(Box::new(resize_filter));
    }

    if matches.is_present(FUNC_UNSHARPEN) {
        let index = matches.index_of(FUNC_UNSHARPEN).unwrap() as u32;
        let values: Vec<_> = matches.values_of(FUNC_UNSHARPEN).unwrap().collect();

        let sigma = values[0].parse::<f32>().unwrap_or(0.0);
        let threshold = values[1].parse::<u32>().unwrap_or(0);

        let unsharpen = CmdUnsharpen { index, sigma, threshold };
        cmd_list.commands.push(Box::new(unsharpen));
    }

    println!("########## unsorted ##########");
    for x in 0..cmd_list.commands.len() {
        println!("{}", cmd_list.commands.get(x).unwrap().print());
    }

    cmd_list.commands.sort();

    println!("##########  sorted  ##########");
    for x in 0..cmd_list.commands.len() {
        println!("{}", cmd_list.commands.get(x).unwrap().print());
    }

    // TODO execute
    // cmd_list.commands.get(0).unwrap().execute();

    // more program logic goes here...
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