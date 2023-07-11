# IoArgoprojWorkflowV1alpha1Workflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | [**crate::models::ObjectMeta**](ObjectMeta.md) |  | 
**spec** | [**crate::models::IoArgoprojWorkflowV1alpha1WorkflowSpec**](io.argoproj.workflow.v1alpha1.WorkflowSpec.md) |  | 
**status** | Option<[**crate::models::IoArgoprojWorkflowV1alpha1WorkflowStatus**](io.argoproj.workflow.v1alpha1.WorkflowStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

