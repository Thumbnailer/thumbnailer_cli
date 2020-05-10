use thumbnailer::{BoxPosition, GenericThumbnail, StaticThumbnail};

use crate::commands::Command;

/// Representation of the combine-command as a struct
pub struct CmdCombine {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `StaticThumbmnail` struct as image
    image: StaticThumbnail,
    /// Contains the `BoxPosition` enum as option
    position: BoxPosition,
}

impl CmdCombine {
    /// Returns a new `CmdCombine` struct with defined:
    /// * `ìndex`: position of arguments list
    /// * `image`: `StaticThumbmnail` struct as image
    /// * `position`: `BoxPosition` enum as option
    pub fn new(index: u32, image: StaticThumbnail, position: BoxPosition) -> Self {
        CmdCombine { index, image, position }
    }
}

impl Command for CmdCombine {
    /// This function calls the actual combine command, depending on the values given by the members of `CmdCombine`-struct.
    ///
    /// Returns the `GenericThumbnail` in which the photo has been inserted
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdCombine`-struct
    /// * `image` - The `GenericThumbnail` in which the photo should be inserted
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.combine(self.image.clone(), self.position)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdCombine`-struct
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

    /// This function returns a formatted String, depending on the values given by the members of `CmdCombine`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdCombine`-struct
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