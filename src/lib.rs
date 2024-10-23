//! In domain-driven design, entity types should be compared by identity rather than value.
//!
//! [dervy] allows you to annotate your domain entities in order to derive
//! implementations of [PartialEq], [Eq], and [Hash] that only consider identity for equality.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident};

/// Derive [PartialEq], [Eq], and [Hash] for this entity type by considering
/// the field annotated with `#[dervy(id)]`
///
/// Example:
///
/// ```
/// #[derive(Clone, Debug, dervy::Entity)]
/// struct MyEntity {
///     #[dervy(id)]
///     my_entity_id: i32,
///     other_field: bool,
///     // ...
/// }
///
/// // structs are equal by value
/// let ent1 = MyEntity { my_entity_id: 0, other_field: false };
/// let mut ent2 = ent1.clone();
/// assert_eq!(ent1, ent2);
///
/// // structs are still equal by identity
/// ent2.other_field = true;
/// assert_eq!(ent1, ent2);
///
/// // structs are no longer equal by identity
/// ent2.my_entity_id += 1;
/// assert_ne!(ent1, ent2);
/// ent2.my_entity_id -= 1;
///
/// // structs are hashed by identity
/// let mut map = std::collections::HashMap::<MyEntity, bool>::new();
/// map.insert(ent1, true);
/// assert!(map.contains_key(&ent2));
/// ```
#[proc_macro_derive(Entity, attributes(dervy))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let id_field = find_id_field(&input.data);

    let expanded = quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.#id_field == other.#id_field
            }
        }

        impl Eq for #name {}

        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.#id_field.hash(state);
            }
        }
    };

    TokenStream::from(expanded)
}

fn find_id_field(data: &Data) -> Ident {
    if let Data::Struct(data_struct) = data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                if has_dervy_id_attribute(&field.attrs) {
                    return field.ident.as_ref().unwrap().clone();
                }
            }
        }
    }
    panic!("No field with #[dervy(id)] attribute found");
}

fn has_dervy_id_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("dervy")
            && attr
                .parse_args::<Ident>()
                .into_iter()
                .any(|ident| ident == "id")
    })
}
