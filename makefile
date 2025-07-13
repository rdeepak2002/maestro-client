OS := $(shell uname)
ifeq ($(OS),Darwin)
    EXT := dylib
else ifeq ($(OS),MINGW64_NT)
    EXT := dll
else ifeq ($(OS),MINGW32_NT)
    EXT := dll
else ifneq (,$(findstring CYGWIN,$(OS)))
    EXT := dll
else ifneq (,$(findstring MSYS,$(OS)))
    EXT := dll
else
    EXT := so
endif

build:
	cargo build --release
	rm -rf dist/
	mkdir dist
	cp ./target/release/libschedulerclient_jdk23.$(EXT) dist/libschedulerclient_jdk23.$(EXT)
	cp ./target/release/libschedulerclient_py.$(EXT) dist/schedulerclient_py.so

test-rust: build
	cargo test

test-python: build
	rm -f target/release/libschedulerclient_py.$(EXT)
	PYTHONPATH='./dist/' python3 scheduler-client-py/schedulerclient.py

test-jdk23: build
	cd scheduler-client-jdk23/ && \
		javac SchedulerClient.java && \
		java -ea -Djava.library.path='../dist/' SchedulerClient

test: test-rust test-python test-jdk23

clean:
	cargo clean
	rm -rf dist
	rm -rf scheduler-client-jdk23/SchedulerClient.class
