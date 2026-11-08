//! Terminal session lifecycle helpers for Ratatui/Crossterm TUIs.

use std::io::{self, Stdout};
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

static RESTORED: AtomicBool = AtomicBool::new(true);

/// Options controlling terminal setup behavior.
#[derive(Debug, Clone, Copy)]
pub struct TerminalOptions {
    pub alternate_screen: bool,
    pub mouse_capture: bool,
    pub bracketed_paste: bool,
    pub hide_cursor: bool,
    pub install_panic_hook: bool,
}

impl Default for TerminalOptions {
    fn default() -> Self {
        Self {
            alternate_screen: true,
            mouse_capture: true,
            bracketed_paste: true,
            hide_cursor: true,
            install_panic_hook: true,
        }
    }
}

/// A terminal session that restores state on drop.
pub struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    active: bool,
    options: TerminalOptions,
}

impl TerminalSession {
    /// Initialize the terminal with default options.
    pub fn init() -> io::Result<Self> {
        Self::with_options(TerminalOptions::default())
    }

    /// Initialize the terminal with explicit options.
    pub fn with_options(options: TerminalOptions) -> io::Result<Self> {
        if options.install_panic_hook {
            install_panic_hook(options);
        }
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        if options.alternate_screen {
            execute!(stdout, EnterAlternateScreen)?;
        }
        if options.mouse_capture {
            execute!(stdout, EnableMouseCapture)?;
        }
        if options.bracketed_paste {
            execute!(stdout, EnableBracketedPaste)?;
        }
        if options.hide_cursor {
            execute!(stdout, Hide)?;
        }
        RESTORED.store(false, Ordering::SeqCst);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(Self {
            terminal,
            active: true,
            options,
        })
    }

    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    pub fn size(&self) -> io::Result<ratatui::layout::Size> {
        self.terminal.size()
    }

    pub fn draw<F>(&mut self, render: F) -> io::Result<()>
    where
        F: FnOnce(&mut ratatui::Frame<'_>),
    {
        self.terminal.draw(render)?;
        Ok(())
    }

    /// Restore terminal state. Safe to call more than once.
    pub fn restore(&mut self) {
        if !self.active {
            return;
        }
        self.active = false;
        restore_with_options(self.options);
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        self.restore();
    }
}

fn restore_with_options(options: TerminalOptions) {
    if RESTORED.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = disable_raw_mode();
    let mut stdout = io::stdout();
    if options.hide_cursor {
        let _ = execute!(stdout, Show);
    }
    if options.bracketed_paste {
        let _ = execute!(stdout, DisableBracketedPaste);
    }
    if options.mouse_capture {
        let _ = execute!(stdout, DisableMouseCapture);
    }
    if options.alternate_screen {
        let _ = execute!(stdout, LeaveAlternateScreen);
    }
}

fn install_panic_hook(options: TerminalOptions) {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_with_options(options);
        previous(info);
    }));
}
