extern crate winapi;

use winapi::um::winuser::{
    CreateDialogParamW, SetWindowPos, HWND_TOP, MAKEINTRESOURCEW, SWP_NOACTIVATE, SWP_NOZORDER,
    SWP_SHOWWINDOW,
};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{COLOR_WINDOW, HBRUSH};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, LoadCursorW, LoadMenuW, RegisterClassW, IDC_ARROW, WM_COMMAND,
    WNDCLASSW, WS_CHILD, WS_CLIPCHILDREN, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            match wparam.0 as u16 {
                666 => {
                    unsafe {
                        windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
                    } // Quit the application
                }
                _ => {}
            }
        }
        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

pub unsafe fn create_main_window(h_instance: HINSTANCE) -> HWND {
    let class_name = "MainAppClass\0".encode_utf16().collect::<Vec<u16>>();

    let wnd_class = WNDCLASSW {
        hInstance: h_instance,
        lpszClassName: PCWSTR(class_name.as_ptr()),
        lpfnWndProc: Some(window_proc),
        hbrBackground: HBRUSH(COLOR_WINDOW.0 as *mut _),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        ..Default::default()
    };

    RegisterClassW(&wnd_class);

    let hwnd = CreateWindowExW(
        windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
        PCWSTR(class_name.as_ptr()),
        PCWSTR(
            "My Rust Windows App\0"
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
        ),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPCHILDREN, // Make sure the window clips children to avoid overlapping
        100,
        100,
        1024,
        768,
        None,
        LoadMenuW(h_instance, PCWSTR(MAKEINTRESOURCEW(100) as *const u16)).unwrap(),
        h_instance,
        Some(std::ptr::null_mut()),
    )
    .unwrap();

    // Create the About dialog as a child window within the main window
    let about_hwnd = CreateDialogParamW(
        h_instance.0 as *mut _, // Convert HINSTANCE to raw pointer
        MAKEINTRESOURCEW(700),
        hwnd.0 as *mut _, // Set hwnd as the parent of the dialog
        None,
        0,
    );

    if about_hwnd.is_null() {
        println!("Failed to create About dialog");
    } else {
        println!("Successfully created About dialog");
        // Manually set the size and position of the dialog
        SetWindowPos(
            about_hwnd as *mut _,
            HWND_TOP,
            0,
            0,
            1024, // Width matching the main window
            768,  // Height matching the main window
            SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
    }

    hwnd
}
