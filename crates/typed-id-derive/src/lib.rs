use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for creating type-safe ID newtypes.
///
/// Adds methods and trait implementations to an existing newtype struct
/// wrapping `uuid::Uuid`. The struct must be defined as a tuple struct
/// with a single `uuid::Uuid` field. Standard derives (`Clone`, `Copy`,
/// `Debug`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`) and serde
/// derives must be applied manually.
///
/// # Generated items
///
/// - `new(id: Uuid) -> Self` constructor
/// - `as_uuid() -> Uuid` accessor
/// - `parse(s: &str) -> Option<Self>` parser
/// - `nil() -> Self` zero ID
/// - `is_nil() -> bool` nil check
/// - `Display`, `From<Uuid>`, `Into<Uuid>` implementations
///
/// # Example
/// ```ignore
/// use typedids::TypedId;
///
/// #[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
/// #[serde(transparent)]
/// pub struct UserId(#[serde(with = "uuid::serde::hyphenated")] uuid::Uuid);
///
/// let id = UserId::new(uuid::Uuid::now_v7());
/// assert_eq!(id.as_uuid(), id.into());
/// ```
#[proc_macro_derive(TypedId, attributes(id))]
pub fn derive_typed_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // `#[id(new = false)]` / `#[id(as_uuid = false)]`: hosts whose macro
    // already emits a random-ID `new()` (v4/v7 policy lives there) or a
    // by-reference `as_uuid` suppress the generated versions and keep
    // their own. Everything else is always generated.
    let mut gen_new = true;
    let mut gen_as_uuid = true;
    for attr in &input.attrs {
        if !attr.path().is_ident("id") {
            continue;
        }
        let ok = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        );
        if let Ok(nested) = ok {
            for meta in nested {
                if let syn::Meta::NameValue(nv) = &meta {
                    let key = nv.path.get_ident().map(|i| i.to_string());
                    let value = match &nv.value {
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Bool(b),
                            ..
                        }) => b.value,
                        _ => true,
                    };
                    match key.as_deref() {
                        Some("new") => gen_new = value,
                        Some("as_uuid") => gen_as_uuid = value,
                        _ => {}
                    }
                }
            }
        }
    }

    let new_impl = if gen_new {
        quote! {
            /// Create from a UUID.
            pub const fn new(id: ::uuid::Uuid) -> Self {
                Self(id)
            }
        }
    } else {
        quote! {}
    };
    let as_uuid_impl = if gen_as_uuid {
        quote! {
            /// Get the inner UUID.
            pub fn as_uuid(&self) -> ::uuid::Uuid {
                self.0
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl #name {
            #new_impl
            #as_uuid_impl

            /// Parse from a string.
            pub fn parse(s: &str) -> Option<Self> {
                s.parse::<::uuid::Uuid>().ok().map(Self)
            }

            /// Nil (zero) ID.
            pub const fn nil() -> Self {
                Self(::uuid::Uuid::nil())
            }

            /// Check if this is a nil (zero) ID.
            pub fn is_nil(&self) -> bool {
                self.0.is_nil()
            }
        }

        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.0.hyphenated())
            }
        }

        impl From<::uuid::Uuid> for #name {
            fn from(id: ::uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl From<#name> for ::uuid::Uuid {
            fn from(id: #name) -> Self {
                id.0
            }
        }
    };

    TokenStream::from(expanded)
}
