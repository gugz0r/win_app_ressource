use windows::Win32::Foundation::{HWND, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, SetWindowPos, HWND_TOP, SWP_NOACTIVATE, SWP_NOZORDER, WS_CHILD, WS_VISIBLE, WINDOW_EX_STYLE, HMENU};
use windows::core::PCWSTR; // Corrected import

pub unsafe fn create_close_button(hwnd: HWND, h_instance: HINSTANCE) {
    // Create the "Close" button within the main window
    let button_class_name = PCWSTR("BUTTON\0".encode_utf16().collect::<Vec<u16>>().as_ptr());
    let button_text = PCWSTR("Close\0".encode_utf16().collect::<Vec<u16>>().as_ptr());

    // Combine the styles that are compatible with WINDOW_STYLE
    let style = WS_CHILD | WS_VISIBLE;

    let hwnd_button = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        button_class_name,
        button_text,
        style, // Use the combined style
        850,   // x position
        700,   // y position
        100,   // width
        30,    // height
        hwnd,
        HMENU(1001 as _), // Use null_mut() or cast to HMENU
        h_instance,
        None,
    )
    .expect("Failed to create 'Close' button");

    // Set the button position within the parent window
    SetWindowPos(
        hwnd_button,
        HWND_TOP,
        850,  // x position
        700,  // y position
        100,  // width
        30,   // height
        SWP_NOZORDER | SWP_NOACTIVATE,
    )
    .expect("Failed to position 'Close' button");
}
