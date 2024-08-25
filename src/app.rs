use crate::window::{create_main_window, display_main_dialog};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, ShowWindow, TranslateMessage, MSG, SW_SHOW,
};

pub fn run() {
    unsafe {
        // Get the instance handle of the application
        let h_instance = GetModuleHandleW(None).unwrap();

        // Create the main window
        let hwnd = create_main_window(h_instance.into());

        // Show the window
        let _ = ShowWindow(hwnd, SW_SHOW);

        // Display the main dialog within the main window (if needed)
        display_main_dialog(hwnd, h_instance.into());

        // Main message loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
