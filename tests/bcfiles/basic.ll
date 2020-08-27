; ModuleID = 'basic.c'
source_filename = "basic.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @no_args_zero() local_unnamed_addr #0 {
  ret i32 0
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @no_args_nozero() local_unnamed_addr #0 {
  ret i32 1
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @one_arg(i32) local_unnamed_addr #0 {
  %2 = add nsw i32 %0, -3
  ret i32 %2
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @two_args(i32, i32) local_unnamed_addr #0 {
  %3 = add i32 %0, -3
  %4 = add i32 %3, %1
  ret i32 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @three_args(i32, i32, i32) local_unnamed_addr #0 {
  %4 = add i32 %0, -3
  %5 = add i32 %4, %1
  %6 = add i32 %5, %2
  ret i32 %6
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @four_args(i32, i32, i32, i32) local_unnamed_addr #0 {
  %5 = add i32 %0, -3
  %6 = add i32 %5, %1
  %7 = add i32 %6, %2
  %8 = add i32 %7, %3
  ret i32 %8
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @five_args(i32, i32, i32, i32, i32) local_unnamed_addr #0 {
  %6 = add i32 %0, -3
  %7 = add i32 %6, %1
  %8 = add i32 %7, %2
  %9 = add i32 %8, %3
  %10 = add i32 %9, %4
  ret i32 %10
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @binops(i32, i32) local_unnamed_addr #0 {
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
define i32 @conditional_true(i32, i32) local_unnamed_addr #0 {
  %3 = icmp sgt i32 %0, %1
  br i1 %3, label %4, label %8

4:                                                ; preds = %2
  %5 = add nsw i32 %0, -1
  %6 = add nsw i32 %1, -1
  %7 = mul nsw i32 %6, %5
  br label %12

8:                                                ; preds = %2
  %9 = add nsw i32 %1, %0
  %10 = srem i32 %9, 3
  %11 = add nsw i32 %10, 10
  br label %12

12:                                               ; preds = %8, %4
  %13 = phi i32 [ %7, %4 ], [ %11, %8 ]
  ret i32 %13
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @conditional_false(i32, i32) local_unnamed_addr #0 {
  %3 = icmp sgt i32 %0, %1
  br i1 %3, label %4, label %8

4:                                                ; preds = %2
  %5 = add nsw i32 %1, %0
  %6 = srem i32 %5, 3
  %7 = add nsw i32 %6, 10
  br label %12

8:                                                ; preds = %2
  %9 = add nsw i32 %0, -1
  %10 = add nsw i32 %1, -1
  %11 = mul nsw i32 %10, %9
  br label %12

12:                                               ; preds = %8, %4
  %13 = phi i32 [ %7, %4 ], [ %11, %8 ]
  ret i32 %13
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @conditional_nozero(i32, i32) local_unnamed_addr #0 {
  %3 = icmp sgt i32 %0, 2
  br i1 %3, label %14, label %4

4:                                                ; preds = %2
  %5 = icmp slt i32 %1, 1
  br i1 %5, label %6, label %8

6:                                                ; preds = %4
  %7 = add nsw i32 %1, -3
  br label %14

8:                                                ; preds = %4
  %9 = icmp slt i32 %0, 1
  br i1 %9, label %10, label %12

10:                                               ; preds = %8
  %11 = add nsw i32 %0, -7
  br label %14

12:                                               ; preds = %8
  %13 = mul nsw i32 %1, %0
  br label %14

14:                                               ; preds = %2, %12, %10, %6
  %15 = phi i32 [ %7, %6 ], [ %11, %10 ], [ %13, %12 ], [ %0, %2 ]
  ret i32 %15
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @conditional_with_and(i32, i32) local_unnamed_addr #0 {
  %3 = icmp slt i32 %0, 4
  %4 = icmp slt i32 %1, 5
  %5 = or i1 %3, %4
  %6 = zext i1 %5 to i32
  ret i32 %6
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @has_switch(i32, i32) local_unnamed_addr #0 {
  %3 = sub nsw i32 %0, %1
  switch i32 %3, label %12 [
    i32 0, label %14
    i32 1, label %4
    i32 2, label %5
    i32 3, label %7
    i32 33, label %10
    i32 451, label %11
  ]

4:                                                ; preds = %2
  br label %14

5:                                                ; preds = %2
  %6 = add nsw i32 %0, -3
  br label %14

7:                                                ; preds = %2
  %8 = mul nsw i32 %1, %0
  %9 = add nsw i32 %8, 1
  br label %14

10:                                               ; preds = %2
  br label %14

11:                                               ; preds = %2
  br label %14

12:                                               ; preds = %2
  %13 = add nsw i32 %3, -1
  br label %14

14:                                               ; preds = %2, %12, %11, %10, %7, %5, %4
  %15 = phi i32 [ %13, %12 ], [ -5, %11 ], [ -300, %10 ], [ %9, %7 ], [ %6, %5 ], [ 3, %4 ], [ -1, %2 ]
  ret i32 %15
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define signext i8 @int8t(i8 signext, i8 signext) local_unnamed_addr #0 {
  %3 = add i8 %0, -3
  %4 = add i8 %3, %1
  ret i8 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define signext i16 @int16t(i16 signext, i16 signext) local_unnamed_addr #0 {
  %3 = add i16 %0, -3
  %4 = add i16 %3, %1
  ret i16 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @int32t(i32, i32) local_unnamed_addr #0 {
  %3 = add i32 %0, -3
  %4 = add i32 %3, %1
  ret i32 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i64 @int64t(i64, i64) local_unnamed_addr #0 {
  %3 = add i64 %0, -3
  %4 = add i64 %3, %1
  ret i64 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i64 @mixed_bitwidths(i8 signext, i16 signext, i32, i64) local_unnamed_addr #0 {
  %5 = sext i8 %0 to i32
  %6 = sext i16 %1 to i32
  %7 = add nsw i32 %6, %5
  %8 = add nsw i32 %7, %2
  %9 = sext i32 %8 to i64
  %10 = add i64 %3, -3
  %11 = add i64 %10, %9
  ret i64 %11
}

attributes #0 = { norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
