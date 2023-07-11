/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupVersionResource : +protobuf.options.(gogoproto.goproto_stringer)=false



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupVersionResource {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GroupVersionResource {
    /// +protobuf.options.(gogoproto.goproto_stringer)=false
    pub fn new() -> GroupVersionResource {
        GroupVersionResource {
            group: None,
            resource: None,
            version: None,
        }
    }
}

