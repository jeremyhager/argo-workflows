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
pub struct IoArgoprojEventsV1alpha1StorageGridEventSource {
    /// APIURL is the url of the storagegrid api.
    #[serde(rename = "apiURL", skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(rename = "authToken", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<Box<crate::models::SecretKeySelector>>,
    /// Name of the bucket to register notifications for.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoArgoprojEventsV1alpha1StorageGridFilter>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "topicArn", skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Box<crate::models::IoArgoprojEventsV1alpha1WebhookContext>>,
}

impl IoArgoprojEventsV1alpha1StorageGridEventSource {
    pub fn new() -> IoArgoprojEventsV1alpha1StorageGridEventSource {
        IoArgoprojEventsV1alpha1StorageGridEventSource {
            api_url: None,
            auth_token: None,
            bucket: None,
            events: None,
            filter: None,
            metadata: None,
            region: None,
            topic_arn: None,
            webhook: None,
        }
    }
}

