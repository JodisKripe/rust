/// Foreign Function Interface
/// It's the means by which Rust can access functionality within external C libraries.
/// The most common use case is to access the native Windows APIs.
/// 
/// This code can only run properly on windows environment. 
use std::ptr::null_mut;
use libc::c_void; // cargo add libc

extern "C"{
    fn abs(input: i32) -> i32;
}

#[link(name = "kernel32")]
extern{
    fn GetLastError() -> i32;
    fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: bool, dwProcessId: u32) -> *mut c_void;
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void; // LPCWSTR --> pointer to the null-terminated 16-bit unicode string, in terms of rust is *const u16
}

fn main() {
    let some:i32 = -27;
    let handle = unsafe {
        abs(some)
    };

    dbg!(handle);
    // if handle == null_mut() {
    //     println!("OpenProcess failed:");
    // } else {
    //     println!("hProcess: {:x?}", handle);
    // }
    let handle = unsafe{
        OpenProcess(0xF01FF, false, 26798)
    };

    if handle == null_mut() {
        println!("OpenProcess Failed: {}", unsafe{GetLastError()});
    } else{
        println!("hProcess: {:x?}", handle);
    }
    let result = Open_process(0xF01FF, false, 26789);
    match result {
        Ok(h) => println!("hProcess {:x?}",handle),
        Err(e) => println!("Error: {}",e)
    }
}

fn open_process(desired_access: u32, inherit_handle: bool, process_id: u32) -> Result<*mut c_void, u32> {
    let handle = unsafe{
        OpenProcess(desired_access,inherit_handle,process_id)
    };
    if handle == null_mut() {
        Err( unsafe { GetLastError() as u32 } )
    } else {
        Ok(handle)
    }
    // example of a safe wrapper around the unsafe operations of OpenProcess and GetLastError
}

fn load_library(file_name: &str) -> Result<*mut c_void, u32> {
    let mut name = file_name.encode_utf16().collect::<Vec<u16>>();
    if *name.last().unwrap() != 0{
        name.push(0);
    }

    let handle = unsafe{ LoadLibraryW(name.as_ptr())};

    if handle == null_mut(){
        Err(unsafe {GetLastError() as u32})
    } else {
        Ok(handle)
    }
}


type Handle = *mut c_void;
type Win32Error = u32;
type ProcessAccessRights = u32;

fn Open_process(desired_access: ProcessAccessRights, inherit_handle: bool, process_id: u32) -> Result<Handle,Win32Error>{
    let handle = unsafe{
        OpenProcess(desired_access, inherit_handle, process_id)
    };

    if handle == null_mut() {
        Err(unsafe {
            GetLastError() as u32
        })
    } else{
        Ok(handle)
    }
} // This safe wrapper uses the alloted type definitions for ProcessAccessRights and Win32Error which will be much more better for the quality of life
// 
// The real benefits come when calling the function.  Having the ProcessAccessRights constants saves us from having to remember or look up the correct values all the time; 
// and having the defined data types (Handle and Win32Error) makes the annotations by the IDE more meaningful.  
// At a glance we can see the result we're getting is either a Handle or Win32Error, rather than generic *mut c_void and u32 types.