//! - UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#woff-data>
//! - W3C <https://www.w3.org/TR/WOFF/>


/// WOFF Data
pub struct Woff {
    major_version: u8,
    minor_version: u8,
    metadata_unique_id: UniqueID,
    metadata_vendor: Vendor,
    metadata_credits: Credit,
    metadata_description: Description,
    metadata_licence: Licence,
    metadata_copyright: Copyright,
    metadata_trademark: Trademark,
    metadata_licensee: Licensee,
    metadata_extensions: Vec<Extension>
}

pub struct UniqueID {
    id: String,
}

pub struct Vendor {
    name: String,
    url: Option<String>,
    dir: Option<String>,
    class: Option<String>,
}

pub struct Credit {
    name: String,
    url: Option<String>,
    role: Option<String>,
    dir: Option<String>,
    class: Option<String>,
}

pub struct Description {
    url: Option<String>,
    text: Vec<Text>
}

pub struct Licence {
    url: Option<String>,
    id: Option<String>,
    text: Vec<Text>
}

pub struct Copyright {
    text: Vec<Text>
}

pub struct Trademark {

}

pub struct Licensee {

}

pub struct Text {

}

pub struct Extension {

}

pub struct ExtensioneItem {
    
}

pub struct ExtensionName {

}

pub struct ExtensionValue {

}

pub struct Guideline {

}
