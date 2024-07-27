iter-tuple
==========

Rust iterator for tuple through proc-macro2 struct Vec AnyValue of polars DataFrame


How to use
----------

```rust
use iter_tuple::tuple_derive;
use polars::prelude::{DataFrame, AnyValue, Schema}; // , Field, DataType
use egui_dataframe::{row_schema, named_schema, to_any};

/// auto defines struct RecTpl with tuple_derive
#[tuple_derive(UInt64, Utf8)]
pub type Tpl<'a> = (u64, &'a str);

pub fn main() {
  let rows = [
    (0, "a"),
    (1, "b"),
    (2, "c")
  ].into_iter().map(|r|
    row_schema(RecTpl::into_iter(r).collect())).collect::<Vec<_>>();

  // or can use row_schema(RecTpl::from(r).into_iter().collect())

  let schema = Schema::from(&rows[0]);
  let df = DataFrame::from_rows_iter_and_schema(rows.iter(), &schema);
  let mut df = df.expect("create DataFrame");
  let n = vec!["id", "string"];
  df.set_column_names(&n).expect("set column names");
  let sc = named_schema(&df, n);
  assert_eq!(df.schema(), sc); // OK all

  let df = df.select(["string", "id"]).expect("select columns");
  println!("{:?}", df.head(Some(100)));
}
```


Sample
------

- [https://crates.io/crates/egui-dataframe-sample](https://crates.io/crates/egui-dataframe-sample)
- [https://github.com/nomissbowling/egui-dataframe-sample](https://github.com/nomissbowling/egui-dataframe-sample)


Requirements
------------

- [https://github.com/pola-rs/polars](https://github.com/pola-rs/polars)
- [polars](https://crates.io/crates/polars)
- [polars-utils](https://crates.io/crates/polars-utils)

- [https://crates.io/crates/egui-dataframe](https://crates.io/crates/egui-dataframe)
- [https://github.com/nomissbowling/egui-dataframe](https://github.com/nomissbowling/egui-dataframe)


Links
-----

- [https://crates.io/crates/iter-tuple](https://crates.io/crates/iter-tuple)
- [https://github.com/nomissbowling/iter-tuple](https://github.com/nomissbowling/iter-tuple)


License
-------

MIT License
