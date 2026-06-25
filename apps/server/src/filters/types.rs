use amaterasu_macros::api_type;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[api_type("filter")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Entity {
    Tracks,
    Albums,
}

#[api_type("filter")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Field {
    /// ILIKE on a primary text column (track title, album title, etc.)
    Text,
    Tag,
    Album,
    Artist,
    Year,
    DurationMs,
    /// Whether the current user has favorited the track. Tracks only.
    Favorite,
}

#[api_type("filter")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Operator {
    Contains,
    Eq,
    In,
    Between,
}

#[api_type("filter")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FilterValue {
    Text {
        value: String,
    },
    Id {
        value: Uuid,
    },
    Ids {
        value: Vec<Uuid>,
    },
    /// Inclusive integer range (either bound optional).
    IntRange {
        min: Option<i64>,
        max: Option<i64>,
    },
    Bool {
        value: bool,
    },
}

/// Recursive filter tree.
///
/// `negate` on either variant wraps the emitted SQL fragment in `NOT (...)`.
#[api_type("filter")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FilterNode {
    Group {
        op: GroupOp,
        #[serde(default)]
        negate: bool,
        children: Vec<FilterNode>,
    },
    Leaf {
        field: Field,
        op: Operator,
        value: FilterValue,
        #[serde(default)]
        negate: bool,
    },
}

#[api_type("filter")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GroupOp {
    And,
    Or,
}

impl FilterNode {
    /// Total number of nodes (used to bound query cost).
    pub fn node_count(&self) -> usize {
        match self {
            FilterNode::Leaf { .. } => 1,
            FilterNode::Group { children, .. } => {
                1 + children.iter().map(FilterNode::node_count).sum::<usize>()
            }
        }
    }

    /// Maximum depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            FilterNode::Leaf { .. } => 1,
            FilterNode::Group { children, .. } => {
                1 + children.iter().map(FilterNode::depth).max().unwrap_or(0)
            }
        }
    }
}
