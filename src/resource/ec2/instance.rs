use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "AdditionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,

    #[serde(rename = "Affinity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity: Option<String>,

    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    #[serde(rename = "BlockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,

    #[serde(rename = "CpuOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_options: Option<CpuOptions>,

    #[serde(rename = "CreditSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_specification: Option<CreditSpecification>,

    #[serde(rename = "DisableApiTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_api_termination: Option<bool>,

    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,

    #[serde(rename = "ElasticGpuSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_gpu_specifications: Option<Vec<ElasticGpuSpecification>>,

    #[serde(rename = "ElasticInferenceAccelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_inference_accelerators: Option<Vec<ElasticInferenceAccelerator>>,

    #[serde(rename = "HibernationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hibernation_options: Option<HibernationOptions>,

    #[serde(rename = "HostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,

    #[serde(rename = "HostResourceGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_resource_group_arn: Option<String>,

    #[serde(rename = "IamInstanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,

    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    #[serde(rename = "InstanceInitiatedShutdownBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_initiated_shutdown_behavior: Option<InstanceInitiatedShutdownBehavior>,

    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<InstanceType>,

    #[serde(rename = "Ipv6AddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address_count: Option<usize>,

    #[serde(rename = "Ipv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,

    #[serde(rename = "KernelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,

    #[serde(rename = "KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,

    #[serde(rename = "LaunchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,

    #[serde(rename = "LicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,

    #[serde(rename = "Monitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<bool>,

    #[serde(rename = "NetworkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,

    #[serde(rename = "PlacementGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_name: Option<String>,

    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,

    #[serde(rename = "RamdiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_disk_id: Option<String>,

    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    #[serde(rename = "SourceDestCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<bool>,

    #[serde(rename = "SsmAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_associations: Option<Vec<SsmAssociation>>,

    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,

    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<Tenancy>,

    #[serde(rename = "UserData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,

    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            additional_info: None,
            affinity: None,
            availability_zone: None,
            block_device_mappings: None,
            cpu_options: None,
            credit_specification: None,
            disable_api_termination: None,
            ebs_optimized: None,
            elastic_gpu_specifications: None,
            elastic_inference_accelerators: None,
            hibernation_options: None,
            host_id: None,
            host_resource_group_arn: None,
            iam_instance_profile: None,
            image_id: None,
            instance_initiated_shutdown_behavior: None,
            instance_type: None,
            ipv6_address_count: None,
            ipv6_addresses: None,
            kernel_id: None,
            key_name: None,
            launch_template: None,
            license_specifications: None,
            monitoring: None,
            network_interfaces: None,
            placement_group_name: None,
            private_ip_address: None,
            ram_disk_id: None,
            security_group_ids: None,
            security_groups: None,
            source_dest_check: None,
            ssm_associations: None,
            subnet_id: None,
            tags: None,
            tenancy: None,
            user_data: None,
            volumes: None,
        }
    }

    pub fn add_instance_type(mut self, instance_type: InstanceType) -> Self {
        self.instance_type = Some(instance_type);

        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockDeviceMapping {
    #[serde(rename = "DeviceName")]
    pub device_name: String,

    #[serde(rename = "Ebs")]
    pub ebs: Option<EBS>,

    #[serde(rename = "NoDevice")]
    pub no_device: Option<NoDevice>,

    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EBS {
    #[serde(rename = "DeleteOnTermination")]
    delete_on_termination: Option<bool>,

    #[serde(rename = "Encrypted")]
    encrypted: Option<bool>,

    #[serde(rename = "Iops")]
    iops: Option<usize>,

    #[serde(rename = "KmsKeyId")]
    kms_key_id: Option<String>,

    #[serde(rename = "SnapshotId")]
    snapshot_id: Option<String>,

    #[serde(rename = "VolumeSize")]
    volume_size: Option<usize>,

    #[serde(rename = "VolumeType")]
    volume_type: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoDevice {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuOptions {
    #[serde(rename = "CoreCount")]
    core_count: Option<usize>,

    #[serde(rename = "ThreadsPerCore")]
    threads_per_core: Option<usize>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditSpecification {
    #[serde(rename = "CPUCredits")]
    cpu_credits: Option<CPUCredits>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CPUCredits {
    #[serde(rename = "standard")]
    Standard,

    #[serde(rename = "unlimited")]
    Unlimited,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticGpuSpecification {
    #[serde(rename = "Type")]
    elastic_gpu_specification_type: ElasticGpuSpecificationType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ElasticGpuSpecificationType {
    #[serde(rename = "eg1.medium")]
    Medium,

    #[serde(rename = "eg1.large")]
    Large,

    #[serde(rename = "eg1.xlarge")]
    XLarge,

    #[serde(rename = "eg1.2xlarge")]
    XXLarge,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticInferenceAccelerator {
    #[serde(rename = "Count")]
    count: Option<usize>,

    #[serde(rename = "Type")]
    elastic_inference_accelerator_type: ElasticInferenceAcceleratorType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ElasticInferenceAcceleratorType {
    #[serde(rename = "eia1.medium")]
    Medium,

    #[serde(rename = "eia1.large")]
    Large,

    #[serde(rename = "eia1.xlarge")]
    XLarge,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HibernationOptions {
    #[serde(rename = "Configured")]
    configured: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstanceInitiatedShutdownBehavior {
    #[serde(rename = "stop")]
    Stop,

    #[serde(rename = "terminate")]
    Terminate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceIpv6Address {
    #[serde(rename = "Ipv6Address")]
    ipv6_address: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "LaunchTemplateId")]
    launch_template_id: Option<String>,

    #[serde(rename = "LaunchTemplateName")]
    launch_template_name: Option<String>,

    #[serde(rename = "LaunchTemplateVersion")]
    launch_template_version: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseSpecification {
    #[serde(rename = "LicenseConfigurationArn")]
    license_configuration_arn: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(rename = "AssociatePublicIpAddress")]
    associate_public_ip_address: Option<bool>,

    #[serde(rename = "DeleteOnTermination")]
    delete_on_termination: Option<bool>,

    #[serde(rename = "Description")]
    description: Option<String>,

    #[serde(rename = "DeviceIndex")]
    device_index: Option<String>,

    #[serde(rename = "GroupSet")]
    group_set: Option<Vec<String>>,

    #[serde(rename = "Ipv6AddressCount")]
    ipv6_address_count: Option<usize>,

    #[serde(rename = "Ipv6Addresses")]
    ipv6_addresses: Option<Vec<InstanceIpv6Address>>,

    #[serde(rename = "NetworkInterfaceId")]
    network_interfaces_id: Option<String>,

    #[serde(rename = "PrivateIpAddress")]
    private_ip_address: Option<String>,

    #[serde(rename = "PrivateIpAddresses")]
    private_ip_addresses: Option<Vec<PrivateIpAddressSpecification>>,

    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    secondary_private_ip_address_count: Option<usize>,

    #[serde(rename = "SubnetId")]
    subnet_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateIpAddressSpecification {
    #[serde(rename = "Primary")]
    primary: bool,

    #[serde(rename = "PrivateIpAddress")]
    private_ip_address: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SsmAssociation {
    #[serde(rename = "AssociationParameters")]
    association_parameters: Option<Vec<AssociationParameter>>,

    #[serde(rename = "DocumentName")]
    document_name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociationParameter {
    #[serde(rename = "Key")]
    key: String,

    #[serde(rename = "Value")]
    value: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    key: String,

    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Tenancy {
    #[serde(rename = "dedicated")]
    Dedicated,

    #[serde(rename = "default")]
    Default,

    #[serde(rename = "host")]
    Host,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(rename = "Device")]
    device: String,

    #[serde(rename = "VolumeId")]
    volume_id: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstanceType {
    #[serde(rename = "a1.2xlarge")]
    A1XXLarge,

    #[serde(rename = "a1.4xlarge")]
    A1_4XLarge,

    #[serde(rename = "a1.large")]
    A1_Large,

    #[serde(rename = "a1.medium")]
    A1_Medium,

    #[serde(rename = "a1.metal")]
    A1_Metal,

    #[serde(rename = "a1.xlarge")]
    A1_XLarge,

    #[serde(rename = "c1.medium")]
    C1_Medium,

    #[serde(rename = "c1.xlarge")]
    C1_XLarge,

    #[serde(rename = "c3.2xlarge")]
    C3_2XLarge,

    #[serde(rename = "c3.4xlarge")]
    C3_4XLarge,

    #[serde(rename = "c3.8xlarge")]
    C3_8XLarge,

    #[serde(rename = "c3.large")]
    C3_Large,

    #[serde(rename = "c3.xlarge")]
    C3_XLarge,

    #[serde(rename = "c4.2xlarge")]
    C4_2XLarge,

    #[serde(rename = "c4.4xlarge")]
    C4_4XLarge,

    #[serde(rename = "c4.8xlarge")]
    C4_8XLarge,

    #[serde(rename = "c4.large")]
    C4_Large,

    #[serde(rename = "c4.xlarge")]
    C4_XLarge,

    #[serde(rename = "c5.12xlarge")]
    C5_12XLarge,

    #[serde(rename = "c5.18xlarge")]
    C5_18XLarge,

    #[serde(rename = "c5.24xlarge")]
    C5_24XLarge,

    #[serde(rename = "c5.2xlarge")]
    C5_2xLArge,

    #[serde(rename = "c5.4xlarge")]
    C5_4xlarge,

    #[serde(rename = "c5.9xlarge")]
    C5_9xlarge,

    #[serde(rename = "c5.large")]
    C5_large,

    #[serde(rename = "c5.metal")]
    C5_metal,

    #[serde(rename = "c5.xlarge")]
    C5_xlarge,

    #[serde(rename = "c5d.12xlarge")]
    C5D_12XLarge,

    #[serde(rename = "c5d.18xlarge")]
    C5D_18XLarge,

    #[serde(rename = "c5d.24xlarge")]
    C5D_24XLarge,

    #[serde(rename = "c5d.2xlarge")]
    C5D_2XLarge,

    #[serde(rename = "c5d.4xlarge")]
    C5D_4XLarge,

    #[serde(rename = "c5d.9xlarge")]
    C5D_9XLarge,

    #[serde(rename = "c5d.large")]
    C5D_Large,

    #[serde(rename = "c5d.metal")]
    C5D_Metal,

    #[serde(rename = "c5d.xlarge")]
    C5D_XLarge,

    #[serde(rename = "c5n.18xlarge")]
    C5N_18XLarge,

    #[serde(rename = "c5n.2xlarge")]
    C5N_2XLarge,

    #[serde(rename = "c5n.4xlarge")]
    C5N_4XLarge,

    #[serde(rename = "c5n.9xlarge")]
    C5N_9XLarge,

    #[serde(rename = "c5n.large")]
    C5N_Large,

    #[serde(rename = "c5n.xlarge")]
    C5N_XLarge,

    #[serde(rename = "cc1.4xlarge")]
    CC1_4XLarge,

    #[serde(rename = "cc2.8xlarge")]
    CC2_8XLarge,

    #[serde(rename = "cg1.4xlarge")]
    CG1_4XLarge,

    #[serde(rename = "cr1.8xlarge")]
    CR1_8XLarge,

    #[serde(rename = "d2.2xlarge")]
    D2_2XLarge,

    #[serde(rename = "d2.4xlarge")]
    D2_4XLarge,

    #[serde(rename = "d2.8xlarge")]
    D2_8XLarge,

    #[serde(rename = "d2.xlarge")]
    D2_XLarge,

    #[serde(rename = "f1.16xlarge")]
    F1_16XLarge,

    #[serde(rename = "f1.2xlarge")]
    F1_2XLarge,

    #[serde(rename = "f1.4xlarge")]
    F1_4XLarge,

    #[serde(rename = "g2.2xlarge")]
    G2_2XLarge,

    #[serde(rename = "g2.8xlarge")]
    G2_8XLarge,

    #[serde(rename = "g3.16xlarge")]
    G3_16XLarge,

    #[serde(rename = "g3.4xlarge")]
    G3_4XLarge,

    #[serde(rename = "g3.8xlarge")]
    G3_8XLarge,

    #[serde(rename = "g3s.xlarge")]
    G3S_XLarge,

    #[serde(rename = "g4dn.12xlarge")]
    G4DN_12XLarge,

    #[serde(rename = "g4dn.16xlarge")]
    G4DN_16XLarge,

    #[serde(rename = "g4dn.2xlarge")]
    G4DN_2XLarge,

    #[serde(rename = "g4dn.4xlarge")]
    G4DN_4XLarge,

    #[serde(rename = "g4dn.8xlarge")]
    G4DN_8XLarge,

    #[serde(rename = "g4dn.xlarge")]
    G4DN_XLarge,

    #[serde(rename = "h1.16xlarge")]
    H1_16XLarge,

    #[serde(rename = "h1.2xlarge")]
    H1_2XLarge,

    #[serde(rename = "h1.4xlarge")]
    H1_4XLarge,

    #[serde(rename = "h1.8xlarge")]
    H1_8XLarge,

    #[serde(rename = "hi1.4xlarge")]
    HI1_4XLarge,

    #[serde(rename = "hs1.8xlarge")]
    HS1_8XLarge,

    #[serde(rename = "i2.2xlarge")]
    I2_2XLarge,

    #[serde(rename = "i2.4xlarge")]
    I2_4XLarge,

    #[serde(rename = "i2.8xlarge")]
    I2_8XLarge,

    #[serde(rename = "i2.xlarge")]
    I2_XLarge,

    #[serde(rename = "i3.16xlarge")]
    I3_16XLarge,

    #[serde(rename = "i3.2xlarge")]
    I3_2XLarge,

    #[serde(rename = "i3.4xlarge")]
    I3_4XLarge,

    #[serde(rename = "i3.8xlarge")]
    I3_8XLarge,

    #[serde(rename = "i3.large")]
    I3_large,

    #[serde(rename = "i3.metal")]
    I3_Metal,

    #[serde(rename = "i3.xlarge")]
    I3_XLarge,

    #[serde(rename = "i3en.12xlarge")]
    I3EN_12XLarge,

    #[serde(rename = "i3en.24xlarge")]
    I3EN_24XLarge,

    #[serde(rename = "i3en.2xlarge")]
    I3EN_2XLarge,

    #[serde(rename = "i3en.3xlarge")]
    I3EN_3XLarge,

    #[serde(rename = "i3en.6xlarge")]
    I3EN_6XLarge,

    #[serde(rename = "i3en.large")]
    I3EN_Large,

    #[serde(rename = "i3en.metal")]
    I3EN_Metal,

    #[serde(rename = "i3en.xlarge")]
    I3EN_XLarge,

    #[serde(rename = "inf1.24xlarge")]
    INF1_24XLarge,

    #[serde(rename = "inf1.2xlarge")]
    INF1_2XLarge,

    #[serde(rename = "inf1.6xlarge")]
    INF1_6XLarge,

    #[serde(rename = "inf1.xlarge")]
    INF1_XLarge,

    #[serde(rename = "m1.large")]
    M1_Large,

    #[serde(rename = "m1.medium")]
    M1_Medium,

    #[serde(rename = "m1.small")]
    M1_Small,

    #[serde(rename = "m1.xlarge")]
    M1_XLarge,

    #[serde(rename = "m2.2xlarge")]
    M2_2XLarge,

    #[serde(rename = "m2.4xlarge")]
    M2_4XLarge,

    #[serde(rename = "m2.xlarge")]
    M2_XLarge,

    #[serde(rename = "m3.2xlarge")]
    M3_2XLarge,

    #[serde(rename = "m3.large")]
    M3_Large,

    #[serde(rename = "m3.medium")]
    M3_Medium,

    #[serde(rename = "m3.xlarge")]
    M3_XLarge,

    #[serde(rename = "m4.10xlarge")]
    M4_10XLarge,

    #[serde(rename = "m4.16xlarge")]
    M4_16XLarge,

    #[serde(rename = "m4.2xlarge")]
    M4_2XLarge,

    #[serde(rename = "m4.4xlarge")]
    M4_4XLarge,

    #[serde(rename = "m4.large")]
    M4_Large,

    #[serde(rename = "m4.xlarge")]
    M4_XLarge,

    #[serde(rename = "m5.12xlarge")]
    M5_12XLarge,

    #[serde(rename = "m5.16xlarge")]
    M5_16XLarge,

    #[serde(rename = "m5.24xlarge")]
    M5_24XLarge,

    #[serde(rename = "m5.2xlarge")]
    M5_2XLarge,

    #[serde(rename = "m5.4xlarge")]
    M5_4XLarge,

    #[serde(rename = "m5.8xlarge")]
    M5_8XLarge,

    #[serde(rename = "m5.large")]
    M5_Large,

    #[serde(rename = "m5.metal")]
    M5_Metal,

    #[serde(rename = "m5.xlarge")]
    M5_XLarge,

    #[serde(rename = "m5a.12xlarge")]
    M5A_12XLarge,

    #[serde(rename = "m5a.16xlarge")]
    M5A_16XLarge,

    #[serde(rename = "m5a.24xlarge")]
    M5A_24XLarge,

    #[serde(rename = "m5a.2xlarge")]
    M5A_2XLarge,

    #[serde(rename = "m5a.4xlarge")]
    M5A_4XLarge,

    #[serde(rename = "m5a.8xlarge")]
    M5A_8XLarge,

    #[serde(rename = "m5a.large")]
    M5A_Large,

    #[serde(rename = "m5a.xlarge")]
    M5A_XLarge,

    #[serde(rename = "m5ad.12xlarge")]
    M5AD_12XLarge,

    #[serde(rename = "m5ad.16xlarge")]
    M5AD_16XLarge,

    #[serde(rename = "m5ad.24xlarge")]
    M5AD_24XLarge,

    #[serde(rename = "m5ad.2xlarge")]
    M5AD_2XLarge,

    #[serde(rename = "m5ad.4xlarge")]
    M5AD_4XLarge,

    #[serde(rename = "m5ad.8xlarge")]
    M5AD_8XLarge,

    #[serde(rename = "m5ad.large")]
    M5AD_Large,

    #[serde(rename = "m5ad.xlarge")]
    M5AD_XLarge,

    #[serde(rename = "m5d.12xlarge")]
    M5D_12XLarge,

    #[serde(rename = "m5d.16xlarge")]
    M5D_16XLarge,

    #[serde(rename = "m5d.24xlarge")]
    M5D_24XLarge,

    #[serde(rename = "m5d.2xlarge")]
    M5D_2XLarge,

    #[serde(rename = "m5d.4xlarge")]
    M5D_4XLarge,

    #[serde(rename = "m5d.8xlarge")]
    M5D_8XLarge,

    #[serde(rename = "m5d.large")]
    M5D_Large,

    #[serde(rename = "m5d.metal")]
    M5D_Metal,

    #[serde(rename = "m5d.xlarge")]
    M5D_XLarge,

    #[serde(rename = "m5dn.12xlarge")]
    M5DN_12XLarge,

    #[serde(rename = "m5dn.16xlarge")]
    M5DN_16XLarge,

    #[serde(rename = "m5dn.24xlarge")]
    M5DN_24XLarge,

    #[serde(rename = "m5dn.2xlarge")]
    M5DN_2XLarge,

    #[serde(rename = "m5dn.4xlarge")]
    M5DN_4XLarge,

    #[serde(rename = "m5dn.8xlarge")]
    M5DN_8XLarge,

    #[serde(rename = "m5dn.large")]
    M5DN_Large,

    #[serde(rename = "m5dn.xlarge")]
    M5DN_XLarge,

    #[serde(rename = "m5n.12xlarge")]
    M5N_12XLarge,

    #[serde(rename = "m5n.16xlarge")]
    M5N_16XLarge,

    #[serde(rename = "m5n.24xlarge")]
    M5N_24XLarge,

    #[serde(rename = "m5n.2xlarge")]
    M5N_2XLarge,

    #[serde(rename = "m5n.4xlarge")]
    M5N_4XLarge,

    #[serde(rename = "m5n.8xlarge")]
    M5N_8XLarge,

    #[serde(rename = "m5n.large")]
    M5N_Large,

    #[serde(rename = "m5n.xlarge")]
    M5N_XLarge,

    #[serde(rename = "m6g.12xlarge")]
    M6G_12XLarge,

    #[serde(rename = "m6g.16xlarge")]
    M6G_16XLarge,

    #[serde(rename = "m6g.2xlarge")]
    M6G_2XLarge,

    #[serde(rename = "m6g.4xlarge")]
    M6G_4XLarge,

    #[serde(rename = "m6g.8xlarge")]
    M6G_8XLarge,

    #[serde(rename = "m6g.large")]
    M6G_Large,

    #[serde(rename = "m6g.medium")]
    M6G_Medium,

    #[serde(rename = "m6g.metal")]
    M6G_Metal,

    #[serde(rename = "m6g.xlarge")]
    M6G_XLarge,

    #[serde(rename = "p2.16xlarge")]
    P2_16XLarge,

    #[serde(rename = "p2.8xlarge")]
    P2_8XLarge,

    #[serde(rename = "p2.xlarge")]
    P2_XLarge,

    #[serde(rename = "p3.16xlarge")]
    P3_16XLarge,

    #[serde(rename = "p3.2xlarge")]
    P3_2XLarge,

    #[serde(rename = "p3.8xlarge")]
    P3_8XLarge,

    #[serde(rename = "p3dn.24xlarge")]
    P3DN_24XLarge,

    #[serde(rename = "r3.2xlarge")]
    R3_2XLarge,

    #[serde(rename = "r3.4xlarge")]
    R3_4XLarge,

    #[serde(rename = "r3.8xlarge")]
    R3_8XLarge,

    #[serde(rename = "r3.large")]
    R3_Large,

    #[serde(rename = "r3.xlarge")]
    R3_XLarge,

    #[serde(rename = "r4.16xlarge")]
    R4_16XLarge,

    #[serde(rename = "r4.2xlarge")]
    R4_2XLarge,

    #[serde(rename = "r4.4xlarge")]
    R4_4XLarge,

    #[serde(rename = "r4.8xlarge")]
    R4_8XLarge,

    #[serde(rename = "r4.large")]
    R4_Large,

    #[serde(rename = "r4.xlarge")]
    R4_XLarge,

    #[serde(rename = "r5.12xlarge")]
    R5_12XLarge,

    #[serde(rename = "r5.16xlarge")]
    R5_16XLarge,

    #[serde(rename = "r5.24xlarge")]
    R5_24XLarge,

    #[serde(rename = "r5.2xlarge")]
    R5_2XLarge,

    #[serde(rename = "r5.4xlarge")]
    R5_4XLarge,

    #[serde(rename = "r5.8xlarge")]
    R5_8XLarge,

    #[serde(rename = "r5.large")]
    R5_Large,

    #[serde(rename = "r5.metal")]
    R5_Metal,

    #[serde(rename = "r5.xlarge")]
    R5_XLarge,

    #[serde(rename = "r5a.12xlarge")]
    R5A_12XLarge,

    #[serde(rename = "r5a.16xlarge")]
    R5A_16XLarge,

    #[serde(rename = "r5a.24xlarge")]
    R5A_24XLarge,

    #[serde(rename = "r5a.2xlarge")]
    R5A_2XLarge,

    #[serde(rename = "r5a.4xlarge")]
    R5A_4XLarge,

    #[serde(rename = "r5a.8xlarge")]
    R5A_8XLarge,

    #[serde(rename = "r5a.large")]
    R5A_Large,

    #[serde(rename = "r5a.xlarge")]
    R5A_XLarge,

    #[serde(rename = "r5ad.12xlarge")]
    R5AD_12XLarge,

    #[serde(rename = "r5ad.16xlarge")]
    R5AD_16XLarge,

    #[serde(rename = "r5ad.24xlarge")]
    R5AD_24XLarge,

    #[serde(rename = "r5ad.2xlarge")]
    R5AD_2XLarge,

    #[serde(rename = "r5ad.4xlarge")]
    R5AD_4XLarge,

    #[serde(rename = "r5ad.8xlarge")]
    R5AD_8XLarge,

    #[serde(rename = "r5ad.large")]
    R5AD_Large,

    #[serde(rename = "r5ad.xlarge")]
    R5AD_XLarge,

    #[serde(rename = "r5d.12xlarge")]
    R5D_12XLarge,

    #[serde(rename = "r5d.16xlarge")]
    R5D_16XLarge,

    #[serde(rename = "r5d.24xlarge")]
    R5D_24XLarge,

    #[serde(rename = "r5d.2xlarge")]
    R5D_2XLarge,

    #[serde(rename = "r5d.4xlarge")]
    R5D_4XLarge,

    #[serde(rename = "r5d.8xlarge")]
    R5D_8XLarge,

    #[serde(rename = "r5d.large")]
    R5D_Large,

    #[serde(rename = "r5d.metal")]
    R5D_Metal,

    #[serde(rename = "r5d.xlarge")]
    R5D_XLarge,

    #[serde(rename = "r5dn.12xlarge")]
    R5DN_12XLarge,

    #[serde(rename = "r5dn.16xlarge")]
    R5DN_16XLarge,

    #[serde(rename = "r5dn.24xlarge")]
    R5DN_24XLarge,

    #[serde(rename = "r5dn.2xlarge")]
    R5DN_2XLarge,

    #[serde(rename = "r5dn.4xlarge")]
    R5DN_4XLarge,

    #[serde(rename = "r5dn.8xlarge")]
    R5DN_8XLarge,

    #[serde(rename = "r5dn.large")]
    R5DN_Large,

    #[serde(rename = "r5dn.xlarge")]
    R5DN_XLarge,

    #[serde(rename = "r5n.12xlarge")]
    R5N_12XLarge,

    #[serde(rename = "r5n.16xlarge")]
    R5N_16XLarge,

    #[serde(rename = "r5n.24xlarge")]
    R5N_24XLarge,

    #[serde(rename = "r5n.2xlarge")]
    R5N_2XLarge,

    #[serde(rename = "r5n.4xlarge")]
    R5N_4XLarge,

    #[serde(rename = "r5n.8xlarge")]
    R5N_8XLarge,

    #[serde(rename = "r5n.large")]
    R5N_Large,

    #[serde(rename = "r5n.xlarge")]
    R5N_XLarge,

    #[serde(rename = "t1.micro")]
    T1_Micro,

    #[serde(rename = "t2.2xlarge")]
    T2_2XLarge,

    #[serde(rename = "t2.large")]
    T2_Large,

    #[serde(rename = "t2.medium")]
    T2_Medium,

    #[serde(rename = "t2.micro")]
    T2_Micro,

    #[serde(rename = "t2.nano")]
    T2_Nano,

    #[serde(rename = "t2.small")]
    T2_Small,

    #[serde(rename = "t2.xlarge")]
    T2_XLarge,

    #[serde(rename = "t3.2xlarge")]
    T3_2XLarge,

    #[serde(rename = "t3.large")]
    T3_Large,

    #[serde(rename = "t3.medium")]
    T3_Medium,

    #[serde(rename = "t3.micro")]
    T3_Micro,

    #[serde(rename = "t3.nano")]
    T3_Nano,

    #[serde(rename = "t3.small")]
    T3_Small,

    #[serde(rename = "t3.xlarge")]
    T3_XLarge,

    #[serde(rename = "t3a.2xlarge")]
    T3A_2XLarge,

    #[serde(rename = "t3a.large")]
    T3A_Large,

    #[serde(rename = "t3a.medium")]
    T3A_Medium,

    #[serde(rename = "t3a.micro")]
    T3A_Micro,

    #[serde(rename = "t3a.nano")]
    T3A_Nano,

    #[serde(rename = "t3a.small")]
    T3A_Small,

    #[serde(rename = "t3a.xlarge")]
    T3A_XLarge,

    #[serde(rename = "u-12tb1.metal")]
    U_12TB1_Metal,

    #[serde(rename = "u-18tb1.metal")]
    U_18TB1_Metal,

    #[serde(rename = "u-24tb1.metal")]
    U_24TB1_Metal,

    #[serde(rename = "u-6tb1.metal")]
    U_6TB1_Metal,

    #[serde(rename = "u-9tb1.metal")]
    U_9TB1_Metal,

    #[serde(rename = "x1.16xlarge")]
    X1_16XLarge,

    #[serde(rename = "x1.32xlarge")]
    X1_32XLarge,

    #[serde(rename = "x1e.16xlarge")]
    X1E_16XLarge,

    #[serde(rename = "x1e.2xlarge")]
    X1E_2XLarge,

    #[serde(rename = "x1e.32xlarge")]
    X1E_32XLarge,

    #[serde(rename = "x1e.4xlarge")]
    X1E_4XLarge,

    #[serde(rename = "x1e.8xlarge")]
    X1E_8XLarge,

    #[serde(rename = "x1e.xlarge")]
    X1E_XLarge,

    #[serde(rename = "z1d.12xlarge")]
    Z1D_12XLarge,

    #[serde(rename = "z1d.2xlarge")]
    Z1D_2XLarge,

    #[serde(rename = "z1d.3xlarge")]
    Z1D_3XLarge,

    #[serde(rename = "z1d.6xlarge")]
    Z1D_6XLarge,

    #[serde(rename = "z1d.large")]
    Z1D_Large,

    #[serde(rename = "z1d.metal")]
    Z1D_Metal,

    #[serde(rename = "z1d.xlarge")]
    Z1D_XLarge,
}
