use star_loader::*;

fn main() {
}

/*

extern crate syscalls;
use syscalls::{Sysno, syscall};

#[cfg(not(any(
    target_arch = "x86_64",
    //target_arch = "aarch64",
    //target_arch = "riscv64",
)))]
compile_error!("unsupport arch");

cfg_if::cfg_if! {
  if #[cfg(target_arch = "x86_64")] {
    todo!();
  } else if #[cfg(target_arch = "riscv64")]{
    compile_error!("unsupport arch");
  } else if #[cfg(target_arch="aarch64")]{
    compile_error!("unsupport arch");
  }
}




match unsafe { syscall!(Sysno::clone3, ... ) } {
  Ok(0) => {
    // Child process
    // Now we need to trampoline into the loaded stuff
    asm!{
    };
    syscall1(Sysno::exit, 0usize).unwrap();
    unreachable!()
  }
  Ok(pid) => {
    // Parent process
    syscall1(Sysno::exit, 0usize).unwrap();
    unreachable!()
  }
  Err(err) => {
    eprintln!("clone() failed: {}", err);
    syscall1(Sysno::exit, 1usize).unwrap();
    unreachable!()
  }
}

 */
