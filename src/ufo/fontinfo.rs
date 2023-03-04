/*!
<https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/>
*/

pub mod guidelines;
pub mod opentype;
pub mod postscript;
pub mod woff;

use opentype::Opentype;
use postscript::Postscript;

#[derive(Debug)]
pub struct FontInfo {
    identification: Identification,
    legal: Legal,
    dimension: Dimension,
    misc: Option<Miscellaneous>,
    opentype: Opentype,
    postscript: Postscript,
}

/**
Generic Identification Information
<https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-identification-information>
*/
#[derive(Debug)]
pub struct Identification {
    /**
    Family name. Note: The specification is agnostic about how this value relates to openTypeNamePreferredFamilyName.
    */
    family_name: String,
    /**
    Style name. Note: The specification is agnostic about how this value relates to openTypeNamePreferredSubfamilyName.
    */
    style_name: String,
    /**
    Family name used for bold, italic and bold italic style mapping.
    */
    style_map_family_name: Option<String>,
    /**
    Style map style. The possible values are regular, italic, bold and bold italic. These are case sensitive.
    */
    style_map_style_name: Option<String>,
    /**
    Major version.
    */
    version_major: u8,
    /**
    Minor version.
    */
    version_minor: u8,
    /**
    The year the font was created. This attribute is deprecated as of version 2. Its presence should not be relied upon by authoring tools. However, it may occur in a font’s info so authoring tools should preserve it if present.
    */
    year: u16
}

/**
Generic Legal Information
<https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-legal-information>
*/
#[derive(Debug)]
pub struct Legal {
    /// Copyright statement.
    copyright: String,
    /// Trademark statement.
    trademark: String
}

/**
Generic Dimension Information
<https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-dimension-information>
*/
#[derive(Debug)]
pub struct Dimension {
    /**
    Units per em.
    */
    units_per_em: f32,
    /**
    Descender value. Note: The specification is agnostic about the relationship to the more specific vertical metric values.
    */
    descender: f32,
    /**
    x-height value.
    */
    pub x_height: f32,
    /**
    Cap height value.
    */
    pub cap_height: f32,
    /**
    Ascender value. Note: The specification is agnostic about the relationship to the more specific vertical metric values.
    */
    ascender: f32,
    /**
    Italic angle. This must be an angle in counter-clockwise degrees from the vertical.
    */
    italic_angle: f32,
}

/**
Generic Miscellaneous Information
<https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-miscellaneous-information>
*/
#[derive(Debug)]
pub struct Miscellaneous {
    /// Arbitrary note about the font.
    note: Option<String>
}
