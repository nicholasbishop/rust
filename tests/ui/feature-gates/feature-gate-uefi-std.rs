// needs-llvm-components: x86_64-unknown-uefi
// compile-flags: --target=x86_64-unknown-uefi --crate-type=rlib

//~^^ ERROR can't find crate for `std`

// The UEFI targets require `#![no_std]` unless the `uefi_std` feature is enabled,
// so an empty source file is sufficient to trigger an error.
