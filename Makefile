LIB_PATH = klangfarbrs/target/debug/libklangfarbrs.so

all: refresh_lib

build:
	cd klangfarbrs && cargo build

refresh_lib: build
	cp $(LIB_PATH) klangfarb
