use egui_glium::EguiGlium;
use glium::{glutin, Display, Surface};
use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};

use crate::window_manager::WindowManager;

use egui::{Align, Layout};

pub struct Interface {
    display: Display,
    egui_glium: EguiGlium,
    window_manager: WindowManager,
}

impl Interface {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_resizable(false)
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top(true);

        let context_builder = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true);

        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
        let egui_glium = EguiGlium::new(&display);
        let window_manager = WindowManager::new("INSIDE", &display).unwrap();

        Self {
            display,
            egui_glium,
            window_manager,
        }
    }

    fn redraw(&mut self, control_flow: &mut ControlFlow) {
        self.egui_glium.begin_frame(&self.display);

        let frame = egui::containers::Frame {
            margin: egui::vec2(5.0, 5.0),
            corner_radius: 5.0,
            shadow: egui::epaint::Shadow {
                extrusion: 0.0,
                ..Default::default()
            },
            fill: egui::Color32::DARK_GRAY,
            stroke: egui::Stroke::new(2.0, egui::Color32::BLACK),
        };

        let mut open = true;
        egui::Window::new("test")
            .frame(frame)
            .resizable(false)
            .fixed_pos((0.0, 0.0))
            .fixed_size((100.0, 50.0))
            .open(&mut open)
            .show(self.egui_glium.ctx(), |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    if ui.button("abc").clicked() {
                        println!("Yo");
                    };
                });
                self.window_manager
                    .update_window_size(&self.display, ui.ctx().used_rect());
            });

        let (needs_repaint, shapes) = self.egui_glium.end_frame(&self.display);

        if !open {
            *control_flow = ControlFlow::Exit;
        } else if needs_repaint {
            self.display.gl_window().window().request_redraw();
            *control_flow = ControlFlow::Poll;
        } else {
            *control_flow = ControlFlow::Wait;
        }

        let mut target = self.display.draw();

        let color = egui::Rgba::TRANSPARENT;
        target.clear_color_srgb(color[0], color[1], color[2], color[3]);

        self.egui_glium.paint(&self.display, &mut target, shapes);

        target.finish().unwrap();
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, mut control_flow| match event {
            Event::RedrawEventsCleared => {
                self.window_manager.update_window_pos(&self.display);
                self.window_manager.update_window_visibility();
                self.redraw(&mut control_flow)
            }

            Event::WindowEvent { event, .. } => {
                if self.egui_glium.is_quit_event(&event) {
                    *control_flow = ControlFlow::Exit;
                }

                self.egui_glium.on_event(&event);

                self.display.gl_window().window().request_redraw();
            }
            _ => (),
        });
    }
}
