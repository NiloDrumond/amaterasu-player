use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, Query, TermQuery};
use tantivy::schema::IndexRecordOption;
use tantivy::{TantivyDocument, Term};
use uuid::Uuid;

use crate::dto::response::{SearchEntityType, SearchHit};

use super::index::SearchIndex;

pub struct SearchQueryOptions<'a> {
    pub q: &'a str,
    /// If set, only return hits of these types. Empty means all.
    pub types: &'a [SearchEntityType],
    /// If set, filter playlist/collection hits to this user.
    pub user_id: Option<Uuid>,
    pub limit: usize,
    pub offset: usize,
}

impl SearchIndex {
    pub fn search(&self, opts: &SearchQueryOptions<'_>) -> Result<Vec<SearchHit>> {
        let q = opts.q.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }

        let searcher = self.reader.searcher();

        // Tokenize the query lightly: lowercase + split on whitespace.
        // Each token becomes a fuzzy clause; all must match (AND).
        let tokens: Vec<String> = q
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if tokens.is_empty() {
            return Ok(Vec::new());
        }

        // Per-token fuzzy clauses against the `text` field.
        let mut text_clauses: Vec<(Occur, Box<dyn Query>)> = Vec::with_capacity(tokens.len());
        for tok in &tokens {
            let distance = fuzzy_distance(tok);
            let prefix_len = if tok.len() <= 3 { 0 } else { 1 };
            let term = Term::from_field_text(self.schema.text, tok);
            let q: Box<dyn Query> =
                Box::new(FuzzyTermQuery::new_prefix(term, distance, prefix_len > 0));
            text_clauses.push((Occur::Must, q));
        }
        let text_query: Box<dyn Query> = Box::new(BooleanQuery::new(text_clauses));

        // Optional entity-type filter.
        let mut outer: Vec<(Occur, Box<dyn Query>)> = vec![(Occur::Must, text_query)];

        if !opts.types.is_empty() {
            let mut type_clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();
            for t in opts.types {
                let term = Term::from_field_text(self.schema.entity_type, t.as_str());
                let q: Box<dyn Query> = Box::new(TermQuery::new(term, IndexRecordOption::Basic));
                type_clauses.push((Occur::Should, q));
            }
            outer.push((Occur::Must, Box::new(BooleanQuery::new(type_clauses))));
        }

        // User-scoping: results are kept if either (entity is public) OR (entity has matching user_id).
        // Public entities have no user_id field. We approximate by:
        //   - if user_id provided: include docs that have NO user_id, OR have the matching user_id.
        // We implement this with: (NOT has_user_scoped_type) OR (user_id = me).
        // Simpler: post-filter in Rust based on stored user_id field by scanning.
        // For simplicity & correctness, post-filter after collection.
        // Over-fetch to absorb user-scoping post-filter losses.
        let take = (opts.limit + opts.offset).max(1) * 2;
        let bool_query = BooleanQuery::new(outer);
        let top_docs = searcher.search(&bool_query, &TopDocs::with_limit(take).order_by_score())?;

        let mut hits: Vec<SearchHit> = Vec::new();
        for (score, addr) in top_docs {
            let doc: TantivyDocument = searcher.doc(addr)?;
            use tantivy::schema::document::Value;

            let kind_str = doc
                .get_first(self.schema.entity_type)
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let kind = match SearchEntityType::parse(kind_str) {
                Some(k) => k,
                None => continue,
            };
            let id_full = doc
                .get_first(self.schema.id)
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let id_str = id_full.split(':').next_back().unwrap_or("");
            let id = match Uuid::parse_str(id_str) {
                Ok(u) => u,
                Err(_) => continue,
            };
            let title = doc
                .get_first(self.schema.title)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let subtitle = doc
                .get_first(self.schema.subtitle)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let doc_user_id = doc
                .get_first(self.schema.user_id)
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok());

            // User-scoping post-filter:
            // playlists/collections must match opts.user_id (if provided);
            // everything else passes.
            let user_scoped = matches!(
                kind,
                SearchEntityType::Playlist | SearchEntityType::Collection
            );
            if user_scoped {
                match (opts.user_id, doc_user_id) {
                    (Some(me), Some(other)) if me == other => {}
                    _ => continue,
                }
            }

            hits.push(SearchHit {
                id,
                kind,
                title,
                subtitle,
                score,
            });
        }

        // Apply offset & limit after user filtering.
        let out: Vec<SearchHit> = hits
            .into_iter()
            .skip(opts.offset)
            .take(opts.limit)
            .collect();
        Ok(out)
    }
}

fn fuzzy_distance(token: &str) -> u8 {
    let n = token.chars().count();
    if n <= 3 {
        0
    } else if n <= 6 {
        1
    } else {
        2
    }
}
