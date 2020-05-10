use thumbnailer::{GenericThumbnail, ResampleFilter, Resize};

use crate::commands::Command;

/// Representation of the resizeFilter-command as a struct
pub struct CmdResizeFilter {
    /// Contains the `index` as u32 of arguments list
    index: u32,
    /// Contains the `Resize` enum as option
    size: Resize,
    /// Contains the `ResampleFilter` enum as option
    filter: ResampleFilter,
}

impl CmdResizeFilter {
    /// Returns a new `CmdResizeFilter` struct with defined:
    /// * `ìndex`: position in arguments list
    /// * `size`: `Resize` enum as option
    /// * `filter`: `ResampleFilter` enum as option
    pub fn new(index: u32, size: Resize, filter: ResampleFilter) -> Self {
        CmdResizeFilter {
            index,
            size,
            filter,
        }
    }
}

impl Command for CmdResizeFilter {
    /// This function calls the actual resize_filter command, depending on the values given by the members of `CmdResizeFilter`-struct.
    ///
    /// Returns the, with the given filter, resized `GenericThumbnail`
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdResizeFilter`-struct
    /// * `image` - The `GenericThumbnail` to be resized with a given filter
    fn execute<'s>(&self, image: &'s mut dyn GenericThumbnail) -> &'s mut dyn GenericThumbnail {
        image.resize_filter(self.size, self.filter)
    }

    /// This function returns the `index` as u32 of arguments list.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdResizeFilter`-struct
    ///
    /// # Examples
    /// ```
    /// let index = 11;
    /// let resize_filter = CmdResizeFilter::new(index, Resize::BoundingBox(400, 300), ResampleFilter::Nearest);
    /// println!("index = {}", resize_filter.get_index());
    /// assert_eq!(resize_filter.get_index(), 11, "testing resize_filter.get_index() with index = {}", index);
    /// ```
    fn get_index(&self) -> u32 {
        self.index
    }

    /// This function returns a formatted String, depending on the `index` and the values given by the members of `CmdResizeFilter`-struct.
    ///
    /// # Arguments
    ///
    /// * `&self` - the `CmdResizeFilter`-struct
    ///
    /// # Examples
    /// ```
    /// let resize_filter = CmdResizeFilter::new(11, Resize::BoundingBox(400, 300), ResampleFilter::Nearest);
    /// println!("{}", resize_filter.print());
    /// ```
    fn print(&self) -> String {
        format!(
            "► {:02}. resize_filter:\t{:?}\tfilter: {:?}",
            self.index, self.size, self.filter
        )
    }
}
