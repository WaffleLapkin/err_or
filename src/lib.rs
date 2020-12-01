//! This crate adds [`err_or`] and [`err_or_else`] methods to the [`Option`]
//! type which allow converting `Option<E>` into `Result<_, E>`.
//!
//! Those methods were proposed in [`rust-lang/rust#73040`](https://github.com/rust-lang/rust/pull/73040)
//! PR, but it was closed.
//!
//! This crate is `#![no_std]` compatible.
//!
//! [`err_or`]: OptionExt::err_or
//! [`err_or_else`]: OptionExt::err_or_else
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs, broken_intra_doc_links)]

/// Extension trait for [`Option`].
pub trait OptionExt {
    /// Value type, i.e. `T` for `Option<T>`.
    type Item;

    /// Transforms the `Option<T>` into a [`Result<O, T>`], mapping [`Some(v)`]
    /// to [`Err(v)`] and [`None`] to [`Ok(ok)`].
    ///
    /// Arguments passed to `err_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use
    /// [`err_or_else`], which is lazily evaluated.
    ///
    /// [`Result<O, T>`]: core::result::Result
    /// [`Err(v)`]: core::result::Result#variant.Err
    /// [`Ok(ok)`]: core::result::Result#variant.Ok
    /// [`None`]: core::option::Option#variant.None
    /// [`Some(v)`]: core::option::Option#variant.Some
    /// [`err_or_else`]: #tymethod.err_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use err_or::OptionExt;
    ///
    /// let x = Some("foo");
    /// assert_eq!(x.err_or(0), Err("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.err_or(0), Ok(0));
    /// ```
    fn err_or<O>(self, ok: O) -> Result<O, Self::Item>;

    /// Transforms the `Option<T>` into a [`Result<O, T>`], mapping [`Some(v)`]
    /// to [`Err(v)`] and [`None`] to [`Ok(ok())`].
    ///
    /// [`Result<O, T>`]: core::result::Result
    /// [`Err(v)`]: core::result::Result#variant.Err
    /// [`Ok(ok())`]: core::result::Result#variant.Ok
    /// [`None`]: core::option::Option#variant.None
    /// [`Some(v)`]: core::option::Option#variant.Some
    ///
    /// # Examples
    ///
    /// ```
    /// use err_or::OptionExt;
    ///
    /// let x = Some("foo");
    /// assert_eq!(x.err_or_else(|| 0), Err("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.err_or_else(|| 0), Ok(0));
    /// ```
    fn err_or_else<O, F>(self, ok: F) -> Result<O, Self::Item>
    where
        F: FnOnce() -> O;
}

impl<T> OptionExt for Option<T> {
    type Item = T;

    #[inline]
    fn err_or<O>(self, ok: O) -> Result<O, Self::Item> {
        match self {
            Some(v) => Err(v),
            None => Ok(ok),
        }
    }

    #[inline]
    fn err_or_else<O, F>(self, ok: F) -> Result<O, Self::Item>
    where
        F: FnOnce() -> O,
    {
        match self {
            Some(v) => Err(v),
            None => Ok(ok()),
        }
    }
}
