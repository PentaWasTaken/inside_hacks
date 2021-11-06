use glium::Display;

use glutin::dpi::{PhysicalPosition, PhysicalSize};

use std::mem::{size_of, transmute};
use std::ffi::CString;
use std::ptr;

use windows::Win32::Foundation::{HWND, RECT, PSTR};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;

pub struct WindowManager {
    game_window: HWND,
}

impl WindowManager {
    pub fn new(win_name: &str) -> Option<Self> {
        let win_ptr = CString::new(win_name).unwrap();
        let win_name = PSTR(win_ptr.as_ptr() as *mut u8);

        let a = PSTR(ptr::null_mut());

        let hwnd = unsafe { FindWindowA(a, win_name)};

        if hwnd.0 != 0 {
            Some(Self { game_window: hwnd })
        } else {
            None
        }
    }

    pub fn update_window_sizepos(&self, display: &Display) {
        let win_rect = &mut [0u8; size_of::<RECT>()];

        unsafe {
            DwmGetWindowAttribute(
                self.game_window,
                DWMWINDOWATTRIBUTE(9),
                win_rect as *mut u8 as *mut _,
                size_of::<RECT>() as u32,
            )
            .unwrap();
        }
        let win_rect: RECT = unsafe { transmute(*win_rect) };

        let gl_window = display.gl_window();
        let window = gl_window.window();
        window.set_outer_position(PhysicalPosition::new(win_rect.left, win_rect.top));

        let width = win_rect.right - win_rect.left;
        let height = win_rect.bottom - win_rect.top;
        window.set_inner_size(PhysicalSize::new(width, height));
    }
}
