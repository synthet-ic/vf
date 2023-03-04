//! UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#guidelines>

#[derive(Debug)]
pub struct Guideline {
    x: f32,
    y: f32,
    angle: f32,
    name: Option<String>,
    colour: Option<String>,
    identifier: Option<String>
}
