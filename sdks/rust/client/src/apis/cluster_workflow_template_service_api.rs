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

/// struct for passing parameters to the method `create_cluster_workflow_template`
#[derive(Clone, Debug)]
pub struct CreateClusterWorkflowTemplateParams {
    pub body: crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateCreateRequest
}

/// struct for passing parameters to the method `delete_cluster_workflow_template`
#[derive(Clone, Debug)]
pub struct DeleteClusterWorkflowTemplateParams {
    pub name: String,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. +optional.
    pub delete_options_grace_period_seconds: Option<String>,
    /// Specifies the target UID. +optional.
    pub delete_options_preconditions_uid: Option<String>,
    /// Specifies the target ResourceVersion +optional.
    pub delete_options_preconditions_resource_version: Option<String>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. +optional.
    pub delete_options_orphan_dependents: Option<bool>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. +optional.
    pub delete_options_propagation_policy: Option<String>,
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed +optional.
    pub delete_options_dry_run: Option<Vec<String>>
}

/// struct for passing parameters to the method `get_cluster_workflow_template`
#[derive(Clone, Debug)]
pub struct GetClusterWorkflowTemplateParams {
    pub name: String,
    /// resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset +optional
    pub get_options_resource_version: Option<String>
}

/// struct for passing parameters to the method `lint_cluster_workflow_template`
#[derive(Clone, Debug)]
pub struct LintClusterWorkflowTemplateParams {
    pub body: crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateLintRequest
}

/// struct for passing parameters to the method `list_cluster_workflow_templates`
#[derive(Clone, Debug)]
pub struct ListClusterWorkflowTemplatesParams {
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
    pub list_options_continue: Option<String>
}

/// struct for passing parameters to the method `update_cluster_workflow_template`
#[derive(Clone, Debug)]
pub struct UpdateClusterWorkflowTemplateParams {
    /// DEPRECATED: This field is ignored.
    pub name: String,
    pub body: crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateUpdateRequest
}


/// struct for typed successes of method `create_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClusterWorkflowTemplateSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `delete_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClusterWorkflowTemplateSuccess {
    Status200(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `get_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterWorkflowTemplateSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `lint_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LintClusterWorkflowTemplateSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_cluster_workflow_templates`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClusterWorkflowTemplatesSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateList),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `update_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateClusterWorkflowTemplateSuccess {
    Status200(crate::models::IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClusterWorkflowTemplateError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClusterWorkflowTemplateError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterWorkflowTemplateError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `lint_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LintClusterWorkflowTemplateError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_cluster_workflow_templates`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClusterWorkflowTemplatesError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_cluster_workflow_template`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateClusterWorkflowTemplateError {
    DefaultResponse(crate::models::GrpcGatewayRuntimeError),
    UnknownValue(serde_json::Value),
}


pub async fn create_cluster_workflow_template(configuration: &configuration::Configuration, params: CreateClusterWorkflowTemplateParams) -> Result<ResponseContent<CreateClusterWorkflowTemplateSuccess>, Error<CreateClusterWorkflowTemplateError>> {
    // unbox the parameters
    let body = params.body;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cluster-workflow-templates", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<CreateClusterWorkflowTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateClusterWorkflowTemplateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_cluster_workflow_template(configuration: &configuration::Configuration, params: DeleteClusterWorkflowTemplateParams) -> Result<ResponseContent<DeleteClusterWorkflowTemplateSuccess>, Error<DeleteClusterWorkflowTemplateError>> {
    // unbox the parameters
    let name = params.name;
    let delete_options_grace_period_seconds = params.delete_options_grace_period_seconds;
    let delete_options_preconditions_uid = params.delete_options_preconditions_uid;
    let delete_options_preconditions_resource_version = params.delete_options_preconditions_resource_version;
    let delete_options_orphan_dependents = params.delete_options_orphan_dependents;
    let delete_options_propagation_policy = params.delete_options_propagation_policy;
    let delete_options_dry_run = params.delete_options_dry_run;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cluster-workflow-templates/{name}", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = delete_options_grace_period_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.gracePeriodSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_preconditions_uid {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.preconditions.uid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_preconditions_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.preconditions.resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_orphan_dependents {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.orphanDependents", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_propagation_policy {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.propagationPolicy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = delete_options_dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("deleteOptions.dryRun", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
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
        let local_var_entity: Option<DeleteClusterWorkflowTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteClusterWorkflowTemplateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cluster_workflow_template(configuration: &configuration::Configuration, params: GetClusterWorkflowTemplateParams) -> Result<ResponseContent<GetClusterWorkflowTemplateSuccess>, Error<GetClusterWorkflowTemplateError>> {
    // unbox the parameters
    let name = params.name;
    let get_options_resource_version = params.get_options_resource_version;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cluster-workflow-templates/{name}", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = get_options_resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("getOptions.resourceVersion", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetClusterWorkflowTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetClusterWorkflowTemplateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn lint_cluster_workflow_template(configuration: &configuration::Configuration, params: LintClusterWorkflowTemplateParams) -> Result<ResponseContent<LintClusterWorkflowTemplateSuccess>, Error<LintClusterWorkflowTemplateError>> {
    // unbox the parameters
    let body = params.body;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cluster-workflow-templates/lint", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<LintClusterWorkflowTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<LintClusterWorkflowTemplateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_cluster_workflow_templates(configuration: &configuration::Configuration, params: ListClusterWorkflowTemplatesParams) -> Result<ResponseContent<ListClusterWorkflowTemplatesSuccess>, Error<ListClusterWorkflowTemplatesError>> {
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


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cluster-workflow-templates", configuration.base_path);
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
        let local_var_entity: Option<ListClusterWorkflowTemplatesSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListClusterWorkflowTemplatesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_cluster_workflow_template(configuration: &configuration::Configuration, params: UpdateClusterWorkflowTemplateParams) -> Result<ResponseContent<UpdateClusterWorkflowTemplateSuccess>, Error<UpdateClusterWorkflowTemplateError>> {
    // unbox the parameters
    let name = params.name;
    let body = params.body;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cluster-workflow-templates/{name}", configuration.base_path, name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<UpdateClusterWorkflowTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateClusterWorkflowTemplateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

