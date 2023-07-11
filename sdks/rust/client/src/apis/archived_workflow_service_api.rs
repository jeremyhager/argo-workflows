/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `delete_archived_workflow`
#[derive(Clone, Debug)]
pub struct DeleteArchivedWorkflowParams {
    pub uid: String,
    pub namespace: Option<String>
}

/// struct for passing parameters to the method `get_archived_workflow`
#[derive(Clone, Debug)]
pub struct GetArchivedWorkflowParams {
    pub uid: String,
    pub namespace: Option<String>,
    pub name: Option<String>
}

/// struct for passing parameters to the method `list_archived_workflow_label_keys`
#[derive(Clone, Debug)]
pub struct ListArchivedWorkflowLabelKeysParams {
    pub namespace: Option<String>
}

/// struct for passing parameters to the method `list_archived_workflow_label_values`
#[derive(Clone, Debug)]
pub struct ListArchivedWorkflowLabelValuesParams {
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything. +optional.
    pub list_options_label_selector: Option<String>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything. +optional.
    pub list_options_field_selector: Option<String>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. +optional.
    pub list_options_watch: Option<bool>,
    /// allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. +optional.
    pub list_options_allow_watch_bookmarks: Option<bool>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset +optional
    pub list_options_resource_version: Option<String>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset +optional
    pub list_options_resource_version_match: Option<String>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. +optional.
    pub list_options_timeout_seconds: Option<String>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub list_options_limit: Option<String>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub list_options_continue: Option<String>,
    pub namespace: Option<String>
}

/// struct for passing parameters to the method `list_archived_workflows`
#[derive(Clone, Debug)]
pub struct ListArchivedWorkflowsParams {
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything. +optional.
    pub list_options_label_selector: Option<String>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything. +optional.
    pub list_options_field_selector: Option<String>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. +optional.
    pub list_options_watch: Option<bool>,
    /// allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. +optional.
    pub list_options_allow_watch_bookmarks: Option<bool>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset +optional
    pub list_options_resource_version: Option<String>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset +optional
    pub list_options_resource_version_match: Option<String>,
    /// Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. +optional.
    pub list_options_timeout_seconds: Option<String>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub list_options_limit: Option<String>,
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub list_options_continue: Option<String>,
    pub name_prefix: Option<String>,
    pub namespace: Option<String>
}

/// struct for passing parameters to the method `resubmit_archived_workflow`
#[derive(Clone, Debug)]
pub struct ResubmitArchivedWorkflowParams {
    pub uid: String,
    pub body: crate::models::IoArgoprojWorkflowV1alpha1ResubmitArchivedWorkflowRequest
}

/// struct for passing parameters to the method `retry_archived_workflow`
#[derive(Clone, Debug)]
pub struct RetryArchivedWorkflowParams {
    pub uid: String,
    pub body: crate::models::IoArgoprojWorkflowV1alpha1RetryArchivedWorkflowRequest
}


/// struct for typed successes of method `delete_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteArchivedWorkflowSuccess {
    Status200(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `get_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetArchivedWorkflowSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1Workflow),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_archived_workflow_label_keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchivedWorkflowLabelKeysSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1LabelKeys),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_archived_workflow_label_values`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchivedWorkflowLabelValuesSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1LabelValues),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_archived_workflows`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchivedWorkflowsSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1WorkflowList),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `resubmit_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResubmitArchivedWorkflowSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1Workflow),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `retry_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetryArchivedWorkflowSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1Workflow),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteArchivedWorkflowError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetArchivedWorkflowError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_archived_workflow_label_keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchivedWorkflowLabelKeysError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_archived_workflow_label_values`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchivedWorkflowLabelValuesError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_archived_workflows`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchivedWorkflowsError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `resubmit_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResubmitArchivedWorkflowError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `retry_archived_workflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetryArchivedWorkflowError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}


pub async fn delete_archived_workflow(configuration: &configuration::Configuration, params: DeleteArchivedWorkflowParams) -> Result<ResponseContent<DeleteArchivedWorkflowSuccess>, Error<DeleteArchivedWorkflowError>> {
    // unbox the parameters
    let uid = params.uid;
    let namespace = params.namespace;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows/{uid}", configuration.base_path, uid=crate::apis::urlencode(uid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DeleteArchivedWorkflowSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteArchivedWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_archived_workflow(configuration: &configuration::Configuration, params: GetArchivedWorkflowParams) -> Result<ResponseContent<GetArchivedWorkflowSuccess>, Error<GetArchivedWorkflowError>> {
    // unbox the parameters
    let uid = params.uid;
    let namespace = params.namespace;
    let name = params.name;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows/{uid}", configuration.base_path, uid=crate::apis::urlencode(uid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetArchivedWorkflowSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetArchivedWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_archived_workflow_label_keys(configuration: &configuration::Configuration, params: ListArchivedWorkflowLabelKeysParams) -> Result<ResponseContent<ListArchivedWorkflowLabelKeysSuccess>, Error<ListArchivedWorkflowLabelKeysError>> {
    // unbox the parameters
    let namespace = params.namespace;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows-label-keys", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListArchivedWorkflowLabelKeysSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListArchivedWorkflowLabelKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_archived_workflow_label_values(configuration: &configuration::Configuration, params: ListArchivedWorkflowLabelValuesParams) -> Result<ResponseContent<ListArchivedWorkflowLabelValuesSuccess>, Error<ListArchivedWorkflowLabelValuesError>> {
    // unbox the parameters
    let list_options_label_selector = params.list_options_label_selector;
    let list_options_field_selector = params.list_options_field_selector;
    let list_options_watch = params.list_options_watch;
    let list_options_allow_watch_bookmarks = params.list_options_allow_watch_bookmarks;
    let list_options_resource_version = params.list_options_resource_version;
    let list_options_resource_version_match = params.list_options_resource_version_match;
    let list_options_timeout_seconds = params.list_options_timeout_seconds;
    let list_options_limit = params.list_options_limit;
    let list_options_continue = params.list_options_continue;
    let namespace = params.namespace;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows-label-values", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = list_options_label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.labelSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_field_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.fieldSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_watch {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.watch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_allow_watch_bookmarks {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.allowWatchBookmarks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_resource_version_match {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersionMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_timeout_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.timeoutSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_limit {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_continue {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.continue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListArchivedWorkflowLabelValuesSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListArchivedWorkflowLabelValuesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_archived_workflows(configuration: &configuration::Configuration, params: ListArchivedWorkflowsParams) -> Result<ResponseContent<ListArchivedWorkflowsSuccess>, Error<ListArchivedWorkflowsError>> {
    // unbox the parameters
    let list_options_label_selector = params.list_options_label_selector;
    let list_options_field_selector = params.list_options_field_selector;
    let list_options_watch = params.list_options_watch;
    let list_options_allow_watch_bookmarks = params.list_options_allow_watch_bookmarks;
    let list_options_resource_version = params.list_options_resource_version;
    let list_options_resource_version_match = params.list_options_resource_version_match;
    let list_options_timeout_seconds = params.list_options_timeout_seconds;
    let list_options_limit = params.list_options_limit;
    let list_options_continue = params.list_options_continue;
    let name_prefix = params.name_prefix;
    let namespace = params.namespace;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = list_options_label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.labelSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_field_selector {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.fieldSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_watch {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.watch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_allow_watch_bookmarks {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.allowWatchBookmarks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_resource_version_match {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.resourceVersionMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_timeout_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.timeoutSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_limit {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = list_options_continue {
        local_var_req_builder = local_var_req_builder.query(&[("listOptions.continue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name_prefix {
        local_var_req_builder = local_var_req_builder.query(&[("namePrefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListArchivedWorkflowsSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListArchivedWorkflowsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn resubmit_archived_workflow(configuration: &configuration::Configuration, params: ResubmitArchivedWorkflowParams) -> Result<ResponseContent<ResubmitArchivedWorkflowSuccess>, Error<ResubmitArchivedWorkflowError>> {
    // unbox the parameters
    let uid = params.uid;
    let body = params.body;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows/{uid}/resubmit", configuration.base_path, uid=crate::apis::urlencode(uid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ResubmitArchivedWorkflowSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ResubmitArchivedWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn retry_archived_workflow(configuration: &configuration::Configuration, params: RetryArchivedWorkflowParams) -> Result<ResponseContent<RetryArchivedWorkflowSuccess>, Error<RetryArchivedWorkflowError>> {
    // unbox the parameters
    let uid = params.uid;
    let body = params.body;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/archived-workflows/{uid}/retry", configuration.base_path, uid=crate::apis::urlencode(uid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<RetryArchivedWorkflowSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<RetryArchivedWorkflowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
