/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PodDnsConfigOption : PodDNSConfigOption defines DNS resolver options of a pod.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PodDnsConfigOption {
    /// Required.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PodDnsConfigOption {
    /// PodDNSConfigOption defines DNS resolver options of a pod.
    pub fn new() -> PodDnsConfigOption {
        PodDnsConfigOption {
            name: None,
            value: None,
        }
    }
}


