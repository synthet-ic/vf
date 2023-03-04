pub enum FamilyClass {
    NoClassification,  // 0
    OldstyleSerifs(OldstyleSerifs),  // 1
    TransitionalSerifs(TransitionalSerifs),  // 2
}

pub enum OldstyleSerifs {
    NoClassification = 0,
    IBMRoundedLegibility = 1,
    Garalde = 2,
    Venetian = 3,
    ModifiedVenetian = 4,
    DutchModern = 5,
    DutchTraditional = 6,
    Contemporary = 7,
    Calligraphic = 8,
    Miscellaneous = 15
}

pub enum TransitionalSerifs {

}
