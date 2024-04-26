// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// This function is marked as `unsafe` because it operates with raw pointers
/// and relies on the caller to guarantee the following conditions:
///
/// - The `address` parameter must be a valid memory address pointing to a mutable `u32` value.
/// - There must not be any other references (mutable or immutable) to the memory location
///   pointed to by `address` for the duration of the function's execution.
///
/// Violating any of these conditions can lead to undefined behavior, including but not limited to
/// data races, memory corruption, and crashes.
unsafe fn modify_by_address(address: usize) {
    // Safety: Dereferencing a raw pointer obtained from `address` is safe under the
    // assumption that the caller guarantees the validity of the pointer.
    let value_ptr = address as *mut u32;
    // SAFETY: We assume that `address` points to a valid memory location containing a `u32`.
    let value = value_ptr.as_mut().unwrap();
    // Modify the value.
    *value = 0xAABBCCDD;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}
