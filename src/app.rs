use windows::{
    Win32::{UI::WindowsAndMessaging::*, Foundation::GetHandleInformation}
};

struct App {

}

impl App {
    fn init() {
        let className = "main_window";
        let windowName = "Asteriods MF";
        unsafe {
            let inst = GetModuleHandle();

            CreateWindowExA(
                WS_EX_OVERLAPPEDWINDOW, className, windowName, WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, std::ptr::null(),
                std::ptr::null(), hinstance, std::ptr::null());
        }
    }
}
