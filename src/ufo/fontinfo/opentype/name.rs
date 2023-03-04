//! - UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-name-table-fields>
//! - OpenType <https://learn.microsoft.com/en-us/typography/opentype/spec/name>

/// OpenType `name` Table Fields
#[derive(Debug)]
pub struct Name {
    designer: String,
    designer_url: String,
    manufacturer: String,
    manufacturer_url: String
}
