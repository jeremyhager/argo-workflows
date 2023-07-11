/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpGetAction : HTTPGetAction describes an action based on HTTP Get requests.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpGetAction {
    /// Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders", skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<Vec<crate::models::HttpHeader>>,
    /// Path to access on the HTTP server.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "port")]
    pub port: String,
    /// Scheme to use for connecting to the host. Defaults to HTTP.  Possible enum values:  - `\"HTTP\"` means that the scheme used will be http://  - `\"HTTPS\"` means that the scheme used will be https://
    #[serde(rename = "scheme", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<Scheme>,
}

impl HttpGetAction {
    /// HTTPGetAction describes an action based on HTTP Get requests.
    pub fn new(port: String) -> HttpGetAction {
        HttpGetAction {
            host: None,
            http_headers: None,
            path: None,
            port,
            scheme: None,
        }
    }
}

/// Scheme to use for connecting to the host. Defaults to HTTP.  Possible enum values:  - `\"HTTP\"` means that the scheme used will be http://  - `\"HTTPS\"` means that the scheme used will be https://
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "HTTPS")]
    HTTPS,
}
