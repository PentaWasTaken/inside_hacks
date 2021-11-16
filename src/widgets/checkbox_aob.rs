use memhack::scan::aob_scan_first;
use memhack::MemHook;

use crate::widgets::Widget;

pub struct CheckboxAOB {
    name: String,
    aob: Vec<u8>,
    search_range: (Option<usize>, Option<usize>),
    activated_val: Vec<u8>,
    deactivated_val: Vec<u8>,
    activated: bool,
}

impl CheckboxAOB {
    pub fn new(
        name: String,
        aob: Vec<u8>,
        search_range: (Option<usize>, Option<usize>),
        activated_val: Vec<u8>,
        deactivated_val: Vec<u8>,
    ) -> Self {
        Self {
            name,
            aob,
            search_range,
            activated_val,
            deactivated_val,
            activated: false,
        }
    }

    fn activate(&mut self, memhook: &MemHook) {
        let address = aob_scan_first(memhook, &self.aob, self.search_range.0, self.search_range.1);

        if let Some(address) = address {
            println!("Activate");
            let _ = memhook.write_bytes(address, &self.activated_val);
        }
    }

    fn deactivate(&mut self, memhook: &MemHook) {
        let t_aob = [
            self.activated_val.clone(),
            self.aob[self.activated_val.len()..].to_vec(),
        ]
        .concat();
        let address = aob_scan_first(memhook, &t_aob, self.search_range.0, self.search_range.1);

        if let Some(address) = address {
            println!("Deactivate");
            let _ = memhook.write_bytes(address, &self.deactivated_val);
        }
    }
}

impl Widget for CheckboxAOB {
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
