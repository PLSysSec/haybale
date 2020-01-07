CC=clang-9
CXX=$$LLVMPATH/bin/clang++
CFLAGS=-O3
RUSTC=rustc
RUSTFLAGS=--crate-type=lib

.PHONY: all
all: basic.bc basic.ll \
			memory.bc memory.ll \
			loop.bc loop.ll \
			struct.bc struct.ll \
			linkedlist.bc linkedlist.ll \
			call.bc call.ll \
			crossmod.bc crossmod.ll \
			globals.bc globals.ll \
			globals_initialization_1.bc globals_initialization_1.ll \
			globals_initialization_2.bc globals_initialization_2.ll \
			functionptr.bc functionptr.ll \
			simd.bc simd.ll \
			simd_cl.bc simd_cl.ll \
			throwcatch.bc throwcatch.ll \
			abort.bc abort.ll \
			panic.bc panic.ll \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.c
	$(CC) $(CFLAGS) -c -emit-llvm $^ -o $@

%.ll : %.cpp
	$(CXX) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.cpp
	$(CXX) $(CFLAGS) -c -emit-llvm $^ -o $@

%.ll : %.cl
	$(CC) $(CFLAGS) -Xclang -finclude-default-header -S -emit-llvm $^ -o $@

%.bc : %.cl
	$(CC) $(CFLAGS) -Xclang -finclude-default-header -c -emit-llvm $^ -o $@

%.ll : %.rs
	$(RUSTC) $(RUSTFLAGS) --emit=llvm-ir $^ -o $@

%.bc : %.rs
	$(RUSTC) $(RUSTFLAGS) --emit=llvm-bc $^ -o $@

# use -O1 on loop.c
loop.ll : loop.c
	$(CC) -O1 -S -emit-llvm $^ -o $@
loop.bc : loop.c
	$(CC) -O1 -c -emit-llvm $^ -o $@

# use -O0 on struct.c and linkedlist.c
struct.ll : struct.c
	$(CC) -O0 -S -emit-llvm $^ -o $@
struct.bc : struct.c
	$(CC) -O0 -c -emit-llvm $^ -o $@
linkedlist.ll : linkedlist.c
	$(CC) -O0 -S -emit-llvm $^ -o $@
linkedlist.bc : linkedlist.c
	$(CC) -O0 -c -emit-llvm $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
