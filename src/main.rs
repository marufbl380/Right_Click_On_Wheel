use std::panic;
use std::sync::Once;
use windows::core::Result;
use windows::Win32::Foundation::{LRESULT, WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, CallNextHookEx, UnhookWindowsHookEx,
    GetMessageW, TranslateMessage, DispatchMessageW,
    WH_MOUSE_LL, MSG, PostQuitMessage, MSLLHOOKSTRUCT,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, MOUSEINPUT, INPUT_MOUSE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    SendInput, GetAsyncKeyState,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const VK_SHIFT: i32 = 0x10;
const VK_CTRL: i32 = 0x11;
const VK_Q: i32 = 0x51;
static CLEANUP: Once = Once::new();
static IS_PROCESSING: AtomicBool = AtomicBool::new(false);
static mut LAST_EVENT_TIME: Option<Instant> = None;
const MIN_EVENT_INTERVAL: Duration = Duration::from_millis(50);

unsafe extern "system" fn low_level_mouse_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    const WM_MOUSEWHEEL: u32 = 0x020A;
    const WM_MOUSEHWHEEL: u32 = 0x020E;
    const EXTRA_INFO_MARKER: usize = 0xDEADBEEF;

    let hook_struct = &*(lparam.0 as *const MSLLHOOKSTRUCT);

    if hook_struct.dwExtraInfo == EXTRA_INFO_MARKER {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    if wparam.0 as u32 == WM_MOUSEWHEEL || wparam.0 as u32 == WM_MOUSEHWHEEL {
        if IS_PROCESSING.load(Ordering::SeqCst) {
            return LRESULT(1);
        }

        let current_time = Instant::now();
        if let Some(last_time) = LAST_EVENT_TIME {
            if current_time.duration_since(last_time) < MIN_EVENT_INTERVAL {
                return LRESULT(1);
            }
        }
        LAST_EVENT_TIME = Some(current_time);

        IS_PROCESSING.store(true, Ordering::SeqCst);

        let inputs = [
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 { mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_RIGHTDOWN,
                    time: 0,
                    dwExtraInfo: EXTRA_INFO_MARKER,
                }},
            },
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 { mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_RIGHTUP,
                    time: 0,
                    dwExtraInfo: EXTRA_INFO_MARKER,
                }},
            },
        ];

        let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        if sent != inputs.len() as u32 {
            IS_PROCESSING.store(false, Ordering::SeqCst);
            return CallNextHookEx(None, code, wparam, lparam);
        }

        IS_PROCESSING.store(false, Ordering::SeqCst);
        
        return LRESULT(1);
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn main() -> Result<()> {
    panic::set_hook(Box::new(|_| {
        unsafe {
            CLEANUP.call_once(|| {
                println!("Emergency cleanup initiated");
                IS_PROCESSING.store(false, Ordering::SeqCst);
            });
        }
    }));

    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), Some(hinstance.into()), 0)?;
        
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if (GetAsyncKeyState(VK_CTRL) as i16) < 0 
               && (GetAsyncKeyState(VK_SHIFT) as i16) < 0 
               && (GetAsyncKeyState(VK_Q) as i16) < 0 {
                println!("Force terminating process...");
                let _ = UnhookWindowsHookEx(hook);
                std::process::Command::new("taskkill")
                    .args(["/F", "/PID", &std::process::id().to_string()])
                    .spawn()
                    .ok();
                std::process::exit(0);
            }
        }

        let _ = UnhookWindowsHookEx(hook)?;
    }
    Ok(())
}