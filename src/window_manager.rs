use glium::Display;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use std::ffi::CString;
use std::mem::{size_of, transmute};
use std::ptr;

use windows::Win32::Foundation::{HWND, PSTR, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, GetForegroundWindow, GetSystemMetrics, SetForegroundWindow, SetWindowLongA,
    ShowWindow, SHOW_WINDOW_CMD, SYSTEM_METRICS_INDEX, WINDOW_LONG_PTR_INDEX,
};

use glutin::dpi::{PhysicalPosition, PhysicalSize};

use egui::Rect;

pub struct WindowManager {
    game_window: HWND,
    overlay_window: HWND,
    win_offset: (i32, i32),
}

impl WindowManager {
    pub fn new(win_name: &str, display: &Display) -> Option<Self> {
        let win_ptr = CString::new(win_name).unwrap();
        let win_name = PSTR(win_ptr.as_ptr() as *mut u8);

        let a = PSTR(ptr::null_mut());

        let game_window = unsafe { FindWindowA(a, win_name) };

        let overlay_window = display.gl_window().window().raw_window_handle();

        if let RawWindowHandle::Windows(overlay_window) = overlay_window {
            let overlay_window = HWND(overlay_window.hwnd as _);
            unsafe {
                SetWindowLongA(overlay_window, WINDOW_LONG_PTR_INDEX(-20), 0x80);
            }

            let x_offset = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(6)) };

            let y_offset = unsafe {
                GetSystemMetrics(SYSTEM_METRICS_INDEX(33))
                    + GetSystemMetrics(SYSTEM_METRICS_INDEX(92))
                    + GetSystemMetrics(SYSTEM_METRICS_INDEX(4))
            };
            if game_window.0 != 0 {
                unsafe {
                    SetForegroundWindow(game_window);
                }
                return Some(Self {
                    game_window,
                    overlay_window,
                    win_offset: (x_offset, y_offset),
                });
            }
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

        let gl_win = display.gl_window();
        let display_win = gl_win.window();
        display_win.set_outer_position(PhysicalPosition::new(
            win_rect.left + self.win_offset.0,
            win_rect.top + self.win_offset.1,
        ));
    }

    pub fn update_window_size(&self, display: &Display, rect: Rect) {
        let gl_win = display.gl_window();
        let display_win = gl_win.window();
        display_win.set_inner_size(PhysicalSize::new(rect.width(), rect.height()));
    }

    pub fn update_window_visibility(&self) {
        let foreground_window = unsafe { GetForegroundWindow() };
        if foreground_window == self.overlay_window || foreground_window == self.game_window || foreground_window == HWND(0) {
            unsafe {
                ShowWindow(self.overlay_window, SHOW_WINDOW_CMD(5));
            }
        } else {
            unsafe {    
                ShowWindow(self.overlay_window, SHOW_WINDOW_CMD(0));
            }
        }
    }
}
