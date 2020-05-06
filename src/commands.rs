use std::cmp::Eq;
use std::cmp::Ordering;

use thumbnailer::{GenericThumbnail, StaticThumbnail};
use thumbnailer::{BoxPosition, Crop, Exif, Orientation, ResampleFilter, Resize};

/// Commands
pub trait CommandTrait {
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

/// Representation of the blur-command as a struct
pub struct CmdBlur {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub sigma: f32,
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

/// Representation of the brighten-command as a struct
pub struct CmdBrighten {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub value: i32,
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

/// Representation of the contrast-command as a struct
pub struct CmdContrast {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub value: f32,
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

/// Representation of the combine-command as a struct
pub struct CmdCombine {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub image: StaticThumbnail,
    pub position: BoxPosition,
}

impl CommandTrait for CmdCombine {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::combine(image, self.image.clone(), self.position);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. combine:\tposition = {:?}", self.index, self.position)
    }
}

/// Representation of the crop-command as a struct
pub struct CmdCrop {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub config: Crop,
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

/// Representation of the exif-command as a struct
pub struct CmdExif {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub metadata: Exif,
}

impl CommandTrait for CmdExif {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::exif(image, self.metadata.clone());
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. exif:\t\t{:?}", self.index, self.metadata)
    }
}

/// Representation of the flip-command as a struct
pub struct CmdFlip {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub orientation: Orientation,
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

/// Representation of the huerotate-command as a struct
pub struct CmdHuerotate {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub degree: i32,
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

/// Representation of the invert-command as a struct
pub struct CmdInvert {
    /// contains the index of arguments list as u32
    pub index: u32,
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

/// Representation of the resize-command as a struct
pub struct CmdResize {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub size: Resize,
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

/// Representation of the resizeFilter-command as a struct
pub struct CmdResizeFilter {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub size: Resize,
    pub filter: ResampleFilter,
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

/// Representation of the text-command as a struct
pub struct CmdText {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub text: String,
    pub position: BoxPosition,
}

impl CommandTrait for CmdText {
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::text(image, self.text.clone(), self.position);
    }

    fn get_index(&self) -> u32 {
        self.index
    }

    fn print(&self) -> String {
        format!("► {:02}. text:\t\t{:?}\ttext: {}", self.index, self.position, self.text)
    }
}

/// Representation of the unsharpen-command as a struct
pub struct CmdUnsharpen {
    /// contains the index of arguments list as u32
    pub index: u32,
    pub sigma: f32,
    pub threshold: i32,
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