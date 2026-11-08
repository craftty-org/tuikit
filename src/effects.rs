//! Thin optional wrappers around `tachyonfx`.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use tachyonfx::EffectManager;

/// A small wrapper that can be enabled/disabled.
#[derive(Default)]
pub struct FxManager {
    manager: EffectManager<()>,
    enabled: bool,
}

impl FxManager {
    pub fn new(enabled: bool) -> Self {
        Self {
            manager: EffectManager::default(),
            enabled,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.manager = EffectManager::default();
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn inner_mut(&mut self) -> &mut EffectManager<()> {
        &mut self.manager
    }

    pub fn process(&mut self, dt: std::time::Duration, buf: &mut Buffer, area: Rect) {
        if self.enabled {
            self.manager.process_effects(dt.into(), buf, area);
        }
    }
}
