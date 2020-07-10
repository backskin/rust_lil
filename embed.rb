require 'ffi'
puts 'here'
module Hello
  extend FFI::Library
  ffi_lib 'target/release/lib_ffi.so'
  attach_function :process, [], :void
end

Hello.process

puts 'сделано!'