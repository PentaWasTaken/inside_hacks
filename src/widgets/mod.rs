mod valueeditor;
use valueeditor::ValueEditor;

mod checkbox;
use checkbox::Checkbox;

mod savestate;
use savestate::Savestate;

mod valueeditor_multiple;
use valueeditor_multiple::ValueEditorMultiple;

mod checkpoint_loader;
use checkpoint_loader::CheckpointLoader;

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

        let sizes = (90.0, 60.0, 100.0);

        let x_pos_edit = Box::new(ValueEditor::<f32>::new(
            "X-Position".to_string(),
            inside_base,
            vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10],
            sizes,
        ));

        let y_pos_edit = Box::new(ValueEditor::<f32>::new(
            "Y-Position".to_string(),
            inside_base,
            vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x14],
            sizes,
        ));

        let z_pos_edit = Box::new(ValueEditor::<f32>::new(
            "Z-Position".to_string(),
            inside_base,
            vec![0x1001FA0, 0x260, 0x2E8, 0x318, 0x10, 0x28, 0x40, 0x18],
            sizes,
        ));

        let jump_strength_edit = Box::new(ValueEditor::<f32>::new(
            "Jump Strength".to_string(),
            inside_base,
            vec![
                0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x68, 0x30, 0x18, 0x14,
            ],
            sizes,
        ));

        let run_speed_edit = Box::new(ValueEditor::<f32>::new(
            "Run Speed".to_string(),
            inside_base,
            vec![
                0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x68, 0x28, 0x14,
            ],
            sizes,
        ));

        let swim_speed_edit = Box::new(ValueEditorMultiple::<f32, 3>::new(
            "Swim Speed".to_string(),
            inside_base,
            [
                vec![
                    0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x68, 0x68, 0x1C,
                ],
                vec![
                    0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x68, 0x68, 0x18,
                ],
                vec![
                    0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x68, 0x68, 0x20,
                ],
            ],
            sizes,
        ));

        let sub_speed_edit = Box::new(ValueEditor::<f32>::new(
            "Sub Speed".to_string(),
            inside_base,
            vec![0xF8D708, 0x118, 0xC0, 0x868, 0x158, 0x20, 0x100, 0x1EC],
            sizes,
        ));

        let breath_edit = Box::new(ValueEditor::<f32>::new(
            "Breath".to_string(),
            inside_base,
            vec![0xF92610, 0x18, 0xE0, 0x98, 0x508, 0x20, 0x28, 0x104],
            sizes,
        ));

        let save_state = Box::new(Savestate::new(
            inside_base,
            [
                vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10],
                vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x14],
                vec![0x1001FA0, 0x260, 0x2E8, 0x318, 0x10, 0x28, 0x40, 0x18],
            ],
        ));

        let checkpoint_loader = Box::new(CheckpointLoader::new(
            inside_base,
            vec![0xF92820, 0x8, 0x50, 0x0, 0x18, 0x18, 0x60, 0x58, 0x6C],
            vec![0xF92820, 0x8, 0x50, 0x0, 0x18, 0x18, 0x60, 0x58, 0x70],
            vec![0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x2B],
        ));

        let fall_damage_hack = Box::new(Checkbox::new(
            "Disable Fall Damage".to_string(),
            inside_base,
            vec![0xF8D5D0, 0x38, 0xDC8, 0x28, 0x0, 0x130, 0xB4, 0xC2C],
            vec![0xFF, 0x90, 0xC0, 0x00, 0x00, 0x00],
            vec![0x90; 6],
        ));

        let instant_boost_hack = Box::new(Checkbox::new(
            "Submersible Instant Boost".to_string(),
            inside_base,
            vec![0xF8D708, 0x118, 0xC0, 0x868, 0x158, 0x20, 0x100, 0x70, 0xB4],
            vec![0x0, 0x0, 0x0, 0x3F, 0x0, 0x0, 0x80, 0x40],
            vec![0x0; 8],
        ));

        let boy_size_edit = Box::new(ValueEditor::<f32>::new(
            "Boy Size".to_string(),
            inside_base,
            vec![
                0xF99BA8, 0xC8, 0x78, 0x188, 0x28, 0xD8, 0x20, 0x48, 0x20, 0x90,
            ],
            sizes,
        ));

        Self {
            widgets: vec![
                x_pos_edit,
                y_pos_edit,
                z_pos_edit,
                jump_strength_edit,
                run_speed_edit,
                swim_speed_edit,
                sub_speed_edit,
                breath_edit,
                boy_size_edit,
                save_state,
                checkpoint_loader,
                fall_damage_hack,
                instant_boost_hack,
            ],
            memhook,
        }
    }

    pub fn display(&mut self, ui: &mut Ui) {
        for widget in &mut self.widgets {
            widget.draw(ui, &self.memhook);
        }
    }

    pub fn close(&self) {
        self.widgets.iter().for_each(|f| f.close(&self.memhook));
    }
}

pub trait Widget {
    fn draw(&mut self, ui: &mut Ui, memhook: &MemHook);

    fn close(&self, memhook: &MemHook);
}
