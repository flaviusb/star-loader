/*


Binary File ⇆ Rust Representation of the Unprocessed Thing ⇆ Plain text representation ⇄ Process image state

Everything that 'runs and affects lib state as such' eg fixusp, initialisers, dependencies etc etc needs to have a way to mark that it has been run already, so that the Process Image State can be represented faithfully by the other forms of the thing, and also so that you can eg 'rebuild deps', 'introspect fixups', etc etc.

They can also be purged, for example when compiling you might want to purge that stuff to a) save space b) Maybe make it faster? c) make it harder to de-monolithicise d) hide non-public APIs etc so that people don't depend on unkept invariants etc

Compile time
Load-time
Run-time

Run initialisers
Patchups


Processing of the Unprocessed Thing
☑ Compile time linking (← maybe we have a thing to do this for compilers... ???)
☐ Load-time linking
→ ☐ Run initialisers...
☐ Run-time linking

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
