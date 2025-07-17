use crate::table::*;
use crate::types::*;
use core::arch::asm;

pub fn ___syscall_exit(_exit_c: _DWORD) {
    unsafe {
        asm!(
            "mov rax, {__syscall:r}",
            "push {__code:r}",
            "push rax",
            "int 80h",
            "add rsp, 16",
            "ret",
            __syscall = in(reg) _SYS_EXIT,
            __code = in(reg) _exit_c,
            options(nostack)
        );
    }
}

pub fn ___syscall_sync() {
    unsafe {
        asm!(
            "mov rax, {__syscall:r}",
            "push rax",
            "int 80h",
            "add rsp, 8",
            "ret",
            __syscall = in(reg) _SYS_SYNC,
            options(nostack)
        );
    }
}

pub fn ___syscall_shutdown() {
    unsafe {
        asm!(
            "mov rax, {__syscall:r}",
            "push rax",
            "int 80h",
            "add rsp, 8",
            "ret",
            __syscall = in(reg) _SYS_SHUTDOWN,
            options(nostack)
        );
    }
}

pub fn ___syscall_reboot() {
    unsafe {
        asm!(
            "mov rax, {__syscall:r}",
            "push rax",
            "int 80h",
            "add rsp, 8",
            "ret",
            __syscall = in(reg) _SYS_REBOOT,
            options(nostack)
        );
    }
}

