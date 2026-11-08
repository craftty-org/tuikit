//! Schema-light theme catalog helpers.
//!
//! This format is intentionally flexible: applications can load `name`/`light`/`colors` entries
//! and convert the color map into their own theme structures.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// A generic theme entry backed by a string color map (typically `#rrggbb` values).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThemeCatalogEntry {
    pub name: String,
    #[serde(default)]
    pub light: bool,
    #[serde(default)]
    pub colors: BTreeMap<String, String>,
}

/// TOML document shape: `{ themes = [...] }`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ThemeCatalogFile {
    #[serde(default)]
    pub themes: Vec<ThemeCatalogEntry>,
}

/// Parse a theme catalog from TOML text.
pub fn load_catalog_from_str(toml_text: &str) -> Result<Vec<ThemeCatalogEntry>, toml::de::Error> {
    let file = toml::from_str::<ThemeCatalogFile>(toml_text)?;
    Ok(file.themes)
}

/// Load a theme catalog from a TOML file path. Returns an empty vec on read/parse failure.
pub fn load_catalog_from_path(path: impl AsRef<Path>) -> Vec<ThemeCatalogEntry> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| load_catalog_from_str(&s).ok())
        .unwrap_or_default()
}

/// Merge catalogs by theme name. Entries in `overrides` replace or append to `base`.
pub fn merge_catalogs(
    mut base: Vec<ThemeCatalogEntry>,
    overrides: Vec<ThemeCatalogEntry>,
) -> Vec<ThemeCatalogEntry> {
    for entry in overrides {
        if let Some(existing) = base.iter_mut().find(|t| t.name == entry.name) {
            *existing = entry;
        } else {
            base.push(entry);
        }
    }
    base
}

/// Parse a flat single-theme TOML object into a catalog entry (useful for `themes/*.toml` files).
pub fn load_flat_theme_from_str(toml_text: &str) -> Result<ThemeCatalogEntry, toml::de::Error> {
    #[derive(Deserialize)]
    struct Flat {
        name: String,
        #[serde(default)]
        light: bool,
        #[serde(flatten)]
        colors: BTreeMap<String, toml::Value>,
    }
    let flat = toml::from_str::<Flat>(toml_text)?;
    let mut colors = BTreeMap::new();
    for (k, v) in flat.colors {
        if let Some(s) = v.as_str() {
            colors.insert(k, s.to_string());
        }
    }
    Ok(ThemeCatalogEntry {
        name: flat.name,
        light: flat.light,
        colors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_catalog() {
        let toml = r##"
[[themes]]
name = "Demo"
light = false
[themes.colors]
bg = "#000000"
fg = "#ffffff"
"##;
        let themes = load_catalog_from_str(toml).unwrap();
        assert_eq!(themes[0].name, "Demo");
        assert_eq!(themes[0].colors.get("bg").map(String::as_str), Some("#000000"));
    }

    #[test]
    fn merges_by_name() {
        let base = vec![ThemeCatalogEntry { name: "A".into(), light: false, colors: BTreeMap::new() }];
        let over = vec![ThemeCatalogEntry { name: "A".into(), light: true, colors: BTreeMap::new() }];
        let merged = merge_catalogs(base, over);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].light);
    }
}
