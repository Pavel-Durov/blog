BIN_OUT=./dist
OUT_PROG=${BIN_OUT}/my_program

build: build-lib build-c

build-lib:
	cargo build

build-c:
	clang -o ${OUT_PROG} ./src/main.c ./target/debug/deps/libld_wrap_rust.so -Wl,--wrap=pthread_create

run: build-lib build-c
	./dist/my_program
