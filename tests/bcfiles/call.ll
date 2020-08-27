; ModuleID = 'call.c'
source_filename = "call.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

; Function Attrs: noinline norecurse nounwind readnone ssp uwtable
define i32 @simple_callee(i32, i32) local_unnamed_addr #0 {
  %3 = sub nsw i32 %0, %1
  ret i32 %3
}

; Function Attrs: noinline norecurse nounwind readnone ssp uwtable
define i32 @simple_caller(i32) local_unnamed_addr #0 {
  %2 = tail call i32 @simple_callee(i32 %0, i32 3)
  ret i32 %2
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @conditional_caller(i32, i32) local_unnamed_addr #1 {
  %3 = icmp sgt i32 %1, 5
  br i1 %3, label %4, label %6

4:                                                ; preds = %2
  %5 = tail call i32 @simple_callee(i32 %0, i32 3)
  br label %8

6:                                                ; preds = %2
  %7 = add nsw i32 %1, 10
  br label %8

8:                                                ; preds = %6, %4
  %9 = phi i32 [ %5, %4 ], [ %7, %6 ]
  ret i32 %9
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @twice_caller(i32) local_unnamed_addr #1 {
  %2 = tail call i32 @simple_callee(i32 %0, i32 5)
  %3 = tail call i32 @simple_callee(i32 %0, i32 1)
  %4 = add nsw i32 %3, %2
  ret i32 %4
}

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @nested_caller(i32, i32) local_unnamed_addr #1 {
  %3 = add nsw i32 %1, %0
  %4 = tail call i32 @simple_caller(i32 %3)
  ret i32 %4
}

; Function Attrs: noinline nounwind ssp uwtable
define i32 @callee_with_loop(i32, i32) local_unnamed_addr #2 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = bitcast i32* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %5)
  store volatile i32 0, i32* %3, align 4, !tbaa !3
  %6 = bitcast i32* %4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %6)
  store volatile i32 0, i32* %4, align 4, !tbaa !3
  %7 = load volatile i32, i32* %4, align 4, !tbaa !3
  %8 = icmp slt i32 %7, %0
  br i1 %8, label %13, label %9

9:                                                ; preds = %13, %2
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %6)
  %10 = load volatile i32, i32* %3, align 4, !tbaa !3
  %11 = sub i32 -27, %1
  %12 = add i32 %11, %10
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %5)
  ret i32 %12

13:                                               ; preds = %2, %13
  %14 = load volatile i32, i32* %3, align 4, !tbaa !3
  %15 = add nsw i32 %14, 10
  store volatile i32 %15, i32* %3, align 4, !tbaa !3
  %16 = load volatile i32, i32* %4, align 4, !tbaa !3
  %17 = add nsw i32 %16, 1
  store volatile i32 %17, i32* %4, align 4, !tbaa !3
  %18 = load volatile i32, i32* %4, align 4, !tbaa !3
  %19 = icmp slt i32 %18, %0
  br i1 %19, label %13, label %9
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #3

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #3

; Function Attrs: nounwind ssp uwtable
define i32 @caller_of_loop(i32) local_unnamed_addr #4 {
  %2 = tail call i32 @callee_with_loop(i32 %0, i32 3)
  ret i32 %2
}

; Function Attrs: nounwind ssp uwtable
define i32 @caller_with_loop(i32) local_unnamed_addr #4 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  %4 = load volatile i32, i32* %2, align 4, !tbaa !3
  %5 = icmp slt i32 %4, %0
  br i1 %5, label %10, label %8

6:                                                ; preds = %10
  %7 = add i32 %14, -14
  br label %8

8:                                                ; preds = %6, %1
  %9 = phi i32 [ -14, %1 ], [ %7, %6 ]
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3)
  ret i32 %9

10:                                               ; preds = %1, %10
  %11 = phi i32 [ %14, %10 ], [ 0, %1 ]
  %12 = add nsw i32 %11, 3
  %13 = tail call i32 @simple_callee(i32 %12, i32 1)
  %14 = add nsw i32 %13, %11
  %15 = load volatile i32, i32* %2, align 4, !tbaa !3
  %16 = add nsw i32 %15, 1
  store volatile i32 %16, i32* %2, align 4, !tbaa !3
  %17 = load volatile i32, i32* %2, align 4, !tbaa !3
  %18 = icmp slt i32 %17, %0
  br i1 %18, label %10, label %6
}

; Function Attrs: noinline nounwind readnone ssp uwtable
define i32 @recursive_simple(i32) local_unnamed_addr #5 {
  %2 = shl nsw i32 %0, 1
  %3 = icmp slt i32 %0, -1000
  br i1 %3, label %9, label %4

4:                                                ; preds = %1
  %5 = icmp sgt i32 %0, 12
  br i1 %5, label %9, label %6

6:                                                ; preds = %4
  %7 = tail call i32 @recursive_simple(i32 %2)
  %8 = add nsw i32 %7, -44
  ret i32 %8

9:                                                ; preds = %4, %1
  %10 = phi i32 [ -1, %1 ], [ %2, %4 ]
  ret i32 %10
}

; Function Attrs: noinline nounwind readnone ssp uwtable
define i32 @recursive_double(i32) local_unnamed_addr #5 {
  %2 = shl nsw i32 %0, 1
  %3 = icmp slt i32 %0, -1000
  br i1 %3, label %20, label %4

4:                                                ; preds = %1
  %5 = icmp sgt i32 %0, 500
  br i1 %5, label %20, label %6

6:                                                ; preds = %4
  %7 = icmp sgt i32 %0, 12
  br i1 %7, label %8, label %12

8:                                                ; preds = %6
  %9 = add nsw i32 %2, 7
  %10 = tail call i32 @recursive_double(i32 %9)
  %11 = add nsw i32 %10, 1
  ret i32 %11

12:                                               ; preds = %6
  %13 = icmp slt i32 %0, -5
  br i1 %13, label %14, label %18

14:                                               ; preds = %12
  %15 = sub nsw i32 0, %2
  %16 = tail call i32 @recursive_double(i32 %15)
  %17 = add nsw i32 %16, -1
  ret i32 %17

18:                                               ; preds = %12
  %19 = add nsw i32 %2, -23
  br label %20

20:                                               ; preds = %4, %1, %18
  %21 = phi i32 [ %19, %18 ], [ -1, %1 ], [ %2, %4 ]
  ret i32 %21
}

; Function Attrs: noinline nounwind readnone ssp uwtable
define i32 @recursive_not_tail(i32) local_unnamed_addr #5 {
  %2 = icmp sgt i32 %0, 100
  br i1 %2, label %3, label %5

3:                                                ; preds = %1
  %4 = add nuw nsw i32 %0, 10
  br label %15

5:                                                ; preds = %1
  %6 = add nsw i32 %0, 20
  %7 = tail call i32 @recursive_not_tail(i32 %6)
  %8 = and i32 %7, 1
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %10, label %12

10:                                               ; preds = %5
  %11 = srem i32 %7, 3
  br label %15

12:                                               ; preds = %5
  %13 = srem i32 %7, 5
  %14 = add nsw i32 %13, -8
  br label %15

15:                                               ; preds = %10, %12, %3
  %16 = phi i32 [ %4, %3 ], [ %11, %10 ], [ %14, %12 ]
  ret i32 %16
}

; Function Attrs: noinline nounwind readnone ssp uwtable
define i32 @recursive_and_normal_caller(i32) local_unnamed_addr #5 {
  %2 = icmp slt i32 %0, 0
  br i1 %2, label %10, label %3

3:                                                ; preds = %1
  %4 = shl nsw i32 %0, 1
  %5 = tail call i32 @simple_callee(i32 %4, i32 3)
  %6 = icmp sgt i32 %5, 25
  br i1 %6, label %10, label %7

7:                                                ; preds = %3
  %8 = tail call i32 @recursive_and_normal_caller(i32 %4)
  %9 = add nsw i32 %8, -44
  ret i32 %9

10:                                               ; preds = %3, %1
  %11 = phi i32 [ -1, %1 ], [ %4, %3 ]
  ret i32 %11
}

; Function Attrs: noinline nounwind readnone ssp uwtable
define i32 @mutually_recursive_a(i32) local_unnamed_addr #5 {
  %2 = icmp sgt i32 %0, 5
  br i1 %2, label %7, label %3

3:                                                ; preds = %1
  %4 = shl nsw i32 %0, 1
  %5 = tail call i32 @mutually_recursive_b(i32 %4)
  %6 = add nsw i32 %5, -1
  br label %7

7:                                                ; preds = %1, %3
  %8 = phi i32 [ %6, %3 ], [ %0, %1 ]
  ret i32 %8
}

; Function Attrs: noinline nounwind readnone ssp uwtable
define i32 @mutually_recursive_b(i32) local_unnamed_addr #5 {
  %2 = icmp slt i32 %0, 0
  br i1 %2, label %7, label %3

3:                                                ; preds = %1
  %4 = add nsw i32 %0, -2
  %5 = tail call i32 @mutually_recursive_a(i32 %4)
  %6 = add nsw i32 %5, -2
  br label %7

7:                                                ; preds = %1, %3
  %8 = phi i32 [ %6, %3 ], [ %0, %1 ]
  ret i32 %8
}

attributes #0 = { noinline norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { noinline nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { argmemonly nounwind }
attributes #4 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #5 = { noinline nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
