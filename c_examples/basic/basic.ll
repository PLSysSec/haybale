; ModuleID = 'basic/basic.c'
source_filename = "basic/basic.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @basic(i32, i32) local_unnamed_addr #0 {
  %3 = add i32 %0, -3
  %4 = add i32 %3, %1
  ret i32 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @basic_binops(i32, i32) local_unnamed_addr #0 {
  %3 = mul i32 %0, -77
  %4 = add i32 %0, 1
  %5 = add i32 %4, %1
  %6 = add i32 %5, %3
  %7 = and i32 %6, 23
  %8 = or i32 %0, 99
  %9 = sdiv i32 %7, %8
  %10 = xor i32 %9, %0
  %11 = shl i32 %6, 3
  %12 = srem i32 %10, %11
  %13 = ashr i32 %12, %9
  ret i32 %13
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @basic_if(i32, i32) local_unnamed_addr #0 {
  %3 = icmp slt i32 %0, %1
  br i1 %3, label %4, label %6

; <label>:4:                                      ; preds = %2
  %5 = sub nsw i32 %0, %1
  br label %10

; <label>:6:                                      ; preds = %2
  %7 = add nsw i32 %0, -1
  %8 = add nsw i32 %1, -1
  %9 = mul nsw i32 %8, %7
  br label %10

; <label>:10:                                     ; preds = %6, %4
  %11 = phi i32 [ %5, %4 ], [ %9, %6 ]
  ret i32 %11
}

attributes #0 = { norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 7.0.1 (tags/RELEASE_701/final)"}
