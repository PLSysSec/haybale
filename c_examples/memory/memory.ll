; ModuleID = 'memory/memory.c'
source_filename = "memory/memory.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

; Function Attrs: norecurse nounwind ssp uwtable
define i32 @load_and_store(i32*, i32) local_unnamed_addr #0 {
  %3 = add nsw i32 %1, -3
  store volatile i32 %3, i32* %0, align 4, !tbaa !3
  %4 = load volatile i32, i32* %0, align 4, !tbaa !3
  ret i32 %4
}

; Function Attrs: nounwind ssp uwtable
define i32 @local_ptr(i32) local_unnamed_addr #1 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3)
  %4 = add nsw i32 %0, -3
  store volatile i32 %4, i32* %2, align 4, !tbaa !3
  %5 = load volatile i32, i32* %2, align 4, !tbaa !3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3)
  ret i32 %5
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64, i8* nocapture) #2

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64, i8* nocapture) #2

; Function Attrs: norecurse nounwind ssp uwtable
define i32 @overwrite(i32*, i32) local_unnamed_addr #0 {
  store volatile i32 0, i32* %0, align 4, !tbaa !3
  store volatile i32 2, i32* %0, align 4, !tbaa !3
  %3 = add nsw i32 %1, -3
  store volatile i32 %3, i32* %0, align 4, !tbaa !3
  %4 = load volatile i32, i32* %0, align 4, !tbaa !3
  ret i32 %4
}

; Function Attrs: norecurse nounwind ssp uwtable
define i32 @load_and_store_mult(i32*, i32) local_unnamed_addr #0 {
  store volatile i32 %1, i32* %0, align 4, !tbaa !3
  %3 = load volatile i32, i32* %0, align 4, !tbaa !3
  %4 = add nsw i32 %3, 10
  store volatile i32 %4, i32* %0, align 4, !tbaa !3
  %5 = load volatile i32, i32* %0, align 4, !tbaa !3
  %6 = add nsw i32 %5, -13
  store volatile i32 %6, i32* %0, align 4, !tbaa !3
  %7 = load volatile i32, i32* %0, align 4, !tbaa !3
  ret i32 %7
}

; Function Attrs: norecurse nounwind ssp uwtable
define i32 @array(i32*, i32) local_unnamed_addr #0 {
  %3 = add nsw i32 %1, -3
  %4 = getelementptr inbounds i32, i32* %0, i64 10
  store volatile i32 %3, i32* %4, align 4, !tbaa !3
  %5 = add nsw i32 %1, 3
  store volatile i32 %5, i32* %0, align 4, !tbaa !3
  %6 = load volatile i32, i32* %4, align 4, !tbaa !3
  ret i32 %6
}

; Function Attrs: norecurse nounwind ssp uwtable
define i32 @pointer_arith(i32*, i32) local_unnamed_addr #0 {
  store volatile i32 %1, i32* %0, align 4, !tbaa !3
  %3 = getelementptr inbounds i32, i32* %0, i64 1
  %4 = add nsw i32 %1, -1
  store volatile i32 %4, i32* %3, align 4, !tbaa !3
  %5 = getelementptr inbounds i32, i32* %0, i64 2
  %6 = add nsw i32 %1, -2
  store volatile i32 %6, i32* %5, align 4, !tbaa !3
  %7 = getelementptr inbounds i32, i32* %0, i64 3
  %8 = add nsw i32 %1, -3
  store volatile i32 %8, i32* %7, align 4, !tbaa !3
  %9 = getelementptr inbounds i32, i32* %0, i64 4
  %10 = add nsw i32 %1, -4
  store volatile i32 %10, i32* %9, align 4, !tbaa !3
  %11 = getelementptr inbounds i32, i32* %0, i64 5
  %12 = add nsw i32 %1, -5
  store volatile i32 %12, i32* %11, align 4, !tbaa !3
  %13 = getelementptr inbounds i32, i32* %0, i64 6
  %14 = add nsw i32 %1, -6
  store volatile i32 %14, i32* %13, align 4, !tbaa !3
  %15 = load volatile i32, i32* %7, align 4, !tbaa !3
  ret i32 %15
}

attributes #0 = { norecurse nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { argmemonly nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 7.0.1 (tags/RELEASE_701/final)"}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
