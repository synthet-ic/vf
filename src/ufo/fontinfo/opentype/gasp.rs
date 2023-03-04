//! UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-gasp-table-fields>
//! OpenType <https://learn.microsoft.com/en-us/typography/opentype/spec/gasp>

/// OpenType `gasp` Table Fields
#[derive(Debug)]
pub struct Gasp {
    ranges: Vec<Range>
}

#[derive(Debug)]
pub struct Range {
    max_ppem: u16,
    gasp_behaviour: Vec<RangeGaspBehaviour>
}

#[derive(Debug)]
pub enum RangeGaspBehaviour {
    Gridfit = 0,
    Dogray = 1,
    SymmetricGridfit = 2,
    SymmetricSmoothing = 3
}
