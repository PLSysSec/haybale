CC=clang-8
CFLAGS=-O3

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

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.c
	$(CC) $(CFLAGS) -c -emit-llvm $^ -o $@

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
