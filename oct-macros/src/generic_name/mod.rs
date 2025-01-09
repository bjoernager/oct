// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fmt;
use std::fmt::{Debug, Formatter};
use syn::{
	GenericParam,
	Generics,
	Ident,
	Lifetime,
	Token,
	punctuated::Punctuated,
};

/// A name of a genric.
#[derive(Clone)]
pub enum GenericName {
	/// Denotes a generic constant.
	Const(Ident),

	/// Denotes a generic lifetime.
	Lifetime(Lifetime),

	/// Denotes a generic type.
	Ty(Ident),
}

impl GenericName {
	/// Extracts the names of the given generics.
	#[must_use]
	pub fn extract_from(generics: &Generics) -> Punctuated<Self, Token![,]> {
		let mut names = Punctuated::new();

		for generic in &generics.params {
			let name = match *generic {
				GenericParam::Const(   ref param) => Self::Const(   param.ident.clone()),
				GenericParam::Lifetime(ref param) => Self::Lifetime(param.lifetime.clone()),
				GenericParam::Type(    ref param) => Self::Ty(    param.ident.clone()),
			};

			names.push(name);
		}

		names
	}
}

impl Debug for GenericName {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let ident = match *self {
			| Self::Const(ref ident)
			| Self::Lifetime(Lifetime { ref ident, .. })
			| Self::Ty(ref ident)
			=> ident,
		};

		Debug::fmt(ident, f)
	}
}

impl ToTokens for GenericName {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match *self {
			| Self::Const(ref ident)
			| Self::Ty( ref ident)
			=> ident.to_tokens(tokens),

			Self::Lifetime(ref lifetime) => lifetime.to_tokens(tokens),
		}
	}
}
