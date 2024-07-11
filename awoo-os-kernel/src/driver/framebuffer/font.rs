use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

/// Additional vertical space between lines
pub const LINE_SPACING: usize = 2;

/// Additional horizontal space between characters.
pub const LETTER_SPACING: usize = 0;

/// Padding from the border. Prevent that font is too close to border.
pub const BORDER_PADDING: usize = 1;

/// Height of each char raster. The font size is ~0.84% of this. Thus, this is the line height that
/// enables multiple characters to be side-by-side and appear optically in one line in a natural way.
pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

/// The width of each single symbol of the mono space font.
pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

/// Backup character if a desired symbol is not available by the font.
/// The '�' character requires the feature "unicode-specials".
pub const BACKUP_CHAR: char = '�';

pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

/// Returns the raster of the given char or the raster of [`font_constants::BACKUP_CHAR`].
pub fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}
