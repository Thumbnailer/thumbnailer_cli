use thumbnailer::{GenericThumbnail, Resize};

use crate::commands::Command;

/// Representation of the resize-command as a struct
pub struct CmdResize {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `Resize` enum as option
    size: Resize,
}

impl CmdResize {
    /// Returns a new `CmdResize` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `size`: `Resize` enum as option
    pub fn new(index: u32, size: Resize) -> Self {
        CmdResize { index, size }
    }
}

impl Command for CmdResize {
    /// This function calls the actual resize command, depending on the values given by the members of `CmdResize`-struct.
    ///
    /// Returns the resized `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdResize`-struct
    /// * `image` - The `GenericThumbnail` to be resized
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.resize(self.size)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdResize`-struct
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

    /// This function returns a formatted String, depending on the `index` and the values given by the members of `CmdResize`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdResize`-struct
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