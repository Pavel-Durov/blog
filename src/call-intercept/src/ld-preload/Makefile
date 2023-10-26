BIN_OUT=./dist

build:
	mkdir -p ${BIN_OUT}
	clang++ -shared -o ${BIN_OUT}/pthread_intercept.so pthread_intercept.cpp -ldl
	clang++ -o ${BIN_OUT}/my_program main.cpp

run: build
	LD_PRELOAD=${PWD}/dist/pthread_intercept.so ${BIN_OUT}/my_program
