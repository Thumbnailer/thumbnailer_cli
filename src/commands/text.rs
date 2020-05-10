use thumbnailer::{BoxPosition, GenericThumbnail};

use crate::commands::Command;

/// Representation of the text-command as a struct
pub struct CmdText {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the text as String to print into the supplied image(s)
    text: String,
    /// Contains the `BoxPosition` enum as option
    position: BoxPosition,
}

impl CmdText {
    /// Returns a new `CmdText` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `text`: String to print into the supplied image(s)
    /// * `value`: `BoxPosition` enum as option
    pub fn new(index: u32, text: String, position: BoxPosition) -> Self {
        CmdText {
            index,
            text,
            position,
        }
    }
}

impl Command for CmdText {
    /// This function calls the actual text command, depending on the values given by the members of `CmdText`-struct.
    ///
    /// Returns the `GenericThumbnail` in which the `text` has been inserted
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdText`-struct
    /// * `image` - The `GenericThumbnail` in which the `text` should be inserted
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.text(self.text.clone(), self.position)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdText`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 13;
    /// let text = CmdText::new(index, String::from("(c) Thumbnailer"), BoxPosition::TopLeft(37, 28));
    /// println!("index = {}", text.get_index());
    /// assert_eq!(text.get_index(), 13, "testing text.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the values given by the members of `CmdText`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdText`-struct
    ///
    /// # Examples
    /// ```
    /// let text = CmdText::new(13, String::from("(c) Thumbnailer"), BoxPosition::TopLeft(37, 28));
    /// println!("{}", text.print());
    /// ```
    fn print(&self) -> String {
        format!(
            "► {:02}. text:\t\t{:?}\ttext: {}",
            self.index, self.position, self.text
        )
    }
}
