use windows::{Win32::{
    Foundation::{HWND, LPARAM},UI::WindowsAndMessaging::{EnumWindows, GetWindowTextA}
}, core::BOOL};
fn main()-> windows::core::Result<()> {
    unsafe {
        EnumWindows(Some(enum_window), LPARAM(0))
    }
}

extern "system" fn enum_window(hwnd:HWND,parm:LPARAM)-> BOOL{
    let mut buf = vec![0u8;256]; 
    let written_len = unsafe { GetWindowTextA(hwnd,&mut buf) };
    buf.truncate(written_len as usize);
    println!("{:?}:{:?}", hwnd,String::from_utf8_lossy(&buf));
    BOOL(1)
}