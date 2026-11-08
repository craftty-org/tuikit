//! Theme and color primitives.
//!
//! `color`, `contrast`, and `hsv` are always available. Catalog/palette helpers are feature-gated.

pub mod color;
pub mod contrast;
pub mod hsv;

#[cfg(feature = "theme-catalog")]
#[cfg_attr(docsrs, doc(cfg(feature = "theme-catalog")))]
pub mod catalog;

#[cfg(feature = "palette")]
#[cfg_attr(docsrs, doc(cfg(feature = "palette")))]
pub mod palette;
