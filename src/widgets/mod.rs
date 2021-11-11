mod valueeditor;
use valueeditor::ValueEditor;

use egui::Ui;

use memhack::MemHook;

pub struct Widgets {
    widgets: Vec<Box<dyn Widget>>,
    memhook: MemHook,
}

impl Widgets {
    pub fn new() -> Self {
        let memhook = MemHook::from_process("INSIDE.exe").unwrap();

        let inside_base = memhook.get_module_base_address("INSIDE.exe").unwrap();

        let x_pos_edit = Box::new(ValueEditor::<f32>::new(
            "X-Position".to_string(),
            inside_base,
            vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10],
        ));

        Self {
            widgets: vec![x_pos_edit],
            memhook,
        }
    }

    pub fn display(&mut self, ui: &mut Ui) {
        for widget in &mut self.widgets {
            widget.draw(ui, &self.memhook);
        }
    }
}

pub trait Widget {
    fn draw(&mut self, ui: &mut Ui, memhook: &MemHook);
}
