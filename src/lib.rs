#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
use std::{fmt::Error, os::windows, thread::sleep};

#[napi]
unsafe fn send_right_click_at(x: i32, y: i32) {
  send_click_at(x, y, WM_MBUTTONDOWN, WM_LBUTTONUP);
}
#[napi]
unsafe fn send_left_click_at(x: i32, y: i32) {
  send_click_at(x, y, WM_LBUTTONDOWN, WM_LBUTTONUP);
}
#[napi]
unsafe fn send_middle_click_at(x: i32, y: i32) {
  send_click_at(x, y, WM_MBUTTONDOWN, WM_LBUTTONUP);
}


use ::windows::{
    Foundation::Point,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::{
            Input::KeyboardAndMouse::{
                EnableWindow, GetCapture, GetFocus, IsWindowEnabled, SetActiveWindow, SetCapture,
                SetFocus,
            },
            WindowsAndMessaging::{
                ChildWindowFromPoint, ChildWindowFromPointEx, GetCursorPos, GetParent,
                GetWindowInfo, GetWindowTextA, PostMessageA, SendMessageA, WindowFromPoint,
                CWP_SKIPDISABLED, CWP_SKIPINVISIBLE, WINDOWINFO, WM_LBUTTONDOWN, WM_LBUTTONUP,
                WM_MBUTTONDOWN, WM_MOUSEACTIVATE, WM_NCHITTEST, WM_PAINT, WM_PARENTNOTIFY,
            },
        },
    },
    UI::Input,
};

unsafe fn get_window_at(point: ::windows::Win32::Foundation::POINT) -> Result<HWND, Error> {
  
        let win = WindowFromPoint(point);
        Ok(win)
    
}
unsafe fn get_relative_point(
    win: HWND,
    point: ::windows::Win32::Foundation::POINT,
) -> ::windows::Win32::Foundation::POINT {
    
        let mut info = WINDOWINFO::default();
        GetWindowInfo(win, &mut info).unwrap();
        let p2 = ::windows::Win32::Foundation::POINT {
            x: point.x - info.rcClient.left,
            y: point.y - info.rcClient.top,
        };
        return p2;
    
}
unsafe fn get_child_window_at_abs_point(parent: HWND, point: ::windows::Win32::Foundation::POINT) -> HWND {
   
        let mut info = WINDOWINFO::default();
        GetWindowInfo(parent, &mut info).unwrap();
        let p2 = get_relative_point(parent, point);

        let win = ChildWindowFromPoint(parent, p2);
        return win;
    
}
unsafe fn get_top_parent(win: HWND) -> HWND {
   
        let mut parent = win;
        loop {
            let p = GetParent(parent);
            print!("Parent: {:?}", p);
            if p.is_err() {
                break;
            }
            parent = p.unwrap();
        }
        return parent;
    
}
unsafe fn print_window_text(win: HWND) {
    let mut lpstring = [0u8; 1024];
    GetWindowTextA(win, &mut lpstring);

    println!(
        "Window Text: {:?}",
        String::from_utf8_lossy(&lpstring).replace("\0", "")
    );
}
unsafe fn print_window_info(win: HWND) {
    let mut info = WINDOWINFO::default();
    GetWindowInfo(win, &mut info).unwrap();
    println!("Window Info: {:?}", info);
}

unsafe fn send_click_at(x: i32, y: i32, button_down: u32, button_up: u32) {
    let point = ::windows::Win32::Foundation::POINT { x, y };
    let win = get_window_at(point).unwrap();
    send_click(win, point , button_down, button_up);
}


unsafe fn send_click(win: HWND, point: ::windows::Win32::Foundation::POINT, button_down: u32, button_up: u32) {
    let p = get_relative_point(win, point);
    let lparam = LPARAM((p.y as isize) << 16 | (p.x as isize) & 0xFFFF);
    let top_parent = get_top_parent(win);
    println!("Top Parent: {:?}", top_parent);
    //notify parent

    let hittest = SendMessageA(
        win,
        WM_NCHITTEST,
        None,
        LPARAM((point.y as isize) << 16 | (point.x as isize) & 0xFFFF),
    );

    println!("Hit Test: {:?}", hittest);

    let activate = SendMessageA(
        win,
        WM_MOUSEACTIVATE,
        WPARAM(top_parent.0 as usize),
        LPARAM(hittest.0 as isize | (button_down as isize) << 16),
    );
    println!("Activate: {:?}", activate);
    PostMessageA(win, button_down, WPARAM(1), lparam).unwrap();
    PostMessageA(win, button_up, None, lparam).unwrap();

    // notify parent
    let parent_notify = SendMessageA(
        top_parent,
        WM_PARENTNOTIFY,
        WPARAM(button_down as usize),
        LPARAM(win.0 as isize),
    );
    println!("Parent Notify: {:?}", parent_notify);


}
