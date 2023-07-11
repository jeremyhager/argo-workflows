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
pub struct IoArgoprojEventsV1alpha1EventDependency {
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventSourceName", skip_serializing_if = "Option::is_none")]
    pub event_source_name: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Box<crate::models::IoArgoprojEventsV1alpha1EventDependencyFilter>>,
    /// FiltersLogicalOperator defines how different filters are evaluated together. Available values: and (&&), or (||) Is optional and if left blank treated as and (&&).
    #[serde(rename = "filtersLogicalOperator", skip_serializing_if = "Option::is_none")]
    pub filters_logical_operator: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<Box<crate::models::IoArgoprojEventsV1alpha1EventDependencyTransformer>>,
}

impl IoArgoprojEventsV1alpha1EventDependency {
    pub fn new() -> IoArgoprojEventsV1alpha1EventDependency {
        IoArgoprojEventsV1alpha1EventDependency {
            event_name: None,
            event_source_name: None,
            filters: None,
            filters_logical_operator: None,
            name: None,
            transform: None,
        }
    }
}

