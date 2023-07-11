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
pub struct IoArgoprojEventsV1alpha1RedisStreamEventSource {
    #[serde(rename = "consumerGroup", skip_serializing_if = "Option::is_none")]
    pub consumer_group: Option<String>,
    #[serde(rename = "db", skip_serializing_if = "Option::is_none")]
    pub db: Option<i32>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::IoArgoprojEventsV1alpha1EventSourceFilter>>,
    #[serde(rename = "hostAddress", skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    #[serde(rename = "maxMsgCountPerRead", skip_serializing_if = "Option::is_none")]
    pub max_msg_count_per_read: Option<i32>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<crate::models::SecretKeySelector>>,
    /// Streams to look for entries. XREADGROUP is used on all streams using a single consumer group.
    #[serde(rename = "streams", skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<String>>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<crate::models::IoArgoprojEventsV1alpha1TlsConfig>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl IoArgoprojEventsV1alpha1RedisStreamEventSource {
    pub fn new() -> IoArgoprojEventsV1alpha1RedisStreamEventSource {
        IoArgoprojEventsV1alpha1RedisStreamEventSource {
            consumer_group: None,
            db: None,
            filter: None,
            host_address: None,
            max_msg_count_per_read: None,
            metadata: None,
            password: None,
            streams: None,
            tls: None,
            username: None,
        }
    }
}


