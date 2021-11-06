#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub type ContainerServiceOsDisk = i32;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerServiceStorageProfile {
    StorageAccount,
    ManagedDisks,
}
pub type ContainerServiceVnetSubnetId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerServiceVmSize {
    #[serde(rename = "Standard_A1")]
    StandardA1,
    #[serde(rename = "Standard_A10")]
    StandardA10,
    #[serde(rename = "Standard_A11")]
    StandardA11,
    #[serde(rename = "Standard_A1_v2")]
    StandardA1V2,
    #[serde(rename = "Standard_A2")]
    StandardA2,
    #[serde(rename = "Standard_A2_v2")]
    StandardA2V2,
    #[serde(rename = "Standard_A2m_v2")]
    StandardA2mV2,
    #[serde(rename = "Standard_A3")]
    StandardA3,
    #[serde(rename = "Standard_A4")]
    StandardA4,
    #[serde(rename = "Standard_A4_v2")]
    StandardA4V2,
    #[serde(rename = "Standard_A4m_v2")]
    StandardA4mV2,
    #[serde(rename = "Standard_A5")]
    StandardA5,
    #[serde(rename = "Standard_A6")]
    StandardA6,
    #[serde(rename = "Standard_A7")]
    StandardA7,
    #[serde(rename = "Standard_A8")]
    StandardA8,
    #[serde(rename = "Standard_A8_v2")]
    StandardA8V2,
    #[serde(rename = "Standard_A8m_v2")]
    StandardA8mV2,
    #[serde(rename = "Standard_A9")]
    StandardA9,
    #[serde(rename = "Standard_B2ms")]
    StandardB2ms,
    #[serde(rename = "Standard_B2s")]
    StandardB2s,
    #[serde(rename = "Standard_B4ms")]
    StandardB4ms,
    #[serde(rename = "Standard_B8ms")]
    StandardB8ms,
    #[serde(rename = "Standard_D1")]
    StandardD1,
    #[serde(rename = "Standard_D11")]
    StandardD11,
    #[serde(rename = "Standard_D11_v2")]
    StandardD11V2,
    #[serde(rename = "Standard_D11_v2_Promo")]
    StandardD11V2Promo,
    #[serde(rename = "Standard_D12")]
    StandardD12,
    #[serde(rename = "Standard_D12_v2")]
    StandardD12V2,
    #[serde(rename = "Standard_D12_v2_Promo")]
    StandardD12V2Promo,
    #[serde(rename = "Standard_D13")]
    StandardD13,
    #[serde(rename = "Standard_D13_v2")]
    StandardD13V2,
    #[serde(rename = "Standard_D13_v2_Promo")]
    StandardD13V2Promo,
    #[serde(rename = "Standard_D14")]
    StandardD14,
    #[serde(rename = "Standard_D14_v2")]
    StandardD14V2,
    #[serde(rename = "Standard_D14_v2_Promo")]
    StandardD14V2Promo,
    #[serde(rename = "Standard_D15_v2")]
    StandardD15V2,
    #[serde(rename = "Standard_D16_v3")]
    StandardD16V3,
    #[serde(rename = "Standard_D16s_v3")]
    StandardD16sV3,
    #[serde(rename = "Standard_D1_v2")]
    StandardD1V2,
    #[serde(rename = "Standard_D2")]
    StandardD2,
    #[serde(rename = "Standard_D2_v2")]
    StandardD2V2,
    #[serde(rename = "Standard_D2_v2_Promo")]
    StandardD2V2Promo,
    #[serde(rename = "Standard_D2_v3")]
    StandardD2V3,
    #[serde(rename = "Standard_D2s_v3")]
    StandardD2sV3,
    #[serde(rename = "Standard_D3")]
    StandardD3,
    #[serde(rename = "Standard_D32_v3")]
    StandardD32V3,
    #[serde(rename = "Standard_D32s_v3")]
    StandardD32sV3,
    #[serde(rename = "Standard_D3_v2")]
    StandardD3V2,
    #[serde(rename = "Standard_D3_v2_Promo")]
    StandardD3V2Promo,
    #[serde(rename = "Standard_D4")]
    StandardD4,
    #[serde(rename = "Standard_D4_v2")]
    StandardD4V2,
    #[serde(rename = "Standard_D4_v2_Promo")]
    StandardD4V2Promo,
    #[serde(rename = "Standard_D4_v3")]
    StandardD4V3,
    #[serde(rename = "Standard_D4s_v3")]
    StandardD4sV3,
    #[serde(rename = "Standard_D5_v2")]
    StandardD5V2,
    #[serde(rename = "Standard_D5_v2_Promo")]
    StandardD5V2Promo,
    #[serde(rename = "Standard_D64_v3")]
    StandardD64V3,
    #[serde(rename = "Standard_D64s_v3")]
    StandardD64sV3,
    #[serde(rename = "Standard_D8_v3")]
    StandardD8V3,
    #[serde(rename = "Standard_D8s_v3")]
    StandardD8sV3,
    #[serde(rename = "Standard_DS1")]
    StandardDs1,
    #[serde(rename = "Standard_DS11")]
    StandardDs11,
    #[serde(rename = "Standard_DS11_v2")]
    StandardDs11V2,
    #[serde(rename = "Standard_DS11_v2_Promo")]
    StandardDs11V2Promo,
    #[serde(rename = "Standard_DS12")]
    StandardDs12,
    #[serde(rename = "Standard_DS12_v2")]
    StandardDs12V2,
    #[serde(rename = "Standard_DS12_v2_Promo")]
    StandardDs12V2Promo,
    #[serde(rename = "Standard_DS13")]
    StandardDs13,
    #[serde(rename = "Standard_DS13-2_v2")]
    StandardDs132V2,
    #[serde(rename = "Standard_DS13-4_v2")]
    StandardDs134V2,
    #[serde(rename = "Standard_DS13_v2")]
    StandardDs13V2,
    #[serde(rename = "Standard_DS13_v2_Promo")]
    StandardDs13V2Promo,
    #[serde(rename = "Standard_DS14")]
    StandardDs14,
    #[serde(rename = "Standard_DS14-4_v2")]
    StandardDs144V2,
    #[serde(rename = "Standard_DS14-8_v2")]
    StandardDs148V2,
    #[serde(rename = "Standard_DS14_v2")]
    StandardDs14V2,
    #[serde(rename = "Standard_DS14_v2_Promo")]
    StandardDs14V2Promo,
    #[serde(rename = "Standard_DS15_v2")]
    StandardDs15V2,
    #[serde(rename = "Standard_DS1_v2")]
    StandardDs1V2,
    #[serde(rename = "Standard_DS2")]
    StandardDs2,
    #[serde(rename = "Standard_DS2_v2")]
    StandardDs2V2,
    #[serde(rename = "Standard_DS2_v2_Promo")]
    StandardDs2V2Promo,
    #[serde(rename = "Standard_DS3")]
    StandardDs3,
    #[serde(rename = "Standard_DS3_v2")]
    StandardDs3V2,
    #[serde(rename = "Standard_DS3_v2_Promo")]
    StandardDs3V2Promo,
    #[serde(rename = "Standard_DS4")]
    StandardDs4,
    #[serde(rename = "Standard_DS4_v2")]
    StandardDs4V2,
    #[serde(rename = "Standard_DS4_v2_Promo")]
    StandardDs4V2Promo,
    #[serde(rename = "Standard_DS5_v2")]
    StandardDs5V2,
    #[serde(rename = "Standard_DS5_v2_Promo")]
    StandardDs5V2Promo,
    #[serde(rename = "Standard_E16_v3")]
    StandardE16V3,
    #[serde(rename = "Standard_E16s_v3")]
    StandardE16sV3,
    #[serde(rename = "Standard_E2_v3")]
    StandardE2V3,
    #[serde(rename = "Standard_E2s_v3")]
    StandardE2sV3,
    #[serde(rename = "Standard_E32-16s_v3")]
    StandardE3216sV3,
    #[serde(rename = "Standard_E32-8s_v3")]
    StandardE328sV3,
    #[serde(rename = "Standard_E32_v3")]
    StandardE32V3,
    #[serde(rename = "Standard_E32s_v3")]
    StandardE32sV3,
    #[serde(rename = "Standard_E4_v3")]
    StandardE4V3,
    #[serde(rename = "Standard_E4s_v3")]
    StandardE4sV3,
    #[serde(rename = "Standard_E64-16s_v3")]
    StandardE6416sV3,
    #[serde(rename = "Standard_E64-32s_v3")]
    StandardE6432sV3,
    #[serde(rename = "Standard_E64_v3")]
    StandardE64V3,
    #[serde(rename = "Standard_E64s_v3")]
    StandardE64sV3,
    #[serde(rename = "Standard_E8_v3")]
    StandardE8V3,
    #[serde(rename = "Standard_E8s_v3")]
    StandardE8sV3,
    #[serde(rename = "Standard_F1")]
    StandardF1,
    #[serde(rename = "Standard_F16")]
    StandardF16,
    #[serde(rename = "Standard_F16s")]
    StandardF16s,
    #[serde(rename = "Standard_F16s_v2")]
    StandardF16sV2,
    #[serde(rename = "Standard_F1s")]
    StandardF1s,
    #[serde(rename = "Standard_F2")]
    StandardF2,
    #[serde(rename = "Standard_F2s")]
    StandardF2s,
    #[serde(rename = "Standard_F2s_v2")]
    StandardF2sV2,
    #[serde(rename = "Standard_F32s_v2")]
    StandardF32sV2,
    #[serde(rename = "Standard_F4")]
    StandardF4,
    #[serde(rename = "Standard_F4s")]
    StandardF4s,
    #[serde(rename = "Standard_F4s_v2")]
    StandardF4sV2,
    #[serde(rename = "Standard_F64s_v2")]
    StandardF64sV2,
    #[serde(rename = "Standard_F72s_v2")]
    StandardF72sV2,
    #[serde(rename = "Standard_F8")]
    StandardF8,
    #[serde(rename = "Standard_F8s")]
    StandardF8s,
    #[serde(rename = "Standard_F8s_v2")]
    StandardF8sV2,
    #[serde(rename = "Standard_G1")]
    StandardG1,
    #[serde(rename = "Standard_G2")]
    StandardG2,
    #[serde(rename = "Standard_G3")]
    StandardG3,
    #[serde(rename = "Standard_G4")]
    StandardG4,
    #[serde(rename = "Standard_G5")]
    StandardG5,
    #[serde(rename = "Standard_GS1")]
    StandardGs1,
    #[serde(rename = "Standard_GS2")]
    StandardGs2,
    #[serde(rename = "Standard_GS3")]
    StandardGs3,
    #[serde(rename = "Standard_GS4")]
    StandardGs4,
    #[serde(rename = "Standard_GS4-4")]
    StandardGs44,
    #[serde(rename = "Standard_GS4-8")]
    StandardGs48,
    #[serde(rename = "Standard_GS5")]
    StandardGs5,
    #[serde(rename = "Standard_GS5-16")]
    StandardGs516,
    #[serde(rename = "Standard_GS5-8")]
    StandardGs58,
    #[serde(rename = "Standard_H16")]
    StandardH16,
    #[serde(rename = "Standard_H16m")]
    StandardH16m,
    #[serde(rename = "Standard_H16mr")]
    StandardH16mr,
    #[serde(rename = "Standard_H16r")]
    StandardH16r,
    #[serde(rename = "Standard_H8")]
    StandardH8,
    #[serde(rename = "Standard_H8m")]
    StandardH8m,
    #[serde(rename = "Standard_L16s")]
    StandardL16s,
    #[serde(rename = "Standard_L32s")]
    StandardL32s,
    #[serde(rename = "Standard_L4s")]
    StandardL4s,
    #[serde(rename = "Standard_L8s")]
    StandardL8s,
    #[serde(rename = "Standard_M128-32ms")]
    StandardM12832ms,
    #[serde(rename = "Standard_M128-64ms")]
    StandardM12864ms,
    #[serde(rename = "Standard_M128ms")]
    StandardM128ms,
    #[serde(rename = "Standard_M128s")]
    StandardM128s,
    #[serde(rename = "Standard_M64-16ms")]
    StandardM6416ms,
    #[serde(rename = "Standard_M64-32ms")]
    StandardM6432ms,
    #[serde(rename = "Standard_M64ms")]
    StandardM64ms,
    #[serde(rename = "Standard_M64s")]
    StandardM64s,
    #[serde(rename = "Standard_NC12")]
    StandardNc12,
    #[serde(rename = "Standard_NC12s_v2")]
    StandardNc12sV2,
    #[serde(rename = "Standard_NC12s_v3")]
    StandardNc12sV3,
    #[serde(rename = "Standard_NC24")]
    StandardNc24,
    #[serde(rename = "Standard_NC24r")]
    StandardNc24r,
    #[serde(rename = "Standard_NC24rs_v2")]
    StandardNc24rsV2,
    #[serde(rename = "Standard_NC24rs_v3")]
    StandardNc24rsV3,
    #[serde(rename = "Standard_NC24s_v2")]
    StandardNc24sV2,
    #[serde(rename = "Standard_NC24s_v3")]
    StandardNc24sV3,
    #[serde(rename = "Standard_NC6")]
    StandardNc6,
    #[serde(rename = "Standard_NC6s_v2")]
    StandardNc6sV2,
    #[serde(rename = "Standard_NC6s_v3")]
    StandardNc6sV3,
    #[serde(rename = "Standard_ND12s")]
    StandardNd12s,
    #[serde(rename = "Standard_ND24rs")]
    StandardNd24rs,
    #[serde(rename = "Standard_ND24s")]
    StandardNd24s,
    #[serde(rename = "Standard_ND6s")]
    StandardNd6s,
    #[serde(rename = "Standard_NV12")]
    StandardNv12,
    #[serde(rename = "Standard_NV24")]
    StandardNv24,
    #[serde(rename = "Standard_NV6")]
    StandardNv6,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceCustomProfile {
    pub orchestrator: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretRef {
    #[serde(rename = "vaultID")]
    pub vault_id: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceServicePrincipalProfile {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "keyVaultSecretRef", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_secret_ref: Option<KeyVaultSecretRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceOrchestratorProfile {
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: container_service_orchestrator_profile::OrchestratorType,
    #[serde(rename = "orchestratorVersion", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_version: Option<String>,
}
pub mod container_service_orchestrator_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrchestratorType {
        Kubernetes,
        Swarm,
        #[serde(rename = "DCOS")]
        Dcos,
        #[serde(rename = "DockerCE")]
        DockerCe,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceMasterProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<container_service_master_profile::Count>,
    #[serde(rename = "dnsPrefix")]
    pub dns_prefix: String,
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[serde(rename = "firstConsecutiveStaticIP", default, skip_serializing_if = "Option::is_none")]
    pub first_consecutive_static_ip: Option<String>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
pub mod container_service_master_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Count {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceAgentPoolProfile {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[serde(rename = "dnsPrefix", default, skip_serializing_if = "Option::is_none")]
    pub dns_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<i64>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceWindowsProfile {
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[serde(rename = "adminPassword")]
    pub admin_password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceLinuxProfile {
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    pub ssh: ContainerServiceSshConfiguration,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshConfiguration {
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<ContainerServiceSshPublicKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshPublicKey {
    #[serde(rename = "keyData")]
    pub key_data: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceDiagnosticsProfile {
    #[serde(rename = "vmDiagnostics")]
    pub vm_diagnostics: ContainerServiceVmDiagnostics,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceVmDiagnostics {
    pub enabled: bool,
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerService>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "orchestratorProfile")]
    pub orchestrator_profile: ContainerServiceOrchestratorProfile,
    #[serde(rename = "customProfile", default, skip_serializing_if = "Option::is_none")]
    pub custom_profile: Option<ContainerServiceCustomProfile>,
    #[serde(rename = "servicePrincipalProfile", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_profile: Option<ContainerServiceServicePrincipalProfile>,
    #[serde(rename = "masterProfile")]
    pub master_profile: ContainerServiceMasterProfile,
    #[serde(rename = "agentPoolProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ContainerServiceAgentPoolProfile>,
    #[serde(rename = "windowsProfile", default, skip_serializing_if = "Option::is_none")]
    pub windows_profile: Option<ContainerServiceWindowsProfile>,
    #[serde(rename = "linuxProfile")]
    pub linux_profile: ContainerServiceLinuxProfile,
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<ContainerServiceDiagnosticsProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Linux,
    Windows,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationValueDisplay>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationValueDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterServicePrincipalProfile {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAgentPoolProfile {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[serde(rename = "maxPods", default, skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceNetworkProfile {
    #[serde(rename = "networkPlugin", default, skip_serializing_if = "Option::is_none")]
    pub network_plugin: Option<container_service_network_profile::NetworkPlugin>,
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<container_service_network_profile::NetworkPolicy>,
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
}
pub mod container_service_network_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NetworkPlugin {
        #[serde(rename = "azure")]
        Azure,
        #[serde(rename = "kubenet")]
        Kubenet,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NetworkPolicy {
        #[serde(rename = "calico")]
        Calico,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedCluster>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[serde(rename = "dnsPrefix", default, skip_serializing_if = "Option::is_none")]
    pub dns_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "agentPoolProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ManagedClusterAgentPoolProfile>,
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<ContainerServiceLinuxProfile>,
    #[serde(rename = "servicePrincipalProfile", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_profile: Option<ManagedClusterServicePrincipalProfile>,
    #[serde(rename = "addonProfiles", default, skip_serializing_if = "Option::is_none")]
    pub addon_profiles: Option<serde_json::Value>,
    #[serde(rename = "nodeResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub node_resource_group: Option<String>,
    #[serde(rename = "enableRBAC", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<ContainerServiceNetworkProfile>,
    #[serde(rename = "aadProfile", default, skip_serializing_if = "Option::is_none")]
    pub aad_profile: Option<ManagedClusterAadProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorProfile {
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: String,
    #[serde(rename = "orchestratorVersion")]
    pub orchestrator_version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAccessProfile {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessProfile {
    #[serde(rename = "kubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterPoolUpgradeProfile {
    #[serde(rename = "kubernetesVersion")]
    pub kubernetes_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "osType")]
    pub os_type: OsType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upgrades: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterUpgradeProfileProperties {
    #[serde(rename = "controlPlaneProfile")]
    pub control_plane_profile: ManagedClusterPoolUpgradeProfile,
    #[serde(rename = "agentPoolProfiles")]
    pub agent_pool_profiles: Vec<ManagedClusterPoolUpgradeProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAadProfile {
    #[serde(rename = "clientAppID")]
    pub client_app_id: String,
    #[serde(rename = "serverAppID")]
    pub server_app_id: String,
    #[serde(rename = "serverAppSecret", default, skip_serializing_if = "Option::is_none")]
    pub server_app_secret: Option<String>,
    #[serde(rename = "tenantID", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAddonProfile {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterUpgradeProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub properties: ManagedClusterUpgradeProfileProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialResults {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kubeconfigs: Vec<CredentialResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorVersionProfile {
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: String,
    #[serde(rename = "orchestratorVersion")]
    pub orchestrator_version: String,
    pub default: bool,
    pub upgrades: Vec<OrchestratorProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorVersionProfileProperties {
    pub orchestrators: Vec<OrchestratorVersionProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorVersionProfileListResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub properties: OrchestratorVersionProfileProperties,
}
