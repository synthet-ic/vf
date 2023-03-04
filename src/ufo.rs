//! <https://unifiedfontobject.org/>
//! <https://unifiedfontobject.org/versions/ufo3/>

#![allow(special_module_name)]

pub mod designspace;
pub mod fontinfo;
pub mod glyphs;
pub mod groups;
pub mod kerning;
pub mod layercontents;
pub mod lib;
pub mod metainfo;

use std::{fs::DirEntry, collections::HashMap};
use kfl_plist::Dict;

pub use designspace::Designspace;
pub use fontinfo::FontInfo;
pub use glyphs::{
    glif::Glyph
};
pub use groups::Groups;
pub use kerning::Kerning;
pub use layercontents::LayerContents;
pub use lib::Lib;
pub use metainfo::MetaInfo;

#[derive(Debug)]
pub struct UFO {
    pub meta_info: MetaInfo,
    pub font_info: FontInfo,
    pub groups: Groups,
    pub kerning: Kerning,
    pub lib: Lib,
    pub layers: HashMap<&'static str, Layer>,
    pub glyphs: Vec<Glyph>,
    pub data: DirEntry  // TODO 
}

#[derive(Debug)]
pub struct Layer {
    pub glyphs: Vec<Glyph>,
    pub lib: Dict
}
