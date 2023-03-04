/*!
UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-hhea-table-fields>
OpenType <https://learn.microsoft.com/en-us/typography/opentype/spec/hhea>
*/

/// OpenType `hhea` Table Fields
#[derive(Debug)]
pub struct Hhea {
    ascender: i32,
    descender: i32,
    line_gap: i32,
}
