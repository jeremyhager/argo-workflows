/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1ArtifactLocation : ArtifactLocation describes a location for a single or multiple artifacts. It is used as single artifact in the context of inputs/outputs (e.g. outputs.artifacts.artname). It is also used to describe the location of multiple artifacts such as the archive location of a single workflow step, which the executor will use as a default location to store its files.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactLocation {
    /// ArchiveLogs indicates if the container logs should be archived
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,
    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1ArtifactoryArtifact>>,
    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1AzureArtifact>>,
    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1GcsArtifact>>,
    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1GitArtifact>>,
    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HdfsArtifact>>,
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HttpArtifact>>,
    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1OssArtifact>>,
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1RawArtifact>>,
    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1S3Artifact>>,
}

impl IoArgoprojWorkflowV1alpha1ArtifactLocation {
    /// ArtifactLocation describes a location for a single or multiple artifacts. It is used as single artifact in the context of inputs/outputs (e.g. outputs.artifacts.artname). It is also used to describe the location of multiple artifacts such as the archive location of a single workflow step, which the executor will use as a default location to store its files.
    pub fn new() -> IoArgoprojWorkflowV1alpha1ArtifactLocation {
        IoArgoprojWorkflowV1alpha1ArtifactLocation {
            archive_logs: None,
            artifactory: None,
            azure: None,
            gcs: None,
            git: None,
            hdfs: None,
            http: None,
            oss: None,
            raw: None,
            s3: None,
        }
    }
}


