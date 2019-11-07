use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::derive2::{ctx::Context, derive, r#impl::Impl};

use super::DeriveKind;

pub struct Enum;

impl Enum {
    pub fn parse(
        _derive: &DeriveKind,
        context: &Context,
        _impl: &Impl<'_>,
        data: &mut syn::DataEnum,
    ) -> derive::Result<Self> {
        /* let variant = if let Some(variant) = variant.into() {
            match Variant::parse(context, variant) {
                Ok(variant) => Some(variant),
                Err(error) => return Err(error),
            }
        } else {
            None
        }; */

        context.error(data.enum_token, "cannot derive for enums yet");
        Err(())
    }
}

impl ToTokens for Enum {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}
