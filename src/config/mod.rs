
//! Loading configuration (.yaml/.yml) files into a structure for easy usage
//!
//! # Basic usage:
//! ```rust
//! #[macro_use]
//! extern crate amethyst;
//! 
//! use amethyst::config::{Yaml, Element, ConfigMeta, ConfigError};
//! use std::path::Path;
//!
//! config!(Config {
//!     amount: i32 = 50,
//! });
//!
//! fn main() {
//!     let config = Config::default();
//!     assert_eq!(config.amount, 50);
//! }
//! ```
//!
//! `Config` is the name of the rust struct that will be generated by the macro. It can be anything
//! as long as it would be a valid struct in its context. e.g. no other structs by the same name.
//!
//! The inner fields of `Config` can be summed up as:
//!
//! ```ignore rust
//! name: type = default,
//! ```
//!
//! The field name will be looked up when attempting to load from a .yml/.yaml file. If it is
//! found then the value will be converted from a yaml type to a rust type and assigned to the
//! field.
//!
//! In the case that the value is either the wrong type from the field's or simply cannot be
//! found in the file, the field will be defaulted to `default`.
//!
//! 
//! In addition to basic types, any struct created through the `config!` macro will automatically
//! implement the [`Element`](trait.Element.html) trait. Meaning you can nest configuration structs
//! inside of eachother as such:
//!
//! ```rust
//! # #[macro_use] extern crate amethyst;
//! # use amethyst::config::{Yaml, Element, ConfigMeta, ConfigError};
//! # use std::path::Path;
//!
//! config!(NestedConfig {
//!     some_field: [i64; 3] = [1, 2, 3],
//! });
//! 
//! config!(Config {
//!     nested: NestedConfig = NestedConfig::default(),
//! });
//! # fn main() { }
//! ```
//! 
//! # External .yml/.yaml files
//! In the event that a config is getting too long, you can define it in the .yml/.yaml file as
//! "extern"
//!
//! example: 
//!
//! ```yaml
//! display: "extern"
//! ```
//!
//! This works similarly to rust's module system. It will first search for "\\display\\config.yml"
//! in the current context. If it cannot find it, then it will look for "\\display.yml". If it
//! cannot find either of these, then the value will be defaulted in addition to `display` being
//! overwritten if you called `write_file()`.
//!
//! # Enums
//! While this is little more than just a more convenient conversion tool, `config_enum!`
//! automatically implements the Element trait for a basic enum. Does not provide possibilities
//! for data holding enums, only conversion between a string and then enum.
//!
//! ```rust
//! # #[macro_use] extern crate amethyst;
//! # use amethyst::config::{Yaml, Element, ConfigMeta, ConfigError};
//! # use std::path::Path;
//! config_enum!(EnumName {
//!     Option1,
//!     Option2,
//! });
//!
//! config!(Config {
//!     field: EnumName = EnumName::Option2,
//! });
//!
//! fn main() {
//!     let config = Config::default();
//!     assert_eq!(config.field, EnumName::Option2);
//! }
//! ```

use std::path::Path;
pub use yaml_rust::Yaml;

#[macro_use]
mod definitions;
mod yaml;

pub use config::yaml::{Element, to_string};
pub use config::definitions::{ConfigMeta, ConfigError};

// Defines types along with defaulting values
config_enum!(Test {
    Option1,
    Option2,
    Option3,
});

config!(DisplayConfig {
    brightness: f64 = 1.0,
    fullscreen: bool = false,
    size: [u16; 2] = [1024, 768],
});

config!(LoggingConfig {
    file_path: String = "new_project.log".to_string(),
    output_level: String = "warn".to_string(),
    logging_level: String = "debug".to_string(),
});

config!(InnerInnerConfig {
    field: u64 = 58123,
});

config!(InnerConfig {
    inner_inner: InnerInnerConfig = InnerInnerConfig::default(),
});

config!(Config {
    title: String = "Amethyst game".to_string(),
    en: Test = Test::Option1,
    display: DisplayConfig = DisplayConfig::default(),
    logging: LoggingConfig = LoggingConfig::default(),
    inner: InnerConfig = InnerConfig::default(),
    inner_inner: InnerInnerConfig = InnerInnerConfig::default(),
});