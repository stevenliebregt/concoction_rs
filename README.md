# Concoction

Concoction contains a set of utility functions/types/macros that seemed useful at some point.

# Newtypes

## FromInner

The `FromInner` derive macro is used to generate the boilerplate of a `From<T>` implementation where `T` is the inner type of your newtype struct.

Should your newtype struct need lifetimes, you can use the `#[from_inner(lifetimes = 'a, 'b, 'c)]` attribute to pass them along.

### Example

```rust
#[derive(FromInner)]
#[from_inner(lifetimes = 'a, 'b)]
struct MyNewtype<'a, 'b>(&'a Vec<&'b str>);
```

The above would generate the following `From` implementation.

```rust
impl<'a, 'b> From<&'a Vec<&'b str>> for MyNewtype<'a, 'b> {
    fn from(inner: &'a Vec<&'b str>) -> Self {
        Self(inner)
    }
}
```