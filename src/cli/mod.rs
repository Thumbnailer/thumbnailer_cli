pub use arguments::get_matches;
pub use parser::read_commands;

use crate::commands::Command;

pub mod arguments;
pub mod parser;

pub const NAME_FILE_IN: &str = "INPUT_PATH";
pub const NAME_FILE_OUT: &str = "OUTPUT_PATH";

const ARG_BLUR: &str = "blur";
const ARG_BRIGHTEN: &str = "brighten";
const ARG_CONTRAST: &str = "contrast";
const ARG_COMBINE_TL: &str = "combine_tl";
const ARG_COMBINE_TR: &str = "combine_tr";
const ARG_COMBINE_BL: &str = "combine_bl";
const ARG_COMBINE_BR: &str = "combine_br";
const ARG_CROP_BOX: &str = "crop_box";
const ARG_CROP_RATIO: &str = "crop_ratio";
const ARG_EXIF: &str = "exif";
const ARG_FLIP_HORIZONTAL: &str = "flip_horizontal";
const ARG_FLIP_VERTICAL: &str = "flip_vertical";
const ARG_HUEROTATE: &str = "huerotate";
const ARG_INVERT: &str = "invert";
const ARG_RESIZE: &str = "resize";
const ARG_RESIZE_N: &str = "resize_n";
const ARG_RESIZE_T: &str = "resize_t";
const ARG_RESIZE_C: &str = "resize_c";
const ARG_RESIZE_G: &str = "resize_g";
const ARG_RESIZE_L: &str = "resize_l";
const ARG_ROTATE90: &str = "rotate90";
const ARG_ROTATE180: &str = "rotate180";
const ARG_ROTATE270: &str = "rotate270";
const ARG_TEXT_TL: &str = "text_tl";
const ARG_TEXT_TR: &str = "text_tr";
const ARG_TEXT_BL: &str = "text_bl";
const ARG_TEXT_BR: &str = "text_br";
const ARG_UNSHARPEN: &str = "unsharpen";
const ARG_PRESET: &str = "preset";

const GROUP_COMBINE: &str = "Combine";
const GROUP_CROP: &str = "Crop";
const GROUP_FLIP: &str = "Flip";
const GROUP_RESIZE: &str = "Resize";
const GROUP_ROTATE: &str = "Rotate";
const GROUP_TEXT: &str = "Text";

const VAL_COMBINE: [&str; 3] = ["IMAGE_PATH", "x_offset", "y_offset"];
const VAL_RESIZE: [&str; 3] = ["nwidth", "nheight", "exact"];
const VAL_TEXT: [&str; 3] = ["text", "x_offset", "y_offset"];

const PRESETS: [&str; 3] = ["app_copyright", "full_hd", "background"];

/// Representation of the command-list as a struct
pub struct Commands {
    /// Contains the implementors of `Command` to apply a list of operations, which are provided by `thumbnailer`, on the supplied image(s)
    pub(crate) commands: Vec<Box<dyn Command>>,
}
