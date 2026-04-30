//! Lint that every struct/enum in apps/server/src/dto/ deriving Serialize or
//! Deserialize also has #[api_type(...)] or #[serde(rename_all = ...)] so the
//! wire format is explicit.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use syn::{Attribute, Item};
use walkdir::WalkDir;

fn main() -> ExitCode {
    let dto_dir = dto_dir();
    if !dto_dir.is_dir() {
        eprintln!("ERROR: {} not found", dto_dir.display());
        return ExitCode::FAILURE;
    }

    let mut failures: Vec<String> = Vec::new();

    for entry in WalkDir::new(&dto_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        match check_file(path) {
            Ok(mut f) => failures.append(&mut f),
            Err(e) => eprintln!("warning: failed to parse {}: {}", path.display(), e),
        }
    }

    if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        eprintln!("ERROR: DTO items missing explicit field-naming attribute:");
        for f in &failures {
            eprintln!("  {f}");
        }
        ExitCode::FAILURE
    }
}

fn dto_dir() -> PathBuf {
    // tools/dto-lint -> repo root -> apps/server/src/dto
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .unwrap_or(manifest)
        .join("apps/server/src/dto")
}

fn check_file(path: &Path) -> syn::Result<Vec<String>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
    let file = syn::parse_file(&content)?;
    let mut failures = Vec::new();
    for item in &file.items {
        if let Some(failure) = check_item(path, item) {
            failures.push(failure);
        }
    }
    Ok(failures)
}

fn check_item(path: &Path, item: &Item) -> Option<String> {
    let (name, attrs, span) = match item {
        Item::Struct(s) => (s.ident.to_string(), &s.attrs, s.ident.span()),
        Item::Enum(e) => (e.ident.to_string(), &e.attrs, e.ident.span()),
        _ => return None,
    };

    if !derives_serde(attrs) || has_rename_attr(attrs) {
        return None;
    }

    let line = span.start().line;
    Some(format!(
        "{}:{}: {} derives Serialize/Deserialize without #[api_type(...)] or #[serde(rename_all = ...)]",
        path.display(),
        line,
        name,
    ))
}

fn derives_serde(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("Serialize") || meta.path.is_ident("Deserialize") {
                found = true;
            }
            Ok(())
        });
        found
    })
}

fn has_rename_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("api_type") {
            return true;
        }
        if attr.path().is_ident("serde") {
            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename_all") {
                    found = true;
                }
                Ok(())
            });
            return found;
        }
        false
    })
}
