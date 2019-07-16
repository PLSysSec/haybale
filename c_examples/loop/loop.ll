; ModuleID = 'loop/loop.c'
source_filename = "loop/loop.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

; Function Attrs: nounwind ssp uwtable
define i32 @while_loop(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %4)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  %5 = bitcast i32* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %5)
  store volatile i32 0, i32* %3, align 4, !tbaa !3
  br label %6

; <label>:6:                                      ; preds = %6, %1
  %7 = load volatile i32, i32* %2, align 4, !tbaa !3
  %8 = add nsw i32 %7, 1
  store volatile i32 %8, i32* %2, align 4, !tbaa !3
  %9 = load volatile i32, i32* %3, align 4, !tbaa !3
  %10 = add nsw i32 %9, 1
  store volatile i32 %10, i32* %3, align 4, !tbaa !3
  %11 = icmp slt i32 %10, %0
  br i1 %11, label %6, label %12

; <label>:12:                                     ; preds = %6
  %13 = load volatile i32, i32* %2, align 4, !tbaa !3
  %14 = add nsw i32 %13, -3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %5)
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %4)
  ret i32 %14
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64, i8* nocapture) #1

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64, i8* nocapture) #1

; Function Attrs: nounwind ssp uwtable
define i32 @for_loop(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  %4 = icmp sgt i32 %0, 0
  %5 = load volatile i32, i32* %2, align 4, !tbaa !3
  br i1 %4, label %9, label %6

; <label>:6:                                      ; preds = %9, %1
  %7 = phi i32 [ %5, %1 ], [ %14, %9 ]
  %8 = add nsw i32 %7, -3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3)
  ret i32 %8

; <label>:9:                                      ; preds = %1, %9
  %10 = phi i32 [ %14, %9 ], [ %5, %1 ]
  %11 = phi i32 [ %13, %9 ], [ 0, %1 ]
  %12 = add nsw i32 %10, 1
  store volatile i32 %12, i32* %2, align 4, !tbaa !3
  %13 = add nuw nsw i32 %11, 1
  %14 = load volatile i32, i32* %2, align 4, !tbaa !3
  %15 = icmp eq i32 %13, %0
  br i1 %15, label %6, label %9
}

; Function Attrs: nounwind ssp uwtable
define i32 @loop_zero_iterations(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3)
  store volatile i32 3, i32* %2, align 4, !tbaa !3
  %4 = icmp sgt i32 %0, 0
  %5 = icmp sgt i32 %0, -1
  %6 = load volatile i32, i32* %2, align 4, !tbaa !3
  br i1 %4, label %10, label %7

; <label>:7:                                      ; preds = %10, %1
  %8 = phi i32 [ %6, %1 ], [ %17, %10 ]
  %9 = add nsw i32 %8, -3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3)
  ret i32 %9

; <label>:10:                                     ; preds = %1, %10
  %11 = phi i32 [ %17, %10 ], [ %6, %1 ]
  %12 = phi i32 [ %14, %10 ], [ 0, %1 ]
  %13 = add nsw i32 %11, 1
  store volatile i32 %13, i32* %2, align 4, !tbaa !3
  %14 = add nuw nsw i32 %12, 1
  %15 = icmp slt i32 %14, %0
  %16 = and i1 %5, %15
  %17 = load volatile i32, i32* %2, align 4, !tbaa !3
  br i1 %16, label %10, label %7
}

; Function Attrs: nounwind ssp uwtable
define i32 @loop_with_cond(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %4)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  %5 = bitcast i32* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %5)
  store volatile i32 0, i32* %3, align 4, !tbaa !3
  br label %6

; <label>:6:                                      ; preds = %16, %1
  %7 = load volatile i32, i32* %3, align 4, !tbaa !3
  %8 = srem i32 %7, 3
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %13, label %10

; <label>:10:                                     ; preds = %6
  %11 = load volatile i32, i32* %3, align 4, !tbaa !3
  %12 = icmp sgt i32 %11, 6
  br i1 %12, label %13, label %16

; <label>:13:                                     ; preds = %10, %6
  %14 = load volatile i32, i32* %2, align 4, !tbaa !3
  %15 = add nsw i32 %14, 1
  store volatile i32 %15, i32* %2, align 4, !tbaa !3
  br label %16

; <label>:16:                                     ; preds = %10, %13
  %17 = load volatile i32, i32* %3, align 4, !tbaa !3
  %18 = add nsw i32 %17, 1
  store volatile i32 %18, i32* %3, align 4, !tbaa !3
  %19 = icmp slt i32 %18, %0
  br i1 %19, label %6, label %20

; <label>:20:                                     ; preds = %16
  %21 = load volatile i32, i32* %2, align 4, !tbaa !3
  %22 = add nsw i32 %21, -3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %5)
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %4)
  ret i32 %22
}

; Function Attrs: nounwind ssp uwtable
define i32 @loop_inside_cond(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  %4 = icmp sgt i32 %0, 7
  br i1 %4, label %5, label %11

; <label>:5:                                      ; preds = %1, %5
  %6 = phi i32 [ %9, %5 ], [ 0, %1 ]
  %7 = load volatile i32, i32* %2, align 4, !tbaa !3
  %8 = add nsw i32 %7, 1
  store volatile i32 %8, i32* %2, align 4, !tbaa !3
  %9 = add nuw nsw i32 %6, 1
  %10 = icmp eq i32 %9, 3
  br i1 %10, label %12, label %5

; <label>:11:                                     ; preds = %1
  store volatile i32 2, i32* %2, align 4, !tbaa !3
  br label %12

; <label>:12:                                     ; preds = %5, %11
  %13 = load volatile i32, i32* %2, align 4, !tbaa !3
  %14 = add nsw i32 %13, -3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3)
  ret i32 %14
}

; Function Attrs: nounwind ssp uwtable
define i32 @loop_over_array(i32) local_unnamed_addr #0 {
  %2 = alloca [10 x i32], align 16
  %3 = bitcast [10 x i32]* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 40, i8* nonnull %3) #2
  br label %7

; <label>:4:                                      ; preds = %7
  %5 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 3
  %6 = load volatile i32, i32* %5, align 4, !tbaa !3
  call void @llvm.lifetime.end.p0i8(i64 40, i8* nonnull %3) #2
  ret i32 %6

; <label>:7:                                      ; preds = %7, %1
  %8 = phi i64 [ 0, %1 ], [ %12, %7 ]
  %9 = trunc i64 %8 to i32
  %10 = sub nsw i32 %0, %9
  %11 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 %8
  store volatile i32 %10, i32* %11, align 4, !tbaa !3
  %12 = add nuw nsw i64 %8, 1
  %13 = icmp eq i64 %12, 10
  br i1 %13, label %4, label %7
}

; Function Attrs: nounwind ssp uwtable
define i32 @sum_of_array(i32) local_unnamed_addr #0 {
  %2 = alloca [10 x i32], align 16
  %3 = bitcast [10 x i32]* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 40, i8* nonnull %3) #2
  br label %4

; <label>:4:                                      ; preds = %4, %1
  %5 = phi i64 [ 0, %1 ], [ %7, %4 ]
  %6 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 %5
  store volatile i32 %0, i32* %6, align 4, !tbaa !3
  %7 = add nuw nsw i64 %5, 1
  %8 = icmp eq i64 %7, 10
  br i1 %8, label %11, label %4

; <label>:9:                                      ; preds = %11
  %10 = add nsw i32 %16, -30
  call void @llvm.lifetime.end.p0i8(i64 40, i8* nonnull %3) #2
  ret i32 %10

; <label>:11:                                     ; preds = %4, %11
  %12 = phi i64 [ %17, %11 ], [ 0, %4 ]
  %13 = phi i32 [ %16, %11 ], [ 0, %4 ]
  %14 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 %12
  %15 = load volatile i32, i32* %14, align 4, !tbaa !3
  %16 = add nsw i32 %15, %13
  %17 = add nuw nsw i64 %12, 1
  %18 = icmp eq i64 %17, 10
  br i1 %18, label %9, label %11
}

; Function Attrs: nounwind ssp uwtable
define i32 @search_array(i32) local_unnamed_addr #0 {
  %2 = alloca [10 x i32], align 16
  %3 = bitcast [10 x i32]* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 40, i8* nonnull %3) #2
  br label %4

; <label>:4:                                      ; preds = %4, %1
  %5 = phi i64 [ 0, %1 ], [ %9, %4 ]
  %6 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 %5
  %7 = trunc i64 %5 to i32
  %8 = mul i32 %7, 3
  store volatile i32 %8, i32* %6, align 4, !tbaa !3
  %9 = add nuw nsw i64 %5, 1
  %10 = icmp eq i64 %9, 10
  br i1 %10, label %11, label %4

; <label>:11:                                     ; preds = %4, %16
  %12 = phi i64 [ %17, %16 ], [ 0, %4 ]
  %13 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 %12
  %14 = load volatile i32, i32* %13, align 4, !tbaa !3
  %15 = icmp sgt i32 %14, 9
  br i1 %15, label %19, label %16

; <label>:16:                                     ; preds = %11
  %17 = add nuw nsw i64 %12, 1
  %18 = icmp ult i64 %17, 10
  br i1 %18, label %11, label %21

; <label>:19:                                     ; preds = %11
  %20 = trunc i64 %12 to i32
  br label %21

; <label>:21:                                     ; preds = %16, %19
  %22 = phi i32 [ %20, %19 ], [ 0, %16 ]
  %23 = sub nsw i32 %0, %22
  call void @llvm.lifetime.end.p0i8(i64 40, i8* nonnull %3) #2
  ret i32 %23
}

; Function Attrs: nounwind ssp uwtable
define i32 @nested_loop(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  %4 = icmp sgt i32 %0, 0
  br i1 %4, label %5, label %7

; <label>:5:                                      ; preds = %1, %10
  %6 = phi i32 [ %11, %10 ], [ 0, %1 ]
  br label %13

; <label>:7:                                      ; preds = %10, %1
  %8 = load volatile i32, i32* %2, align 4, !tbaa !3
  %9 = add nsw i32 %8, -30
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3)
  ret i32 %9

; <label>:10:                                     ; preds = %13
  %11 = add nuw nsw i32 %6, 1
  %12 = icmp eq i32 %11, %0
  br i1 %12, label %7, label %5

; <label>:13:                                     ; preds = %13, %5
  %14 = phi i32 [ 0, %5 ], [ %17, %13 ]
  %15 = load volatile i32, i32* %2, align 4, !tbaa !3
  %16 = add nsw i32 %15, 1
  store volatile i32 %16, i32* %2, align 4, !tbaa !3
  %17 = add nuw nsw i32 %14, 1
  %18 = icmp eq i32 %17, 10
  br i1 %18, label %10, label %13
}

attributes #0 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind }
attributes #2 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 8.0.0 (tags/RELEASE_800/final)"}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
