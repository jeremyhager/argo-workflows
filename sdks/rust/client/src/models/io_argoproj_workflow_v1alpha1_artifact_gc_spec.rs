/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1ArtifactGcSpec : ArtifactGCSpec specifies the Artifacts that need to be deleted



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactGcSpec {
    /// ArtifactsByNode maps Node name to information pertaining to Artifacts on that Node
    #[serde(rename = "artifactsByNode", skip_serializing_if = "Option::is_none")]
    pub artifacts_by_node: Option<::std::collections::HashMap<String, crate::models::IoArgoprojWorkflowV1alpha1ArtifactNodeSpec>>,
}

impl IoArgoprojWorkflowV1alpha1ArtifactGcSpec {
    /// ArtifactGCSpec specifies the Artifacts that need to be deleted
    pub fn new() -> IoArgoprojWorkflowV1alpha1ArtifactGcSpec {
        IoArgoprojWorkflowV1alpha1ArtifactGcSpec {
            artifacts_by_node: None,
        }
    }
}

