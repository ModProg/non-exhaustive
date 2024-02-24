Macro to create non_exhaustive structs and structs with private fields with the [functional update syntax](https://doc.rust-lang.org/reference/expressions/struct-expr.html#functional-update-syntax), i.e., using `..Default::default()`.

Given the foreign structs:
```rust
#[non_exhaustive]
#[derive(Default)]
pub struct NonExhaustive {
  pub field: usize
}

#[derive(Default)]
pub struct PrivateFields {
  pub pub_field: usize,
  private_field: usize
}
```

The following is not possible:
```rust,compile_fail
NonExhaustive {
  field: 1,
  ..Default::default()
};

PrivateFields {
  pub_field: 1,
  ..Default::default()
};
```

[`non_exhaustive!`] remedies that:
```rust
use non_exhaustive::non_exhaustive;
# mod module {#[derive(Default)] pub struct NonExhaustive {pub field: usize, _hack: usize} #[derive(Default)] pub struct PrivateFields { pub pub_field: usize, private_field: usize}} use module::*;

non_exhaustive! {NonExhaustive {
  field: 1,
  ..Default::default()
}};

non_exhaustive! {PrivateFields {
  pub_field: 1,
  ..Default::default()
}};
```

For the common case of using [`Default::default()`], [`non_exhaustive!`] allows omitting the `..expression`:

```rust
use non_exhaustive::non_exhaustive;
# mod module {#[derive(Default)] pub struct NonExhaustive {pub field: usize, _hack: usize} #[derive(Default)] pub struct PrivateFields { pub pub_field: usize, private_field: usize}} use module::*;

non_exhaustive!(NonExhaustive { field: 1 });
non_exhaustive!(PrivateFields { pub_field: 1 });
```

Under the hood, [`non_exhaustive!`] is extremely simple, it expands to:

```rust
# mod module {#[derive(Default)] pub struct NonExhaustive {pub field: usize, _hack: usize}} use module::*;

{
  let mut value: NonExhaustive = Default::default();
  value.field = 1;
  value
};
```
