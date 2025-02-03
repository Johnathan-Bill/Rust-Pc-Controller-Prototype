
mod window_controller
{
    use windows::Win32::{
        Foundation::{HWND,LPARAM,BOOL}, UI::WindowsAndMessaging::EnumWindows,UI::WindowsAndMessaging::{ GetWindowTextW,IsWindowVisible,
IsIconic, ShowWindow, SetForegroundWindow, SW_RESTORE},
    };

    struct Window
    {
        name : String,
        hwnd : HWND
    }
    pub struct WindowController
    {
        os: String,
        open_windows: Vec<Window>
    }

    impl  WindowController
    {

        

    }
}
pub use window_controller::WindowController;