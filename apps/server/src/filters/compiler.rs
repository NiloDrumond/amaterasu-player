//! Compiles a `FilterNode` tree into a parameterized SQL `WHERE` fragment
//! appended to an existing `sqlx::QueryBuilder`.
//!
//! Compilers are entity-specific so we can validate which fields are legal
//! per entity (e.g. `Field::Album` only on tracks).
//!
//! Caller is expected to have already opened a `WHERE` (or `AND`) and pass
//! the builder ready to append a SQL boolean fragment.

use sqlx::{Postgres, QueryBuilder};
use thiserror::Error;
use uuid::Uuid;

use super::types::{Field, FilterNode, FilterValue, GroupOp, Operator};

const MAX_DEPTH: usize = 8;
const MAX_NODES: usize = 64;

#[derive(Debug, Error)]
pub enum FilterCompileError {
    #[error("filter tree exceeds maximum depth of {MAX_DEPTH}")]
    TooDeep,
    #[error("filter tree exceeds maximum node count of {MAX_NODES}")]
    TooLarge,
    #[error("field {field:?} is not allowed for this entity")]
    UnsupportedField { field: Field },
    #[error("operator {op:?} is not valid for field {field:?}")]
    UnsupportedOperator { field: Field, op: Operator },
    #[error("value type does not match operator {op:?} for field {field:?}")]
    ValueTypeMismatch { field: Field, op: Operator },
    #[error("group must contain at least one child")]
    EmptyGroup,
}

/// Pre-validates structural limits on a filter tree.
fn validate_limits(node: &FilterNode) -> Result<(), FilterCompileError> {
    if node.depth() > MAX_DEPTH {
        return Err(FilterCompileError::TooDeep);
    }
    if node.node_count() > MAX_NODES {
        return Err(FilterCompileError::TooLarge);
    }
    Ok(())
}

/// Compiles a filter tree against the `tracks` table.
///
/// The query builder is expected to have a SELECT/FROM clause already; this
/// appends the boolean fragment and pushes binds. The fragment is wrapped in
/// parentheses, so the caller can safely combine it with other conditions
/// using `AND`/`OR`.
pub fn compile_tracks_filter(
    qb: &mut QueryBuilder<'_, Postgres>,
    node: &FilterNode,
    user_id: Uuid,
) -> Result<(), FilterCompileError> {
    validate_limits(node)?;
    compile_node(
        qb,
        node,
        Ctx {
            entity: EntityCtx::Tracks,
            user_id,
        },
    )
}

/// Compiles a filter tree against the `albums` table.
pub fn compile_albums_filter(
    qb: &mut QueryBuilder<'_, Postgres>,
    node: &FilterNode,
) -> Result<(), FilterCompileError> {
    validate_limits(node)?;
    compile_node(
        qb,
        node,
        Ctx {
            entity: EntityCtx::Albums,
            // Albums have no user-scoped fields; never read.
            user_id: Uuid::nil(),
        },
    )
}

#[derive(Clone, Copy)]
enum EntityCtx {
    Tracks,
    Albums,
}

#[derive(Clone, Copy)]
struct Ctx {
    entity: EntityCtx,
    user_id: Uuid,
}

fn compile_node(
    qb: &mut QueryBuilder<'_, Postgres>,
    node: &FilterNode,
    ctx: Ctx,
) -> Result<(), FilterCompileError> {
    match node {
        FilterNode::Group {
            op,
            negate,
            children,
        } => {
            if children.is_empty() {
                return Err(FilterCompileError::EmptyGroup);
            }
            if *negate {
                qb.push("NOT ");
            }
            qb.push("(");
            let separator = match op {
                GroupOp::And => " AND ",
                GroupOp::Or => " OR ",
            };
            for (i, child) in children.iter().enumerate() {
                if i > 0 {
                    qb.push(separator);
                }
                compile_node(qb, child, ctx)?;
            }
            qb.push(")");
            Ok(())
        }
        FilterNode::Leaf {
            field,
            op,
            value,
            negate,
        } => {
            if *negate {
                qb.push("NOT ");
            }
            qb.push("(");
            compile_leaf(qb, *field, *op, value, ctx)?;
            qb.push(")");
            Ok(())
        }
    }
}

fn compile_leaf(
    qb: &mut QueryBuilder<'_, Postgres>,
    field: Field,
    op: Operator,
    value: &FilterValue,
    ctx: Ctx,
) -> Result<(), FilterCompileError> {
    match (ctx.entity, field) {
        (EntityCtx::Tracks, Field::Text) => match (op, value) {
            (Operator::Contains, FilterValue::Text { value: v }) => {
                qb.push("(tracks.title ILIKE ");
                qb.push_bind(like_escape(v));
                qb.push(" OR COALESCE(tracks.original_title, '') ILIKE ");
                qb.push_bind(like_escape(v));
                qb.push(")");
                Ok(())
            }
            _ => Err(FilterCompileError::UnsupportedOperator { field, op }),
        },
        (EntityCtx::Tracks, Field::Album) => match (op, value) {
            (Operator::Eq, FilterValue::Id { value: id }) => {
                qb.push("tracks.album_id = ");
                qb.push_bind(*id);
                Ok(())
            }
            (Operator::In, FilterValue::Ids { value: ids }) => {
                push_uuid_in(qb, "tracks.album_id", ids);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Tracks, Field::Artist) => match (op, value) {
            (Operator::Eq, FilterValue::Id { value: id }) => {
                qb.push("tracks.artist_id = ");
                qb.push_bind(*id);
                Ok(())
            }
            (Operator::In, FilterValue::Ids { value: ids }) => {
                push_uuid_in(qb, "tracks.artist_id", ids);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Tracks, Field::Tag) => match (op, value) {
            (Operator::Eq, FilterValue::Id { value: id }) => {
                push_track_tag_exists(qb, &[*id]);
                Ok(())
            }
            (Operator::In, FilterValue::Ids { value: ids }) => {
                push_track_tag_exists(qb, ids);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Tracks, Field::Year) => match (op, value) {
            (Operator::Between, FilterValue::IntRange { min, max }) => {
                push_year_range(qb, "tracks.date", *min, *max);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Tracks, Field::DurationMs) => match (op, value) {
            (Operator::Between, FilterValue::IntRange { min, max }) => {
                push_int_range(qb, "tracks.duration_ms", *min, *max);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Tracks, Field::Favorite) => match (op, value) {
            (Operator::Eq, FilterValue::Bool { value: want }) => {
                push_track_favorite_exists(qb, ctx.user_id, *want);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },

        (EntityCtx::Albums, Field::Text) => match (op, value) {
            (Operator::Contains, FilterValue::Text { value: v }) => {
                qb.push("albums.title ILIKE ");
                qb.push_bind(like_escape(v));
                Ok(())
            }
            _ => Err(FilterCompileError::UnsupportedOperator { field, op }),
        },
        (EntityCtx::Albums, Field::Artist) => match (op, value) {
            (Operator::Eq, FilterValue::Id { value: id }) => {
                qb.push("albums.artist_id = ");
                qb.push_bind(*id);
                Ok(())
            }
            (Operator::In, FilterValue::Ids { value: ids }) => {
                push_uuid_in(qb, "albums.artist_id", ids);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Albums, Field::Tag) => match (op, value) {
            (Operator::Eq, FilterValue::Id { value: id }) => {
                push_album_tag_exists(qb, &[*id]);
                Ok(())
            }
            (Operator::In, FilterValue::Ids { value: ids }) => {
                push_album_tag_exists(qb, ids);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },
        (EntityCtx::Albums, Field::Year) => match (op, value) {
            (Operator::Between, FilterValue::IntRange { min, max }) => {
                push_year_range(qb, "albums.date", *min, *max);
                Ok(())
            }
            _ => Err(FilterCompileError::ValueTypeMismatch { field, op }),
        },

        // Disallowed combinations:
        (EntityCtx::Albums, Field::Album) => Err(FilterCompileError::UnsupportedField { field }),
        (EntityCtx::Albums, Field::DurationMs) => {
            Err(FilterCompileError::UnsupportedField { field })
        }
        (EntityCtx::Albums, Field::Favorite) => Err(FilterCompileError::UnsupportedField { field }),
    }
}

fn like_escape(input: &str) -> String {
    let escaped: String = input
        .chars()
        .flat_map(|c| match c {
            '%' | '_' | '\\' => vec!['\\', c],
            other => vec![other],
        })
        .collect();
    format!("%{}%", escaped)
}

fn push_uuid_in(qb: &mut QueryBuilder<'_, Postgres>, column: &str, ids: &[Uuid]) {
    if ids.is_empty() {
        // `column = ANY(empty)` is FALSE, which is the correct semantic.
        qb.push("FALSE");
        return;
    }
    qb.push(column)
        .push(" = ANY(")
        .push_bind(ids.to_vec())
        .push(")");
}

fn push_track_tag_exists(qb: &mut QueryBuilder<'_, Postgres>, ids: &[Uuid]) {
    if ids.is_empty() {
        qb.push("FALSE");
        return;
    }
    qb.push(
        "EXISTS (SELECT 1 FROM track_tags tt WHERE tt.track_id = tracks.id AND tt.tag_id = ANY(",
    )
    .push_bind(ids.to_vec())
    .push("))");
}

/// Emits an `EXISTS` (or `NOT EXISTS` when `want` is false) check against the
/// current user's favorites for the track.
fn push_track_favorite_exists(qb: &mut QueryBuilder<'_, Postgres>, user_id: Uuid, want: bool) {
    if !want {
        qb.push("NOT ");
    }
    qb.push(
        "EXISTS (SELECT 1 FROM track_favorites tf WHERE tf.track_id = tracks.id AND tf.user_id = ",
    )
    .push_bind(user_id)
    .push(")");
}

fn push_album_tag_exists(qb: &mut QueryBuilder<'_, Postgres>, ids: &[Uuid]) {
    if ids.is_empty() {
        qb.push("FALSE");
        return;
    }
    qb.push(
        "EXISTS (SELECT 1 FROM album_tags at WHERE at.album_id = albums.id AND at.tag_id = ANY(",
    )
    .push_bind(ids.to_vec())
    .push("))");
}

fn push_int_range(
    qb: &mut QueryBuilder<'_, Postgres>,
    column: &str,
    min: Option<i64>,
    max: Option<i64>,
) {
    let (min, max) = (min, max);
    if min.is_none() && max.is_none() {
        qb.push("TRUE");
        return;
    }
    qb.push("(");
    let mut first = true;
    if let Some(m) = min {
        qb.push(column).push(" >= ").push_bind(m);
        first = false;
    }
    if let Some(m) = max {
        if !first {
            qb.push(" AND ");
        }
        qb.push(column).push(" <= ").push_bind(m);
    }
    qb.push(")");
}

fn push_year_range(
    qb: &mut QueryBuilder<'_, Postgres>,
    column: &str,
    min: Option<i64>,
    max: Option<i64>,
) {
    if min.is_none() && max.is_none() {
        qb.push("TRUE");
        return;
    }
    qb.push("(");
    let mut first = true;
    if let Some(m) = min {
        qb.push("EXTRACT(YEAR FROM ")
            .push(column)
            .push(") >= ")
            .push_bind(m);
        first = false;
    }
    if let Some(m) = max {
        if !first {
            qb.push(" AND ");
        }
        qb.push("EXTRACT(YEAR FROM ")
            .push(column)
            .push(") <= ")
            .push_bind(m);
    }
    qb.push(")");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::types::{Field, FilterNode, FilterValue, GroupOp, Operator};

    fn leaf(field: Field, op: Operator, value: FilterValue) -> FilterNode {
        FilterNode::Leaf {
            field,
            op,
            value,
            negate: false,
        }
    }

    fn negated(mut node: FilterNode) -> FilterNode {
        match &mut node {
            FilterNode::Leaf { negate, .. } => *negate = true,
            FilterNode::Group { negate, .. } => *negate = true,
        }
        node
    }

    fn compile_to_sql(node: &FilterNode) -> String {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        compile_tracks_filter(&mut qb, node, Uuid::nil()).unwrap();
        qb.into_sql()
    }

    #[test]
    fn text_contains_emits_ilike() {
        let n = leaf(
            Field::Text,
            Operator::Contains,
            FilterValue::Text {
                value: "moon".into(),
            },
        );
        let sql = compile_to_sql(&n);
        assert!(sql.contains("ILIKE"));
        assert!(sql.contains("tracks.title"));
    }

    #[test]
    fn negated_leaf_wraps_in_not() {
        let n = negated(leaf(
            Field::Album,
            Operator::Eq,
            FilterValue::Id { value: Uuid::nil() },
        ));
        let sql = compile_to_sql(&n);
        assert!(sql.starts_with("SELECT 1 WHERE NOT ("));
    }

    #[test]
    fn nested_groups() {
        let n = FilterNode::Group {
            op: GroupOp::And,
            negate: false,
            children: vec![
                leaf(
                    Field::Tag,
                    Operator::Eq,
                    FilterValue::Id { value: Uuid::nil() },
                ),
                FilterNode::Group {
                    op: GroupOp::Or,
                    negate: true,
                    children: vec![leaf(
                        Field::Artist,
                        Operator::Eq,
                        FilterValue::Id { value: Uuid::nil() },
                    )],
                },
            ],
        };
        let sql = compile_to_sql(&n);
        assert!(sql.contains(" AND "));
        assert!(sql.contains("NOT ("));
    }

    #[test]
    fn favorite_true_emits_exists() {
        let n = leaf(
            Field::Favorite,
            Operator::Eq,
            FilterValue::Bool { value: true },
        );
        let sql = compile_to_sql(&n);
        assert!(sql.contains("EXISTS (SELECT 1 FROM track_favorites"));
        assert!(!sql.contains("NOT EXISTS"));
    }

    #[test]
    fn favorite_false_emits_not_exists() {
        let n = leaf(
            Field::Favorite,
            Operator::Eq,
            FilterValue::Bool { value: false },
        );
        let sql = compile_to_sql(&n);
        assert!(sql.contains("NOT EXISTS (SELECT 1 FROM track_favorites"));
    }

    #[test]
    fn favorite_rejected_for_albums() {
        let mut qb = QueryBuilder::<Postgres>::new("");
        let n = leaf(
            Field::Favorite,
            Operator::Eq,
            FilterValue::Bool { value: true },
        );
        let err = compile_albums_filter(&mut qb, &n).unwrap_err();
        assert!(matches!(err, FilterCompileError::UnsupportedField { .. }));
    }

    #[test]
    fn unsupported_field_for_entity_rejected() {
        let mut qb = QueryBuilder::<Postgres>::new("");
        let n = leaf(
            Field::Album,
            Operator::Eq,
            FilterValue::Id { value: Uuid::nil() },
        );
        let err = compile_albums_filter(&mut qb, &n).unwrap_err();
        matches!(err, FilterCompileError::UnsupportedField { .. });
    }
}
