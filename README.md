# freebsd-syscalls
This library is a "wrapper" that implements FreeBSD system calls. It has no dependencies and does not use the Rust standard library, while simplifying development and allowing you to abandon FFI. In addition to all of the above, it allows you to reduce overhead and achieve maximum performance in those sections of code where it is required.

# example 


```rust
use freebsd_syscalls::syscalls::___syscall_exit;
fn main() {
  ___syscall_exit(0); // Exit current process with code 0 
}
```

Calling this function is equivalent to :

```asm
mov rax, 1 
push 0
push rax
int 80h 
add rsp, 16 
ret
```

# important

Currently, the library is in a very raw and incomplete state, and therefore, contributions are highly welcomed.
