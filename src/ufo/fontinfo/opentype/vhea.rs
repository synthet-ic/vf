//! - UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-vhea-table-fields>
//! - OpenType <https://learn.microsoft.com/en-gb/typography/opentype/spec/vhea>

/// OpenType `vhea` Table Fields
#[derive(Debug)]
pub struct VerticalHeader {
    /// Ascender value. Corresponds to the OpenType vhea table `vertTypoAscender` field.
    vert_typo_ascender: Option<i32>,
    ///
    vert_typo_descender: Option<i32>,
    ///
    vert_typo_line_gap: Option<i32>,
    ///
    caret_slope_rise: Option<i32>,
    ///
    caret_slope_run: Option<i32>,
    ///
    caret_offset: Option<i32>
}
