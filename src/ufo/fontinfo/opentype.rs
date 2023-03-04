pub mod gasp;
pub mod head;
pub mod hhea;
pub mod name;
pub mod os2;
pub mod vhea;

use gasp::Gasp;
use head::Head;
use hhea::Hhea;
use name::Name;
use os2::OS2;
use vhea::VerticalHeader;

#[derive(Debug)]
pub struct Opentype {
    gasp: Option<Gasp>,
    head: Head,
    hhea: Hhea,
    name: Name,
    os2: OS2,
    vhea: Option<VerticalHeader>
}
