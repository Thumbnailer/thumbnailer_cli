use thumbnailer::{GenericThumbnail, Rotation};

use crate::commands::Command;

/// Representation of the rotate-command as a struct
pub struct CmdRotate {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `Rotation` enum as option
    rotation: Rotation,
}

impl CmdRotate {
    /// Returns a new `CmdRotate` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `rotation`: `Rotation` enum as option
    pub fn new(index: u32, rotation: Rotation) -> Self {
        CmdRotate { index, rotation }
    }
}

impl Command for CmdRotate {
    /// This function calls the actual rotate command, depending on the value given by the `CmdRotate`-struct.
    ///
    /// Returns the rotated `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdRotate`-struct
    /// * `image` - The `GenericThumbnail` to be rotated
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.rotate(self.rotation)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdRotate`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 12;
    /// let rotate = CmdRotate::new(index, Rotation::Rotate90);
    /// println!("index = {}", rotate.get_index());
    /// assert_eq!(rotate.get_index(), 12, "testing rotate.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the value given by the `CmdRotate`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdRotate`-struct
    ///
    /// # Examples
    /// ```
    /// let rotate = CmdRotate::new(12, Rotation::Rotate90);
    /// println!("{}", rotate.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. rotae:\t\trotation = {:?}", self.index, self.rotation)
    }
}
