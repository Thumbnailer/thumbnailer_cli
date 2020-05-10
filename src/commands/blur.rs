use thumbnailer::GenericThumbnail;

use crate::commands::Command;

/// Representation of the blur-command as a struct
pub struct CmdBlur {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the measure as f32 of how much to blur by
    sigma: f32,
}

impl CmdBlur {
    /// Returns a new `CmdBlur` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `sigma`: f32 as measure of how much to blur by, more Information: [Gaussian Blur](https://en.wikipedia.org/wiki/Gaussian_blur)
    pub fn new(index: u32, sigma: f32) -> Self {
        CmdBlur { index, sigma }
    }
}

impl Command for CmdBlur {
    /// This function calls the actual blur command, depending on the `sigma` given by `CmdBlur`-struct.
    ///
    /// Returns the blurred `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdBlur`-struct
    /// * `image` - The `GenericThumbnail` to be blurred
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.blur(self.sigma)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdBlur`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 1;
    /// let blur = CmdBlur::new(index, 0.5);
    /// println!("index = {}", blur.get_index());
    /// assert_eq!(blur.get_index(), 1, "testing blur.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the `sigma` given by `CmdBlur`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdBlur`-struct
    ///
    /// # Examples
    /// ```
    /// let blur = CmdBlur::new(1, 0.5);
    /// println!("{}", blur.print());
    /// ```
    fn print(&self) -> String {
        format!("► {:02}. blur:\t\tsigma = {}", self.index, self.sigma)
    }
}