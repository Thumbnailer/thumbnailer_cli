use thumbnailer::{Crop, GenericThumbnail};

use crate::commands::Command;

/// Representation of the crop-command as a struct
pub struct CmdCrop {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `Crop` enum as option
    config: Crop,
}

impl CmdCrop {
    /// Returns a new `CmdCrop` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `config`: `Crop` enum as option
    pub fn new(index: u32, config: Crop) -> Self {
        CmdCrop { index, config }
    }
}

impl Command for CmdCrop {
    /// This function calls the actual crop command, depending on the values given by the members of `CmdCrop`-struct.
    ///
    /// Returns the cropped `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdCrop`-struct
    /// * `image` - The `GenericThumbnail` to be cropped
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.crop(self.config)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdCrop`-struct
    /// # Examples
    /// ```
    /// let index = 5;
    /// let crop = CmdCrop::new(index, Crop::Ratio(4, 3));
    /// println!("index = {}", crop.get_index());
    /// assert_eq!(crop.get_index(), 5, "testing crop.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the values given by the members of `CmdCrop`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdCrop`-struct
    ///
    /// # Examples
    /// ```
    /// let crop = CmdCrop::new(5, Crop::Ratio(4, 3));
    /// println!("{}", crop.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. crop:\t\t{:?}", self.index, self.config)
    }
}
