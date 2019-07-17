CC=clang-8
CFLAGS=-O3
LLVMAS=llvm-as

.PHONY: all
all: basic/basic.bc basic/basic.ll \
			memory/memory.bc memory/memory.ll \
			loop/loop.bc loop/loop.ll \
			struct/struct.bc struct/struct.ll \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.ll
	$(LLVMAS) $^ -o $@

# use -O1 on loop.c
loop/loop.ll : loop/loop.c
	$(CC) -O1 -S -emit-llvm $^ -o $@

# use -O0 on struct.c
struct/struct.ll : struct/struct.c
	$(CC) -O0 -S -emit-llvm $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
