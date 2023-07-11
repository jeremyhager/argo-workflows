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
pub struct IoArgoprojEventsV1alpha1ResourceFilter {
    #[serde(rename = "afterStart", skip_serializing_if = "Option::is_none")]
    pub after_start: Option<bool>,
    /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::IoArgoprojEventsV1alpha1Selector>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::models::IoArgoprojEventsV1alpha1Selector>>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl IoArgoprojEventsV1alpha1ResourceFilter {
    pub fn new() -> IoArgoprojEventsV1alpha1ResourceFilter {
        IoArgoprojEventsV1alpha1ResourceFilter {
            after_start: None,
            created_by: None,
            fields: None,
            labels: None,
            prefix: None,
        }
    }
}


