//! Generic, reusable primitives for Ratatui/Crossterm terminal applications.
//!
//! Most functionality is feature-gated. Pure utilities (`time`, `text`, and parts of `theme`) are
//! available by default.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod text;
pub mod theme;
pub mod time;

#[cfg(feature = "widgets-ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "widgets-ui")))]
pub mod widgets;

#[cfg(feature = "terminal")]
#[cfg_attr(docsrs, doc(cfg(feature = "terminal")))]
pub mod terminal;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub mod http;

#[cfg(feature = "config")]
#[cfg_attr(docsrs, doc(cfg(feature = "config")))]
pub mod config;

#[cfg(any(feature = "config", feature = "paths"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "config", feature = "paths"))))]
pub mod paths;

#[cfg(feature = "fslog")]
#[cfg_attr(docsrs, doc(cfg(feature = "fslog")))]
pub mod fslog;

#[cfg(feature = "effects")]
#[cfg_attr(docsrs, doc(cfg(feature = "effects")))]
pub mod effects;

#[cfg(any(feature = "cache-sqlite", feature = "cache-codec"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "cache-sqlite", feature = "cache-codec"))))]
pub mod cache;
