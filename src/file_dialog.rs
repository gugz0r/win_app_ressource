use std::ffi::OsString;
use std::path::PathBuf;

use std::fs;
use std::os::windows::ffi::OsStringExt;
use winapi::um::commdlg::{GetOpenFileNameW, OFN_FILEMUSTEXIST, OPENFILENAMEW};

use windows::Win32::UI::WindowsAndMessaging::SendMessageW;

use windows::Win32::UI::Controls::{TCIF_TEXT, TCITEMW, TCM_INSERTITEMW};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, GetDlgItem, MessageBoxW, MB_OK, WINDOW_EX_STYLE, WS_CHILD, WS_GROUP,
    WS_TABSTOP, WS_VISIBLE,
};

use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};

fn show_message_box(hwnd: HWND, text: &str, caption: &str) {
    // Convert the strings to wide strings
    let wide_text: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
    let wide_caption: Vec<u16> = caption.encode_utf16().chain(Some(0)).collect();

    // Call MessageBoxW to display the message
    unsafe {
        MessageBoxW(
            hwnd,
            PCWSTR(wide_text.as_ptr()),
            PCWSTR(wide_caption.as_ptr()),
            MB_OK,
        );
    }
}

pub fn open_file_dialog(hwnd: HWND) -> Option<PathBuf> {
    println!("Open file dialog invoked");

    let mut filename: [u16; 260] = [0; 260];

    let filter = "Text Files\0*.txt\0All Files\0*.*\0\0";
    let mut ofn: OPENFILENAMEW = OPENFILENAMEW {
        lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
        hwndOwner: hwnd.0 as *mut _,
        lpstrFilter: filter.encode_utf16().collect::<Vec<u16>>().as_ptr(),
        lpstrFile: filename.as_mut_ptr(),
        nMaxFile: filename.len() as u32,
        Flags: OFN_FILEMUSTEXIST,
        ..unsafe { std::mem::zeroed() }
    };

    if unsafe { GetOpenFileNameW(&mut ofn) } == 0 {
        println!("No file selected or an error occurred");
        return None;
    }

    println!("File selected");

    let wide_filename = &filename;
    let first_null = wide_filename
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(wide_filename.len());
    let os_string = OsString::from_wide(&wide_filename[..first_null]);
    let path = PathBuf::from(os_string);
    Some(path)
}

pub fn display_file_content(hwnd: HWND, path: &PathBuf) {
    println!("Starting display_file_content function.");
    println!("Parent window handle (HWND): {:?}", hwnd);
    println!("Path to display: {:?}", path);

    // Step 1: Read the content of the file
    match fs::read_to_string(path) {
        Ok(file_content) => {
            println!("File content read successfully.");

            // Step 2: Retrieve the handle of the dialog with ID 700
            println!("Attempting to retrieve dialog with ID 700.");
            let hwnd_dialog_700 = unsafe {
                GetDlgItem(hwnd, 700)
                    .map(|h| {
                        println!("Handle for DialogBox ID 700: {:?}", h);
                        h
                    })
                    .expect("Failed to get dialog with ID 700")
            };

            // Step 3: Retrieve the handle of the tab control with ID 1004
            println!("Attempting to retrieve tab control with ID 1004.");
            let hwnd_tab_1004 = unsafe {
                GetDlgItem(hwnd_dialog_700, 1004)
                    .map(|h| {
                        println!("Handle for TabControl ID 1004: {:?}", h);
                        h
                    })
                    .expect("Failed to get tab control with ID 1004")
            };

            // Step 4: Insert a new tab item
            let tab_text = "File Content";
            let wide_tab_text: Vec<u16> = tab_text.encode_utf16().chain(Some(0)).collect();

            let tc_item = TCITEMW {
                mask: TCIF_TEXT,
                pszText: PWSTR(wide_tab_text.as_ptr() as *mut u16),
                ..Default::default()
            };

            unsafe {
                SendMessageW(
                    hwnd_tab_1004,
                    TCM_INSERTITEMW,
                    WPARAM(0),
                    LPARAM(&tc_item as *const _ as isize),
                );
            }

            // Step 5: Convert the file content to wide string (UTF-16)
            println!("Converting file content to UTF-16.");
            let wide_content: Vec<u16> = file_content.encode_utf16().chain(Some(0)).collect();
            let wide_content_p = PCWSTR(wide_content.as_ptr());

            // Define the class name
            let class_name: Vec<u16> = "STATIC".encode_utf16().chain(Some(0)).collect();

            // Step 6: Create a child control (static text) to display the content within the tab
            println!("Creating a static control to display the content within the tab.");
            let hwnd_static = unsafe {
                CreateWindowExW(
                    WINDOW_EX_STYLE(0),
                    PCWSTR(class_name.as_ptr()),
                    wide_content_p,
                    WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_GROUP,
                    10,
                    10,
                    300,
                    200,
                    hwnd_tab_1004,
                    None,
                    None,
                    Some(std::ptr::null_mut() as *const std::ffi::c_void),
                )
                .expect("Failed to create static control for tab content")
            };

            if hwnd_static.is_invalid() {
                eprintln!("Failed to create static control for tab content");
                show_message_box(
                    hwnd,
                    "Failed to create static control for tab content",
                    "Error",
                );
            } else {
                println!("File content loaded and displayed successfully.");
            }
        }
        Err(e) => {
            eprintln!("Failed to read file: {:?}", e);
            show_message_box(hwnd, "Failed to read the file", "Error");
        }
    }

    println!("Finished display_file_content function.");
}
