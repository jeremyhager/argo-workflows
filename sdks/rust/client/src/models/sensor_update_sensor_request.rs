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
pub struct SensorUpdateSensorRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "sensor", skip_serializing_if = "Option::is_none")]
    pub sensor: Option<Box<crate::models::IoArgoprojEventsV1alpha1Sensor>>,
}

impl SensorUpdateSensorRequest {
    pub fn new() -> SensorUpdateSensorRequest {
        SensorUpdateSensorRequest {
            name: None,
            namespace: None,
            sensor: None,
        }
    }
}


