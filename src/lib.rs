#![no_std]

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;
            // Call it
            f()
        }      
    };
}

// The reset handler

use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    
    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }
    
    let sbss = &_sbss as *const u8;
    let ebss = &_ebss as *const u8;
    let sdata = &_sdata as *const u8;
    let edata = &_edata as *const u8;
    let sidata = &_sidata as *const u8;
    
    let bss_count = ebss as usize - sbss as usize;
    let data_count = edata as usize - sdata as usize;
   
    // Zero out .bss
    ptr::write_bytes(sbss as *mut u8, 0, bss_count);
    // Copy .data
    ptr::copy_nonoverlapping(sidata, sdata as *mut u8, data_count);
    
    extern "Rust" {
        fn main() -> !;
    }
    main()
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
