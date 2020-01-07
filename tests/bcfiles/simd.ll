; ModuleID = 'simd.c'
source_filename = "simd.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

; Function Attrs: nofree noinline norecurse nounwind ssp uwtable
define i32 @callee(i32* nocapture, i32* nocapture, i32* noalias nocapture) local_unnamed_addr #0 {
  store i32 0, i32* %0, align 4, !tbaa !3
  store i32 2, i32* %1, align 4, !tbaa !3
  %4 = getelementptr inbounds i32, i32* %0, i64 1
  store i32 1, i32* %4, align 4, !tbaa !3
  %5 = getelementptr inbounds i32, i32* %1, i64 1
  store i32 3, i32* %5, align 4, !tbaa !3
  %6 = getelementptr inbounds i32, i32* %0, i64 2
  store i32 2, i32* %6, align 4, !tbaa !3
  %7 = getelementptr inbounds i32, i32* %1, i64 2
  store i32 4, i32* %7, align 4, !tbaa !3
  %8 = getelementptr inbounds i32, i32* %0, i64 3
  store i32 3, i32* %8, align 4, !tbaa !3
  %9 = getelementptr inbounds i32, i32* %1, i64 3
  store i32 5, i32* %9, align 4, !tbaa !3
  %10 = getelementptr inbounds i32, i32* %0, i64 4
  store i32 4, i32* %10, align 4, !tbaa !3
  %11 = getelementptr inbounds i32, i32* %1, i64 4
  store i32 6, i32* %11, align 4, !tbaa !3
  %12 = getelementptr inbounds i32, i32* %0, i64 5
  store i32 5, i32* %12, align 4, !tbaa !3
  %13 = getelementptr inbounds i32, i32* %1, i64 5
  store i32 7, i32* %13, align 4, !tbaa !3
  %14 = getelementptr inbounds i32, i32* %0, i64 6
  store i32 6, i32* %14, align 4, !tbaa !3
  %15 = getelementptr inbounds i32, i32* %1, i64 6
  store i32 8, i32* %15, align 4, !tbaa !3
  %16 = getelementptr inbounds i32, i32* %0, i64 7
  store i32 7, i32* %16, align 4, !tbaa !3
  %17 = getelementptr inbounds i32, i32* %1, i64 7
  store i32 9, i32* %17, align 4, !tbaa !3
  %18 = getelementptr inbounds i32, i32* %0, i64 8
  store i32 8, i32* %18, align 4, !tbaa !3
  %19 = getelementptr inbounds i32, i32* %1, i64 8
  store i32 10, i32* %19, align 4, !tbaa !3
  %20 = getelementptr inbounds i32, i32* %0, i64 9
  store i32 9, i32* %20, align 4, !tbaa !3
  %21 = getelementptr inbounds i32, i32* %1, i64 9
  store i32 11, i32* %21, align 4, !tbaa !3
  %22 = getelementptr inbounds i32, i32* %0, i64 10
  store i32 10, i32* %22, align 4, !tbaa !3
  %23 = getelementptr inbounds i32, i32* %1, i64 10
  store i32 12, i32* %23, align 4, !tbaa !3
  %24 = getelementptr inbounds i32, i32* %0, i64 11
  store i32 11, i32* %24, align 4, !tbaa !3
  %25 = getelementptr inbounds i32, i32* %1, i64 11
  store i32 13, i32* %25, align 4, !tbaa !3
  %26 = getelementptr inbounds i32, i32* %0, i64 12
  store i32 12, i32* %26, align 4, !tbaa !3
  %27 = getelementptr inbounds i32, i32* %1, i64 12
  store i32 14, i32* %27, align 4, !tbaa !3
  %28 = getelementptr inbounds i32, i32* %0, i64 13
  store i32 13, i32* %28, align 4, !tbaa !3
  %29 = getelementptr inbounds i32, i32* %1, i64 13
  store i32 15, i32* %29, align 4, !tbaa !3
  %30 = getelementptr inbounds i32, i32* %0, i64 14
  store i32 14, i32* %30, align 4, !tbaa !3
  %31 = getelementptr inbounds i32, i32* %1, i64 14
  store i32 16, i32* %31, align 4, !tbaa !3
  %32 = getelementptr inbounds i32, i32* %0, i64 15
  store i32 15, i32* %32, align 4, !tbaa !3
  %33 = getelementptr inbounds i32, i32* %1, i64 15
  store i32 17, i32* %33, align 4, !tbaa !3
  %34 = bitcast i32* %0 to <4 x i32>*
  %35 = load <4 x i32>, <4 x i32>* %34, align 4, !tbaa !3
  %36 = bitcast i32* %1 to <4 x i32>*
  %37 = load <4 x i32>, <4 x i32>* %36, align 4, !tbaa !3
  %38 = add <4 x i32> %37, %35
  %39 = bitcast i32* %2 to <4 x i32>*
  store <4 x i32> %38, <4 x i32>* %39, align 4, !tbaa !3
  %40 = getelementptr inbounds i32, i32* %2, i64 4
  %41 = bitcast i32* %10 to <4 x i32>*
  %42 = load <4 x i32>, <4 x i32>* %41, align 4, !tbaa !3
  %43 = bitcast i32* %11 to <4 x i32>*
  %44 = load <4 x i32>, <4 x i32>* %43, align 4, !tbaa !3
  %45 = add <4 x i32> %44, %42
  %46 = bitcast i32* %40 to <4 x i32>*
  store <4 x i32> %45, <4 x i32>* %46, align 4, !tbaa !3
  %47 = getelementptr inbounds i32, i32* %2, i64 8
  %48 = bitcast i32* %18 to <4 x i32>*
  %49 = load <4 x i32>, <4 x i32>* %48, align 4, !tbaa !3
  %50 = bitcast i32* %19 to <4 x i32>*
  %51 = load <4 x i32>, <4 x i32>* %50, align 4, !tbaa !3
  %52 = add <4 x i32> %51, %49
  %53 = bitcast i32* %47 to <4 x i32>*
  store <4 x i32> %52, <4 x i32>* %53, align 4, !tbaa !3
  %54 = getelementptr inbounds i32, i32* %2, i64 12
  %55 = bitcast i32* %27 to <2 x i32>*
  %56 = load <2 x i32>, <2 x i32>* %55, align 4, !tbaa !3
  %57 = load i32, i32* %31, align 4, !tbaa !3
  %58 = bitcast i32* %26 to <4 x i32>*
  %59 = load <4 x i32>, <4 x i32>* %58, align 4, !tbaa !3
  %60 = extractelement <2 x i32> %56, i32 0
  %61 = extractelement <2 x i32> %56, i32 1
  %62 = insertelement <4 x i32> <i32 undef, i32 undef, i32 undef, i32 17>, i32 %60, i32 0
  %63 = insertelement <4 x i32> %62, i32 %61, i32 1
  %64 = insertelement <4 x i32> %63, i32 %57, i32 2
  %65 = add <4 x i32> %64, %59
  %66 = bitcast i32* %54 to <4 x i32>*
  store <4 x i32> %65, <4 x i32>* %66, align 4, !tbaa !3
  %67 = extractelement <4 x i32> %38, i32 0
  %68 = extractelement <4 x i32> %38, i32 1
  %69 = add i32 %68, %67
  %70 = extractelement <4 x i32> %38, i32 2
  %71 = add i32 %70, %69
  %72 = extractelement <4 x i32> %38, i32 3
  %73 = add i32 %72, %71
  %74 = extractelement <4 x i32> %45, i32 0
  %75 = add i32 %74, %73
  %76 = extractelement <4 x i32> %45, i32 1
  %77 = add i32 %76, %75
  %78 = extractelement <4 x i32> %45, i32 2
  %79 = add i32 %78, %77
  %80 = extractelement <4 x i32> %45, i32 3
  %81 = add i32 %80, %79
  %82 = extractelement <4 x i32> %52, i32 0
  %83 = add i32 %82, %81
  %84 = extractelement <4 x i32> %52, i32 1
  %85 = add i32 %84, %83
  %86 = extractelement <4 x i32> %52, i32 2
  %87 = add i32 %86, %85
  %88 = extractelement <4 x i32> %52, i32 3
  %89 = add i32 %88, %87
  %90 = extractelement <4 x i32> %65, i32 0
  %91 = add i32 %90, %89
  %92 = extractelement <4 x i32> %65, i32 1
  %93 = add i32 %92, %91
  %94 = extractelement <4 x i32> %65, i32 2
  %95 = add i32 %94, %93
  %96 = extractelement <4 x i32> %65, i32 3
  %97 = add i32 %96, %95
  ret i32 %97
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: nounwind ssp uwtable
define i32 @simd_add_autovectorized() local_unnamed_addr #2 {
  %1 = alloca [16 x i32], align 16
  %2 = alloca [16 x i32], align 16
  %3 = alloca [16 x i32], align 16
  %4 = bitcast [16 x i32]* %1 to i8*
  call void @llvm.lifetime.start.p0i8(i64 64, i8* nonnull %4) #3
  %5 = bitcast [16 x i32]* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 64, i8* nonnull %5) #3
  %6 = bitcast [16 x i32]* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 64, i8* nonnull %6) #3
  %7 = getelementptr inbounds [16 x i32], [16 x i32]* %1, i64 0, i64 0
  %8 = getelementptr inbounds [16 x i32], [16 x i32]* %2, i64 0, i64 0
  %9 = getelementptr inbounds [16 x i32], [16 x i32]* %3, i64 0, i64 0
  %10 = call i32 @callee(i32* nonnull %7, i32* nonnull %8, i32* nonnull %9)
  call void @llvm.lifetime.end.p0i8(i64 64, i8* nonnull %6) #3
  call void @llvm.lifetime.end.p0i8(i64 64, i8* nonnull %5) #3
  call void @llvm.lifetime.end.p0i8(i64 64, i8* nonnull %4) #3
  ret i32 %10
}

attributes #0 = { nofree noinline norecurse nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind }
attributes #2 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.0 (tags/RELEASE_900/final)"}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
