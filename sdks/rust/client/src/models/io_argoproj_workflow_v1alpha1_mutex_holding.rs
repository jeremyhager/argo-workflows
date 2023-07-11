/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1MutexHolding : MutexHolding describes the mutex and the object which is holding it.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1MutexHolding {
    /// Holder is a reference to the object which holds the Mutex. Holding Scenario:   1. Current workflow's NodeID which is holding the lock.      e.g: ${NodeID} Waiting Scenario:   1. Current workflow or other workflow NodeID which is holding the lock.      e.g: ${WorkflowName}/${NodeID}
    #[serde(rename = "holder", skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    /// Reference for the mutex e.g: ${namespace}/mutex/${mutexName}
    #[serde(rename = "mutex", skip_serializing_if = "Option::is_none")]
    pub mutex: Option<String>,
}

impl IoArgoprojWorkflowV1alpha1MutexHolding {
    /// MutexHolding describes the mutex and the object which is holding it.
    pub fn new() -> IoArgoprojWorkflowV1alpha1MutexHolding {
        IoArgoprojWorkflowV1alpha1MutexHolding {
            holder: None,
            mutex: None,
        }
    }
}

