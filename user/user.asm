src/bin/00hello_world.rs src/bin/01store_fault.rs src/bin/02power.rs
target/riscv64gc-unknown-none-elf/release/00hello_world target/riscv64gc-unknown-none-elf/release/01store_fault target/riscv64gc-unknown-none-elf/release/02power
target/riscv64gc-unknown-none-elf/release/00hello_world.bin target/riscv64gc-unknown-none-elf/release/01store_fault.bin target/riscv64gc-unknown-none-elf/release/02power.bin
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/00hello_world --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/00hello_world.bin;  rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/01store_fault --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/01store_fault.bin;  rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/02power --strip-all -O binary  target/riscv64gc-unknown-none-elf/release/02power.bin;