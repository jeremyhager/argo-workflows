/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojEventsV1alpha1PayloadField : PayloadField binds a value at path within the event payload against a name.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojEventsV1alpha1PayloadField {
    /// Name acts as key that holds the value at the path.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path is the JSONPath of the event's (JSON decoded) data key Path is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'. To access an array value use the index as the key. The dot and wildcard characters can be escaped with '\\\\'. See https://github.com/tidwall/gjson#path-syntax for more information on how to use this.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl IoArgoprojEventsV1alpha1PayloadField {
    /// PayloadField binds a value at path within the event payload against a name.
    pub fn new() -> IoArgoprojEventsV1alpha1PayloadField {
        IoArgoprojEventsV1alpha1PayloadField {
            name: None,
            path: None,
        }
    }
}

