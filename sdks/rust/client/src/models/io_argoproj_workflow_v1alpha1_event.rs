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
pub struct IoArgoprojWorkflowV1alpha1Event {
    /// Selector (https://github.com/antonmedv/expr) that we must must match the io.argoproj.workflow.v1alpha1. E.g. `payload.message == \"test\"`
    #[serde(rename = "selector")]
    pub selector: String,
}

impl IoArgoprojWorkflowV1alpha1Event {
    pub fn new(selector: String) -> IoArgoprojWorkflowV1alpha1Event {
        IoArgoprojWorkflowV1alpha1Event {
            selector,
        }
    }
}

