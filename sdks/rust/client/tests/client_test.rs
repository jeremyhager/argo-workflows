use argo_workflows::apis::configuration;
use argo_workflows::apis::workflow_service_api;
use argo_workflows::models::Container;
use argo_workflows::models::IoArgoprojWorkflowV1alpha1Template as WorkflowTemplate;
use argo_workflows::models::IoArgoprojWorkflowV1alpha1Workflow as Workflow;
use argo_workflows::models::IoArgoprojWorkflowV1alpha1WorkflowCreateRequest as CreateRequest;
use argo_workflows::models::IoArgoprojWorkflowV1alpha1WorkflowSpec as WorkflowSpec;
use argo_workflows::models::ObjectMeta;
use core::panic;
use std::env;

use tokio;

#[tokio::test]
async fn test_create_workflow() {
    // There are a few ways to init structs, either mut or create a new struct object and pass that to another struct object
    // No sense in creating 2 objects when 1 will do nothing, so just create a mut for testing.

    // prepare structs
    let mut metadata = ObjectMeta::new();
    metadata.generate_name = Some(String::from("hello-world-"));

    let mut container = Container::new(String::from("argoproj/argosay:v2"));
    container.command = Some(vec![String::from("cowsay")]);
    container.args = Some(vec![String::from("hello, world!")]);

    let mut template = WorkflowTemplate::new();
    template.name = Some(String::from("whalesay"));
    template.container = Some(Box::new(container));

    let mut spec = WorkflowSpec::new();
    spec.entrypoint = Some(String::from("whalesay"));
    spec.templates = Some(vec![template]);

    let manifest = Workflow::new(metadata, spec);

    println!("ARGO_TOKEN: {}", env::var("ARGO_TOKEN").expect("ARGO_TOKEN not set"));

    let config = configuration::Configuration {
        api_key: Some(configuration::ApiKey {
            prefix: Some(String::from("BearerToken")),
            key: env::var("ARGO_TOKEN").expect("ARGO_TOKEN not set"),
        }),
        base_path: String::from("http://127.0.0.1:2746"),
        ..Default::default()
    };

    let mut request = CreateRequest::new();
    request.workflow = Some(Box::new(manifest));

    let create_workflow_params = workflow_service_api::CreateWorkflowParams {
        namespace: String::from("argo"),
        body: request,
    };

    let list_workflow_params = init_workflow_params(String::from("argo"));

    match workflow_service_api::create_workflow(&config, create_workflow_params).await {
        Ok(_) => (),
        Err(err) => {
            println!("Could not create workflow. config:\n{:#?}", &config);
            panic!("{:#?}", &err);
        }
    }

    match workflow_service_api::list_workflows(&config, list_workflow_params).await {
        Ok(success) => match &success.entity {
            Some(value) => match &value {
                workflow_service_api::ListWorkflowsSuccess::Status200(_) => (),
                workflow_service_api::ListWorkflowsSuccess::UnknownValue(unknown_value) => {
                    // Ensure that the workflow was actually created
                    match &unknown_value {
                        serde_json::Value::Object(obj) => match obj.get("items") {
                            Some(obj_items) => match obj_items {
                                serde_json::Value::Null => {
                                    panic!("No workflow items found:\n{:#}", &unknown_value)
                                }
                                _ => (),
                            },
                            _ => (),
                        },
                        _ => panic!("No items found"),
                    }
                }
            },
            None => panic!("No workflows found"),
        },
        Err(err) => panic!("Error getting list of workflows:\n{:#?}", err.to_string()),
    }
}

// openapi gen doesn't create a New method for structs under /apis
/// namespace is the only parameter needed for testing
fn init_workflow_params(namespace: String) -> workflow_service_api::ListWorkflowsParams {
    workflow_service_api::ListWorkflowsParams {
        namespace,
        list_options_label_selector: None,
        list_options_field_selector: None,
        list_options_watch: None,
        list_options_allow_watch_bookmarks: None,
        list_options_resource_version: None,
        list_options_resource_version_match: None,
        list_options_timeout_seconds: None,
        list_options_limit: None,
        list_options_continue: None,
        fields: None,
    }
}