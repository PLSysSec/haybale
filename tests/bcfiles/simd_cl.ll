; ModuleID = 'simd_cl.cl'
source_filename = "simd_cl.cl"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @simd_add(i32, i32) local_unnamed_addr #0 {
  %3 = insertelement <4 x i32> undef, i32 %0, i32 0
  %4 = shufflevector <4 x i32> %3, <4 x i32> undef, <4 x i32> zeroinitializer
  %5 = insertelement <4 x i32> undef, i32 %1, i32 0
  %6 = add i32 %1, 1
  %7 = insertelement <4 x i32> %5, i32 %6, i32 1
  %8 = add i32 %1, 2
  %9 = insertelement <4 x i32> %7, i32 %8, i32 2
  %10 = add i32 %1, 3
  %11 = insertelement <4 x i32> %9, i32 %10, i32 3
  %12 = add <4 x i32> %11, %4
  %13 = shufflevector <4 x i32> %12, <4 x i32> undef, <4 x i32> <i32 2, i32 3, i32 undef, i32 undef>
  %14 = add <4 x i32> %12, %13
  %15 = shufflevector <4 x i32> %14, <4 x i32> undef, <4 x i32> <i32 1, i32 undef, i32 undef, i32 undef>
  %16 = add <4 x i32> %14, %15
  %17 = extractelement <4 x i32> %16, i32 0
  ret i32 %17
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @simd_ops(i32, i32) local_unnamed_addr #0 {
  %3 = insertelement <4 x i32> undef, i32 %0, i32 0
  %4 = shufflevector <4 x i32> %3, <4 x i32> undef, <4 x i32> zeroinitializer
  %5 = insertelement <4 x i32> undef, i32 %1, i32 0
  %6 = add i32 %1, 1
  %7 = insertelement <4 x i32> %5, i32 %6, i32 1
  %8 = add i32 %1, 2
  %9 = insertelement <4 x i32> %7, i32 %8, i32 2
  %10 = add i32 %1, 3
  %11 = insertelement <4 x i32> %9, i32 %10, i32 3
  %12 = add <4 x i32> %11, %4
  %13 = mul <4 x i32> %12, <i32 17, i32 17, i32 17, i32 17>
  %14 = add <4 x i32> %13, <i32 -51, i32 -51, i32 -51, i32 -51>
  %15 = xor <4 x i32> %4, <i32 -4, i32 -4, i32 -4, i32 -4>
  %16 = and <4 x i32> %14, %15
  %17 = or <4 x i32> %16, %11
  %18 = lshr <4 x i32> %17, <i32 2, i32 2, i32 2, i32 2>
  %19 = shl <4 x i32> %18, <i32 2, i32 3, i32 4, i32 5>
  %20 = shufflevector <4 x i32> %19, <4 x i32> undef, <4 x i32> <i32 2, i32 3, i32 undef, i32 undef>
  %21 = add <4 x i32> %19, %20
  %22 = shufflevector <4 x i32> %21, <4 x i32> undef, <4 x i32> <i32 1, i32 undef, i32 undef, i32 undef>
  %23 = add <4 x i32> %21, %22
  %24 = extractelement <4 x i32> %23, i32 0
  ret i32 %24
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @simd_select(i32, i32) local_unnamed_addr #0 {
  %3 = insertelement <4 x i32> undef, i32 %0, i32 0
  %4 = shufflevector <4 x i32> %3, <4 x i32> undef, <4 x i32> zeroinitializer
  %5 = insertelement <4 x i32> undef, i32 %1, i32 0
  %6 = add i32 %1, 1
  %7 = insertelement <4 x i32> %5, i32 %6, i32 1
  %8 = add i32 %1, 2
  %9 = insertelement <4 x i32> %7, i32 %8, i32 2
  %10 = add i32 %1, 3
  %11 = insertelement <4 x i32> %9, i32 %10, i32 3
  %12 = icmp ult <4 x i32> %4, %11
  %13 = select <4 x i1> %12, <4 x i32> %4, <4 x i32> %11
  %14 = shufflevector <4 x i32> %13, <4 x i32> undef, <4 x i32> <i32 2, i32 3, i32 undef, i32 undef>
  %15 = add <4 x i32> %13, %14
  %16 = shufflevector <4 x i32> %15, <4 x i32> undef, <4 x i32> <i32 1, i32 undef, i32 undef, i32 undef>
  %17 = add <4 x i32> %15, %16
  %18 = extractelement <4 x i32> %17, i32 0
  ret i32 %18
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @simd_typeconversions(i32, i32) local_unnamed_addr #0 {
  %3 = insertelement <2 x i32> undef, i32 %1, i32 0
  %4 = shufflevector <2 x i32> %3, <2 x i32> undef, <2 x i32> zeroinitializer
  %5 = add <2 x i32> %4, <i32 10, i32 3>
  %6 = add i32 %1, 30
  %7 = zext i32 %0 to i64
  %8 = insertelement <4 x i64> undef, i64 %7, i32 0
  %9 = shufflevector <4 x i64> %8, <4 x i64> undef, <4 x i32> zeroinitializer
  %10 = zext i32 %1 to i64
  %11 = insertelement <4 x i64> undef, i64 %10, i32 0
  %12 = zext <2 x i32> %5 to <2 x i64>
  %13 = extractelement <2 x i64> %12, i32 0
  %14 = insertelement <4 x i64> %11, i64 %13, i32 1
  %15 = extractelement <2 x i64> %12, i32 1
  %16 = insertelement <4 x i64> %14, i64 %15, i32 2
  %17 = zext i32 %6 to i64
  %18 = insertelement <4 x i64> %16, i64 %17, i32 3
  %19 = sub <4 x i64> %18, %9
  %20 = trunc <4 x i64> %19 to <4 x i32>
  %21 = shufflevector <4 x i32> %20, <4 x i32> undef, <4 x i32> <i32 2, i32 3, i32 undef, i32 undef>
  %22 = add <4 x i32> %21, %20
  %23 = shufflevector <4 x i32> %22, <4 x i32> undef, <4 x i32> <i32 1, i32 undef, i32 undef, i32 undef>
  %24 = add <4 x i32> %22, %23
  %25 = extractelement <4 x i32> %24, i32 0
  ret i32 %25
}

attributes #0 = { norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "denorms-are-zero"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!opencl.ocl.version = !{!2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 1, i32 0}
!3 = !{!"clang version 9.0.1 "}
