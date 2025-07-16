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
