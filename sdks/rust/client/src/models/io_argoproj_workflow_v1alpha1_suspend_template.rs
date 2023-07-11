/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1SuspendTemplate : SuspendTemplate is a template subtype to suspend a workflow at a predetermined point in time



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1SuspendTemplate {
    /// Duration is the seconds to wait before automatically resuming a template. Must be a string. Default unit is seconds. Could also be a Duration, e.g.: \"2m\", \"6h\", \"1d\"
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl IoArgoprojWorkflowV1alpha1SuspendTemplate {
    /// SuspendTemplate is a template subtype to suspend a workflow at a predetermined point in time
    pub fn new() -> IoArgoprojWorkflowV1alpha1SuspendTemplate {
        IoArgoprojWorkflowV1alpha1SuspendTemplate {
            duration: None,
        }
    }
}


