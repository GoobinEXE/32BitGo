use bitgo_core::{KeyboardEdition, UsbId, EIGHT_BIT_DO_VID};

pub const RETRO_KEYBOARD_PIDS_KNOWN: &[u16] = &[];

pub fn is_retro_keyboard_candidate(usb: UsbId, product: &str) -> bool {
    if usb.vid != EIGHT_BIT_DO_VID {
        return false;
    }
    if RETRO_KEYBOARD_PIDS_KNOWN.contains(&usb.pid) {
        return true;
    }
    let p = product.to_ascii_lowercase();
    p.contains("retro") && p.contains("keyboard")
}

pub fn detect_edition(product: &str) -> Option<KeyboardEdition> {
    let p = product.to_ascii_lowercase();
    if p.contains("nes") || p.contains("n edition") || p.contains("n-edition") {
        return Some(KeyboardEdition::Nes);
    }
    if p.contains("retro") && p.contains("keyboard") {
        return Some(KeyboardEdition::Nes);
    }
    None
}
