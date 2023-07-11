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
pub struct IoArgoprojWorkflowV1alpha1Http {
    /// Body is content of the HTTP Request
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "bodyFrom", skip_serializing_if = "Option::is_none")]
    pub body_from: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HttpBodySource>>,
    /// Headers are an optional list of headers to send with HTTP requests
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<crate::models::IoArgoprojWorkflowV1alpha1HttpHeader>>,
    /// InsecureSkipVerify is a bool when if set to true will skip TLS verification for the HTTP client
    #[serde(rename = "insecureSkipVerify", skip_serializing_if = "Option::is_none")]
    pub insecure_skip_verify: Option<bool>,
    /// Method is HTTP methods for HTTP Request
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// SuccessCondition is an expression if evaluated to true is considered successful
    #[serde(rename = "successCondition", skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
    /// TimeoutSeconds is request timeout for HTTP Request. Default is 30 seconds
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// URL of the HTTP Request
    #[serde(rename = "url")]
    pub url: String,
}

impl IoArgoprojWorkflowV1alpha1Http {
    pub fn new(url: String) -> IoArgoprojWorkflowV1alpha1Http {
        IoArgoprojWorkflowV1alpha1Http {
            body: None,
            body_from: None,
            headers: None,
            insecure_skip_verify: None,
            method: None,
            success_condition: None,
            timeout_seconds: None,
            url,
        }
    }
}

