use crate::widgets::Widget;

use memhack::traits::{FromBytes, ToBytes};
use memhack::MemHook;

use std::fmt::Display;
use std::marker::PhantomData;

pub struct ValueEditor<T>
where
    T: FromBytes + ToBytes + Display,
{
    name: String,
    base: usize,
    offsets: Vec<usize>,
    phantom: PhantomData<T>,
}

impl<T> ValueEditor<T>
where
    T: FromBytes + ToBytes + Display,
{
    pub fn new(name: String, base: usize, offsets: Vec<usize>) -> Self {
        Self {
            name,
            base,
            offsets,
            phantom: PhantomData,
        }
    }

    pub fn get_value(&self, memhook: &MemHook) -> T {
        memhook.read_val_ptr(self.base, &self.offsets).unwrap()
    }
}

impl<T> Widget for ValueEditor<T>
where
    T: FromBytes + ToBytes + Display,
{
    fn draw(&mut self, ui: &mut egui::Ui, memhook: &MemHook) {
        ui.horizontal(|ui| {
            ui.label(format!("{}: ", self.name));
            ui.label(format!("{}", self.get_value(memhook)));
        });
    }
}
