// template.yml

use moon_common::Id;
use rustc_hash::FxHashMap;
use schematic::{validate, Config, ValidateError};

#[cfg(feature = "loader")]
use std::path::Path;

macro_rules! var_setting {
    ($name:ident, $ty:ty) => {
        /// Configuration for a template variable.
        #[derive(Clone, Config, Debug, Eq, PartialEq)]
        pub struct $name {
            /// The default value of the variable if none was provided.
            pub default: $ty,
            /// Prompt the user for a value when the generator is running.
            pub prompt: Option<String>,
            /// Marks the variable as required, and will not accept an empty value.
            pub required: Option<bool>,
        }
    };
}

var_setting!(TemplateVariableBoolSetting, bool);
var_setting!(TemplateVariableNumberSetting, usize);
var_setting!(TemplateVariableStringSetting, String);

#[derive(Clone, Config, Debug, Eq, PartialEq)]
pub struct TemplateVariableEnumValueConfig {
    /// A human-readable label for the value.
    pub label: String,
    /// The literal enumerable value.
    pub value: String,
}

#[derive(Clone, Config, Debug, Eq, PartialEq)]
#[config(serde(
    untagged,
    expecting = "expected a value string or value object with label"
))]
pub enum TemplateVariableEnumValue {
    String(String),
    #[setting(nested)]
    Object(TemplateVariableEnumValueConfig),
}

#[derive(Clone, Config, Debug, Eq, PartialEq)]
#[config(serde(untagged))]
pub enum TemplateVariableEnumDefault {
    String(String),
    #[setting(default)]
    Vec(Vec<String>),
}

impl TemplateVariableEnumDefault {
    pub fn to_vec(&self) -> Vec<&String> {
        match self {
            Self::String(value) => vec![value],
            Self::Vec(list) => list.iter().collect(),
        }
    }
}

fn validate_enum_default<C>(
    default_value: &PartialTemplateVariableEnumDefault,
    partial: &PartialTemplateVariableEnumSetting,
    _context: &C,
    _finalize: bool,
) -> Result<(), ValidateError> {
    if let Some(values) = &partial.values {
        if let PartialTemplateVariableEnumDefault::Vec(list) = default_value {
            // Vector is the default value, so check if not-empty
            if !partial.multiple.is_some_and(|m| m) && !list.is_empty() {
                return Err(ValidateError::new(
                    "multiple default values is not allowed unless `multiple` is enabled",
                ));
            }
        }

        let values = values
            .iter()
            .flat_map(|v| match v {
                PartialTemplateVariableEnumValue::String(value) => Some(value),
                PartialTemplateVariableEnumValue::Object(cfg) => cfg.value.as_ref(),
            })
            .collect::<Vec<_>>();

        let matches = match default_value {
            PartialTemplateVariableEnumDefault::String(inner) => values.contains(&inner),
            PartialTemplateVariableEnumDefault::Vec(list) => {
                list.iter().all(|v| values.contains(&v))
            }
        };

        if !matches {
            return Err(ValidateError::new(
                "invalid default value, must be a value configured in `values`",
            ));
        }
    }

    Ok(())
}

#[derive(Clone, Config, Debug, Eq, PartialEq)]
pub struct TemplateVariableEnumSetting {
    /// The default value of the variable if none was provided.
    #[setting(nested, validate = validate_enum_default)]
    pub default: TemplateVariableEnumDefault,

    /// Allows multiple values to be selected.
    pub multiple: Option<bool>,

    /// Prompt the user for a value when the generator is running.
    pub prompt: Option<String>,

    /// List of acceptable values for this variable.
    #[setting(nested)]
    pub values: Vec<TemplateVariableEnumValue>,
}

impl TemplateVariableEnumSetting {
    pub fn get_labels(&self) -> Vec<&String> {
        self.values
            .iter()
            .map(|v| match v {
                TemplateVariableEnumValue::String(value) => value,
                TemplateVariableEnumValue::Object(cfg) => &cfg.label,
            })
            .collect()
    }

    pub fn get_values(&self) -> Vec<&String> {
        self.values
            .iter()
            .map(|v| match v {
                TemplateVariableEnumValue::String(value) => value,
                TemplateVariableEnumValue::Object(cfg) => &cfg.value,
            })
            .collect()
    }
}

/// Each type of template variable.
#[derive(Clone, Config, Debug, Eq, PartialEq)]
#[config(serde(tag = "type", expecting = "expected a supported value type"))]
pub enum TemplateVariable {
    /// A boolean variable.
    #[setting(nested)]
    Boolean(TemplateVariableBoolSetting),
    /// A string enumerable variable.
    #[setting(nested)]
    Enum(TemplateVariableEnumSetting),
    /// A number variable.
    #[setting(nested)]
    Number(TemplateVariableNumberSetting),
    /// A string variable.
    #[setting(nested)]
    String(TemplateVariableStringSetting),
}

/// Configures a template and its files to be scaffolded.
/// Docs: https://moonrepo.dev/docs/config/template
#[derive(Clone, Config, Debug)]
pub struct TemplateConfig {
    #[setting(
        default = "https://moonrepo.dev/schemas/template.json",
        rename = "$schema"
    )]
    pub schema: String,

    /// A description on what the template scaffolds.
    #[setting(validate = validate::not_empty)]
    pub description: String,

    /// A pre-populated destination to scaffold to, relative from the
    /// workspace root.
    pub destination: Option<String>,

    /// Extends one or many other templates.
    pub extends: Vec<Id>,

    /// Overrides the ID of the template, instead of using the folder name.
    pub id: Option<Id>,

    /// A human-readable title for the template.
    #[setting(validate = validate::not_empty)]
    pub title: String,

    /// A mapping of variables that'll be interpolated within each template file.
    /// Variables can also be populated by passing command line arguments.
    #[setting(nested)]
    pub variables: FxHashMap<String, TemplateVariable>,
}

#[cfg(feature = "loader")]
impl TemplateConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> miette::Result<TemplateConfig> {
        use crate::validate::check_yml_extension;
        use moon_common::color;
        use schematic::ConfigLoader;

        let result = ConfigLoader::<TemplateConfig>::new()
            .set_help(color::muted_light(
                "https://moonrepo.dev/docs/config/template",
            ))
            .file(check_yml_extension(path.as_ref()))?
            .load()?;

        Ok(result.config)
    }

    pub fn load_from<P: AsRef<Path>>(template_root: P) -> miette::Result<TemplateConfig> {
        use moon_common::consts;

        Self::load(
            template_root
                .as_ref()
                .join(consts::CONFIG_TEMPLATE_FILENAME),
        )
    }
}
