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
pub struct IoArgoprojWorkflowV1alpha1HttpHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "valueFrom", skip_serializing_if = "Option::is_none")]
    pub value_from: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HttpHeaderSource>>,
}

impl IoArgoprojWorkflowV1alpha1HttpHeader {
    pub fn new(name: String) -> IoArgoprojWorkflowV1alpha1HttpHeader {
        IoArgoprojWorkflowV1alpha1HttpHeader {
            name,
            value: None,
            value_from: None,
        }
    }
}

