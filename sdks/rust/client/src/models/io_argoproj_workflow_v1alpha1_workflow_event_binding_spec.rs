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
pub struct IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec {
    #[serde(rename = "event")]
    pub event: Box<crate::models::IoArgoprojWorkflowV1alpha1Event>,
    #[serde(rename = "submit", skip_serializing_if = "Option::is_none")]
    pub submit: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1Submit>>,
}

impl IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec {
    pub fn new(event: crate::models::IoArgoprojWorkflowV1alpha1Event) -> IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec {
        IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec {
            event: Box::new(event),
            submit: None,
        }
    }
}

