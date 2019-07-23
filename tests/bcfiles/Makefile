CC=clang-8
CFLAGS=-O3
LLVMAS=llvm-as

.PHONY: all
all: basic.bc basic.ll \
			memory.bc memory.ll \
			loop.bc loop.ll \
			struct.bc struct.ll \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.ll
	$(LLVMAS) $^ -o $@

# use -O1 on loop.c
loop.ll : loop.c
	$(CC) -O1 -S -emit-llvm $^ -o $@

# use -O0 on struct.c
struct.ll : struct.c
	$(CC) -O0 -S -emit-llvm $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
