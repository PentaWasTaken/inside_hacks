use glium::Display;

use std::ffi::CString;
use std::mem::{size_of, transmute};
use std::ptr;

use windows::Win32::Foundation::{HWND, PSTR, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetSystemMetrics, SYSTEM_METRICS_INDEX};

use glutin::dpi::{PhysicalPosition, PhysicalSize};

use egui::Rect;

pub struct WindowManager {
    game_window: HWND,
}

impl WindowManager {
    pub fn new(win_name: &str) -> Option<Self> {
        let win_ptr = CString::new(win_name).unwrap();
        let win_name = PSTR(win_ptr.as_ptr() as *mut u8);

        let a = PSTR(ptr::null_mut());

        let game_window = unsafe { FindWindowA(a, win_name) };

        if game_window.0 != 0 {
            return Some(Self { game_window });
        }
        None
    }

    pub fn update_window_pos(&self, display: &Display) {
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

        let x_offset = unsafe {
            GetSystemMetrics(SYSTEM_METRICS_INDEX(6))
        };

        let y_offset = unsafe {
            GetSystemMetrics(SYSTEM_METRICS_INDEX(33))
            + GetSystemMetrics(SYSTEM_METRICS_INDEX(92))
            + GetSystemMetrics(SYSTEM_METRICS_INDEX(4))
        };

        let gl_win = display.gl_window();
        let display_win = gl_win.window();
        display_win.set_outer_position(PhysicalPosition::new(win_rect.left + x_offset, win_rect.top + y_offset));
    }

    pub fn update_window_size(&self, display: &Display, rect: Rect) {
        let gl_win = display.gl_window();
        let display_win = gl_win.window();
        display_win.set_inner_size(PhysicalSize::new(rect.width(), rect.height()));
    }
}
