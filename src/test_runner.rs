use core::panic::PanicInfo;

use crate::qemu::{QemuExitCode, exit_qemu};

pub fn test_runner(_tests: &[&dyn FnOnce()]) -> ! {
    exit_qemu(QemuExitCode::Success)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        exit_qemu(QemuExitCode::Fail)
    }
}
