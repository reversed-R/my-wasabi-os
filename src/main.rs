#![no_std]
#![no_main]
// #![feature(offset_of)]
// the feature `offset_of` has been stable since 1.77.0 and no longer requires an attribute to
// enable

use core::fmt::Write;
use core::panic::PanicInfo;

use my_wasabi_os::arch::x86::hlt;
use my_wasabi_os::graphics::{Bitmap, draw_test_pattern, fill_rect};
use my_wasabi_os::init::init_basic_runtime;
use my_wasabi_os::qemu::{QemuExitCode, exit_qemu};
use my_wasabi_os::uefi::{EfiHandle, EfiMemoryType, EfiSystemTable, VramTextWriter, init_vram};

#[unsafe(no_mangle)]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, 0x000000, 0, 0, vw, vh).expect("fill_rect failed");
    draw_test_pattern(&mut vram);

    let mut w = VramTextWriter::new(&mut vram);

    let memory_map = init_basic_runtime(image_handle, efi_system_table);
    let mut total_memory_pages = 0;
    for e in memory_map.iter() {
        if e.memory_type() == EfiMemoryType::CONVENTIONAL_MEMORY {
            total_memory_pages += e.number_of_pages();
            writeln!(w, "{e:?}").unwrap();
        }
    }
    let total_memory_size_mib = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(
        w,
        "Total: {total_memory_pages} pages = {total_memory_size_mib} MiB"
    )
    .unwrap();

    writeln!(w, "Hello, Non-UEFI World!").unwrap();

    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Fail)
}
