PYPY_VERSION := pypy3.9-v7.3.9
PYTHONPATH :=${PYTHONPATH}:${PWD}/${PYPY_VERSION}-src/

SHELL := /bin/bash
clean:
	rm -rf ${PYPY_VERSION}-src.tar.bz2
	rm -rf ${PYPY_VERSION}-src

get-pypy: clean
	wget -S "https://downloads.python.org/pypy/${PYPY_VERSION}-src.tar.bz2" && tar -xvf "${PYPY_VERSION}-src.tar.bz2"
	wget -S "https://downloads.python.org/pypy/${PYPY_VERSION}-osx64.tar.bz2" && tar -xvf "${PYPY_VERSION}-osx64.tar.bz2"
	rm -rf ./${PYPY_VERSION}.tar.bz2

translate:
	"./${PYPY_VERSION}-src/rpython/bin/rpython" --opt=jit --jit threshold=100   "${PWD}/hello_world.py"

run-translated:
	PYPYLOG=jit-log-opt:./log.txt ./hello_world-c 999999999999999

run-pypy:
	PYPYLOG=jit-log-opt:./log.txt ./${PYPY_VERSION}-osx64/bin/pypy ./hello.py

link:
	sudo ln -s /Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk /Library/Developer/CommandLineTools/SDKs/MacOSX11.sdk

jist-summary:
	PYPYLOG=jit-summary "./${PYPY_VERSION}-osx64/bin/pypy" ./hello_world.py 9