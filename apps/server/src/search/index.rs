use std::path::Path;
use std::sync::Mutex;

use anyhow::{Context, Result};
use tantivy::schema::{Field, Schema, FAST, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy, Term};
use uuid::Uuid;

use crate::dto::response::SearchEntityType;

/// Logical document fed into the index.
pub struct IndexDoc {
    pub kind: SearchEntityType,
    pub id: Uuid,
    /// Concatenated searchable text (title + alt titles + artist name where relevant).
    pub text: String,
    pub title: String,
    pub subtitle: Option<String>,
    /// Only set for user-scoped entities (playlists, collections).
    pub user_id: Option<Uuid>,
}

pub struct SearchSchema {
    pub schema: Schema,
    pub entity_type: Field,
    pub id: Field,
    pub text: Field,
    pub user_id: Field,
    pub title: Field,
    pub subtitle: Field,
}

impl SearchSchema {
    fn build() -> Self {
        let mut builder = Schema::builder();
        let entity_type = builder.add_text_field("entity_type", STRING | STORED | FAST);
        let id = builder.add_text_field("id", STRING | STORED);
        // TEXT field gets the default tokenizer (lowercase + simple split). Tantivy's
        // FuzzyTermQuery operates per-term over this tokenization.
        let text = builder.add_text_field("text", TEXT);
        let user_id = builder.add_text_field("user_id", STRING);
        let title = builder.add_text_field("title", STORED);
        let subtitle = builder.add_text_field("subtitle", STORED);
        SearchSchema {
            schema: builder.build(),
            entity_type,
            id,
            text,
            user_id,
            title,
            subtitle,
        }
    }
}

pub struct SearchIndex {
    // Kept alive to own the underlying mmap directory; reader/writer borrow from it.
    #[allow(dead_code)]
    pub(crate) index: Index,
    pub(crate) schema: SearchSchema,
    /// Tantivy permits a single concurrent IndexWriter per index.
    writer: Mutex<IndexWriter>,
    pub(crate) reader: IndexReader,
}

impl SearchIndex {
    pub fn open(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)
            .with_context(|| format!("creating search index dir at {}", path.display()))?;

        let schema = SearchSchema::build();
        let dir = tantivy::directory::MmapDirectory::open(path)
            .context("opening tantivy mmap directory")?;
        let index = Index::open_or_create(dir, schema.schema.clone())
            .context("opening or creating tantivy index")?;
        let writer: IndexWriter = index
            .writer(50_000_000)
            .context("creating tantivy index writer")?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .context("creating tantivy index reader")?;

        Ok(Self {
            index,
            schema,
            writer: Mutex::new(writer),
            reader,
        })
    }

    fn doc_uid(kind: SearchEntityType, id: Uuid) -> String {
        format!("{}:{}", kind.as_str(), id)
    }

    /// Upsert a single document. The (kind, id) pair is treated as the primary key.
    pub fn upsert(&self, d: &IndexDoc) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| anyhow::anyhow!("search index writer poisoned"))?;
        let uid = Self::doc_uid(d.kind, d.id);
        writer.delete_term(Term::from_field_text(self.schema.id, &uid));

        let mut tdoc: tantivy::TantivyDocument = doc!(
            self.schema.entity_type => d.kind.as_str(),
            self.schema.id => uid,
            self.schema.text => d.text.as_str(),
            self.schema.title => d.title.as_str(),
        );
        if let Some(sub) = &d.subtitle {
            tdoc.add_text(self.schema.subtitle, sub);
        }
        if let Some(uid_user) = d.user_id {
            tdoc.add_text(self.schema.user_id, uid_user.to_string());
        }

        writer.add_document(tdoc)?;
        writer.commit()?;
        Ok(())
    }

    /// Batch upsert without committing per doc. Caller must call `commit()`.
    pub fn upsert_no_commit(&self, d: &IndexDoc) -> Result<()> {
        let writer = self
            .writer
            .lock()
            .map_err(|_| anyhow::anyhow!("search index writer poisoned"))?;
        let uid = Self::doc_uid(d.kind, d.id);
        writer.delete_term(Term::from_field_text(self.schema.id, &uid));

        let mut tdoc: tantivy::TantivyDocument = doc!(
            self.schema.entity_type => d.kind.as_str(),
            self.schema.id => uid,
            self.schema.text => d.text.as_str(),
            self.schema.title => d.title.as_str(),
        );
        if let Some(sub) = &d.subtitle {
            tdoc.add_text(self.schema.subtitle, sub);
        }
        if let Some(uid_user) = d.user_id {
            tdoc.add_text(self.schema.user_id, uid_user.to_string());
        }
        writer.add_document(tdoc)?;
        Ok(())
    }

    pub fn delete(&self, kind: SearchEntityType, id: Uuid) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| anyhow::anyhow!("search index writer poisoned"))?;
        let uid = Self::doc_uid(kind, id);
        writer.delete_term(Term::from_field_text(self.schema.id, &uid));
        writer.commit()?;
        Ok(())
    }

    pub fn commit(&self) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| anyhow::anyhow!("search index writer poisoned"))?;
        writer.commit()?;
        Ok(())
    }

    /// Clear every document. Used by the startup rebuild.
    pub fn clear(&self) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| anyhow::anyhow!("search index writer poisoned"))?;
        writer.delete_all_documents()?;
        writer.commit()?;
        Ok(())
    }
}
