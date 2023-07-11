/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1Sequence : Sequence expands a workflow step into numeric range



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1Sequence {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Format is a printf format string to format the value in the sequence
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

impl IoArgoprojWorkflowV1alpha1Sequence {
    /// Sequence expands a workflow step into numeric range
    pub fn new() -> IoArgoprojWorkflowV1alpha1Sequence {
        IoArgoprojWorkflowV1alpha1Sequence {
            count: None,
            end: None,
            format: None,
            start: None,
        }
    }
}

