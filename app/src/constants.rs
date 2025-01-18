use tauri_plugin_window_state::StateFlags;

pub const WINDOW_STATE_FLAGS: StateFlags = StateFlags::from_bits_truncate(
    StateFlags::FULLSCREEN.bits()
        | StateFlags::MAXIMIZED.bits()
        | StateFlags::POSITION.bits()
        | StateFlags::SIZE.bits()
        | StateFlags::VISIBLE.bits(),
);
