//! Greek Key (Meander) pattern generator.
//!
//! Generates SVG and PNG files containing Greek Key (Meander) border designs
//! in rectangle and circle forms.
//!
//! # Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `native` | yes | Enables PNG rasterization and file I/O via `resvg`. Required for [`rect::generate_pattern_svg`] and [`circle::generate_pattern_svg`]. |
//! | `wasm` | no | Enables `wasm-bindgen` browser exports in [`wasm`]. |
//!
//! To use this crate in a **WASM** environment, disable the default features and
//! call [`rect::generate_svg_string`] / [`circle::generate_svg_string`] instead:
//!
//! ```toml
//! greek-meander = { version = "0.1", default-features = false }
//! ```
//!
//! For browser exports with `wasm-bindgen`, build with the `wasm` feature:
//!
//! ```bash
//! wasm-pack build --target web --no-default-features --features wasm
//! ```
//!
//! # Rectangle example
//!
//! ```
//! # #[cfg(feature = "native")] fn main() {
//! use greek_meander::{GreekKeyRectConfig, VisualOptions};
//!
//! let path = std::env::temp_dir().join("doctest_rect").to_string_lossy().into_owned();
//! let config = GreekKeyRectConfig::new(25, 16, 9, 10, 3.0).unwrap();
//! let visual = VisualOptions::default();
//! greek_meander::rect::generate_pattern_svg(&config, &visual, &path).unwrap();
//! # let _ = std::fs::remove_file(format!("{}.svg", path));
//! # let _ = std::fs::remove_file(format!("{}.png", path));
//! # }
//! # #[cfg(not(feature = "native"))] fn main() {}
//! ```
//!
//! # Circle example
//!
//! ```
//! # #[cfg(feature = "native")] fn main() {
//! use greek_meander::{GreekKeyCircleConfig, VisualOptions};
//!
//! let path = std::env::temp_dir().join("doctest_circle").to_string_lossy().into_owned();
//! let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).unwrap();
//! let visual = VisualOptions::default();
//! greek_meander::circle::generate_pattern_svg(&config, &visual, &path).unwrap();
//! # let _ = std::fs::remove_file(format!("{}.svg", path));
//! # let _ = std::fs::remove_file(format!("{}.png", path));
//! # }
//! # #[cfg(not(feature = "native"))] fn main() {}
//! ```
//!
//! # Ellipse example
//!
//! ```
//! # #[cfg(feature = "native")] fn main() {
//! use greek_meander::{GreekKeyEllipseConfig, VisualOptions};
//!
//! let path = std::env::temp_dir().join("doctest_ellipse").to_string_lossy().into_owned();
//! let config = GreekKeyEllipseConfig::new(200.0, 120.0, 28, 8, 3.0).unwrap();
//! let visual = VisualOptions::default();
//! greek_meander::ellipse::generate_pattern_svg(&config, &visual, &path).unwrap();
//! # let _ = std::fs::remove_file(format!("{}.svg", path));
//! # let _ = std::fs::remove_file(format!("{}.png", path));
//! # }
//! # #[cfg(not(feature = "native"))] fn main() {}
//! ```
//!
//! # WASM / SVG-string example
//!
//! ```
//! use greek_meander::{GreekKeyRectConfig, VisualOptions};
//!
//! let config = GreekKeyRectConfig::new(25, 16, 9, 10, 3.0).unwrap();
//! let svg = greek_meander::rect::generate_svg_string(&config, &VisualOptions::default());
//! assert!(svg.contains("<svg"));
//! ```
//!
//! # Browser WASM exports
//!
//! With the `wasm` feature enabled, [`wasm::rect_generate_svg`] and
//! [`wasm::circle_generate_svg`] expose JavaScript-callable functions that
//! return SVG markup.

pub mod circle;
pub(crate) mod common;
pub mod config;
pub mod ellipse;
pub mod rect;
#[cfg(feature = "wasm")]
pub mod wasm;

pub use common::Point;
pub use config::{
    EllipseRadii, GreekKeyCircleConfig, GreekKeyEllipseConfig, GreekKeyRectConfig, Radii,
    VisualOptions,
};
