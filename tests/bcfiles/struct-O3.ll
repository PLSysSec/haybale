; ModuleID = 'struct-O3.c'
source_filename = "struct-O3.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

%struct.WithPointer = type { %struct.TwoInts, %struct.TwoInts** }
%struct.TwoInts = type { i32, i32 }

; Function Attrs: nofree noinline norecurse nounwind ssp uwtable
define i32 @called(%struct.WithPointer*, i32) local_unnamed_addr #0 {
  %3 = add nsw i32 %1, -3
  %4 = getelementptr inbounds %struct.WithPointer, %struct.WithPointer* %0, i64 0, i32 0, i32 1
  store volatile i32 %3, i32* %4, align 4, !tbaa !3
  %5 = getelementptr inbounds %struct.WithPointer, %struct.WithPointer* %0, i64 0, i32 1
  %6 = load %struct.TwoInts**, %struct.TwoInts*** %5, align 8, !tbaa !10
  %7 = load volatile %struct.TwoInts*, %struct.TwoInts** %6, align 8, !tbaa !11
  %8 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %7, i64 0, i32 1
  %9 = load volatile i32, i32* %8, align 4, !tbaa !12
  ret i32 %9
}

; Function Attrs: nounwind ssp uwtable
define i32 @with_ptr(i32) local_unnamed_addr #1 {
  %2 = alloca %struct.TwoInts*, align 8
  %3 = bitcast %struct.TwoInts** %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %3) #4
  %4 = tail call i8* @malloc(i64 8) #5
  %5 = bitcast %struct.TwoInts** %2 to i8**
  store i8* %4, i8** %5, align 8, !tbaa !11
  %6 = tail call i8* @malloc(i64 16) #5
  %7 = icmp eq i8* %6, null
  %8 = icmp eq i8* %4, null
  %9 = or i1 %8, %7
  br i1 %9, label %18, label %10

10:                                               ; preds = %1
  %11 = bitcast i8* %6 to %struct.WithPointer*
  %12 = bitcast i8* %6 to %struct.TwoInts*
  %13 = getelementptr inbounds i8, i8* %6, i64 4
  %14 = bitcast i8* %13 to i32*
  store volatile i32 0, i32* %14, align 4, !tbaa !3
  %15 = getelementptr inbounds i8, i8* %6, i64 8
  %16 = bitcast i8* %15 to %struct.TwoInts***
  store %struct.TwoInts** %2, %struct.TwoInts*** %16, align 8, !tbaa !10
  store volatile %struct.TwoInts* %12, %struct.TwoInts** %2, align 8, !tbaa !11
  %17 = call i32 @called(%struct.WithPointer* %11, i32 %0)
  br label %18

18:                                               ; preds = %1, %10
  %19 = phi i32 [ %17, %10 ], [ -1, %1 ]
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %3) #4
  ret i32 %19
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #2

; Function Attrs: nofree nounwind allocsize(0)
declare noalias i8* @malloc(i64) local_unnamed_addr #3

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #2

attributes #0 = { nofree noinline norecurse nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { argmemonly nounwind }
attributes #3 = { nofree nounwind allocsize(0) "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #4 = { nounwind }
attributes #5 = { allocsize(0) }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
!3 = !{!4, !6, i64 4}
!4 = !{!"WithPointer", !5, i64 0, !9, i64 8}
!5 = !{!"TwoInts", !6, i64 0, !6, i64 4}
!6 = !{!"int", !7, i64 0}
!7 = !{!"omnipotent char", !8, i64 0}
!8 = !{!"Simple C/C++ TBAA"}
!9 = !{!"any pointer", !7, i64 0}
!10 = !{!4, !9, i64 8}
!11 = !{!9, !9, i64 0}
!12 = !{!5, !6, i64 4}
