use thumbnailer::GenericThumbnail;

use crate::commands::Command;

/// Representation of the invert-command as a struct
pub struct CmdInvert {
    /// Contains the `index` as u32 of arguments list
    index: u32,
}

impl CmdInvert {
    /// Returns a new `CmdInvert` struct with defined:
    /// * `ìndex`: position in arguments list
    pub fn new(index: u32) -> Self {
        CmdInvert { index }
    }
}

impl Command for CmdInvert {
    /// This function calls the actual invert command.
    ///
    /// Returns the inverted `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdInvert`-struct
    /// * `image` - The `GenericThumbnail` to be inverted
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.invert()
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdInvert`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 9;
    /// let invert = CmdInvert::new(index);
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
    /// * `&self` - the `CmdInvert`-struct
    ///
    /// # Examples
    /// ```
    /// let invert = CmdBlur::new(9);
    /// println!("{}", invert.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. invert", self.index)
    }
}