use thumbnailer::{GenericThumbnail, Orientation};

use crate::commands::Command;

/// Representation of the flip-command as a struct
pub struct CmdFlip {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `Orientation` enum as option
    orientation: Orientation,
}

impl CmdFlip {
    /// Returns a new `CmdFlip` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `orientation`: `Orientation` enum as option
    pub fn new(index: u32, orientation: Orientation) -> Self {
        CmdFlip { index, orientation }
    }
}

impl Command for CmdFlip {
    /// This function calls the actual flip command, depending on the value given by the `CmdFlip`-struct.
    ///
    /// Returns the flipped `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdFlip`-struct
    /// * `image` - The `GenericThumbnail` to be flipped
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.flip(self.orientation)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdFlip`-struct
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

    /// This function returns a formatted String, depending on the `index` and the value given by the `CmdFlip`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdFlip`-struct
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
