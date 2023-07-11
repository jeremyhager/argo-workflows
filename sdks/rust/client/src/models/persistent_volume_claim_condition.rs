/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PersistentVolumeClaimCondition : PersistentVolumeClaimCondition contails details about state of pvc



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistentVolumeClaimCondition {
    /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
    #[serde(rename = "lastProbeTime", skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<String>,
    /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"ResizeStarted\" that means the underlying persistent volume is being resized.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    ///    Possible enum values:  - `\"FileSystemResizePending\"` - controller resize is finished and a file system resize is pending on node  - `\"Resizing\"` - a user trigger resize of pvc has been started
    #[serde(rename = "type")]
    pub _type: Type,
}

impl PersistentVolumeClaimCondition {
    /// PersistentVolumeClaimCondition contails details about state of pvc
    pub fn new(status: String, _type: Type) -> PersistentVolumeClaimCondition {
        PersistentVolumeClaimCondition {
            last_probe_time: None,
            last_transition_time: None,
            message: None,
            reason: None,
            status,
            _type,
        }
    }
}

///    Possible enum values:  - `\"FileSystemResizePending\"` - controller resize is finished and a file system resize is pending on node  - `\"Resizing\"` - a user trigger resize of pvc has been started
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FileSystemResizePending")]
    FileSystemResizePending,
    #[serde(rename = "Resizing")]
    Resizing,
}
