mod valueeditor;
use valueeditor::ValueEditor;

mod checkbox_aob;
use checkbox_aob::CheckboxAOB;

use egui::Ui;

use memhack::MemHook;

pub struct Widgets {
    widgets: Vec<Box<dyn Widget>>,
    memhook: MemHook,
}

const FALL_DAMAGE_BYTES: &[u8] = &[
    0x55, 0x48, 0x8B, 0xEC, 0x57, 0x48, 0x83, 0xEC, 0x08, 0x48, 0x8B, 0xF9, 0x48, 0x89, 0x55, 0xF0,
    0x0F, 0xB6, 0x47, 0x28, 0x85, 0xC0, 0x0F, 0x84, 0x3F, 0x00, 0x00, 0x00, 0x48, 0x8B, 0xCF, 0x48,
    0x83, 0xEC, 0x20,
];

impl Widgets {
    pub fn new() -> Self {
        let memhook = MemHook::from_process("INSIDE.exe").unwrap();

        let inside_base = memhook.get_module_base_address("INSIDE.exe").unwrap();
        let mono_base = memhook.get_module_base_address("mono.dll").unwrap();

        let pos_sizes = (90.0, 60.0, 100.0);

        let x_pos_edit = Box::new(ValueEditor::<f32>::new(
            "X-Position".to_string(),
            inside_base,
            vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10],
            pos_sizes,
        ));

        let y_pos_edit = Box::new(ValueEditor::<f32>::new(
            "Y-Position".to_string(),
            inside_base,
            vec![0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x14],
            pos_sizes,
        ));

        let z_pos_edit = Box::new(ValueEditor::<f32>::new(
            "Z-Position".to_string(),
            inside_base,
            vec![0x1001FA0, 0x260, 0x2E8, 0x318, 0x10, 0x28, 0x40, 0x18],
            pos_sizes,
        ));

        let jump_strength_edit = Box::new(ValueEditor::<f32>::new(
            "Jump Strength".to_string(),
            mono_base,
            vec![
                0x294BA8, 0x20, 0x4F8, 0x0, 0x78, 0x88, 0x68, 0x30, 0x18, 0x14,
            ],
            pos_sizes,
        ));

        let fall_damage_hack = Box::new(CheckboxAOB::new(
            "Disable Fall Damage".to_string(),
            FALL_DAMAGE_BYTES.to_vec(),
            (Some(0x2000000), None),
            vec![0xC3],
            vec![0x55],
        ));

        Self {
            widgets: vec![
                x_pos_edit,
                y_pos_edit,
                z_pos_edit,
                jump_strength_edit,
                fall_damage_hack,
            ],
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
