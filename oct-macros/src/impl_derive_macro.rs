// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{GenericName, Repr};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	Data,
	DataEnum,
	DataStruct,
	DeriveInput,
	Path,
	Token,
};

pub fn impl_derive_macro<S, E>(
	input:          DeriveInput,
	trait_path:     Path,
	r#unsafe_token: Option<Token![unsafe]>,
	struct_body:    S,
	enum_body:      E,
) -> TokenStream
where
	S: FnOnce(DataStruct)     -> TokenStream,
	E: FnOnce(DataEnum, Repr) -> TokenStream,
{
	let trait_name = &trait_path
		.segments
		.last()
		.expect("expected non-empty path for derived trait")
		.ident;

	let self_name = &input.ident;

	let body = match input.data {
		Data::Struct(data) => struct_body(data),

		Data::Enum(data) => {
			let repr = Repr::get(&input.attrs).unwrap_or_default();

			enum_body(data, repr)
		}

		Data::Union(..) => panic!("unions cannot derive `{trait_name:?}`"),
	};

	let generic_params = &input.generics.params;
	let generic_where  = &input.generics.where_clause;

	let generic_names = GenericName::extract_from(&input.generics);

	let output = quote! {
		#unsafe_token impl<#generic_params> #trait_path for #self_name<#generic_names>
		#generic_where
		{
			#body
		}
	};

	output
}