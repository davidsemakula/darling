use syn;

use {FromMetaItem, Result};

mod container;
mod forward_attrs;
mod from_derive;
mod from_field;
mod from_variant;
mod meta_item_field;
mod outer_from;
mod variant;

pub use self::container::Core;
pub use self::meta_item_field::MetaItemField;
pub use self::forward_attrs::ForwardAttrs;
pub use self::from_derive::FdiOptions;
pub use self::from_field::FromFieldOptions;
pub use self::from_variant::FromVariantOptions;
pub use self::outer_from::OuterFrom;
pub use self::variant::Variant;

/// A default/fallback expression encountered in attributes during parsing.
#[derive(Debug, Clone)]
pub enum DefaultExpression {
    /// The value should be taken from the `default` instance of the containing struct.
    /// This is not valid in container options.
    Inherit,
    Explicit(syn::Path),
    Trait,
}

#[doc(hidden)]
impl FromMetaItem for DefaultExpression {
    fn from_word() -> Result<Self> {
        Ok(DefaultExpression::Trait)
    }

    fn from_string(lit: &str) -> Result<Self> {
        Ok(DefaultExpression::Explicit(syn::parse_path(lit).unwrap()))
    }
}

/// Middleware for extracting attribute values.
pub trait ParseAttribute: Sized {
    fn parse_attributes(mut self, attrs: &[syn::Attribute]) -> Result<Self> {
        for attr in attrs {
            if attr.name() == "darling" {
                parse_attr(attr, &mut self)?;
            }
        }

        Ok(self)
    }

    /// Read a meta-item, and apply its values to the current instance.
    fn parse_nested(&mut self, mi: &syn::MetaItem) -> Result<()>;
}

fn parse_attr<T: ParseAttribute>(attr: &syn::Attribute, target: &mut T) -> Result<()> {
    if attr.is_sugared_doc {
        return Ok(())
    }

    match attr.value {
        syn::MetaItem::List(_, ref items) => {
            for item in items {
                if let syn::NestedMetaItem::MetaItem(ref mi) = *item {
                    target.parse_nested(mi)?;
                } else {
                    unimplemented!();
                }
            }

            Ok(())
        },
        _ => unimplemented!()
    }
}