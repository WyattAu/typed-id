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
/// use typed_id::TypedId;
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

    let expanded = quote! {
        impl #name {
            /// Create from a UUID.
            pub const fn new(id: ::uuid::Uuid) -> Self {
                Self(id)
            }

            /// Get the inner UUID.
            pub fn as_uuid(&self) -> ::uuid::Uuid {
                self.0
            }

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

        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
