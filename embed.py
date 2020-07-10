from ctypes import cdll

lib = cdll.LoadLibrary("target/release/lib_ffi.so")
lib.process()

print("сделано!")