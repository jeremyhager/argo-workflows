/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1Arguments : Arguments to a template



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1Arguments {
    /// Artifacts is the list of artifacts to pass to the template or workflow
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<crate::models::IoArgoprojWorkflowV1alpha1Artifact>>,
    /// Parameters is the list of parameters to pass to the template or workflow
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::IoArgoprojWorkflowV1alpha1Parameter>>,
}

impl IoArgoprojWorkflowV1alpha1Arguments {
    /// Arguments to a template
    pub fn new() -> IoArgoprojWorkflowV1alpha1Arguments {
        IoArgoprojWorkflowV1alpha1Arguments {
            artifacts: None,
            parameters: None,
        }
    }
}

