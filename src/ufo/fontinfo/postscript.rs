/*!
UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#postscript-specific-data>
<https://adobe-type-tools.github.io/font-tech-notes/pdfs/T1_SPEC.pdf>
*/

/// PostScript Specific Data
#[derive(Debug)]
pub struct Postscript {
    /**
    Name to be used for the `FontName` field in Type 1/CFF table.
    */
    font_name: Option<String>,
    /**
    Name to be used for the `FullName` field in Type 1/CFF table.
    */
    full_name: Option<String>,
    /**
    Artificial slant angle. This must be an angle in counter-clockwise degrees from the vertical. This value is not the same as the italic angle. Font authoring tools may use this value to set the FontMatrix in Type 1/CFF table.
    */
    slant_angle: Option<f32>,
    /**
    A unique ID number as defined in the Type 1/CFF specification.
    */
    unique_id: Option<i32>,
    /**
    Underline thickness value. Corresponds to the Type 1/CFF/post table `UnderlineThickness` field.
    */
    underline_thickness: f32,
    /**
    Underline position value. Corresponds to the Type 1/CFF/post table `UnderlinePosition` field.
    */
    underline_position: f32,
    /**
    Indicates if the font is monospaced. An authoring tool could calculate this automatically, but the designer may wish to override this setting. This corresponds to the Type 1/CFF `isFixedPitched` field
    */
    is_fixed_pitch: Option<bool>,
    /**
    A list of up to 14 integers or floats specifying the values that should be in the Type 1/CFF BlueValues field. This list must contain an even number of integers following the rules defined in the Type 1/CFF specification.
    */
    blue_values: Vec<f32>,
    /**
    A list of up to 10 integers or floats specifying the values that should be in the Type 1/CFF OtherBlues field. This list must contain an even number of integers following the rules defined in the Type 1/CFF specification.
    */
    other_blues: Vec<f32>,
    /**
    A list of up to 14 integers or floats specifying the values that should be in the Type 1/CFF FamilyBlues field. This list must contain an even number of integers following the rules defined in the Type 1/CFF specification.
    */
    family_blues: Vec<f32>,
    /**
    A list of up to 10 integers or floats specifying the values that should be in the Type 1/CFF FamilyOtherBlues field. This list must contain an even number of integers following the rules defined in the Type 1/CFF specification.
    */
    family_other_blues: Vec<f32>,
    /**
    List of horizontal stems sorted in the order specified in the Type 1/CFF specification. Up to 12 integers or floats are possible. This corresponds to the Type 1/CFF `StemSnapH` field.
    */
    stem_snap_h: Vec<f32>,
    /**
    List of vertical stems sorted in the order specified in the Type 1/CFF specification. Up to 12 integers or floats are possible. This corresponds to the Type 1/CFF `StemSnapV` field.
    */
    stem_snap_v: Vec<f32>,
    blue_fuzz: Option<f32>,
    blue_shift: Option<f32>,
    blue_scale: Option<f32>,
    force_bold: Option<bool>,
    default_width_x: Option<f32>,
    nominal_width_x: Option<f32>,
    weight_name: Option<String>,
    default_character: Option<String>,
    windows_character_set: Option<i32>
}
