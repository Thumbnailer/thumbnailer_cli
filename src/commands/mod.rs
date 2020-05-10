use std::cmp::Eq;
use std::cmp::Ordering;

use thumbnailer::GenericThumbnail;

pub use blur::CmdBlur;
pub use brighten::CmdBrighten;
pub use combine::CmdCombine;
pub use contrast::CmdContrast;
pub use crop::CmdCrop;
pub use exif::CmdExif;
pub use flip::CmdFlip;
pub use huerotate::CmdHuerotate;
pub use invert::CmdInvert;
pub use resize::CmdResize;
pub use resize_filter::CmdResizeFilter;
pub use text::CmdText;
pub use unsharpen::CmdUnsharpen;

// Include all submodules
pub mod blur;
pub mod brighten;
pub mod combine;
pub mod contrast;
pub mod crop;
pub mod exif;
pub mod flip;
pub mod huerotate;
pub mod invert;
pub mod resize;
pub mod resize_filter;
pub mod text;
pub mod unsharpen;

/// The `Command` trait.
///
/// This trait allows the dynamic implementation of the actual commands which apply modifications to the supplied image(s) by executing the appropriate operations provided by `thumbnailer`.
/// Passing the image(s) to the execute function should perform the desired modifications to it.
pub trait Command {
    /// Executes the operations of implementors of `Command`, which are provided by `thumbnailer`.
    ///
    /// Returns the modified `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self`: The command containing the `index` as u32 of arguments list (to use the same sequence in ascending order as specified by the user) and all necessary parameters
    /// * `image`: The `GenericThumbnail` to be modified
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail;

    /// This function returns the `index` as u32 of arguments list of implementors of `Command`.
    ///
    /// # Arguments
    ///
    /// * `&self`: The command containing the `index` as u32 of arguments list (to use the same sequence in ascending order as specified by the user) and all necessary parameters
    fn get_index(&self) -> u32;

    /// This function returns a formatted String, depending on the `index` and the values given by command-struct of implementors of `Command`.
    ///
    /// # Arguments
    ///
    /// * `&self`: The command containing the `index` as u32 of arguments list (to use the same sequence in ascending order as specified by the user) and all necessary parameters
    fn print(&self) -> String;
}

impl Ord for dyn Command {
    /// This method returns an [Ordering] between self and other.
    /// By convention, self.cmp(&other) returns the ordering matching the expression self <operator> other if true.
    /// (c) https://doc.rust-lang.org/std/cmp/index.html
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_index().cmp(&other.get_index())
    }
}

impl PartialOrd for dyn Command {
    /// This method returns an ordering between self and other values if one exists.
    /// (c) https://doc.rust-lang.org/std/cmp/index.html
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for dyn Command {
    /// This method tests for self and other values to be equal, and is used by ==.
    /// (c) https://doc.rust-lang.org/std/cmp/index.html
    fn eq(&self, other: &Self) -> bool {
        self.get_index() == other.get_index()
    }
}

impl Eq for dyn Command {}