/*

pub fn load

pub fn star_exec
  takes a fully loaded/linked thing as argument
  allocate some pages and populate them with ...
  syscall: clone3 to make new process image with defined stack etc
  check ret value:
  - in parent, exit()
  - in child, asm!() trampoline to the new stuff


 */


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
