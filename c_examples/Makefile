CC=clang-7
LLVMAS=llvm-as

.PHONY: all
all: basic/basic.bc basic/basic.ll

%.ll : %.c
	$(CC) -S -emit-llvm $^ -o $@

%.bc : %.ll
	$(LLVMAS) $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
