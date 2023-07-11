/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoArgoprojWorkflowV1alpha1ArtifactPaths : ArtifactPaths expands a step from a collection of artifacts



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactPaths {
    #[serde(rename = "archive", skip_serializing_if = "Option::is_none")]
    pub archive: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1ArchiveStrategy>>,
    /// ArchiveLogs indicates if the container logs should be archived
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,
    #[serde(rename = "artifactGC", skip_serializing_if = "Option::is_none")]
    pub artifact_gc: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1ArtifactGc>>,
    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1ArtifactoryArtifact>>,
    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1AzureArtifact>>,
    /// Has this been deleted?
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// From allows an artifact to reference an artifact from a previous step
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// FromExpression, if defined, is evaluated to specify the value for the artifact
    #[serde(rename = "fromExpression", skip_serializing_if = "Option::is_none")]
    pub from_expression: Option<String>,
    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1GcsArtifact>>,
    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1GitArtifact>>,
    /// GlobalName exports an output artifact to the global scope, making it available as '{{io.argoproj.workflow.v1alpha1.outputs.artifacts.XXXX}} and in workflow.status.outputs.artifacts
    #[serde(rename = "globalName", skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HdfsArtifact>>,
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1HttpArtifact>>,
    /// mode bits to use on this file, must be a value between 0 and 0777 set when loading input artifacts.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// name of the artifact. must be unique within a template's inputs/outputs.
    #[serde(rename = "name")]
    pub name: String,
    /// Make Artifacts optional, if Artifacts doesn't generate or exist
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1OssArtifact>>,
    /// Path is the container path to the artifact
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1RawArtifact>>,
    /// If mode is set, apply the permission recursively into the artifact if it is a folder
    #[serde(rename = "recurseMode", skip_serializing_if = "Option::is_none")]
    pub recurse_mode: Option<bool>,
    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<crate::models::IoArgoprojWorkflowV1alpha1S3Artifact>>,
    /// SubPath allows an artifact to be sourced from a subpath within the specified source
    #[serde(rename = "subPath", skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}

impl IoArgoprojWorkflowV1alpha1ArtifactPaths {
    /// ArtifactPaths expands a step from a collection of artifacts
    pub fn new(name: String) -> IoArgoprojWorkflowV1alpha1ArtifactPaths {
        IoArgoprojWorkflowV1alpha1ArtifactPaths {
            archive: None,
            archive_logs: None,
            artifact_gc: None,
            artifactory: None,
            azure: None,
            deleted: None,
            from: None,
            from_expression: None,
            gcs: None,
            git: None,
            global_name: None,
            hdfs: None,
            http: None,
            mode: None,
            name,
            optional: None,
            oss: None,
            path: None,
            raw: None,
            recurse_mode: None,
            s3: None,
            sub_path: None,
        }
    }
}

