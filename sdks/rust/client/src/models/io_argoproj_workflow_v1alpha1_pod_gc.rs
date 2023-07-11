/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1PodGc : PodGC describes how to delete completed pods as they complete



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1PodGc {
    #[serde(rename = "labelSelector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::LabelSelector>>,
    /// Strategy is the strategy to use. One of \"OnPodCompletion\", \"OnPodSuccess\", \"OnWorkflowCompletion\", \"OnWorkflowSuccess\". If unset, does not delete Pods
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl IoArgoprojWorkflowV1alpha1PodGc {
    /// PodGC describes how to delete completed pods as they complete
    pub fn new() -> IoArgoprojWorkflowV1alpha1PodGc {
        IoArgoprojWorkflowV1alpha1PodGc {
            label_selector: None,
            strategy: None,
        }
    }
}

