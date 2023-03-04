//! <https://fonttools.readthedocs.io/en/latest/designspaceLib/index.html>

use std::path::PathBuf;

use kfl::Decode;
use kfl_plist::Dict;

#[derive(Debug, Decode)]
pub struct Designspace {
    #[kfl(property, default = "5.0".into())]
    pub format: String,
    #[kfl(child)]
    pub axes: Axes,
    #[kfl(child)]
    pub labels: Labels,
    #[kfl(child)]
    pub sources: Sources,
    #[kfl(child, default)]
    pub variable_fonts: Option<VariableFonts>,
    #[kfl(child, default)]
    pub instances: Option<Instances>,
    #[kfl(child, default)]
    pub rules: Option<Rules>,
    #[kfl(child, default)]
    pub lib: Option<Lib>
}

#[derive(Debug, Default, Decode)]
pub struct Axes {
    #[kfl(children)]
    pub axes: Vec<Axis>,
    #[kfl(property, default)]
    pub elided_fallback_name: Option<String>
}

#[derive(Debug, Decode)]
pub struct Axis {
    /// Name of the axis that is used in the location elements.
    #[kfl(property)]
    pub name: String,
    /// 4 letters. Some axis tags are registered in the OpenType Specification.
    #[kfl(property)]
    pub tag: String, 
    /// The default value for this axis, in user space coordinates.
    #[kfl(property)]
    pub default: u32,  // TODO 'number'?
    /// Records whether this axis needs to be hidden in interfaces.
    #[kfl(property, default)]
    pub hidden: Option<bool>
}

#[derive(Debug, Default, Decode)]
pub struct Labels(#[kfl(children)] Vec<Label>);

#[derive(Debug, Decode)]
pub struct Label {
    /// The name of this label.
    #[kfl(property)]
    pub name: String,
    /// STAT flag ELIDABLE_AXIS_VALUE_NAME.
    #[kfl(property, default)]
    pub elidable: bool,
    /// STAT flag OLDER_SIBLING_FONT_ATTRIBUTE.
    #[kfl(property, default)]
    pub older_sibling: bool
}

#[derive(Debug, Default, Decode)]
pub struct Sources(#[kfl(children)] pub Vec<Source>);

#[derive(Debug, Decode)]
pub struct Source {
    /**
    The family name of the source font. While this could be extracted from the font data itself, it can be more efficient to add it here.
    */
    #[kfl(property, default)]
    pub family_name: Option<String>,
    /// The style name of the source font.
    #[kfl(property)]
    pub style_name: Option<String>, 
    /**
    A unique name that can be used to identify this font if it needs to be referenced elsewhere.
    */
    #[kfl(property)]
    pub name: String,
    /**
    A path to the source file, relative to the root path of this document. The path can be at the same level as the document or lower.
    */
    #[kfl(property)]
    pub file_name: PathBuf,
    /**
    Name of the layer in the source file. If no layer attribute is given assume the foreground layer should be used.
    */
    #[kfl(property)]
    pub layer: Option<String>,
    /// Defines the coordinates of this source in the design space.
    #[kfl(child)]
    pub location: Option<Location>,
}

#[derive(Debug, Default, Decode)]
pub struct VariableFonts(#[kfl(children)] Vec<VariableFont>);

#[derive(Debug, Decode)]
pub struct VariableFont {
    /**
    Each variable font has a name, that can be used by build tools to refer to the font that gets built from this element.
    */
    #[kfl(property)]
    pub name: String,
    /**
    This filename will be used by tools to decide where to store the built font on the disk. If not given, a filename can be computed from the name. The filename may include an extension (e.g. .ttf) and the build tools can replace that extension with another (e.g. .otf or .woff2) as needed.
    */
    #[kfl(property, default)]
    pub file_name: Option<String>,
    #[kfl(child)]
    pub axis_subsets: Option<AxisSubsets>,
    #[kfl(child)]
    pub lib: Option<Lib>
}

#[derive(Debug, Decode)]
pub struct AxisSubsets(#[kfl(children)] Vec<AxisSubset>);

#[derive(Debug, Decode)]
pub struct AxisSubset {
    /// Name of the axis to subset.
    #[kfl(property)]
    name: String,
    /// Lower end of the range, in user coordinates. If not mentioned, assume the axis’s minimum.
    #[kfl(property)]
    user_minimum: Option<u16>,
    /// Upper end of the range, in user coordinates. If not mentioned, assume the axis’s maximum.
    #[kfl(property)]
    user_maximum: Option<u16>,
    /// New default value of subset axis, in user coordinates. If not mentioned, assume the axis’s default. If the axis’s default falls outside of the subset range, then the new default will be the extremum that is closest to the full axis’s default.
    #[kfl(property)]
    user_default: Option<u16>,
    /// Single value, in user coordinates, at which to snapshot the design space while building this VF.
    #[kfl(property)]
    user_value: Option<u16>
}

#[derive(Debug, Decode)]
pub struct Rules(#[kfl(children)] Vec<Rule>);

#[derive(Debug, Decode)]
pub struct Rule {
    /// A unique name that can be used to identify this rule if it needs to be referenced elsewhere. The name is not important for compiling variable fonts.
    #[kfl(property)]
    pub name: Option<String>,
    #[kfl(children)]
    pub condition_sets: Vec<ConditionSet>,
    #[kfl(children)]
    pub subs: Vec<Sub>,
}

// <conditionsset>
#[derive(Debug, Default, Decode)]
pub struct ConditionSet(#[kfl(children)] Vec<Condition>);

#[derive(Debug, Decode)]
pub struct Condition {
    /// Must match one of the defined `axis` name attributes.
    #[kfl(property)]
    name: String,
    /// Low value, in design coordinates.
    #[kfl(property)]
    minimum: u32,
    /// High value, in design coordinates.
    #[kfl(property)]
    maximum: u32
}

#[derive(Debug, Decode)]
pub struct Sub {
    /// Name of the glyph this rule looks for.
    #[kfl(property)]
    name: String,
    /// Name of the glyph it is replaced with.
    #[kfl(property)]
    with: String,
}

#[derive(Debug, Default, Decode)]
pub struct Instances(#[kfl(children)] pub Vec<Instance>);

#[derive(Debug, Decode)]
pub struct Instance {
    /**
    The family name of the instance font. Corresponds with font.info.familyName
    */
    #[kfl(property)]
    pub family_name: String,
    /**
    The style name of the instance font. Corresponds with font.info.styleName
    */
    #[kfl(property)]
    pub style_name: String,
    /**
    A unique name that can be used to identify this font if it needs to be referenced elsewhere.
    */
    #[kfl(property)]
    pub name: String,
    /**
    Required for MutatorMath. A path to the instance file, relative to the root path of this document. The path can be at the same level as the document or lower.
    */
    #[kfl(property)]
    pub file_name: PathBuf,
    /// Optional for MutatorMath. Corresponds with font.info.postscriptFontName.
    #[kfl(property)]
    pub postscript_font_name: String,
    /// Optional for MutatorMath. Corresponds with styleMapFamilyName.
    #[kfl(property)]
    pub stylemap_family_name: Option<String>,
    /// Optional for MutatorMath. Corresponds with styleMapStyleName
    #[kfl(property)]
    pub stylemap_style_name: Option<String>,
    #[kfl(child)]
    pub location: Option<Location>,
    #[kfl(child)]
    pub lib: Option<Lib>
}

#[derive(Debug, Decode)]
pub struct Lib {
    #[kfl(child)]
    dict: Dict
}

#[derive(Debug, Decode)]
pub struct Location {
    #[kfl(children)]
    dimensions: Vec<Dimension>
}

#[derive(Debug, Decode)]
pub struct Dimension {
    /// Name of the axis.
    #[kfl(property)]
    name: String,
    /// Value on this axis in user coordinates.
    #[kfl(property)]
    user_value: Option<u32>,  // TODO
    /// Value on this axis in design coordinates.
    #[kfl(property)]
    x_value: Option<u32>,  // TODO
    /// Separate value for anisotropic interpolations.
    #[kfl(property)]
    y_value: Option<u32>,  // TODO
}

impl Default for Designspace {
    fn default() -> Self {
        Self {
            format: "5.0".to_string(),
            axes: Axes::default(),
            labels: Labels::default(),
            sources: Sources::default(),
            variable_fonts: None,
            instances: None,
            rules: None,
            lib: None
        }
    }
}

impl Designspace {
    pub fn add_source(&mut self, source: Source) {
        self.sources.0.push(source);
    }

    pub fn add_variable_font(&mut self, variable_font: VariableFont) {
        match &mut self.variable_fonts {
            None => {
                let mut variable_fonts = VariableFonts::default();
                variable_fonts.0.push(variable_font);
                self.variable_fonts = Some(variable_fonts);
            },
            Some(ref mut variable_fonts) => variable_fonts.0.push(variable_font)
        };
    }

    pub fn add_instance(&mut self, instance: Instance) {
        match &mut self.instances {
            None => {
                let mut instances = Instances::default();
                instances.0.push(instance);
                self.instances = Some(instances);
            },
            Some(ref mut instances) => instances.0.push(instance)
        };
    }
}
