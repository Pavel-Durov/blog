BIN_OUT=./dist

build:
	mkdir -p ${BIN_OUT}
	clang++ -c -o ${BIN_OUT}/pthread_intercept.o pthread_intercept.cpp
	clang++ -o ${BIN_OUT}/my_program main.cpp ${BIN_OUT}/pthread_intercept.o -Wl,--wrap=pthread_create

run: build
	${BIN_OUT}/my_program
