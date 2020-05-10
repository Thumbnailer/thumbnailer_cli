use thumbnailer::GenericThumbnail;

use crate::commands::Command;

/// Representation of the unsharpen-command as a struct
pub struct CmdUnsharpen {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the measure as f32 of how much to unsharpen by
    sigma: f32,
    /// Contains the threshold as i32 controls the minimal brightness change or how far apart adjacent tonal values have to be before the filter does anything
    threshold: i32,
}

impl CmdUnsharpen {
    /// Returns a new `CmdBrighten` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `sigma`: f32 as measure of how much to unsharpen by
    /// * `threshold`: i32 controls the minimal brightness change or how far apart adjacent tonal values have to be before the filter does anything
    pub fn new(index: u32, sigma: f32, threshold: i32) -> Self {
        CmdUnsharpen { index, sigma, threshold }
    }
}

impl Command for CmdUnsharpen {
    /// This function calls the actual unsharpen command, depending on the values given by the members of `CmdUnsharpen`-struct.
    ///
    /// Returns the unsharpened `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdUnsharpen`-struct
    /// * `image` - The `GenericThumbnail` to be unsharpened
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.unsharpen(self.sigma, self.threshold)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdUnsharpen`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 14;
    /// let unsharpen = CmdUnsharpen::new(index, 0.3, 2);
    /// println!("index = {}", unsharpen.get_index());
    /// assert_eq!(unsharpen.get_index(), 14, "testing unsharpen.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the values given by the members of `CmdUnsharpen`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdUnsharpen`-struct
    ///
    /// # Examples
    /// ```
    /// let unsharpen = CmdUnsharpen::new(14, 0.3, 2);
    /// println!("{}", unsharpen.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. unsharpen:\tsigma = {}\t\tthreshold = {}", self.index, self.sigma, self.threshold)
    }
}