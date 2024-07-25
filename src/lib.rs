#![doc(html_root_url = "https://docs.rs/iter-tuple/0.1.0")]
//! Rust iterator for tuple through proc-macro2 struct Vec AnyValue of polars DataFrame
//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as PM2TS;
use proc_macro2::{TokenTree, Group, Ident, Literal, Punct};
use proc_macro2::{Delimiter, Span, Spacing};
use quote::{quote, ToTokens}; // quote::ToTokens in proc_macro2
use syn; // syn::{parse_macro_input, ItemFn};

use polars::prelude::{DataFrame, AnyValue, Schema, Field, DataType};

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_iter_tuple() {
    assert_eq!(true, true);
  }
}