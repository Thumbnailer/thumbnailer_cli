use thumbnailer::GenericThumbnail;

use crate::commands::Command;

/// Representation of the brighten-command as a struct
pub struct CmdBrighten {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the amount as i32 to brighten each pixel by
    value: i32,
}

impl CmdBrighten {
    /// Returns a new `CmdBrighten` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `value`: i32 as amount to brighten each pixel by
    pub fn new(index: u32, value: i32) -> Self {
        CmdBrighten { index, value }
    }
}

impl Command for CmdBrighten {
    /// This function calls the actual brighten command, depending on the `value` given by `CmdBrighten`-struct.
    ///
    /// Returns the brightened `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdBrighten`-struct
    /// * `image` - The `GenericThumbnail` to be brightened
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.brighten(self.value)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdBrighten`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 2;
    /// let brighten = CmdBrighten::new(index, 3);
    /// println!("index = {}", brighten.get_index());
    /// assert_eq!(brighten.get_index(), 2, "testing brighten.get_index() with index = {}", index);
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the `value` given by `CmdBrighten`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdBrighten`-struct
    ///
    /// # Examples
    /// ```
    /// let brighten = CmdBrighten::new(2, 3);
    /// println!("{}", brighten.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. brighten:\t\tvalue = {}", self.index, self.value)
    }
}