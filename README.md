# `randsym`

Creates unique identifiers for macros.

[<img alt="github" src="https://img.shields.io/badge/github-dexterhill0/randsym-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/DexterHill0/randsym)
[<img alt="crates.io" src="https://img.shields.io/crates/v/randsym.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/randsym)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-randsym-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/randsym)

`randsym` generates unique identifiers using UUID. The identifies can be used simply to avoid two items with the same identifier, or they can be bound to names allowing the identifiers to be repeated.

The syntax is as follows:

- `/?/` - random identifier
- `/?@the_ident/` - random identifier bound to the name `the_ident`

### Examples:

**No binding**

```rs
randsym::randsym! {
    fn /?/ () -> String {
        "I have a random name!".into()
    }
}
```

**With binding**

```rs
randsym::randsym! {
    fn /?@my_fn/ () -> String {
        "I have a random name!".into()
    }

    println!("{}", /?@my_fn/()); // "I have a random name!"
}
```
