use crate::{ContextMenu, MenuItem, Position};
use std::sync::Arc;
use tauri::{Runtime, Window};

pub fn show_context_menu<R: Runtime>(
    context_menu: Arc<ContextMenu<R>>,
    window: Window<R>,
    pos: Option<Position>,
    items: Option<Vec<MenuItem>>,
) {
    // Noop, to pass CI until upstream supports Linux
}
