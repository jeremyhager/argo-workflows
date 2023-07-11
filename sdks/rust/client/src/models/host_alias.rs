/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HostAlias : HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostAlias {
    /// Hostnames for the above IP address.
    #[serde(rename = "hostnames", skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// IP address of the host file entry.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl HostAlias {
    /// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
    pub fn new() -> HostAlias {
        HostAlias {
            hostnames: None,
            ip: None,
        }
    }
}

