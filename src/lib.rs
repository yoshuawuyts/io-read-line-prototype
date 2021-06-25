//! Prototype for `io::read_line`.
//!
//! This serves as a counterpart to
//! [`io::read_from_string`](https://github.com/rust-lang/rust/issues/80218) and
//! is inspired by proposals such as
//! [`std::io::input`](https://github.com/rust-lang/rust/pull/75435).
//!
//! This method makes it easier to read a line from a reader into a string, and leaves
//! more advances input proposals for later. For example [C++'s
//! text parsing
//! proposal](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p1729r1.html)
//! provides a coherent text parsing counterpart using `format!`-like syntax,
//! and would be interesting to explore porting to Rust.
//!
//! # Examples
//!
//! ```
//! # use io_read_line_prototype::read_line;
//! # use std::io;
//! # fn main() -> io::Result<()> {
//! // Before
//! let mut line = String::new();
//! io::stdin().read_line(&mut line)?;
//!
//! // After
//! let line = read_line(io::stdin())?;
//! # Ok(()) }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![feature(min_specialization)]

use std::io::{self, BufReader, Read};

/// Read a line from a [reader][Read] into a new [`String`].
///
/// This is a convenience function for [`BufRead::read_line`]. Using this
/// function avoids having to create a variable first and provides more type
/// safety since you can only get the buffer out if there were no errors. (If you
/// use [`BufRead::read_line`] you have to remember to check whether the read
/// succeeded because otherwise your buffer will be empty or only partially full.)
///
/// # Performance
///
/// The downside of this function's increased ease of use and type safety is
/// that it gives you less control over performance. For example, you can't
/// pre-allocate memory like you can using [`String::with_capacity`] and
/// [`Read::read_to_string`]. Also, you can't re-use the buffer if an error
/// occurs while reading.
///
/// In many cases, this function's performance will be adequate and the ease of use
/// and type safety tradeoffs will be worth it. However, there are cases where you
/// need more control over performance, and in those cases you should definitely use
/// [`BufRead::read_line`] directly.
///
/// # Errors
///
/// This function forces you to handle errors because the output (the `String`)
/// is wrapped in a [`Result`]. See [`BufRead::read_line`] for the errors
/// that can occur. If any error occurs, you will get an [`Err`], so you
/// don't have to worry about your buffer being empty or partially full.
///
/// [`BufRead::read_line`]: std::io::BufRead::read_line
///
/// # Examples
///
/// ```no_run
/// # use io_read_line_prototype::read_line;
/// # use std::io;
/// fn main() -> io::Result<()> {
///     let stdin = read_line(&mut io::stdin())?;
///     println!("Stdin was:");
///     println!("{}", stdin);
///     Ok(())
/// }
/// ```
pub fn read_line<R: Read>(reader: R) -> io::Result<String> {
    let mut buf = String::new();
    let mut reader = BufReader::new(reader);
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}
