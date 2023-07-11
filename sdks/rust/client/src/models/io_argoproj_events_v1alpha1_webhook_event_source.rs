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
pub struct IoArgoprojEventsV1alpha1WebhookEventSource {
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoArgoprojEventsV1alpha1EventSourceFilter>>,
    #[serde(rename = "webhookContext", skip_serializing_if = "Option::is_none")]
    pub webhook_context: Option<Box<crate::models::IoArgoprojEventsV1alpha1WebhookContext>>,
}

impl IoArgoprojEventsV1alpha1WebhookEventSource {
    pub fn new() -> IoArgoprojEventsV1alpha1WebhookEventSource {
        IoArgoprojEventsV1alpha1WebhookEventSource {
            filter: None,
            webhook_context: None,
        }
    }
}

