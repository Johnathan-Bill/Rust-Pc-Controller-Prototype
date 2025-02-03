
    use windows::Win32::{
        Foundation::{HWND,LPARAM,BOOL}, UI::WindowsAndMessaging::EnumWindows,UI::WindowsAndMessaging::{ GetWindowTextW,IsWindowVisible,
IsIconic, ShowWindow, SetForegroundWindow, SW_RESTORE},
    };
    #[derive(Clone)]
    pub struct Window
    {
        pub title : String,
        hwnd : HWND
    }
    #[derive(Clone)]
    pub struct WindowController
    {
        pub os: String,
        pub open_windows: Vec<Window>
    }
    unsafe impl Send for WindowController {}
    impl  WindowController
    {

        pub fn new(os : String) -> WindowController
        {
            WindowController {os : os,open_windows : vec![]}
        }

     pub fn get_open_windows(&mut self) -> Vec<Window>
    {
        let mut windows : Vec<Window> = vec![];
        unsafe 
        {

            let _ = EnumWindows(Some(Self::get_window_names_proc), LPARAM(&mut windows as *mut Vec<Window> as isize));

            self.open_windows = windows.clone();

            windows
        }
    }

	pub fn set_active_window(&mut self, i : i32)
	{
		let index = i as usize;
		unsafe 
		{
			if IsIconic(self.open_windows[index].hwnd).as_bool()
			{
				let _  =  ShowWindow(self.open_windows[index].hwnd, SW_RESTORE);
			}
			let _ = SetForegroundWindow(self.open_windows[index].hwnd);
		}    
	}

    unsafe extern "system" fn get_window_names_proc(hwnd : HWND, lparam : LPARAM) -> BOOL
    {

        if IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool()
        {
            let windows: &mut Vec<Window> = &mut *(lparam.0 as *mut Vec<Window>);
            let mut text = [0u16; 256];
            let length = GetWindowTextW(hwnd, &mut text);
            let window_title = String::from_utf16_lossy(&text[..length as usize]);

            if !window_title.is_empty() && !Self::in_ignore_list(&window_title){

                windows.push(Window {title : window_title , hwnd});
                }
        }

      BOOL(1)
    }

    fn in_ignore_list(title : &str) -> bool
    {
    
    let ignore_list : Vec<String> = vec!["Settings".to_string()
    ,"Windows Input Experience".to_string(),
    "Program Manager".to_string(),
    "DWM Notification Window".to_string()];
    
    ignore_list.contains(&title.to_string())
    }

    }