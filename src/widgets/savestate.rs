use crate::widgets::Widget;

use egui::{
    widgets::{Button, Label},
    Color32,
};

use memhack::{MemHackError, MemHook};

pub struct Savestate {
    savestates: [Option<(f32, f32, f32)>; 9],
    index: usize,
    base: usize,
    offsets: [Vec<usize>; 3],
}

impl Savestate {
    pub fn new(base: usize, offsets: [Vec<usize>; 3]) -> Self {
        Self {
            savestates: [None; 9],
            index: 1,
            base,
            offsets,
        }
    }

    fn read_value(&self, memhook: &MemHook) -> Result<(f32, f32, f32), MemHackError> {
        let x = memhook.read_val_ptr(self.base, &self.offsets[0])?;
        let y = memhook.read_val_ptr(self.base, &self.offsets[1])?;
        let z = memhook.read_val_ptr(self.base, &self.offsets[2])?;
        Ok((x, y, z))
    }

    fn write_value(&self, memhook: &MemHook) -> Result<(), MemHackError> {
        memhook.write_val_ptr(
            self.base,
            &self.offsets[0],
            self.savestates[self.index - 1].unwrap().0,
        )?;
        memhook.write_val_ptr(
            self.base,
            &self.offsets[1],
            self.savestates[self.index - 1].unwrap().1,
        )?;
        memhook.write_val_ptr(
            self.base,
            &self.offsets[2],
            self.savestates[self.index - 1].unwrap().2,
        )?;
        Ok(())
    }
}

impl Widget for Savestate {
    fn draw(&mut self, ui: &mut egui::Ui, memhook: &memhack::MemHook) {
        ui.horizontal(|ui| {
            ui.add(Label::new("Save State").text_color(Color32::LIGHT_GRAY));

            let response = ui.add_enabled(self.index > 1, Button::new("<"));
            if response.clicked() {
                self.index -= 1;
            }

            ui.add(Label::new(self.index.to_string()).text_color(Color32::LIGHT_GRAY));

            let response = ui.add_enabled(self.index < 9, Button::new(">"));
            if response.clicked() {
                self.index += 1;
            }

            if ui.button("Save").clicked() {
                let new_value = self.read_value(memhook);
                if let Ok(new_value) = new_value {
                    self.savestates[self.index - 1] = Some(new_value);
                }
            }

            let response = ui.add_enabled(
                self.savestates[self.index - 1].is_some(),
                Button::new("Load"),
            );
            if response.clicked() {
                let _ = self.write_value(memhook);
            }
        });
    }

    fn close(&self, _memhook: &MemHook) {}
}
