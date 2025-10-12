TARGET = riscv64gc-unknown-none-elf
TARGET_DIR = target/$(TARGET)/release
KERNEL_ELF = $(TARGET_DIR)/riscv_learning
KERNEL_BIN = $(TARGET_DIR)/kernel.bin
GDB_PORT = 1234

build: 
	@echo "Building for target: $(TARGET)"
	@cargo build --target $(TARGET) --release
	@echo "Creating binary kernel image"
	@rust-objcopy --binary-architecture=riscv64 --strip-all -O binary \
		$(TARGET_DIR)/riscv_learning \
    	$(TARGET_DIR)/kernel.bin

run: build
	@echo "Running in QEMU"
	@qemu-system-riscv64 -m 128M -machine virt -bios default -nographic \
		-kernel $(TARGET_DIR)/kernel.bin \
    	-D qemu.log -d in_asm

clean:
	@echo "Cleaning project"
	@cargo clean
	@rm -f $(TARGET_DIR)/kernel.bin
	@rm -f qemu.log

log:
	@tail -f qemu.log

gdb: build
	@echo "Starting GDB server on port $(GDB_PORT)"
	@qemu-system-riscv64 -m 128M -machine virt -bios default -nographic \
		-kernel $(TARGET_DIR)/kernel.bin \
    	-D qemu.log -d in_asm -S -s

.PHONY: build run clean gdb