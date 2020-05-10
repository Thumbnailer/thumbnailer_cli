use thumbnailer::{Exif, GenericThumbnail};

use crate::commands::Command;

/// Representation of the exif-command as a struct
pub struct CmdExif {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `Exif` enum as option
    metadata: Exif,
}

impl CmdExif {
    /// Returns a new `CmdExif` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `metadata`: `Exif` enum as option
    pub fn new(index: u32, metadata: Exif) -> Self {
        CmdExif { index, metadata }
    }
}

impl Command for CmdExif {
    /// This function calls the actual exif command, depending on the values given by the members of `CmdExif`-struct.
    ///
    /// Returns the `GenericThumbnail` in which the `metadata` has been copied
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdExif`-struct
    /// * `image` - The `GenericThumbnail` in which the `metadata` should be copied
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.exif(self.metadata.clone())
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdExif`-struct
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

    /// This function returns a formatted String, depending on the `index` and the values given by the members of `CmdExif`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdExif`-struct
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
