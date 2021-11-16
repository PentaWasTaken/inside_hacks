use crate::widgets::Widget;

use memhack::traits::{FromBytes, ToBytes};
use memhack::MemHook;

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Add;
use std::str::FromStr;

use egui::widgets::{Label, TextEdit};
use egui::Key;

const HEIGHT: f32 = 20.0;

pub struct ValueEditor<T>
where
    T: FromBytes + ToBytes + Display + FromStr + Add<Output = T> + Default + Copy,
{
    name: String,
    base: usize,
    offsets: Vec<usize>,
    text_buffer: String,
    sizes: (f32, f32, f32),
    locked: bool,
    locked_val: T,
    phantom: PhantomData<T>,
}

impl<T> ValueEditor<T>
where
    T: FromBytes + ToBytes + Display + FromStr + Add<Output = T> + Default + Copy,
{
    pub fn new(name: String, base: usize, offsets: Vec<usize>, sizes: (f32, f32, f32)) -> Self {
        Self {
            name,
            base,
            offsets,
            text_buffer: String::new(),
            sizes,
            locked: false,
            locked_val: T::default(),
            phantom: PhantomData,
        }
    }

    fn read_value(&self, memhook: &MemHook) -> T {
        memhook.read_val_ptr(self.base, &self.offsets).unwrap_or(T::default())
    }

    fn write_value(&self, memhook: &MemHook, value: T) {
        let _ = memhook.write_val_ptr(self.base, &self.offsets, value);
    }
}

impl<T> Widget for ValueEditor<T>
where
    T: FromBytes + ToBytes + Display + FromStr + Add<Output = T> + Default + Copy,
{
    fn draw(&mut self, ui: &mut egui::Ui, memhook: &MemHook) {
        ui.horizontal(|ui| {
            ui.add_sized(
                (self.sizes.0, HEIGHT),
                Label::new(format!("{}: ", self.name)),
            );
            ui.add_sized(
                (self.sizes.1, HEIGHT),
                Label::new(format!("{:.4}", self.read_value(memhook))),
            );

            let text_edit_response = ui.add_sized(
                (self.sizes.2, HEIGHT),
                TextEdit::singleline(&mut self.text_buffer),
            );
            if text_edit_response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                let parsed_value = self.text_buffer.parse::<T>();

                if let Ok(value) = parsed_value {
                    self.write_value(memhook, value);
                    self.text_buffer.clear();
                    self.locked_val = value;
                }
            }

            if ui.button("+/-").clicked() {
                let parsed_value = self.text_buffer.parse::<T>();

                if let Ok(value) = parsed_value {
                    let new_val = value + self.read_value(memhook);
                    self.write_value(memhook, new_val);
                    self.locked_val = new_val;
                }
            }

            ui.checkbox(&mut self.locked, "Locked");

            if self.locked {
                self.write_value(memhook, self.locked_val);
            } else {
                self.locked_val = self.read_value(memhook);
            }
        });
    }
}
