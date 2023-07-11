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
pub struct IoArgoprojWorkflowV1alpha1TransformationStep {
    /// Expression defines an expr expression to apply
    #[serde(rename = "expression")]
    pub expression: String,
}

impl IoArgoprojWorkflowV1alpha1TransformationStep {
    pub fn new(expression: String) -> IoArgoprojWorkflowV1alpha1TransformationStep {
        IoArgoprojWorkflowV1alpha1TransformationStep {
            expression,
        }
    }
}

