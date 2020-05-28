use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    pub resource_type: String,
    pub properties: Properties,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceProperties {
    AdditionalInfo: Option<String>,
    // Affinity: String,
    // AvailabilityZone: String,
    // BlockDeviceMappings:
    //     - BlockDeviceMapping,
    // CpuOptions: CpuOptions,
    // CreditSpecification: CreditSpecification,
    // DisableApiTermination: Boolean,
    // EbsOptimized: Boolean,
    // ElasticGpuSpecifications:
    //     - ElasticGpuSpecification,
    // ElasticInferenceAccelerators:
    //     - ElasticInferenceAccelerator,
    // HibernationOptions: HibernationOptions,
    // HostId: String,
    // HostResourceGroupArn: String,
    // IamInstanceProfile: String,
    // ImageId: String,
    // InstanceInitiatedShutdownBehavior: String,
    // InstanceType: String,
    // Ipv6AddressCount: Integer,
    // Ipv6Addresses:
    //     - InstanceIpv6Address,
    // KernelId: String,
    // KeyName: String,
    // LaunchTemplate: LaunchTemplateSpecification,
    // LicenseSpecifications:
    //     - LicenseSpecification,
    // Monitoring: Boolean,
    // NetworkInterfaces:
    //     - NetworkInterface,
    // PlacementGroupName: String,
    // PrivateIpAddress: String,
    // RamdiskId: String,
    // SecurityGroupIds:
    //     - String,
    // SecurityGroups:
    //     - String,
    // SourceDestCheck: Boolean,
    // SsmAssociations:
    //     - SsmAssociation,
    // SubnetId: String,
    // Tags:
    //     - Tag,
    // Tenancy: String,
    // UserData: String,
    // Volumes:
    //     - Volume,
}
