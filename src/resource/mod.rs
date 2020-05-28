pub mod ec2;

use serde::{Deserialize, Serialize};

/// Enum of AWS Cloudformation Resources
///
/// # Example
///
/// ```
/// let resource = cloudformation::resource::Resource::EC2Instance {
///     properties: Some(cloudformation::resource::ec2::instance::Properties::new()
///         .add_instance_type(cloudformation::resource::ec2::instance::InstanceType::T2_Micro)),
/// };
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Resource {
    #[serde(rename = "AWS::EC2::Instance")]
    EC2Instance {
        #[serde(rename = "Properties")]
        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<ec2::instance::Properties>,
    },
}
