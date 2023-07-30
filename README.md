# Clonelet

A simple macro to capture by `clone` in closures. Your closure doesn't need to live inside a macro.

## Example

```rust
let closure = {
    clone!(x, y, mut z);

    move || {}
};
```

Generates:

```rust
let closure = {
    let x = x.clone();
    let y = y.clone();
    let mut z = z.clone();

    move || {}
};
```
