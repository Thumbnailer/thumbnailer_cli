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
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the measure as f32 of how much to blur by (is set to 0.0 if parsing fails)
    pub sigma: f32,
}

impl CommandTrait for CmdBlur {
    /// This function calls the actual blur command, depending on the sigma given by CmdBlur-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdBlur-struct
    /// * `image` - The GenericThumbnail to be blurred
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::blur(image, self.sigma);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdBlur-struct
    ///
    /// # Examples
    /// ```
    /// let index = 1;
    /// let blur = CmdBlur { index, sigma: 0.5 };
    /// println!("index = {}", blur.get_index());
    /// assert_eq!(blur.get_index(), 1, "testing blur.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the sigma given by CmdBlur-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdBlur-struct
    ///
    /// # Examples
    /// ```
    /// let blur = CmdBlur { index: 1, sigma: 0.5 };
    /// println!("{}", blur.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. blur:\t\tsigma = {}", self.index, self.sigma)
    }
}

/// Representation of the brighten-command as a struct
pub struct CmdBrighten {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the amount as i32 to brighten each pixel by (is set to 0 if parsing fails)
    pub value: i32,
}

impl CommandTrait for CmdBrighten {
    /// This function calls the actual brighten command, depending on the value given by CmdBrighten-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdBrighten-struct
    /// * `image` - The GenericThumbnail to be brightened
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::brighten(image, self.value);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdBrighten-struct
    ///
    /// # Examples
    /// ```
    /// let index = 2;
    /// let brighten = CmdBrighten { index, value: 3 };
    /// println!("index = {}", brighten.get_index());
    /// assert_eq!(brighten.get_index(), 2, "testing brighten.get_index() with index = {}", index);
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the value given by CmdBrighten-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdBrighten-struct
    ///
    /// # Examples
    /// ```
    /// let brighten = CmdBrighten { index: 2, value: 3 };
    /// println!("{}", brighten.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. brighten:\t\tvalue = {}", self.index, self.value)
    }
}

/// Representation of the contrast-command as a struct
pub struct CmdContrast {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the amount as f32 to adjust the contrast by (is set to 0.0 if parsing fails)
    pub value: f32,
}

impl CommandTrait for CmdContrast {
    /// This function calls the actual contrast command, depending on the value given by CmdContrast-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdContrast-struct
    /// * `image` - The GenericThumbnail whose contrast is to be changed
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::contrast(image, self.value);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdContrast-struct
    ///
    /// # Examples
    /// ```
    /// let index = 3;
    /// let contrast = CmdContrast { index, value: 0.8 };
    /// println!("index = {}", contrast.get_index());
    /// assert_eq!(contrast.get_index(), 3, "testing contrast.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the value given by CmdContrast-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdContrast-struct
    ///
    /// # Examples
    /// ```
    /// let contrast = CmdContrast { index: 3, value: 0.8 };
    /// println!("{}", contrast.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. contrast:\tvalue = {}", self.index, self.value)
    }
}

/// Representation of the combine-command as a struct
pub struct CmdCombine {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the StaticThumbnail struct as image
    pub image: StaticThumbnail,
    /// Contains the BoxPosition enum as option
    pub position: BoxPosition,
}

impl CommandTrait for CmdCombine {
    /// This function calls the actual combine command, depending on the values given by the members of CmdCombine-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdCombine-struct
    /// * `image` - The GenericThumbnail in which the photo should be inserted
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::combine(image, self.image.clone(), self.position);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdCombine-struct
    ///
    /// # Examples
    /// ```
    /// let index = 4;
    /// let combine = CmdCombine { index, image: TODO, position: BoxPosition::TopLeft(34, 56) };
    /// println!("index = {}", combine.get_index());
    /// assert_eq!(combine.get_index(), 4, "testing combine.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the values given by the members of CmdCombine-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdCombine-struct
    ///
    /// # Examples
    /// ```
    /// let combine = CmdCombine { index: 4, image: TODO, position: BoxPosition::TopLeft(34, 56) };
    /// println!("{}", combine.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. combine:\tposition = {:?}", self.index, self.position)
    }
}

/// Representation of the crop-command as a struct
pub struct CmdCrop {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the Crop enum as option
    pub config: Crop,
}

impl CommandTrait for CmdCrop {
    /// This function calls the actual crop command, depending on the values given by the members of CmdCrop-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdCrop-struct
    /// * `image` - The GenericThumbnail to be cropped
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::crop(image, self.config);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdCrop-struct
    /// # Examples
    /// ```
    /// let index = 5;
    /// let crop = CmdCrop { index, config: Crop::Ratio(4, 3) };
    /// println!("index = {}", crop.get_index());
    /// assert_eq!(crop.get_index(), 5, "testing crop.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the values given by the members of CmdCrop-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdCrop-struct
    ///
    /// # Examples
    /// ```
    /// let crop = CmdCrop { index: 5, config: Crop::Ratio(4, 3) };
    /// println!("{}", crop.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. crop:\t\t{:?}", self.index, self.config)
    }
}

/// Representation of the exif-command as a struct
pub struct CmdExif {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the Exif enum as option
    pub metadata: Exif,
}

impl CommandTrait for CmdExif {
    /// This function calls the actual exif command, depending on the values given by the members of CmdExif-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdExif-struct
    /// * `image` - The GenericThumbnail in which the metadata should be copied
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::exif(image, self.metadata.clone());
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdExif-struct
    ///
    /// # Examples
    /// ```
    /// let index = 6;
    /// let exif = CmdExif { index, metadata: Exif::Keep };
    /// println!("index = {}", exif.get_index());
    /// assert_eq!(exif.get_index(), 1, "testing exif.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the values given by the members of CmdExif-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdExif-struct
    ///
    /// # Examples
    /// ```
    /// let exif = CmdExif { index: 6, metadata: Exif::Keep };
    /// println!("{}", exif.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. exif:\t\t{:?}", self.index, self.metadata)
    }
}

/// Representation of the flip-command as a struct
pub struct CmdFlip {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the Orientation enum as option
    pub orientation: Orientation,
}

impl CommandTrait for CmdFlip {
    /// This function calls the actual flip command, depending on the value given by the CmdFlip-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdFlip-struct
    /// * `image` - The GenericThumbnail to be flipped
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::flip(image, self.orientation);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdFlip-struct
    ///
    /// # Examples
    /// ```
    /// let index = 7;
    /// let flip = CmdFlip { index, orientation: Orientation::Horizontal};
    /// println!("index = {}", flip.get_index());
    /// assert_eq!(flip.get_index(), 7, "testing flip.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the value given by the CmdFlip-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdFlip-struct
    ///
    /// # Examples
    /// ```
    /// let flip = CmdFlip { index: 7, orientation: Orientation::Horizontal};
    /// println!("{}", flip.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. flip:\t\torientation = {:?}", self.index, self.orientation)
    }
}

/// Representation of the huerotate-command as a struct
pub struct CmdHuerotate {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the amount as i32 in degree to rotate the image by (is set to 0 if parsing fails)
    pub degree: i32,
}

impl CommandTrait for CmdHuerotate {
    /// This function calls the actual huerotate command, depending on the degree given by the CmdHuerotate-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdHuerotate-struct
    /// * `image` - The GenericThumbnail to be rotated
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::huerotate(image, self.degree);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdHuerotate-struct
    ///
    /// # Examples
    /// ```
    /// let index = 8;
    /// let huerotate = CmdHuerotate { index, degree: 90 };
    /// println!("index = {}", huerotate.get_index());
    /// assert_eq!(huerotate.get_index(), 8, "testing huerotate.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the degree given by the CmdHuerotate-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdHuerotate-struct
    ///
    /// # Examples
    /// ```
    /// let huerotate = CmdBlur { index: 8,  degree: 90 };
    /// println!("{}", huerotate.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. huerotate:\tdegree = {}", self.index, self.degree)
    }
}

/// Representation of the invert-command as a struct
pub struct CmdInvert {
    /// Contains the index as u32 of arguments list
    pub index: u32,
}

impl CommandTrait for CmdInvert {
    /// This function calls the actual invert command.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdInvert-struct
    /// * `image` - The GenericThumbnail to be inverted
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::invert(image);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdInvert-struct
    ///
    /// # Examples
    /// ```
    /// let index = 9;
    /// let invert = CmdInvert { index };
    /// println!("index = {}", invert.get_index());
    /// assert_eq!(invert.get_index(), 9, "testing invert.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdInvert-struct
    ///
    /// # Examples
    /// ```
    /// let invert = CmdBlur { index: 9 };
    /// println!("{}", invert.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. invert", self.index)
    }
}

/// Representation of the resize-command as a struct
pub struct CmdResize {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the Resize enum as option
    pub size: Resize,
}

impl CommandTrait for CmdResize {
    /// This function calls the actual resize command, depending on the values given by the members of CmdResize-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdResize-struct
    /// * `image` - The GenericThumbnail to be resized
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::resize(image, self.size);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdResize-struct
    ///
    /// # Examples
    /// ```
    /// let index = 10;
    /// let resize = CmdResize { index, size: Resize::BoundingBox(400, 300) };
    /// println!("index = {}", resize.get_index());
    /// assert_eq!(resize.get_index(), 10, "testing resize.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the values given by the members of CmdResize-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdResize-struct
    ///
    /// # Examples
    /// ```
    /// let resize = CmdResize { index: 10, size: Resize::BoundingBox(400, 300) };
    /// println!("{}", resize.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. resize:\t\t{:?}", self.index, self.size)
    }
}

/// Representation of the resizeFilter-command as a struct
pub struct CmdResizeFilter {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the Resize enum as option
    pub size: Resize,
    /// Contains the ResampleFilter enum as option
    pub filter: ResampleFilter,
}

impl CommandTrait for CmdResizeFilter {
    /// This function calls the actual resize_filter command, depending on the values given by the members of CmdResizeFilter-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdResizeFilter-struct
    /// * `image` - The GenericThumbnail to be resized with a given filter
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::resize_filter(image, self.size, self.filter);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdResizeFilter-struct
    ///
    /// # Examples
    /// ```
    /// let index = 11;
    /// let resize_filter = CmdResizeFilter { index, size: Resize::BoundingBox(400, 300), filter: ResampleFilter::Nearest };
    /// println!("index = {}", resize_filter.get_index());
    /// assert_eq!(resize_filter.get_index(), 10, "testing resize_filter.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the values given by the members of CmdResizeFilter-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdResizeFilter-struct
    ///
    /// # Examples
    /// ```
    /// let resize_filter = CmdResizeFilter { index: 11, size: Resize::BoundingBox(400, 300), filter: ResampleFilter::Nearest };
    /// println!("{}", resize_filter.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. resize_filter:\t{:?}\tfilter: {:?}", self.index, self.size, self.filter)
    }
}

/// Representation of the text-command as a struct
pub struct CmdText {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the text as String to print into the supplied image(s)
    pub text: String,
    /// Contains the BoxPosition enum as option
    pub position: BoxPosition,
}

impl CommandTrait for CmdText {
    /// This function calls the actual text command, depending on the values given by the members of CmdText-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdText-struct
    /// * `image` - The GenericThumbnail in which the text should be inserted
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::text(image, self.text.clone(), self.position);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdText-struct
    ///
    /// # Examples
    /// ```
    /// let index = 12;
    /// let text = CmdText { index, text: String::from("(c) Thumbnailer"), position: BoxPosition::TopLeft(37, 28)  };
    /// println!("index = {}", text.get_index());
    /// assert_eq!(text.get_index(), 12, "testing text.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the values given by the members of CmdText-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdText-struct
    ///
    /// # Examples
    /// ```
    /// let text = CmdText { index: 12, text: String::from("(c) Thumbnailer"), position: BoxPosition::TopLeft(37, 28)  };
    /// println!("{}", text.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. text:\t\t{:?}\ttext: {}", self.index, self.position, self.text)
    }
}

/// Representation of the unsharpen-command as a struct
pub struct CmdUnsharpen {
    /// Contains the index as u32 of arguments list
    pub index: u32,
    /// Contains the measure as f32 of how much to unsharpen by (is set to 0.0 if parsing fails)
    pub sigma: f32,
    /// Contains the threshold as i32 for the difference between
    pub threshold: i32,
}

impl CommandTrait for CmdUnsharpen {
    /// This function calls the actual unsharpen command, depending on the values given by the members of CmdUnsharpen-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdUnsharpen-struct
    /// * `image` - The GenericThumbnail to be unsharpened
    fn execute(&self, image: &mut dyn GenericThumbnail) {
        GenericThumbnail::unsharpen(image, self.sigma, self.threshold);
    }

    /// This function returns the index as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdUnsharpen-struct
    ///
    /// # Examples
    /// ```
    /// let index = 13;
    /// let unsharpen = CmdUnsharpen { index, sigma: 0.3, threshold: 2 };
    /// println!("index = {}", unsharpen.get_index());
    /// assert_eq!(unsharpen.get_index(), 13, "testing unsharpen.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the index and the values given by the members of CmdUnsharpen-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - The CmdUnsharpen-struct
    ///
    /// # Examples
    /// ```
    /// let unsharpen = CmdUnsharpen { index: 13, sigma: 0.3, threshold: 2 };
    /// println!("{}", unsharpen.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. unsharpen:\tsigma = {}\t\tthreshold = {}", self.index, self.sigma, self.threshold)
    }
}