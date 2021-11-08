use egui_glium::EguiGlium;
use glium::{glutin, Display, Surface};
use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};

use crate::window_manager::WindowManager;

use egui::{Layout, Align};

pub struct Interface {
    display: Display,
    event_loop: EventLoop<()>,
}

impl Interface {
    pub fn new(event_loop: EventLoop<()>) -> Self {
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

        Self {
            display,
            event_loop,
        }
    }

    fn redraw(display: &Display, egui_glium: &mut EguiGlium, control_flow: &mut ControlFlow, window_manager: &WindowManager) {
        egui_glium.begin_frame(display);

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

        egui::Window::new("test")
            .frame(frame)
            .resizable(false)
            .fixed_pos((0.0, 0.0))
            .fixed_size((100.0, 50.0))
            .show(egui_glium.ctx(), |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    if ui.button("abc").clicked() {
                        println!("Yo");
                    };
                });
                window_manager.update_window_size(&display, ui.ctx().used_rect());
            });
            
        let (needs_repaint, shapes) = egui_glium.end_frame(display);

        if needs_repaint {
            display.gl_window().window().request_redraw();
            *control_flow = ControlFlow::Poll;
        } else {
            *control_flow = ControlFlow::Wait;
        }

        let mut target = display.draw();

        let color = egui::Rgba::TRANSPARENT;
        target.clear_color_srgb(color[0], color[1], color[2], color[3]);

        egui_glium.paint(&display, &mut target, shapes);

        target.finish().unwrap();
    }

    pub fn run(self) {
        let mut egui_glium = EguiGlium::new(&self.display);

        let window_manager = WindowManager::new("INSIDE").unwrap();

        self.event_loop.run(move |event, _, mut control_flow| {
            match event {
                Event::RedrawEventsCleared => {
                    window_manager.update_window_pos(&self.display);
                    Interface::redraw(&self.display, &mut egui_glium, &mut control_flow, &window_manager);
                }

                Event::WindowEvent { event, .. } => {
                    if egui_glium.is_quit_event(&event) {
                        *control_flow = ControlFlow::Exit;
                    }

                    egui_glium.on_event(&event);

                    self.display.gl_window().window().request_redraw();
                }
                _ => (),
            }
        });
    }
}
