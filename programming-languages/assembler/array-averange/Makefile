all: run

run:
	@mkdir -p build
	@nasm -felf64 main.asm -o build/main.o && ld build/main.o -o ./build/a.out && ./build/a.out

debug: 
	gdb -q ./build/a.out

