source_filename = "<no source file>"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

define i32 @atomicrmwops(i32, i32) local_unnamed_addr {
  %addr = alloca i32, align 4

  ; initial value at %addr is %0
  store i32 %0, i32* %addr, align 4

  %3 = atomicrmw xchg i32* %addr, i32 %1 monotonic
  ; now %3 is %0 and the value is %1

  %4 = atomicrmw add i32* %addr, i32 %3 acquire
  ; now %4 is %1 and the value is %1 + %3 (so, %1 + %0)

  %5 = atomicrmw sub i32* %addr, i32 %1 release
  ; now %5 is %1 + %0 and the value is %0

  %6 = atomicrmw and i32* %addr, i32 %4 acq_rel
  ; now %6 is %0 and the value is %0 & %1

  %7 = atomicrmw xor i32* %addr, i32 3 seq_cst
  ; now %7 is %0 & %1 and the value is (%0 & %1) ^ 3

  %8 = atomicrmw umax i32* %addr, i32 %0 monotonic
  ; now %8 is (%0 & %1) ^ 3 and the value is umax(%8, %0)

  %9 = load i32, i32* %addr, align 4

  ret i32 %9
}
