//! # `randsym`
//!
//! Creates unique identifiers for macros.
//!
//! [![github]](https://github.com/DexterHill0/randsym)&ensp;[![crates-io]](https://crates.io/crates/randsym)&ensp;[![docs-rs]](https://docs.rs/randsym)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! `randsym` generates unique identifiers using UUID. The identifies can be used simply to avoid two items with the same identifier, or they can be bound to names allowing the identifiers to be repeated.
//!
//! The syntax is as follows:
//! - `/?/` - random identifier
//! - `/?@the_ident/` - random identifier bound to the name `the_ident`
//!
//! ### Examples:
//! **No binding**
//! ```rs
//! randsym::randsym! {
//!     fn /?/ () -> String {
//!         "I have a random name!".into()
//!     }
//! }
//! ```
//!
//! **With binding**
//! ```rs
//! randsym::randsym! {
//!     fn /?@my_fn/ () -> String {
//!         "I have a random name!".into()
//!     }
//!
//!     println!("{}", /?@my_fn/()); // "I have a random name!"
//! }
//! ```

use std::collections::HashMap;

use proc_macro::{TokenTree as TT, *};
use uuid::Uuid;

/// Generates a random identifier.
///
/// ```
/// randsym::randsym! {
///     fn /?/ () -> String {
///         "I have a random name!".into()
///     }
/// }
/// ```
/// ------
/// ```
/// randsym::randsym! {
///     fn /?@my_fn/ () -> String {
///         "I have a random name!".into()
///     }
///
///     println!("{}", /?@my_fn/()); // "I have a random name!"
/// }
/// ```
fn replace_syms(tokens: TokenStream, named_syms: &mut HashMap<String, Ident>) -> TokenStream {
    let mut tokens = tokens.into_iter().peekable();
    let mut out = TokenStream::new();

    loop {
        out.extend(Some(match tokens.next() {
            Some(ref p1 @ TT::Punct(ref punct)) if punct.as_char() == '/' => match tokens.peek() {
                Some(TT::Punct(ref punct)) if punct.as_char() == '?' => {
                    let p2 = tokens.next().unwrap();

                    match tokens.peek() {
                        Some(TT::Punct(punct)) if punct.as_char() == '@' => {
                            tokens.next();

                            match tokens.next() {
                                Some(TT::Ident(name)) => {
                                    let name =
                                        named_syms.entry(name.to_string()).or_insert_with(gen_sym);

                                    match tokens.next() {
                                        Some(TT::Punct(ref punct)) if punct.as_char() == '/' => {
                                            TT::Ident(name.clone())
                                        }
                                        Some(tok) => tok,
                                        None => break,
                                    }
                                }
                                Some(tok) => tok,
                                None => break,
                            }
                        }
                        Some(TT::Punct(punct)) if punct.as_char() == '/' => {
                            tokens.next();

                            TT::Ident(gen_sym())
                        }
                        _ => p2,
                    }
                }
                _ => p1.clone(),
            },
            Some(TT::Group(group)) => {
                Group::new(group.delimiter(), replace_syms(group.stream(), named_syms)).into()
            }
            Some(tok) => tok,
            None => break,
        }))
    }

    out
}

#[proc_macro]
pub fn randsym(input: TokenStream) -> TokenStream {
    let mut named_syms = HashMap::new();

    replace_syms(input, &mut named_syms)
}

// generate random ident using simple uuid
fn gen_sym() -> Ident {
    Ident::new(
        &format!("_randsym_{}", Uuid::new_v4().simple()),
        Span::call_site(),
    )
}
