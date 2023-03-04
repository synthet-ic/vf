//! - UFO3 <https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-head-table-fields>
//! - OpenType <https://learn.microsoft.com/en-gb/typography/opentype/spec/head>

// use std::time::Duration;

/// OpenType `head` Table Fields
#[derive(Debug)]
pub struct Head {
    /**
    Creation date. Expressed as a string of the format “YYYY/MM/DD HH:MM:SS”. “YYYY/MM/DD” is year/month/day. The month must be in the range 1-12 and the day must be in the range 1-end of month. “HH:MM:SS” is hour:minute:second. The hour must be in the range 0:23. The minute and second must each be in the range 0-59. The timezone is UTC.
    */
    created: String,
    /**
    Smallest readable size in pixels. Corresponds to the OpenType head table `lowestRecPPEM` field.
    */
    lowest_rec_ppem: Option<u16>,
    /**
    A list of bit numbers indicating the flags. The bit numbers are listed in the OpenType head specification. Corresponds to the OpenType head table `flags` field.
    */
    flags: Option<Vec<Flag>>
}

#[derive(Debug)]
pub enum Flag {
    Baseline = 0,
}
