use thumbnailer::GenericThumbnail;

use crate::commands::Command;

/// Representation of the contrast-command as a struct
pub struct CmdContrast {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the amount as f32 to adjust the contrast by
    value: f32,
}

impl CmdContrast {
    /// Returns a new `CmdContrast` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `value`: f32 as amount to adjust the contrast by
    pub fn new(index: u32, value: f32) -> Self {
        CmdContrast { index, value }
    }
}

impl Command for CmdContrast {
    /// This function calls the actual contrast command, depending on the `value` given by `CmdContrast`-struct.
    ///
    /// Returns the `GenericThumbnail` whose contrast has been changed
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdContrast`-struct
    /// * `image` - The `GenericThumbnail` whose contrast is to be changed
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.contrast(self.value)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdContrast`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 3;
    /// let contrast = CmdContrast::new(index, 0.8);
    /// println!("index = {}", contrast.get_index());
    /// assert_eq!(contrast.get_index(), 3, "testing contrast.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the `value` given by `CmdContrast`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdContrast`-struct
    ///
    /// # Examples
    /// ```
    /// let contrast = CmdContrast::new(3, 0.8);
    /// println!("{}", contrast.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. contrast:\tvalue = {}", self.index, self.value)
    }
}
