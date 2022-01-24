use crate::widgets::Widget;

use egui::widgets::{Button, TextEdit};

use memhack::{MemHackError, MemHook};

const HEIGHT: f32 = 20.0;
const WIDTH: f32 = 25.0;

pub struct CheckpointLoader {
    base: usize,
    section_offsets: Vec<usize>,
    subsection_offsets: Vec<usize>,
    death_offsets: Vec<usize>,
    textbuffer_1: String,
    textbuffer_2: String,
}

impl CheckpointLoader {
    pub fn new(
        base: usize,
        section_offsets: Vec<usize>,
        subsection_offsets: Vec<usize>,
        death_offsets: Vec<usize>,
    ) -> Self {
        Self {
            base,
            section_offsets,
            subsection_offsets,
            death_offsets,
            textbuffer_1: String::new(),
            textbuffer_2: String::new(),
        }
    }

    fn read_value(&self, memhook: &MemHook) -> (u32, u32) {
        let section = memhook
            .read_val_ptr(self.base, &self.section_offsets)
            .unwrap_or_default();
        let subsection = memhook
            .read_val_ptr(self.base, &self.subsection_offsets)
            .unwrap_or_default();
        (section, subsection)
    }

    fn write_value(
        &self,
        section: u32,
        subsection: u32,
        memhook: &MemHook,
    ) -> Result<(), MemHackError> {
        memhook.write_val_ptr(self.base, &self.section_offsets, section)?;
        memhook.write_val_ptr(self.base, &self.subsection_offsets, subsection)?;
        memhook.write_val_ptr(self.base, &self.death_offsets, 1u8)?;
        Ok(())
    }
}

impl Widget for CheckpointLoader {
    fn draw(&mut self, ui: &mut egui::Ui, memhook: &MemHook) {
        ui.horizontal(|ui| {
            let (section, subsection) = self.read_value(memhook);
            ui.label(format!(
                "Checkpoint: {section:0width$}|{subsection}",
                width = 2
            ));

            ui.add_sized(
                (WIDTH, HEIGHT),
                TextEdit::singleline(&mut self.textbuffer_1),
            );
            ui.add_sized(
                (WIDTH, HEIGHT),
                TextEdit::singleline(&mut self.textbuffer_2),
            );

            let response = ui.add_sized((WIDTH, HEIGHT), Button::new("Load"));
            if response.clicked() {
                let value_section: u32 = self.textbuffer_1.parse().unwrap_or(section);
                let value_subsection: u32 = self.textbuffer_2.parse().unwrap_or(subsection);
                if self
                    .write_value(value_section, value_subsection, memhook)
                    .is_ok()
                {
                    self.textbuffer_1 = String::new();
                    self.textbuffer_2 = String::new();
                }
            }
        });
    }

    fn close(&self, _memhook: &MemHook) {}
}
