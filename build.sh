#/bin/sh

cargo build --release --target armv7r-none-eabihf
#cargo build --release
ar -x target/armv7r-none-eabihf/release/libarmv7r_atomicbool_align.rlib

llvm-objdump-14 -d armv7r_atomicbool_align-ab31468205c957d4.armv7r_atomicbool_align.bb74d85ab07ca280-cgu.0.rcgu.o
