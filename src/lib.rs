//! Greek Key (Meander) pattern generator.
//!
//! Generates SVG and PNG files containing Greek Key (Meander) border designs
//! in rectangle and circle forms.
//!
//! # Rectangle example
//!
//! ```
//! use greek_meander::GreekKeyRectConfig;
//!
//! let path = std::env::temp_dir().join("doctest_rect").to_string_lossy().into_owned();
//! let config = GreekKeyRectConfig::new(25, 16, 9, 10, 3.0).unwrap();
//! greek_meander::rect::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
//! # let _ = std::fs::remove_file(format!("{}.svg", path));
//! # let _ = std::fs::remove_file(format!("{}.png", path));
//! ```
//!
//! # Circle example
//!
//! ```
//! use greek_meander::GreekKeyCircleConfig;
//!
//! let path = std::env::temp_dir().join("doctest_circle").to_string_lossy().into_owned();
//! let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).unwrap();
//! greek_meander::circle::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
//! # let _ = std::fs::remove_file(format!("{}.svg", path));
//! # let _ = std::fs::remove_file(format!("{}.png", path));
//! ```

pub mod circle;
pub(crate) mod common;
pub mod config;
pub mod rect;

pub use common::Point;
pub use config::{GreekKeyCircleConfig, GreekKeyRectConfig, Radii};
