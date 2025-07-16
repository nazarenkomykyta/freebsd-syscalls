#![no_std]
pub mod table;
pub mod syscalls;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::syscalls::___syscall_exit;

    #[test]
    fn exit_code_zero() {
        ___syscall_exit(0);
    }

    #[test]
    fn exit_code_one() {
        ___syscall_exit(1);
    }
}
