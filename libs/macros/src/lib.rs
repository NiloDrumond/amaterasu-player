use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Item, LitStr};

fn to_kebab_case(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

/// Attribute macro that prepares a struct/enum for API serialization and TypeScript binding generation.
///
/// Adds:
/// - `#[derive(ts_rs::TS)]`
/// - `#[serde(rename_all = "camelCase")]`
/// - `#[ts(export)]`
/// - `#[ts(export_to = "<subdir>/<kebab-case-name>.ts")]`
///
/// # Usage
/// ```ignore
/// #[api_type("response/auth")]
/// #[derive(Debug, Serialize)]
/// pub struct CurrentUserResponse { .. }
/// // Generates: #[ts(export_to = "response/auth/current-user-response.ts")]
/// ```
#[proc_macro_attribute]
pub fn api_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let subdir = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr as LitStr).value())
    };

    let mut input = parse_macro_input!(item as Item);

    let (ident, attrs) = match &mut input {
        Item::Struct(s) => (&s.ident, &mut s.attrs),
        Item::Enum(e) => (&e.ident, &mut e.attrs),
        _ => {
            return syn::Error::new_spanned(
                quote! { #input },
                "api_type can only be applied to structs and enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let kebab_name = to_kebab_case(&ident.to_string());
    let export_to = match &subdir {
        Some(dir) => format!("{}/{}.ts", dir.trim_end_matches('/'), kebab_name),
        None => format!("{}.ts", kebab_name),
    };

    attrs.push(parse_quote!(#[derive(ts_rs::TS)]));
    attrs.push(parse_quote!(#[serde(rename_all = "camelCase")]));
    attrs.push(parse_quote!(#[ts(export)]));
    attrs.push(parse_quote!(#[ts(export_to = #export_to)]));

    quote! { #input }.into()
}
