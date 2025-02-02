use std::collections::HashMap;

#[cfg(target_os = "windows")]
pub fn get_key_map() -> HashMap<&'static str, &'static str> {
    let mut key_map = HashMap::new();
    key_map.insert("cmd", "Ctrl"); // Alias for "ctrl"
    key_map.insert("cmd_or_ctrl", "Ctrl");  // Alias for "ctrl"
    key_map.insert("shift", "Shift");
    key_map.insert("alt", "Alt");
    key_map.insert("ctrl", "Ctrl");
    key_map.insert("opt", "Alt");  // Alias for "alt"
    key_map.insert("altgr", "AltGr");
    key_map.insert("super", "Super");
    key_map.insert("win", "Win");
    key_map.insert("meta", "Meta");
    key_map.insert("plus", "Plus");
    key_map.insert("space", "Space");
    key_map.insert("tab", "Tab");
    key_map.insert("capslock", "CapsLock");
    key_map.insert("numlock", "NumLock");
    key_map.insert("scrolllock", "ScrollLock");
    key_map.insert("backspace", "Backspace");
    key_map.insert("delete", "Delete");
    key_map.insert("insert", "Insert");
    key_map.insert("return", "Return");
    key_map.insert("enter", "Return");
    key_map.insert("up", "UpArrow");
    key_map.insert("down", "DownArrow");
    key_map.insert("left", "LeftArrow");
    key_map.insert("right", "RightArrow");
    key_map.insert("home", "Home");
    key_map.insert("end", "End");
    key_map.insert("pageup", "PageUp");
    key_map.insert("pagedown", "PageDown");
    key_map.insert("escape", "Escape");
    key_map.insert("esc", "Escape");
    key_map.insert("num0", "Numpad0");
    key_map.insert("num1", "Numpad1");
    key_map.insert("num2", "Numpad2");
    key_map.insert("num3", "Numpad3");
    key_map.insert("num4", "Numpad4");
    key_map.insert("num5", "Numpad5");
    key_map.insert("num6", "Numpad6");
    key_map.insert("num7", "Numpad7");
    key_map.insert("num8", "Numpad8");
    key_map.insert("num9", "Numpad9");
    key_map.insert("numdec", "NumpadDecimal");
    key_map.insert("numadd", "NumpadAdd");
    key_map.insert("numsub", "NumpadSubtract");
    key_map.insert("nummult", "NumpadMultiply");
    key_map.insert("numdiv", "NumpadDivide");
    key_map.insert("f1", "F1");
    key_map.insert("f2", "F2");
    key_map.insert("f3", "F3");
    key_map.insert("f4", "F4");
    key_map.insert("f5", "F5");
    key_map.insert("f6", "F6");
    key_map.insert("f7", "F7");
    key_map.insert("f8", "F8");
    key_map.insert("f9", "F9");
    key_map.insert("f10", "F10");
    key_map.insert("f11", "F11");
    key_map.insert("f12", "F12");
    key_map.insert("f13", "F13");
    key_map.insert("f14", "F14");
    key_map.insert("f15", "F15");
    key_map.insert("f16", "F16");
    key_map.insert("f17", "F17");
    key_map.insert("f18", "F18");
    key_map.insert("f19", "F19");
    key_map.insert("f20", "F20");
    key_map.insert("f21", "F21");
    key_map.insert("f22", "F22");
    key_map.insert("f23", "F23");
    key_map.insert("f24", "F24");

    key_map
}

#[cfg(target_os = "macos")]
pub fn get_key_map() -> HashMap<&'static str, &'static str> {
    let mut key_map = HashMap::new();
    key_map.insert("plus", "+");
    key_map.insert("space", " ");
    key_map.insert("tab", "\u{21e5}");
    key_map.insert("capslock", "\u{1000}");
    key_map.insert("numlock", "\u{1001}");
    key_map.insert("scrolllock", "\u{1002}");
    key_map.insert("backspace", "\u{232b}");
    key_map.insert("delete", "\u{2326}");
    key_map.insert("insert", "\u{2380}");
    key_map.insert("return", "\u{23ce}");
    key_map.insert("enter", "\u{23ce}");
    key_map.insert("up", "\u{2191}");
    key_map.insert("down", "\u{2193}");
    key_map.insert("left", "\u{2190}");
    key_map.insert("right", "\u{2192}");
    key_map.insert("home", "\u{2196}");
    key_map.insert("end", "\u{2198}");
    key_map.insert("pageup", "\u{21DE}");
    key_map.insert("pagedown", "\u{21DF}");
    key_map.insert("escape", "\u{238b}");
    key_map.insert("esc", "\u{238b}");
    key_map.insert("num0", "\u{30}");
    key_map.insert("num1", "\u{31}");
    key_map.insert("num2", "\u{32}");
    key_map.insert("num3", "\u{33}");
    key_map.insert("num4", "\u{34}");
    key_map.insert("num5", "\u{35}");
    key_map.insert("num6", "\u{36}");
    key_map.insert("num7", "\u{37}");
    key_map.insert("num8", "\u{38}");
    key_map.insert("num9", "\u{39}");
    key_map.insert("numdec", "\u{2e}");
    key_map.insert("numadd", "\u{2b}");
    key_map.insert("numsub", "\u{2d}");
    key_map.insert("nummult", "\u{2a}");
    key_map.insert("numdiv", "\u{2f}");
    key_map.insert("f1", "\u{F704}");
    key_map.insert("f2", "\u{F705}");
    key_map.insert("f3", "\u{F706}");
    key_map.insert("f4", "\u{F707}");
    key_map.insert("f5", "\u{F708}");
    key_map.insert("f6", "\u{F709}");
    key_map.insert("f7", "\u{F70A}");
    key_map.insert("f8", "\u{F70B}");
    key_map.insert("f9", "\u{F70C}");
    key_map.insert("f10", "\u{F70D}");
    key_map.insert("f11", "\u{F70E}");
    key_map.insert("f12", "\u{F70F}");
    key_map.insert("f13", "\u{F710}");
    key_map.insert("f14", "\u{F711}");
    key_map.insert("f15", "\u{F712}");
    key_map.insert("f16", "\u{F713}");
    key_map.insert("f17", "\u{F714}");
    key_map.insert("f18", "\u{F715}");
    key_map.insert("f19", "\u{F716}");
    key_map.insert("f20", "\u{F717}");
    key_map.insert("f21", "\u{F718}");
    key_map.insert("f22", "\u{F719}");
    key_map.insert("f23", "\u{F71A}");
    key_map.insert("f24", "\u{F71B}");

    key_map
}

#[cfg(target_os = "macos")]
pub fn get_modifier_map() -> HashMap<&'static str, cocoa::appkit::NSEventModifierFlags> {
    let mut mod_map = HashMap::new();
    mod_map.insert("cmd", cocoa::appkit::NSEventModifierFlags::NSCommandKeyMask);
    mod_map.insert("cmd_or_ctrl", cocoa::appkit::NSEventModifierFlags::NSCommandKeyMask);  // Alias for "cmd"
    mod_map.insert("shift", cocoa::appkit::NSEventModifierFlags::NSShiftKeyMask);
    mod_map.insert("alt", cocoa::appkit::NSEventModifierFlags::NSAlternateKeyMask);
    mod_map.insert("ctrl", cocoa::appkit::NSEventModifierFlags::NSControlKeyMask);
    mod_map.insert("opt", cocoa::appkit::NSEventModifierFlags::NSAlternateKeyMask);  // Alias for "alt"
    mod_map.insert("altgr", cocoa::appkit::NSEventModifierFlags::NSAlternateKeyMask);  // Alias for "alt"
    mod_map.insert("super", cocoa::appkit::NSEventModifierFlags::NSCommandKeyMask);  // Alias for "cmd"
    mod_map.insert("win", cocoa::appkit::NSEventModifierFlags::NSCommandKeyMask);  // Alias for "cmd"
    mod_map.insert("meta", cocoa::appkit::NSEventModifierFlags::NSCommandKeyMask);
    mod_map
}