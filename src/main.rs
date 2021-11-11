mod interface;
use interface::Interface;

mod window_manager;

mod widgets;
use widgets::Widgets;

use glium::glutin::event_loop::EventLoop;

use windows::Win32::System::Console::GetConsoleWindow;
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SHOW_WINDOW_CMD};

fn main() {
    println!("Loading...");
    let widgets = Widgets::new();

    //Hide console
    unsafe {
        let console_window = GetConsoleWindow();
        ShowWindow(console_window, SHOW_WINDOW_CMD(0));
    }
    let event_loop = EventLoop::with_user_event();
    let interface = Interface::new(&event_loop, widgets);

    interface.run(event_loop);
}
