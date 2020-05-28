use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,

    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,

    #[serde(rename = "ConstraintDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contraint_description: Option<String>,

    #[serde(rename = "Default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "MaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<usize>,

    #[serde(rename = "MinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<usize>,

    #[serde(rename = "NoEcho")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_echo: Option<bool>,

    #[serde(rename = "Type")]
    pub parameter_type: ParameterType,
}

impl Parameter {
    pub fn new(parameter_type: ParameterType) -> Self {
        Self {
            allowed_pattern: None,
            allowed_values: None,
            contraint_description: None,
            default: None,
            description: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            no_echo: None,
            parameter_type,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    r#String,
    Number,
    List,
    CommaDelimitedList,
}

#[test]
fn new_parameter() {
    let parameter = Parameter::new(ParameterType::String);
    assert_eq!(
        parameter,
        Parameter {
            allowed_pattern: None,
            allowed_values: None,
            contraint_description: None,
            default: None,
            description: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            no_echo: None,
            parameter_type: ParameterType::String,
        }
    )
}

#[test]
fn display_parameter() {
    let parameter = Parameter::new(ParameterType::String);
    assert_eq!(
        serde_yaml::to_string(&parameter).unwrap(),
        String::from("---\nType: String")
    )
}
