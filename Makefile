.phony: build, run

bin=target/x86_64/debug/bootimage-tinix_rt.bin
img=disk.img
storage=storage.bin
ram_size_mb=512

build:
	cargo build
	cargo bootimage
	qemu-img create $(img) 16M
	dd conv=notrunc if=$(bin) of=$(img)


run:
	qemu-system-x86_64 -hda $(img) -hdb $(storage) -m $(ram_size_mb) -serial stdio

test_bootsectors:
	qemu-system-x86_64 -hda $(storage) -m $(ram_size_mb) -serial stdio

clean:
	qemu-img create $(storage) 32K 
	cargo clean
