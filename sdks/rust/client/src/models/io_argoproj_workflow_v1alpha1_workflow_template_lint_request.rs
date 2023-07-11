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
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest {
    #[serde(rename = "createOptions", skip_serializing_if = "Option::is_none")]
    pub create_options: Option<Box<crate::models::CreateOptions>>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1WorkflowTemplate>>,
}

impl IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest {
    pub fn new() -> IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest {
        IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest {
            create_options: None,
            namespace: None,
            template: None,
        }
    }
}


