; ModuleID = 'functionptr.c'
source_filename = "functionptr.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

%struct.StructWithFuncPtr = type { i32, i32 (i32, i32)* }

; Function Attrs: noinline norecurse nounwind readnone ssp uwtable
define i32 @foo(i32, i32) #0 {
  %3 = add nsw i32 %1, 3
  %4 = mul nsw i32 %3, %0
  ret i32 %4
}

; Function Attrs: noinline norecurse nounwind readnone ssp uwtable
define i32 @bar(i32, i32) #0 {
  %3 = sub nsw i32 %0, %1
  ret i32 %3
}

; Function Attrs: noinline nounwind ssp uwtable
define i32 @calls_fptr(i32 (i32, i32)*, i32) local_unnamed_addr #1 {
  %3 = alloca i32 (i32, i32)*, align 8
  store volatile i32 (i32, i32)* %0, i32 (i32, i32)** %3, align 8, !tbaa !3
  %4 = load volatile i32 (i32, i32)*, i32 (i32, i32)** %3, align 8, !tbaa !3
  %5 = tail call i32 %4(i32 2, i32 3) #4
  %6 = add nsw i32 %5, %1
  ret i32 %6
}

; Function Attrs: noinline norecurse nounwind readnone ssp uwtable
define nonnull i32 (i32, i32)* @get_function_ptr(i1 zeroext) local_unnamed_addr #0 {
  %2 = select i1 %0, i32 (i32, i32)* @foo, i32 (i32, i32)* @bar
  ret i32 (i32, i32)* %2
}

; Function Attrs: nounwind ssp uwtable
define i32 @fptr_driver() local_unnamed_addr #2 {
  %1 = alloca i32 (i32, i32)*, align 8
  %2 = bitcast i32 (i32, i32)** %1 to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %2)
  %3 = tail call i32 (i32, i32)* @get_function_ptr(i1 zeroext true)
  store volatile i32 (i32, i32)* %3, i32 (i32, i32)** %1, align 8, !tbaa !3
  %4 = load volatile i32 (i32, i32)*, i32 (i32, i32)** %1, align 8, !tbaa !3
  %5 = tail call i32 @calls_fptr(i32 (i32, i32)* %4, i32 10)
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %2)
  ret i32 %5
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #3

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #3

; Function Attrs: noinline nounwind ssp uwtable
define i32 @calls_through_struct(%struct.StructWithFuncPtr*) local_unnamed_addr #1 {
  %2 = getelementptr inbounds %struct.StructWithFuncPtr, %struct.StructWithFuncPtr* %0, i64 0, i32 1
  %3 = load volatile i32 (i32, i32)*, i32 (i32, i32)** %2, align 8, !tbaa !7
  %4 = getelementptr inbounds %struct.StructWithFuncPtr, %struct.StructWithFuncPtr* %0, i64 0, i32 0
  %5 = load volatile i32, i32* %4, align 8, !tbaa !10
  %6 = tail call i32 %3(i32 %5, i32 2) #4
  ret i32 %6
}

; Function Attrs: nounwind ssp uwtable
define i32 @struct_driver() local_unnamed_addr #2 {
  %1 = alloca %struct.StructWithFuncPtr, align 8
  %2 = bitcast %struct.StructWithFuncPtr* %1 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %2) #4
  call void @llvm.memset.p0i8.i64(i8* nonnull align 8 %2, i8 0, i64 16, i1 true)
  %3 = tail call i32 (i32, i32)* @get_function_ptr(i1 zeroext true)
  %4 = getelementptr inbounds %struct.StructWithFuncPtr, %struct.StructWithFuncPtr* %1, i64 0, i32 1
  store volatile i32 (i32, i32)* %3, i32 (i32, i32)** %4, align 8, !tbaa !7
  %5 = getelementptr inbounds %struct.StructWithFuncPtr, %struct.StructWithFuncPtr* %1, i64 0, i32 0
  store volatile i32 3, i32* %5, align 8, !tbaa !10
  %6 = call i32 @calls_through_struct(%struct.StructWithFuncPtr* nonnull %1)
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %2) #4
  ret i32 %6
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #3

attributes #0 = { noinline norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { noinline nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { argmemonly nounwind }
attributes #4 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
!3 = !{!4, !4, i64 0}
!4 = !{!"any pointer", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
!7 = !{!8, !4, i64 8}
!8 = !{!"StructWithFuncPtr", !9, i64 0, !4, i64 8}
!9 = !{!"int", !5, i64 0}
!10 = !{!8, !9, i64 0}
