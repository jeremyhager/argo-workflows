/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojEventsV1alpha1GitRemoteConfig {
    /// Name of the remote to fetch from.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URLs the URLs of a remote repository. It must be non-empty. Fetch will always use the first URL, while push will use all of them.
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl IoArgoprojEventsV1alpha1GitRemoteConfig {
    pub fn new() -> IoArgoprojEventsV1alpha1GitRemoteConfig {
        IoArgoprojEventsV1alpha1GitRemoteConfig {
            name: None,
            urls: None,
        }
    }
}


