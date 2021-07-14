.phony: build, run

bin=target/x86_64/debug/bootimage-tinix_rt.bin
img=disk.img

build:
	cargo build
	cargo bootimage
	qemu-img create $(img) 512M
	dd conv=notrunc if=$(bin) of=$(img)


run:
	qemu-system-x86_64 -hda $(img) -hdb test.tfs -m 2048 -serial stdio
	hexdump disk.img -C > "disk.txt"
	hexdump test.tfs -C > "tfs.txt"

test_bootsectors:
	qemu-system-x86_64 -hda test.tfs -m 2048 -serial stdio