# Minmax in Rust with JNI

## How to build and run

### On Linux

Run `./build_and_run.sh` to build the Rust library and run the Java tests with it.

If you want to use it in another project (for example, because you copied `RustPlayer` to your project to test it) then you need a little bit of extra setup.
You need to make sure that you set the `LD_LIBRARY_PATH` environment variable to the what is printed at the end of the script. Then everything will be fine.

If you don't like setting `LD_LIBRARY_PATH` then you can also run `sudo cp target/release/libminmax_wrapper.so /usr/lib/x86_64-linux-gnu/jni/` (if you're not on GNU/Linux because you use Alpine on your desktop, Iglunix or whatever crazy things people are doing these days) replace the `x86_64-linux-gnu` with your target triplet, but you are probably on GNU/Linux and don't need to worry about this and if you aren't you know all of this stuff anyways). After running the command you can just run `RustPlayer` without any setup. And your global Java native libs will be polluted forever. Or until you run `sudo rm /usr/lib/x86_64-linux-gnu/jni/libminmax_wrapper.so`.


### On MacOS

It should probably the same as linux? except for the `cp` part apples native library layout is different. but actually the error message will tell you where it looks so you can just copy it there i think? havent tested it

### On Windows

you're on your own but you can do this, it's similar to linux except you have to run `cargo build --release` yourself and have to `target/release/minmax_wrapper.dll` to the directories where the other library are (the link error message will tell you where that is)