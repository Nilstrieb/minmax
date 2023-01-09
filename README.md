# Minmax in Rust with JNI

## How to build and run

### Prerequisites
- Gradle
- Rust toolchain

### Compile Rust code

`cargo build`

### Assemble shared library

Copy the shared library in `target/debug/librs_wrapper.so` (or a `.dll` on windows) to a known location like the working directory or the system wide library directory.

### Build the jar

Use `./gradlew jar` or whatever you like I don't know.

### Run the program

Maybe it works now. If you get a link error you might need to set the `LD_LIBRARY_PATH` env var to the path of the shared library (on linux). Or you have to copy it to the systems library directory. Or something else. But it should work.