use crate::window::create_main_window;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG,
};
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_SHOW};

pub fn run() {
    unsafe {
        let h_instance = GetModuleHandleW(None).unwrap();

        let hwnd = create_main_window(h_instance.into()); // Convert HMODULE to HINSTANCE

        ShowWindow(hwnd, SW_SHOW);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, hwnd, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
