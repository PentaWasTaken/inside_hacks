mod interface;
use interface::Interface;

mod window_manager;

use glium::glutin::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::with_user_event();
    let interface = Interface::new(&event_loop);

    interface.run(event_loop);
}
