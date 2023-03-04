//! - UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-os2-table-fields>
//! - OpenType <https://learn.microsoft.com/en-us/typography/opentype/spec/os2>

pub mod ibmfc;
pub mod panose;

use panose::Panose;

/// OpenType `OS/2` Table Fields
#[derive(Debug)]
pub struct OS2 {
    /// Width class value. Must be in the range 1-9. Corresponds to the OpenType OS/2 table `usWidthClass` field.
    width_class: Option<u8>,
    /// Weight class value. Corresponds to the OpenType OS/2 table `usWeightClass` field.
    weight_class: Option<u16>,
    /// A list of bit numbers indicating the bits that should be set in fsSelection. The bit numbers are listed in the OpenType OS/2 specification. Corresponds to the OpenType OS/2 table `selection` field. Note: Bits 0 (italic), 5 (bold) and 6 (regular) must not be set here. These bits should be taken from the generic styleMapStyleName attribute.
    selection: Option<Vec<Selection>>,
    vendor_id: Option<String>,
    /// The list must contain 10 non-negative integers that represent the setting for each category in the Panose specification. The integers correspond with the option numbers in each of the Panose categories. This corresponds to the OpenType OS/2 table `Panose` field.
    panose: Option<Panose>,
    /// Two integers representing the IBM font class and font subclass of the font. The first number, representing the class ID, must be in the range 0-14. The second number, representing the subclass, must be in the range 0-15. The numbers are listed in the OpenType OS/2 specification. Corresponds to the OpenType OS/2 table `sFamilyClass` field.
    family_class: Option<(i16, i16)>,
    /// A list of bit numbers that are supported Unicode ranges in the font. The bit numbers are listed in the OpenType OS/2 specification. Corresponds to the OpenType OS/2 table `ulUnicodeRange1`, `ulUnicodeRange2`, `ulUnicodeRange3` and `ulUnicodeRange4` fields.
    unicode_ranges: Option<(u32, u32, u32, u32)>,
    /// A list of bit numbers that are supported code page ranges in the font. The bit numbers are listed in the OpenType OS/2 specification. Corresponds to the OpenType OS/2 table `ulCodePageRange1` and `ulCodePageRange2` fields.
    code_page_ranges: Option<(u32, u32)>,
    typo_ascender: i16,
    typo_descender: i16,
    typo_line_gap: i16,
    win_ascent: u16,
    win_descent: u16,
    r#type: Vec<u16>,
    /// Subscript horizontal font size. Corresponds to the OpenType OS/2 table `ySubscriptXSize` field.
    subscript_x_size: Option<i16>,
    /// Subscript vertical font size. Corresponds to the OpenType OS/2 table `ySubscriptYSize` field.
    subscript_y_size: Option<i16>
}

#[derive(Debug)]
pub enum Selection {
    Italic = 0,
    Underscore = 1,
    Negative = 2,
    Outlined = 3,
    Strikeout = 4,
    Bold = 5,
    Regular = 6,
    UseTypoMetrics = 7,
    Wws = 8,
    Oblique = 9
}

// #[derive(Serialize, Deserialize)]
// pub enum Type {

// }
