//! <https://unifiedfontobject.org/versions/ufo3/glyphs/glif/>

use kfl::{Decode, DecodeScalar};
use kfl_plist::Dict;

#[derive(Debug, Decode)]
pub struct Glyph {
    #[kfl(property)]
    pub name: String,
    #[kfl(property)]
    format: String,
    #[kfl(child, default)]
    advance: Option<Advance>,
    #[kfl(child)]
    pub unicode: Option<Unicode>,
    // // image: Image,
    #[kfl(children)]
    guidelines: Vec<Guideline>,
    #[kfl(children)]
    anchors: Vec<Anchor>,
    #[kfl(child)]
    outline: Option<Outline>,
    // note: Option<String>,
    #[kfl(child)]
    pub lib: Option<Dict>
}

#[derive(Debug, Decode, Default)]
pub struct Advance {
    #[kfl(property, default)]
    width: Option<f32>,
    #[kfl(property, default)]
    height: Option<f32>
}

#[derive(Debug, Decode)]
pub struct Unicode {
    #[kfl(property)]
    hex: String
}

#[derive(Debug, Decode)]
pub struct Image {
    file_name: String
}

#[derive(Debug, Decode)]
pub struct Guideline {
    #[kfl(property)]
    x: f32,
    #[kfl(property)]
    y: f32,
    #[kfl(property)]
    angle: Option<f32>,
    #[kfl(property)]
    name: Option<String>,
    #[kfl(property)]
    colour: Option<String>,
    #[kfl(property)]
    identifier: Option<String>
}

#[derive(Debug, Decode)]
pub struct Anchor {
    #[kfl(property, default)]
    x: Option<f32>,
    #[kfl(property, default)]
    y: Option<f32>,
    #[kfl(property)]
    name: Option<String>,
    #[kfl(property)]
    colour: Option<String>,
    #[kfl(property)]
    identifier: Option<String>
}

#[derive(Debug, Decode)]
pub struct Outline {
    #[kfl(children)]
    components: Vec<Component>,
    #[kfl(children)]
    contours: Vec<Contour>
}

#[derive(Debug, Decode)]
pub struct Component {
    #[kfl(property)]
    base: Option<String>,
    #[kfl(property, default)]
    x_scale: Option<i32>,
    #[kfl(property, default)]
    y_scale: Option<i32>,
    #[kfl(property, default)]
    x_offset: Option<i32>,
    #[kfl(property, default)]
    y_offset: Option<i32>,
    #[kfl(property, default)]
    identifier: Option<String>
}

#[derive(Debug, Decode)]
pub struct Contour {
    #[kfl(property)]
    identifier: Option<String>,
    points: Vec<Point>
}

#[derive(Debug, Decode)]
pub struct Point {
    #[kfl(property, default)]
    x: Option<f32>,
    #[kfl(property, default)]
    y: Option<f32>,
    #[kfl(property, default)]
    r#type: PointType,
    #[kfl(property)]
    smooth: Option<bool>,
    #[kfl(property)]
    name: Option<String>,
    #[kfl(property)]
    identifier: Option<String>
}

#[derive(Debug, DecodeScalar, Default)]
pub enum PointType {
    Move,
    Line,
    #[default]
    Offcurve,
    Curve,
    Qcurve
}
