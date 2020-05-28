//! Cloudformation
//!
//! `cloudformation` is a library for generating and managing cloudformation templates

mod condition;
mod mapping;
mod metadata;
mod output;
mod parameter;
mod transform;

pub mod resource;

pub use condition::Condition;
pub use mapping::Mapping;
pub use metadata::Metadata;
pub use output::Output;
pub use parameter::{Parameter, ParameterType};
pub use resource::Resource;
pub use transform::Transform;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Cloudformation Template
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    /// AWS Template Format Version - Currently hard coded to 2010-09-09
    #[serde(rename = "AWSTemplateFormatVersion")]
    format_version: String,

    /// Optional Description for Cloudformation Template
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Optional Metadata for Cloudformation Template
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, Metadata>>,

    /// Optional Parameters for Cloudformation Template
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<HashMap<String, Parameter>>,

    /// Optional Mappings for Cloudformation Template
    #[serde(rename = "Mappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mappings: Option<HashMap<String, Mapping>>,

    /// Optional Conditions for Cloudformation Template
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<HashMap<String, Condition>>,

    /// Optional Transform for Cloudformation Template
    #[serde(rename = "Transform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<Transform>,

    /// Optional Resource for Cloudformation Template
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<HashMap<String, Resource>>,

    /// Optional Outputs for Cloudformation Template
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<HashMap<String, Output>>,
}

impl Template {
    /// Returns a new, empty, Cloudformation Template
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new();
    /// ```
    pub fn new() -> Self {
        Self {
            format_version: String::from("2010-09-09"),
            description: None,
            metadata: None,
            parameters: None,
            mappings: None,
            conditions: None,
            transform: None,
            resources: None,
            outputs: None,
        }
    }

    /// Returns a new Cloudformation Template generated from existing file
    ///
    /// # Arguments
    ///
    /// * `file &str` - name of the template file to read from
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::from("example.yml");
    /// ```
    pub fn from(file: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let template = serde_yaml::from_reader(reader)?;

        Ok(template)
    }

    /// Returns a std::io::Result after atempting to write the Template to a file
    ///
    /// # Arguments
    ///
    /// * `output: &str` - name of the file to write to
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// Template::new()
    ///     .write("example.yml").expect("Error writing to file");
    /// ```
    pub fn write(&self, output: &str) -> std::io::Result<()> {
        let mut file = File::create(output)?;
        file.write_all(serde_yaml::to_string(self).unwrap().as_bytes())?;

        Ok(())
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `description: &str` - description of the template
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_description("This is an example");
    /// ```
    pub fn add_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the metadata
    /// * `data: cloudformation::Metadata` - template metadata
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_metadata("Example", cloudformation::Metadata {});
    /// ```
    pub fn add_metadata(mut self, name: &str, data: Metadata) -> Self {
        if let Some(ref mut metadata) = self.metadata {
            metadata.insert(name.to_string(), data);
        } else {
            let mut metadata: HashMap<String, Metadata> = HashMap::new();
            metadata.insert(name.to_string(), data);

            self.metadata = Some(metadata);
        }

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the parameter
    /// * `parameter: cloudformation::Parameter`
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_metadata("Example", cloudformation::Metadata {});
    /// ```
    pub fn add_parameter(mut self, name: &str, parameter: Parameter) -> Self {
        if let Some(ref mut parameters) = self.parameters {
            parameters.insert(name.to_string(), parameter);
        } else {
            let mut parameters: HashMap<String, Parameter> = HashMap::new();
            parameters.insert(name.to_string(), parameter);

            self.parameters = Some(parameters);
        }

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the mapping
    /// * `mapping: cloudformation::Mapping`
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_metadata("Example", cloudformation::Metadata {});
    /// ```
    pub fn add_mapping(mut self, name: &str, mapping: Mapping) -> Self {
        if let Some(ref mut mappings) = self.mappings {
            mappings.insert(name.to_string(), mapping);
        } else {
            let mut mappings: HashMap<String, Mapping> = HashMap::new();
            mappings.insert(name.to_string(), mapping);

            self.mappings = Some(mappings);
        }

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the condition
    /// * `condition: cloudformation::Condition`
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_condition("Example", cloudformation::Condition {});
    /// ```
    pub fn add_condition(mut self, name: &str, condition: Condition) -> Self {
        if let Some(ref mut conditions) = self.conditions {
            conditions.insert(name.to_string(), condition);
        } else {
            let mut conditions: HashMap<String, Condition> = HashMap::new();
            conditions.insert(name.to_string(), condition);

            self.conditions = Some(conditions);
        }

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the transform
    /// * `transform: cloudformation::Transform`
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_transform("Example");
    /// ```
    pub fn add_transform(mut self, transform: &str) -> Self {
        if let Some(ref mut transforms) = self.transform {
            transforms.0.push(transform.to_string());
        } else {
            let transforms: Transform = Transform::new(transform.to_string());

            self.transform = Some(transforms);
        }

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the resource
    /// * `resource: cloudformation::Resource`
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_resource("Example", cloudformation::Resource::EC2Instance {
    ///         properties: Some(cloudformation::resource::ec2::instance::Properties::new()
    ///             .add_instance_type(cloudformation::resource::ec2::instance::InstanceType::T2_Micro)
    ///         )
    ///     });
    /// ```
    pub fn add_resource(mut self, name: &str, resource: Resource) -> Self {
        if let Some(ref mut resources) = self.resources {
            resources.insert(name.to_string(), resource);
        } else {
            let mut resources: HashMap<String, Resource> = HashMap::new();
            resources.insert(name.to_string(), resource);

            self.resources = Some(resources)
        }

        self
    }

    /// Returns Self
    ///
    /// # Arguments
    ///
    /// * `name: &str` - name of the output
    /// * `output: cloudformation::Output`
    ///
    /// # Example
    ///
    /// ```
    /// use cloudformation::Template;
    /// let template = Template::new()
    ///     .add_output("Example", cloudformation::Output {
    ///         description: None,
    ///     });
    /// ```
    pub fn add_output(mut self, name: &str, output: Output) -> Self {
        if let Some(ref mut outputs) = self.outputs {
            outputs.insert(name.to_string(), output);
        } else {
            let mut outputs: HashMap<String, Output> = HashMap::new();
            outputs.insert(name.to_string(), output);

            self.outputs = Some(outputs);
        }

        self
    }
}

#[test]
fn new_template() {
    let template = Template::new();
    assert_eq!(
        template,
        Template {
            format_version: String::from("2010-09-09"),
            description: None,
            metadata: None,
            parameters: None,
            mappings: None,
            conditions: None,
            transform: None,
            resources: None,
            outputs: None,
        }
    );
}

#[test]
fn add_description() {
    let template = Template::new().add_description("This is a test");
    assert_eq!(
        template,
        Template {
            format_version: String::from("2010-09-09"),
            description: Some(String::from("This is a test")),
            metadata: None,
            parameters: None,
            mappings: None,
            conditions: None,
            transform: None,
            resources: None,
            outputs: None,
        }
    );
}

#[test]
fn add_parameter() {
    let template = Template::new().add_parameter("Test", Parameter::new(ParameterType::String));

    let mut parameters = HashMap::new();
    parameters.insert(
        String::from("Test"),
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
        },
    );
    assert_eq!(
        template,
        Template {
            format_version: String::from("2010-09-09"),
            description: None,
            metadata: None,
            parameters: Some(parameters),
            mappings: None,
            conditions: None,
            transform: None,
            resources: None,
            outputs: None,
        }
    );
}

#[test]
fn add_resource() {
    let template = Template::new().add_resource("Test", Resource::EC2Instance { properties: None });

    let mut resources = HashMap::new();
    resources.insert(
        String::from("Test"),
        Resource::EC2Instance { properties: None },
    );
    assert_eq!(
        template,
        Template {
            format_version: String::from("2010-09-09"),
            description: None,
            metadata: None,
            parameters: None,
            mappings: None,
            conditions: None,
            transform: None,
            resources: Some(resources),
            outputs: None,
        }
    )
}

#[test]
fn display_template() {
    let template = Template::new();
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from("---\nAWSTemplateFormatVersion: 2010-09-09",)
    );
}

#[test]
fn display_template_with_description() {
    let template = Template::new().add_description("This is a test");
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from("---\nAWSTemplateFormatVersion: 2010-09-09\nDescription: This is a test")
    );
}

#[test]
fn display_template_with_parameter() {
    let template = Template::new().add_parameter("Test", Parameter::new(ParameterType::String));
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from(
            "---\nAWSTemplateFormatVersion: 2010-09-09\nParameters:\n  Test:\n    Type: String"
        )
    );
}

// ToDo - order of output is not consistent
// #[test]
// fn display_template_with_parameters() {
//     let mut template = Template::new();
//     template.add_parameter(String::from("Test"), Parameter::new(ParameterType::String));
//     template.add_parameter(String::from("Test2"), Parameter::new(ParameterType::String));
//     assert_eq!(
//         serde_yaml::to_string(&template).unwrap(),
//         String::from(
//             "---\nAWSTemplateFormatVersion: 2010-09-09\nParameters:\n  Test:\n    Type: String\n  Test2:\n    Type: String"
//         )
//     );
// }

#[test]
fn display_template_with_duplicate_parameters() {
    let template = Template::new()
        .add_parameter("Test", Parameter::new(ParameterType::String))
        .add_parameter("Test", Parameter::new(ParameterType::Number));
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from(
            "---\nAWSTemplateFormatVersion: 2010-09-09\nParameters:\n  Test:\n    Type: Number"
        )
    );
}

#[test]
fn display_template_with_description_and_parameter() {
    let template = Template::new()
        .add_description("This is a test")
        .add_parameter("Test", Parameter::new(ParameterType::String));
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from(
            "---\nAWSTemplateFormatVersion: 2010-09-09\nDescription: This is a test\nParameters:\n  Test:\n    Type: String"
        )
    );
}

#[test]
fn display_template_with_resource() {
    let template = Template::new().add_resource("Test", Resource::EC2Instance { properties: None });
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from(
            "---\nAWSTemplateFormatVersion: 2010-09-09\nResources:\n  Test:\n    Type: \"AWS::EC2::Instance\""
        )
    );
}

#[test]
fn display_template_with_duplicate_resources() {
    let template = Template::new()
        .add_resource("Test", Resource::EC2Instance { properties: None })
        .add_resource("Test", Resource::EC2Instance { properties: None });
    assert_eq!(
        serde_yaml::to_string(&template).unwrap(),
        String::from(
            "---\nAWSTemplateFormatVersion: 2010-09-09\nResources:\n  Test:\n    Type: \"AWS::EC2::Instance\""
        )
    );
}

// ToDo: Order of output is not consistent
// #[test]
// fn display_template_with_resources() {
//     let mut template = Template::new();
//     template.add_resource(
//         String::from("Test"),
//         Resource::new(String::from("AWS::EC2::Instance")),
//     );
//     template.add_resource(
//         String::from("Test2"),
//         Resource::new(String::from("AWS::EC2::Instance")),
//     );
//     assert_eq!(
//         serde_yaml::to_string(&template).unwrap(),
//         String::from(
//             "---\nAWSTemplateFormatVersion: 2010-09-09\nResources:\n  Test:\n    Type: \"AWS::EC2::Instance\"\n  Test2:\n    Type: \"AWS::EC2::Instance\""
//         )
//     );
// }
