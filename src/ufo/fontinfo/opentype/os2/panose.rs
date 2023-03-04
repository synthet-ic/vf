#[derive(Debug)]
pub enum Panose {
  LatinText(
      SerifStyle, Weight, Proportion, Contrast, StrokeVariation,
      ArmStyle, Letterform, Midline, XHeight),
  LatinHandWritten(
      ToolKind, Weight, Spacing, AspectRatio, Contrast, Topology, Form,
      Finials),
  LatinDecorative(),
  LatinSymbol()
}

#[derive(Debug)]
pub enum FamilyKind {
    Any = 0,
    NoFit = 1,
    LatinText = 2,
    LatinHandWritten = 3,
    LatinDecorative = 4,
    LatinSymbol = 5
}

#[derive(Debug)]
pub enum SerifStyle {
    Any = 0,
    NoFit = 1,
    Cove = 2,
    ObtuseCove = 3,
    SquareCove = 4,
    ObtuseSquareCove = 5,
    Square = 6,
    Thin = 7,
    Oval = 8,
    Exaggerated = 9,
    Triangle = 10,
    NormalSans = 11,
    ObtuseSans = 12,
    PerpendicularSans = 13,
    Flared = 14,
    Rounded = 15,
}

#[derive(Debug)]
pub enum Weight {
    Any = 0,
    NoFit = 1,
    VeryLight = 2,
    Light = 3,
    Thin = 4,
    Book = 5,
    Medium = 6,
    Demi = 7,
    Bold = 8,
    Heavy = 9,
    Black = 10,
    ExtraBlack = 11,
}

#[derive(Debug)]
pub enum Proportion {
    Any = 0,
    NoFit = 1,
    OldStyle = 2,
    Modern = 3,
    EvenWidth = 4,
    Extended = 5,
    Condensed = 6,
    VeryExtended = 7,
    VeryCondensed = 8,
    Monospaced = 9,
}

#[derive(Debug)]
pub enum Contrast {
    Any = 0,
    NoFit = 1,
    None = 2,
    VeryLow = 3,
    Low = 4,
    MediumLow = 5,
    Medium = 6,
    MediumHigh = 7,
    High = 8,
    VeryHigh = 9,
}

#[derive(Debug)]
pub enum StrokeVariation {
    Any = 0,
    NoFit = 1,
    NoVariation = 2,
    GradualDiagonal = 3,
    GradualTransitional = 4,
    GradualVertical = 5,
    GradualHorizontal = 6,
    RapidVertical = 7,
    RapidHorizontal = 8,
    InstantVertical = 9,
    InstantHorizontal = 10,
}

#[derive(Debug)]
pub enum ArmStyle {
    Any = 0,
    NoFit = 1,
    StraightArmsHorizontal = 2,
    StraightArmsWedge = 3,
    StraightArmsVertical = 4,
    StraightArmsSingleSerif = 5,
    StraightArmsDoubleSerif = 6,
    NonStraightHorizontal = 7,
    NonStraightWedge = 8,
    NonStraightVertical = 9,
    NonStraightSingleSerif = 10,
    NonStraightDoubleSerif = 11,
}

#[derive(Debug)]
pub enum Letterform {
    Any = 0,
    NoFit = 1,
    NormalContact = 2,
    NormalWeighted = 3,
    NormalBoxed = 4,
    NormalFlattened = 5,
    NormalRounded = 6,
    NormalOffCenter = 7,
    NormalSquare = 8,
    ObliqueContact = 9,
    ObliqueWeighted = 10,
    ObliqueBoxed = 11,
    ObliqueFlattened = 12,
    ObliqueRounded = 13,
    ObliqueOffCenter = 14,
    ObliqueSquare = 15,
}

#[derive(Debug)]
pub enum Midline {
    Any = 0,
    NoFit = 1,
    StandardTrimmed = 2,
    StandardPointed = 3,
    StandardSerifed = 4,
    HighTrimmed = 5,
    HighPointed = 6,
    HighSerifed = 7,
    ConstantTrimmed = 8,
    ConstantPointed = 9,
    ConstantSerifed = 10,
    LowTrimmed = 11,
    LowPointed = 12,
    LowSerifed = 13,
}

#[derive(Debug)]
pub enum XHeight {
    Any = 0,
    NoFit = 1,
    ConstantSmall = 2,
    ConstantStandard = 3,
    ConstantLarge = 4,
    DuckingSmall = 5,
    DuckingStandard = 6,
    DuckingLarge = 7,
}

#[derive(Debug)]
pub enum ToolKind {
    Any = 0,
    NoFit = 1,
    FlatNib = 2,
    PressurePoint = 3,
    Engraved = 4,
    Ball = 5,
    Brush = 6,
    Rough = 7,
    FeltPenBrushTip = 8,
    WildBrush = 9,
}

#[derive(Debug)]
pub enum Spacing {
    Any = 0,
    NoFit = 1,
    ProportionalSpaced = 2,
    Monospaced = 3,
}

#[derive(Debug)]
pub enum AspectRatio {
    Any = 0,
    NoFit = 1,
    VeryCondensed = 2,
    Condensed = 3,
    Normal = 4,
    Expanded = 5,
    VeryExpanded = 6,
}

#[derive(Debug)]
pub enum Topology {
    Any = 0,
    NoFit = 1,
    RomanDisconnected = 2,
    RomanTrailing = 3,
    RomanConnected = 4,
    CursiveDisconnected = 5,
    CursiveTrailing = 6,
    CursiveConnected = 7,
    BlackletterDisconnected = 8,
    BlackletterTrailing = 9,
    BlackletterConnected = 10,
}

#[derive(Debug)]
pub enum Form {
    Any = 0,
    NoFit = 1,
    UprightNoWrapping = 2,
    UprightSomeWrapping = 3,
    UprightMoreWrapping = 4,
    UprightExtremeWrapping = 5,
    ObliqueNoWrapping = 6,
    ObliqueSomeWrapping = 7,
    ObliqueMoreWrapping = 8,
    ObliqueExtremeWrapping = 9,
    ExaggeratedNoWrapping = 10,
    ExaggeratedSomeWrapping = 11,
    ExaggeratedMoreWrapping = 12,
    ExaggeratedExtremeWrapping = 13,
}

#[derive(Debug)]
pub enum Finials {
    Any = 0,
    NoFit = 1,
    NoneNoLoops = 2,
    NoneClosedLoops = 3,
    NoneOpenLoops = 4,
    SharpNoLoops = 5,
    SharpClosedLoops = 6,
    SharpOpenLoops = 7,
    TaperedNoLoops = 8,
    TaperedClosedLoops = 9,
    TaperedOpenLoops = 10,
    RoundNoLoops = 11,
    RoundClosedLoops = 12,
    RoundOpenLoops = 13,
}
