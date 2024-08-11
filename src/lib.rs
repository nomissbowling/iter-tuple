#![doc(html_root_url = "https://docs.rs/iter-tuple/0.2.5")]
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
//! # Optional
//! - [https://crates.io/crates/sqlite](https://crates.io/crates/sqlite)
//! - [https://crates.io/crates/polars-sqlite](https://crates.io/crates/polars-sqlite)
//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as PM2TS;
use proc_macro2::{TokenTree, Ident, Literal}; // Group, Punct
use proc_macro2::{Span}; // Delimiter, Spacing
use quote::{quote, ToTokens}; // quote::ToTokens in proc_macro2
use syn; // syn::{parse_macro_input, ItemFn};
use std::ops::Deref;

/// concat ident to pre ast (before parse as syn::Ident)
/// - a: true: as is, false: to lowercase
fn pre_ast_ident(pre: &str, id: &Ident, post: &str, a: bool) -> TokenStream {
//  let ast_id: syn::Ident = syn::parse_quote! { XXX#id }; // unknown prefix
//  let id: TokenStream = quote! { XXX#id }.into(); // unknown prefix
  let mut s = id.to_string();
  if !a { s = s.to_lowercase(); }
  let str_id = &format!("{}{}{}", pre, s, post);
  let mut ts: PM2TS = PM2TS::new(); // proc_macro2::TokenStream
  Ident::new(str_id, Span::call_site()).to_tokens(&mut ts);
  ts.into()
}

/// concat string literal to pre ast (before prepare as Literal)
/// - a: true: as is, false: to lowercase
fn pre_ast_literal(pre: &str, id: &Ident, post: &str, a: bool) -> TokenStream {
  let mut s = id.to_string();
  if !a { s = s.to_lowercase(); }
  let str_id = &format!("{}{}{}", pre, s, post);
  let mut ts: PM2TS = PM2TS::new(); // proc_macro2::TokenStream
  Literal::string(str_id).to_tokens(&mut ts);
  ts.into()
}

/// usize to pre ast (before parse as Literal)
fn pre_ast_usize(n: usize) -> TokenStream {
  let mut ts: PM2TS = PM2TS::new(); // proc_macro2::TokenStream
  Literal::usize_unsuffixed(n).to_tokens(&mut ts); // #n is usize_suffixed
  ts.into()
}

/// from polars DataType to primitive type (proc_macro2::TokenStream)
fn ast_dtype(dt: &Ident) -> PM2TS {
  match dt.to_string().as_str() {
  "Int64" => quote! { i64 },
  "Int32" => quote! { i32 },
  "Int16" => quote! { i16 },
  "Int8" => quote! { i8 },
  "UInt64" => quote! { u64 },
  "UInt32" => quote! { u32 },
  "UInt16" => quote! { u16 },
  "UInt8" => quote! { u8 },
  "Float64" => quote! { f64 }, // Decimal in polars latest
  "Float32" => quote! { f32 }, // Decimal in polars latest
  "Utf8" => quote! { &'a str }, // polars version 0.25.1
  "String" => quote! { &'a str }, // polars latest
  "Boolean" => quote! { bool },
  "Binary" => quote! { Vec<u8> },
  "Null" => quote! { i64 }, // must check later
  "Unknown" => quote! { i64 }, // must check later
  _ => quote! { i64 } // must check later
  }
}

/// from polars DataType to sqlite3 type WR (proc_macro2::TokenStream)
fn ast_dtype_sqlite3_vec(dt: &Ident, ast_id: &Ident) -> PM2TS {
  match dt.to_string().as_str() {
  "Int64" => quote! { self.#ast_id },
  "Int32" => quote! { (self.#ast_id as i64) },
  "Int16" => quote! { (self.#ast_id as i64) },
  "Int8" => quote! { (self.#ast_id as i64) },
  "UInt64" => quote! { (self.#ast_id as i64) },
  "UInt32" => quote! { (self.#ast_id as i64) },
  "UInt16" => quote! { (self.#ast_id as i64) },
  "UInt8" => quote! { (self.#ast_id as i64) },
  "Float64" => quote! { self.#ast_id }, // Decimal in polars latest
  "Float32" => quote! { (self.#ast_id as f64) }, // Decimal in polars latest
  "Utf8" => quote! { self.#ast_id }, // polars version 0.25.1
  "String" => quote! { self.#ast_id }, // polars latest
  "Boolean" => quote! { (if self.#ast_id {"T"} else {"F"}) },
  "Binary" => quote! { (&self.#ast_id[..]) },
  "Null" => quote! { self.#ast_id }, // must check later
  "Unknown" => quote! { self.#ast_id }, // must check later
  _ => quote! { self.#ast_id } // must check later
  }
}

/// from polars DataType to sqlite3 type RD (tuple of proc_macro2::TokenStream)
fn ast_dtype_sqlite3_col(dt: &Ident) -> (PM2TS, PM2TS) {
  match dt.to_string().as_str() {
  "Int64" => (quote! { i64 }, quote! {}),
  "Int32" => (quote! { i64 }, quote! { as i32 }),
  "Int16" => (quote! { i64 }, quote! { as i16 }),
  "Int8" => (quote! { i64 }, quote! { as i8 }),
  "UInt64" => (quote! { i64 }, quote! { as u64 }),
  "UInt32" => (quote! { i64 }, quote! { as u32 }),
  "UInt16" => (quote! { i64 }, quote! { as u16 }),
  "UInt8" => (quote! { i64 }, quote! { as u8 }),
  "Float64" => (quote! { f64 }, quote! {}), // Decimal in polars latest
  "Float32" => (quote! { f64 }, quote! { as f32 }), // Decimal in polars latest
  "Utf8" => (quote! { &'a str }, quote! {}), // polars version 0.25.1
  "String" => (quote! { &'a str }, quote! {}), // polars latest
  "Boolean" => (quote! { &'a str }, quote! { == "T" }), // not impl. trait From
  "Binary" => (quote! { &[u8] }, quote! { .to_vec() }), // not impl. trait From
  "Null" => (quote! { i64 }, quote! {}), // must check later
  "Unknown" => (quote! { i64 }, quote! {}), // must check later
  _ => (quote! { i64 }, quote! {}) // must check later
  }
}

/// from attr to tuple of sqlite3 cols
fn sqlite3_cols(attr: PM2TS, n: &mut usize) -> TokenStream {
  let mut cols = quote! {};
  for tt in attr { // not use .into_iter().enumerate() to count skip Punct ','
    match tt {
    TokenTree::Ident(dt) => { // match only Ident
//      println!("{}: {:?}", n, dt);
      let i = pre_ast_usize(*n); // outside of macro call
      let ast_i = syn::parse_macro_input!(i as Literal);
      let (t, p) = ast_dtype_sqlite3_col(&dt);
      cols = quote! {
        #cols
        row.read::<#t, _>(#ast_i) #p,
      };
      *n += 1;
    },
    _ => {} // skip Punct ',' etc
    }
  }
  quote! { (#cols) }.into()
}

/// from attr to vec of cols type
fn type_cols(attr: PM2TS) -> TokenStream {
  let mut cols = quote! {};
  for tt in attr { // not use .into_iter().enumerate() to count skip Punct ','
    match tt {
    TokenTree::Ident(dt) => { // match only Ident
//      println!("{:?}", dt);
      cols = quote! {
        #cols
        DataType::#dt,
      };
    },
    _ => {} // skip Punct ',' etc
    }
  }
  quote! { vec![#cols] }.into()
}

/// from attr to vec of cols
fn vec_cols(attr: PM2TS, n: &mut usize) -> TokenStream {
  let mut cols = quote! {};
  for tt in attr { // not use .into_iter().enumerate() to count skip Punct ','
    match tt {
    TokenTree::Ident(dt) => { // match only Ident
//      println!("{}: {:?}", n, dt);
      let i = pre_ast_usize(*n); // outside of macro call
      let ast_i = syn::parse_macro_input!(i as Literal);
      let v = match dt.to_string().as_str() {
      // vec_cols through "Boolean" => ...
      // "Binary" => quote! { &t.#ast_i }, // use below (can't use .to_owned())
      "Binary" => quote! { to_any!(t.#ast_i, DataType::BinaryOwned) },
      // _ => quote! { t.#ast_i } // skip (use below) for support BinaryOwned
      _ => quote! { to_any!(t.#ast_i, DataType::#dt) }
      };
      cols = quote! {
        #cols
        // #v.into(), // it is not whole implemented in some version of polars
        // AnyValue::#dt(#v), // same as below
        // to_any!(#v, DataType::#dt), // skip (use below) for Binary Owned
        #v,
      };
      *n += 1;
    },
    _ => {} // skip Punct ',' etc
    }
  }
  quote! { let v = vec![#cols]; }.into()
}

/// from attr to sqlite3 vec of member tuple
fn sqlite3_vec(mns: &Vec<Ident>, dts: &Vec<Ident>) -> TokenStream {
  let mut members = quote! {};
  for (i, n) in mns.iter().enumerate() {
    let tag = pre_ast_literal(":", n, "", true);
    let ast_tag = syn::parse_macro_input!(tag as Literal); // be TokenStream
    let id = pre_ast_ident("", n, "", true);
    let ast_id = syn::parse_macro_input!(id as syn::Ident); // be TokenStream
    let v = ast_dtype_sqlite3_vec(&dts[i], &ast_id);
    members = quote! {
      #members
      (stringify!(#ast_tag), #v.into()),
    }
  }
  quote! { vec![#members] }.into() // be TokenStream
}

/// from attr to from_tuple of member
fn from_tuple_members(mns: &Vec<Ident>) -> TokenStream {
  let mut members = quote! {};
  for (i, n) in mns.iter().enumerate() {
    let id = pre_ast_ident("", n, "", true);
    let ast_id = syn::parse_macro_input!(id as syn::Ident); // be TokenStream
    let u = pre_ast_usize(i); // outside of macro call
    let ast_i = syn::parse_macro_input!(u as Literal);
    members = quote! {
      #members
      #ast_id: t.#ast_i,
    };
  }
  members.into() // be TokenStream
}

/// from attr to to_tuple of member
fn to_tuple_members(mns: &Vec<Ident>, dts: &Vec<Ident>) -> TokenStream {
  let mut members = quote! {};
  for (i, n) in mns.iter().enumerate() {
    let id = pre_ast_ident("", n, "", true);
    let ast_id = syn::parse_macro_input!(id as syn::Ident); // be TokenStream
    let v = match dts[i].to_string().as_str() {
    // to_tuple_members through "Boolean" => ...
    "Binary" => quote! { self.#ast_id.clone() },
    _ => quote! { self.#ast_id }
    };
    members = quote! {
      #members
      #v,
    };
  }
  quote! { (#members) }.into() // be TokenStream
}

/// from attr to list of DataType
fn type_members(dts: &Vec<Ident>) -> TokenStream {
  let mut members = quote! {};
  for dt in dts.iter() {
    members = quote! {
      #members
      DataType::#dt,
    };
  }
  quote! { vec![#members] }.into() // be TokenStream
}

/// from attr to list of member stringify
fn str_members(mns: &Vec<Ident>) -> TokenStream {
  let mut members = quote! {};
  for n in mns.iter() {
    let id = pre_ast_ident("", n, "", true);
    let ast_id = syn::parse_macro_input!(id as syn::Ident); // be TokenStream
    members = quote! {
      #members
      stringify!(#ast_id),
    };
  }
  quote! { vec![#members] }.into() // be TokenStream
}

/// from attr to list of member and DataType
fn list_members(mns: &Vec<Ident>, dts: &Vec<Ident>) -> TokenStream {
  let mut members = quote! {};
  for (i, n) in mns.iter().enumerate() {
    let id = pre_ast_ident("", n, "", true);
    let ast_id = syn::parse_macro_input!(id as syn::Ident); // be TokenStream
    let dt = ast_dtype(&dts[i]);
    members = quote! {
      #members
      ///
      pub #ast_id: #dt,
    };
  }
  members.into() // be TokenStream
}

/// from attr to tuple of Vec member name and Vec DataType
fn parse_attr(attr: PM2TS) -> (Vec<Ident>, Vec<Ident>) {
  let (mut mns, mut dts) = (Vec::<Ident>::new(), Vec::<Ident>::new());
  let mut i = 0usize;
  for tt in attr { // not use .into_iter().enumerate() to count skip Punct ','
    match tt {
    TokenTree::Group(gp) => { // match only Group
      for t in gp.stream() {
        match t {
        TokenTree::Ident(dt) => { // match only Ident
//          println!("{}: {:?}", n, dt);
          if i == 0 { mns.push(dt); } else { dts.push(dt); }
        },
        _ => {} // skip Punct ',' etc
        }
      }
      i += 1;
    },
    _ => {} // skip Punct ',' etc
    }
  }
  (mns, dts)
}

/// check type is tuple and elem length (pipeline for TokenStream)
fn tuple_check(item: TokenStream, n: usize, f: &str) -> TokenStream {
  let ast = syn::parse_macro_input!(item as syn::ItemType);
//  dbg!(ast.clone());
  let ty = &ast.ty; // syn::Type::Tuple (syn::ItemType -> ty: Box<syn::Type>)
//  println!("{:?}", ty);
  let elem_len = match ty.deref() {
  syn::Type::Tuple(typetuple) => { // syn::TypeTuple != syn::Type::Tuple
//    println!("{:?}", typetuple);
    typetuple.elems.len()
  },
  _ => { panic!("{} requires type alias of tuple", f); }
  };
//  println!("{}", elem_len);
  if elem_len != n { panic!("{} attributes not match with tuple", f); }
  ast.into_token_stream().into()
}

/// struct_derive
/// - (optional)
#[proc_macro_attribute]
pub fn struct_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
//  println!("{:?}", attr);
  let (mns, dts) = parse_attr(attr.into());
  let (m, n) = (mns.len(), dts.len());
  if m != n { panic!("struct_derive attributes not same length"); }
  let ast_type_members: PM2TS = type_members(&dts).into();
//  dbg!(ast_type_members.clone());
  let ast_str_members: PM2TS = str_members(&mns).into();
//  dbg!(ast_str_members.clone());
  let ast_list_members: PM2TS = list_members(&mns, &dts).into();
//  dbg!(ast_list_members.clone());
  let ast_to_tuple_members: PM2TS = to_tuple_members(&mns, &dts).into();
//  dbg!(ast_to_tuple_members.clone());
  let ast_from_tuple_members: PM2TS = from_tuple_members(&mns).into();
//  dbg!(ast_from_tuple_members.clone());
  let ast_sqlite3_vec: PM2TS = sqlite3_vec(&mns, &dts).into();
//  dbg!(ast_sqlite3_vec.clone());

  let tp = tuple_check(item, n, "struct_derive");
  let ast = syn::parse_macro_input!(tp as syn::ItemType);

  let tpl_id = &ast.ident;
//  println!("{:?}", tpl_id);
  let st_id = pre_ast_ident("St", tpl_id, "", true); // outside of macro call
  let ast_st_id = syn::parse_macro_input!(st_id as syn::Ident);
//  dbg!(ast_st_id.clone());
  let fnc_id = pre_ast_ident("to_", tpl_id, "", false); // to lowercase
  let ast_fnc_id = syn::parse_macro_input!(fnc_id as syn::Ident);
//  dbg!(ast_fnc_id.clone());
  let rec_id = pre_ast_ident("Rec", tpl_id, "", true); // outside of macro call
  let ast_rec_id = syn::parse_macro_input!(rec_id as syn::Ident);
//  dbg!(ast_rec_id.clone());

  quote! {
#ast
///
pub struct #ast_st_id<'a> {
  #ast_list_members
}
///
impl<'a> #ast_st_id<'a> {
  ///
  pub fn members() -> Vec<&'a str> {
    #ast_str_members
  }
  ///
  pub fn types() -> Vec<DataType> {
    #ast_type_members
  }
  ///
  pub fn #ast_fnc_id(&self) -> #tpl_id<'_> {
    #ast_to_tuple_members
  }
}
///
impl<'a> IntoAnyValueVec<'a> for #ast_st_id<'a> {
  ///
  fn into_vec(self) -> Vec<AnyValue<'a>> {
//    #ast_rec_id::from(self.#ast_fnc_id()).v // can't reference to data owned
    #ast_rec_id::from(#ast_to_tuple_members).v
  }
}
///
impl<'a> ToSqlite3ValueVec for #ast_st_id<'a> {
  ///
  fn to_sqlite3_vec(&self) -> Vec<(&'_ str, sqlite::Value)> {
    #ast_sqlite3_vec
  }
}
///
impl<'a> From<#tpl_id<'a>> for #ast_st_id<'a> {
  ///
  fn from(t: #tpl_id<'a>) -> #ast_st_id<'_> {
    #ast_st_id{#ast_from_tuple_members}
  }
}
///
impl<'a> From<&'a sqlite::Row> for #ast_st_id<'a> {
  ///
  fn from(row: &'a sqlite::Row) -> #ast_st_id<'_> {
    #ast_st_id::from(#ast_fnc_id(row))
  }
}
  }.into()
/*
  dbg!(ast.clone());
  ast.into_token_stream().into()
*/
}

/// tuple_sqlite3
/// - (optional) see crate sqlite https://crates.io/crates/sqlite
#[proc_macro_attribute]
pub fn tuple_sqlite3(attr: TokenStream, item: TokenStream) -> TokenStream {
//  println!("{:?}", attr);
  let mut n = 0usize;
  let ts_cols = sqlite3_cols(attr.into(), &mut n); // outside of macro call
  let ast_cols = syn::parse_macro_input!(ts_cols as syn::Expr);
//  dbg!(ast_cols.clone());

  let tp = tuple_check(item, n, "tuple_sqlite3");
  let ast = syn::parse_macro_input!(tp as syn::ItemType);

  let tpl_id = &ast.ident;
//  println!("{:?}", tpl_id);
  let rec_id = pre_ast_ident("Rec", tpl_id, "", true); // outside of macro call
  let ast_rec_id = syn::parse_macro_input!(rec_id as syn::Ident);
//  dbg!(ast_rec_id.clone());
  let fnc_id = pre_ast_ident("to_", tpl_id, "", false); // to lowercase
  let ast_fnc_id = syn::parse_macro_input!(fnc_id as syn::Ident);
//  dbg!(ast_fnc_id.clone());

  quote! {
#ast
///
pub fn #ast_fnc_id<'a>(row: &'a sqlite::Row) -> #tpl_id<'_> {
  #ast_cols
}
///
impl<'a> From<&'a sqlite::Row> for #ast_rec_id<'a> {
  ///
  fn from(row: &'a sqlite::Row) -> #ast_rec_id<'_> {
    #ast_rec_id::from(#ast_fnc_id(row))
  }
}
  }.into()
/*
  dbg!(ast.clone());
  ast.into_token_stream().into()
*/
}

/// tuple_derive
/// - Utf8, UInt64, Int64, UInt32, Int32, Float64, Float32, Boolean, Binary, ...
/// - see Enum polars::datatypes::DataType
#[proc_macro_attribute]
pub fn tuple_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
//  println!("{:?}", attr);
  let type_cols = type_cols(attr.clone().into()); // outside of macro call
  let ast_type_members = syn::parse_macro_input!(type_cols as syn::Expr);
//  dbg!(ast_type_members.clone());

  let mut n = 0usize;
  let ts_cols = vec_cols(attr.into(), &mut n); // outside of macro call
  let ast_cols = syn::parse_macro_input!(ts_cols as syn::Stmt);
//  dbg!(ast_cols.clone());

  let tp = tuple_check(item, n, "tuple_derive");
  let ast = syn::parse_macro_input!(tp as syn::ItemType);

  let tpl_id = &ast.ident;
//  println!("{:?}", tpl_id);
  let rec_id = pre_ast_ident("Rec", tpl_id, "", true); // outside of macro call
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
  fn from(t: #tpl_id<'a>) -> #ast_rec_id<'_> {
    #ast_cols
    #ast_rec_id{v}
  }
}
///
impl<'a> #ast_rec_id<'a> {
  ///
  pub fn types() -> Vec<DataType> {
    #ast_type_members
  }
  ///
  pub fn into_iter(t: #tpl_id<'a>) -> std::vec::IntoIter<AnyValue<'_>> {
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