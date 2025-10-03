TARGET = riscv64gc-unknown-none-elf
TARGET_DIR = target/$(TARGET)/release

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

.PHONY: build run