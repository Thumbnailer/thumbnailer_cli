use thumbnailer::GenericThumbnail;

use crate::commands::Command;

/// Representation of the huerotate-command as a struct
pub struct CmdHuerotate {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the amount as i32 in degree to rotate the image by
    degree: i32,
}

impl CmdHuerotate {
    /// Returns a new `CmdHuerotate` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `degree`: i32 as amount in degree to rotate the image by
    pub fn new(index: u32, degree: i32) -> Self {
        CmdHuerotate { index, degree }
    }
}

impl Command for CmdHuerotate {
    /// This function calls the actual huerotate command, depending on the `degree` given by the `CmdHuerotate`-struct.
    ///
    /// Returns the rotated `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdHuerotate`-struct
    /// * `image` - The `GenericThumbnail` to be rotated
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.huerotate(self.degree)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdHuerotate`-struct
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

    /// This function returns a formatted String, depending on the `index` and the `degree` given by the `CmdHuerotate`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdHuerotate`-struct
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
