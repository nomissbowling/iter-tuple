#![doc(html_root_url = "https://docs.rs/iter-tuple/0.1.0")]
//! Rust iterator for tuple through proc-macro2 struct Vec AnyValue of polars DataFrame
//!
//! # Sample
//!
//! - [https://crates.io/crates/egui-dataframe-sample](https://crates.io/crates/egui-dataframe-sample)
//! - [https://github.com/nomissbowling/egui-dataframe-sample](https://github.com/nomissbowling/egui-dataframe-sample)
//!
//! # Requirements
//!
//! - [https://github.com/pola-rs/polars](https://github.com/pola-rs/polars)
//! - [polars](https://crates.io/crates/polars)
//! - [polars-utils](https://crates.io/crates/polars-utils)
//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as PM2TS;
use proc_macro2::{TokenTree, Ident, Literal}; // Group, Punct
use proc_macro2::{Span}; // Delimiter, Spacing
use quote::{quote, ToTokens}; // quote::ToTokens in proc_macro2
use syn; // syn::{parse_macro_input, ItemFn};
use std::ops::Deref;

#[proc_macro_attribute]
pub fn tuple_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
//  println!("{:?}", attr);
  let ts: PM2TS = attr.into();
  let mut n = 0usize;
  let mut cols = quote! {};
  for tt in ts { // not use .into_iter().enumerate() to count skip Punct ','
    match tt {
    TokenTree::Ident(dt) => { // match only Ident
//      println!("{}: {:?}", n, dt);
      let mut ts: PM2TS = PM2TS::new(); // proc_macro2::TokenStream
      Literal::usize_unsuffixed(n).to_tokens(&mut ts); // #n is usize_suffixed
      let i: TokenStream = ts.into();
      let ast_i = syn::parse_macro_input!(i as Literal);
      cols = quote! {
        #cols
        to_any!(t.#ast_i, DataType::#dt),
      };
      n += 1;
    },
    _ => {} // skip Punct ',' etc
    }
  }
  let ts_cols: TokenStream = quote! { let v = vec![#cols]; }.into();
  let ast_cols = syn::parse_macro_input!(ts_cols as syn::Stmt);
//  dbg!(ast_cols.clone());

  let ast = syn::parse_macro_input!(item as syn::ItemType);
//  dbg!(ast.clone());

  let ty = &ast.ty; // syn::Type::Tuple (syn::ItemType -> ty: Box<syn::Type>)
//  println!("{:?}", ty);
  let elem_len = match ty.deref() {
  syn::Type::Tuple(typetuple) => { // syn::TypeTuple != syn::Type::Tuple
//    println!("{:?}", typetuple);
    typetuple.elems.len()
  },
  _ => { panic!("tuple_derive requires type alias of tuple"); }
  };
//  println!("{}", elem_len);
  if elem_len != n { panic!("tuple_derive attributes not match with tuple"); }

  let tpl_id = &ast.ident;
//  println!("{:?}", tpl_id);

//  let ast_rec_id: syn::Ident = syn::parse_quote! { Rec#tpl_id }; // uk prefix
//  let rec_id: TokenStream = quote! { Rec#tpl_id }.into(); // unknown prefix
  let rec_id = &format!("Rec{}", tpl_id.to_string());

  let mut ts: PM2TS = PM2TS::new(); // proc_macro2::TokenStream
  Ident::new(rec_id, Span::call_site()).to_tokens(&mut ts);
  let rec_id: TokenStream = ts.into();
  let ast_rec_id = syn::parse_macro_input!(rec_id as syn::Ident);
//  dbg!(ast_rec_id.clone());

  quote! {
#ast
///
pub struct #ast_rec_id<'a> {
  ///
  pub v: Vec<AnyValue<'a>>
}
///
impl<'a> IntoIterator for #ast_rec_id<'a> {
  ///
  type Item = AnyValue<'a>;
  ///
  type IntoIter = std::vec::IntoIter<Self::Item>;
  //type IntoIter: Iterator<Item = Self::Item>;
  ///
  fn into_iter(self) -> Self::IntoIter {
    self.v.into_iter()
  }
}
///
impl<'a> From<#tpl_id<'a>> for #ast_rec_id<'a> {
  ///
  fn from(t: #tpl_id<'a>) -> #ast_rec_id<'a> {
    #ast_cols
    #ast_rec_id{v}
  }
}
///
impl<'a> #ast_rec_id<'a> {
  ///
  pub fn into_iter(t: #tpl_id<'a>) -> std::vec::IntoIter<AnyValue<'a>> {
    #ast_rec_id::from(t).into_iter()
  }
}
  }.into()
/*
  dbg!(ast.clone());
  ast.into_token_stream().into()
*/
}

/// tests
#[cfg(test)]
mod tests {
//  use super::*;

  /// [-- --nocapture] [-- --show-output]
  /// can't use a procedural macro from the same crate that defines it
  #[test]
  fn test_iter_tuple() {
    assert_eq!(true, true);
  }
}