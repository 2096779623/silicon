//! `silicon` is a tool to create beautiful image of your source code.
//!
//! # Example
//!
//! ```
//! use syntect::easy::HighlightLines;
//! use syntect::util::LinesWithEndings;
//! use silicon::utils::ShadowAdder;
//! use silicon::formatter::ImageFormatterBuilder;
//! use silicon::assets::HighlightingAssets;
//!
//! let ha = HighlightingAssets::new();
//! let (ps, ts) = (ha.syntax_set, ha.theme_set);
//! let code = r#"fn main() {
//!     println!("Hello, world!");
//! }
//! "#;
//!
//! let syntax = ps.find_syntax_by_token("rs").unwrap();
//! let theme = &ts.themes["Dracula"];
//!
//! let mut h = HighlightLines::new(syntax, theme);
//! let highlight = LinesWithEndings::from(&code)
//!     .map(|line| h.highlight(line, &ps))
//!     .collect::<Vec<_>>();
//!
//! let mut formatter = ImageFormatterBuilder::new()
//!     .font(vec![("Hack", 26.0)])
//!     .shadow_adder(ShadowAdder::default())
//!     .build()
//!     .unwrap();
//! let image = formatter.format(&highlight, theme);
//!
//! image.save("hello.png").unwrap();
//! ```
#[macro_use]
extern crate log;

pub mod assets;
pub mod blur;
pub mod directories;
pub mod error;
pub mod font;
pub mod formatter;
pub mod hb_wrapper;
pub mod utils;
