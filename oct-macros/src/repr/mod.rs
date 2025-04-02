// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::iter;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
	Attribute,
	Ident,
	Path,
	PathSegment,
	Type,
	TypePath,
};
use syn::ext::IdentExt;

/// A derivable enumeration representation.
///
/// Any type can, *in theory*, be used as a discriminant.
/// This type, however, only includes primitives.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Repr {
	U8,
	I8,
	U16,
	I16,
	U32,
	I32,
	U64,
	I64,
	U128,
	I128,
	Usize,
	Isize,
}

impl Repr {
	#[inline]
	#[must_use]
	pub fn get(attrs: &[Attribute]) -> Option<Self> {
		let mut this = None;

		for attr in attrs {
			if attr.path().is_ident("repr") {
				attr.parse_nested_meta(|meta| {
					let ident = meta.path.require_ident()?;

					match &*ident.unraw().to_string() {
						"u8"    => this = Some(Self::U8),
						"i8"    => this = Some(Self::I8),
						"u16"   => this = Some(Self::U16),
						"i16"   => this = Some(Self::I16),
						"u32"   => this = Some(Self::U32),
						"i32"   => this = Some(Self::I32),
						"u64"   => this = Some(Self::U64),
						"i64"   => this = Some(Self::I64),
						"u128"  => this = Some(Self::U128),
						"i128"  => this = Some(Self::I128),
						"usize" => this = Some(Self::Usize),
						"isize" => this = Some(Self::Isize),

						_ => panic!("`{ident}` is not a derivable enumeration representation"),
					}

					Ok(())
				}).unwrap();
			}

			// Ignore all other attributes.
		}

		this
	}

	#[inline]
	#[must_use]
	pub const fn to_str(self) -> &'static str {
		match self {
			Self::U8    => "u8",
			Self::I8    => "i8",
			Self::U16   => "u16",
			Self::I16   => "i16",
			Self::U32   => "u32",
			Self::I32   => "i32",
			Self::U64   => "u64",
			Self::I64   => "i64",
			Self::U128  => "u128",
			Self::I128  => "i128",
			Self::Usize => "usize",
			Self::Isize => "isize",
		}
	}

	#[inline(always)]
	#[must_use]
	pub fn to_ident(self, span: Span) -> Ident {
		let ident = self.to_str();

		Ident::new(ident, span)
	}

	#[inline(always)]
	#[must_use]
	pub fn to_path(self, span: Span) -> Path {
		let ident = self.to_ident(span);

		Path {
			leading_colon: None,
			segments: iter::once(PathSegment {
				ident,
				arguments: Default::default(),
			}).collect(),
		}
	}

	#[inline]
	#[must_use]
	pub fn to_type(self, span: Span) -> Type {
		Type::Path(TypePath {
			qself: None,
			path:  self.to_path(span),
		})
	}
}

impl Default for Repr {
	#[inline(always)]
	fn default() -> Self {
		Self::Isize
	}
}

impl ToTokens for Repr {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.to_ident(Span::call_site()).to_tokens(tokens);
	}
}
