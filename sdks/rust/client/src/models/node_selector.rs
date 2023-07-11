/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NodeSelector : A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<crate::models::NodeSelectorTerm>,
}

impl NodeSelector {
    /// A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
    pub fn new(node_selector_terms: Vec<crate::models::NodeSelectorTerm>) -> NodeSelector {
        NodeSelector {
            node_selector_terms,
        }
    }
}

