/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1ArtifactRepository : ArtifactRepository represents an artifact repository in which a controller will store its artifacts



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactRepository {
    /// ArchiveLogs enables log archiving
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,
    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1ArtifactoryArtifactRepository>>,
    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1AzureArtifactRepository>>,
    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1GcsArtifactRepository>>,
    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HdfsArtifactRepository>>,
    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1OssArtifactRepository>>,
    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1S3ArtifactRepository>>,
}

impl IoArgoprojWorkflowV1alpha1ArtifactRepository {
    /// ArtifactRepository represents an artifact repository in which a controller will store its artifacts
    pub fn new() -> IoArgoprojWorkflowV1alpha1ArtifactRepository {
        IoArgoprojWorkflowV1alpha1ArtifactRepository {
            archive_logs: None,
            artifactory: None,
            azure: None,
            gcs: None,
            hdfs: None,
            oss: None,
            s3: None,
        }
    }
}

