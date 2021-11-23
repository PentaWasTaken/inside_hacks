use memhack::MemHook;

use crate::widgets::Widget;

pub struct Checkbox {
    name: String,
    base: usize,
    offsets: Vec<usize>,
    original_bytes: Vec<u8>,
    replace_bytes: Vec<u8>,
    activated: bool,
}

impl Checkbox {
    pub fn new(name: String, base: usize, offsets: Vec<usize>, original_bytes: Vec<u8>, replace_bytes: Vec<u8>) -> Self {
        Self {
            name,
            base,
            offsets,
            original_bytes,
            replace_bytes,
            activated: false,
        }
    }

    pub fn activate(&self, memhook: &MemHook) {
        let _ = memhook.write_bytes_ptr(self.base, &self.offsets, &self.replace_bytes);
    }

    pub fn deactivate(&self, memhook: &MemHook) {
        let _ = memhook.write_bytes_ptr(self.base, &self.offsets, &self.original_bytes);
    }
}

impl Widget for Checkbox {
    fn draw(&mut self, ui: &mut egui::Ui, memhook: &MemHook) {
        let prev_activated = self.activated;
        ui.checkbox(&mut self.activated, &self.name);

        if !prev_activated && self.activated {
            self.activate(memhook);
        } else if prev_activated && !self.activated {
            self.deactivate(memhook);
        }
    }
}