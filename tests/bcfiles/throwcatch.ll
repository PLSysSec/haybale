; ModuleID = 'throwcatch.cpp'
source_filename = "throwcatch.cpp"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

@_ZTIi = external constant i8*
@_ZTIh = external constant i8*

; Function Attrs: ssp uwtable
define i32 @doesnt_throw(i32) local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %2 = alloca i32, align 4
  %3 = alloca i8, align 1
  %4 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %4)
  store volatile i32 0, i32* %2, align 4, !tbaa !3
  call void @llvm.lifetime.start.p0i8(i64 1, i8* nonnull %3)
  store volatile i8 0, i8* %3, align 1, !tbaa !7
  store volatile i32 10, i32* %2, align 4, !tbaa !3
  %5 = load volatile i8, i8* %3, align 1, !tbaa !7
  %6 = and i8 %5, 1
  %7 = icmp eq i8 %6, 0
  br i1 %7, label %15, label %8

8:                                                ; preds = %1
  %9 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %10 = bitcast i8* %9 to i32*
  store i32 20, i32* %10, align 16, !tbaa !3
  invoke void @__cxa_throw(i8* %9, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %35 unwind label %11

11:                                               ; preds = %8
  %12 = landingpad { i8*, i32 }
          catch i8* null
  %13 = extractvalue { i8*, i32 } %12, 0
  %14 = tail call i8* @__cxa_begin_catch(i8* %13) #4
  tail call void @__cxa_end_catch()
  br label %33

15:                                               ; preds = %1
  %16 = load volatile i32, i32* %2, align 4, !tbaa !3
  %17 = add nsw i32 %16, 1
  store volatile i32 %17, i32* %2, align 4, !tbaa !3
  %18 = load volatile i8, i8* %3, align 1, !tbaa !7
  %19 = and i8 %18, 1
  %20 = icmp eq i8 %19, 0
  br i1 %20, label %28, label %21

21:                                               ; preds = %15
  %22 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %23 = bitcast i8* %22 to i32*
  store i32 20, i32* %23, align 16, !tbaa !3
  invoke void @__cxa_throw(i8* %22, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %35 unwind label %24

24:                                               ; preds = %21
  %25 = landingpad { i8*, i32 }
          catch i8* null
  %26 = extractvalue { i8*, i32 } %25, 0
  %27 = tail call i8* @__cxa_begin_catch(i8* %26) #4
  tail call void @__cxa_end_catch()
  br label %33

28:                                               ; preds = %15
  %29 = load volatile i32, i32* %2, align 4, !tbaa !3
  %30 = add nsw i32 %29, %0
  %31 = icmp slt i32 %30, 100
  %32 = select i1 %31, i32 1, i32 2
  br label %33

33:                                               ; preds = %28, %24, %11
  %34 = phi i32 [ -1, %11 ], [ -2, %24 ], [ %32, %28 ]
  call void @llvm.lifetime.end.p0i8(i64 1, i8* nonnull %3)
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %4)
  ret i32 %34

35:                                               ; preds = %21, %8
  unreachable
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

declare i8* @__cxa_allocate_exception(i64) local_unnamed_addr

declare void @__cxa_throw(i8*, i8*, i8*) local_unnamed_addr

declare i32 @__gxx_personality_v0(...)

declare i8* @__cxa_begin_catch(i8*) local_unnamed_addr

declare void @__cxa_end_catch() local_unnamed_addr

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: ssp uwtable
define i32 @_Z14throw_uncaughti(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  store volatile i32 %0, i32* %2, align 4, !tbaa !3
  %3 = load volatile i32, i32* %2, align 4, !tbaa !3
  %4 = and i32 %3, 1
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %7, label %6

6:                                                ; preds = %1
  ret i32 2

7:                                                ; preds = %1
  %8 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %9 = bitcast i8* %8 to i32*
  store i32 20, i32* %9, align 16, !tbaa !3
  tail call void @__cxa_throw(i8* %8, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
  unreachable
}

; Function Attrs: ssp uwtable
define i32 @_Z21throw_multiple_valuesi(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  store volatile i32 %0, i32* %2, align 4, !tbaa !3
  %3 = load volatile i32, i32* %2, align 4, !tbaa !3
  %4 = srem i32 %3, 4
  switch i32 %4, label %9 [
    i32 1, label %12
    i32 2, label %5
    i32 3, label %6
  ]

5:                                                ; preds = %1
  br label %12

6:                                                ; preds = %1
  %7 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %8 = bitcast i8* %7 to i32*
  store i32 3, i32* %8, align 16, !tbaa !3
  tail call void @__cxa_throw(i8* %7, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
  unreachable

9:                                                ; preds = %1
  %10 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %11 = bitcast i8* %10 to i32*
  store i32 4, i32* %11, align 16, !tbaa !3
  tail call void @__cxa_throw(i8* %10, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
  unreachable

12:                                               ; preds = %1, %5
  %13 = phi i32 [ 2, %5 ], [ %4, %1 ]
  ret i32 %13
}

; Function Attrs: ssp uwtable
define i32 @_Z24throw_uncaught_wrongtypei(i32) local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %2 = alloca i32, align 4
  store volatile i32 %0, i32* %2, align 4, !tbaa !3
  %3 = load volatile i32, i32* %2, align 4, !tbaa !3
  %4 = and i32 %3, 1
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %6, label %17

6:                                                ; preds = %1
  %7 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %8 = bitcast i8* %7 to i32*
  store i32 20, i32* %8, align 16, !tbaa !3
  invoke void @__cxa_throw(i8* %7, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %20 unwind label %9

9:                                                ; preds = %6
  %10 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIh to i8*)
  %11 = extractvalue { i8*, i32 } %10, 1
  %12 = tail call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIh to i8*)) #4
  %13 = icmp eq i32 %11, %12
  br i1 %13, label %14, label %19

14:                                               ; preds = %9
  %15 = extractvalue { i8*, i32 } %10, 0
  %16 = tail call i8* @__cxa_begin_catch(i8* %15) #4
  tail call void @__cxa_end_catch() #4
  br label %17

17:                                               ; preds = %1, %14
  %18 = phi i32 [ 10, %14 ], [ 2, %1 ]
  ret i32 %18

19:                                               ; preds = %9
  resume { i8*, i32 } %10

20:                                               ; preds = %6
  unreachable
}

; Function Attrs: nounwind readnone
declare i32 @llvm.eh.typeid.for(i8*) #2

; Function Attrs: noinline ssp uwtable
define void @_Z19throw_uncaught_voidPVi(i32*) local_unnamed_addr #3 {
  %2 = load volatile i32, i32* %0, align 4, !tbaa !3
  %3 = icmp eq i32 %2, 0
  br i1 %3, label %4, label %5

4:                                                ; preds = %1
  store volatile i32 1, i32* %0, align 4, !tbaa !3
  ret void

5:                                                ; preds = %1
  %6 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %7 = bitcast i8* %6 to i32*
  store i32 20, i32* %7, align 16, !tbaa !3
  tail call void @__cxa_throw(i8* %6, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
  unreachable
}

; Function Attrs: ssp uwtable
define i32 @_Z21throw_uncaught_calleri(i32) local_unnamed_addr #0 {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3) #4
  store volatile i32 %0, i32* %2, align 4, !tbaa !3
  call void @_Z19throw_uncaught_voidPVi(i32* nonnull %2)
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3) #4
  ret i32 1
}

; Function Attrs: ssp uwtable
define i32 @_Z24throw_and_catch_wildcardb(i1 zeroext) local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  br i1 %0, label %2, label %9

2:                                                ; preds = %1
  %3 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %4 = bitcast i8* %3 to i32*
  store i32 20, i32* %4, align 16, !tbaa !3
  invoke void @__cxa_throw(i8* %3, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %11 unwind label %5

5:                                                ; preds = %2
  %6 = landingpad { i8*, i32 }
          catch i8* null
  %7 = extractvalue { i8*, i32 } %6, 0
  %8 = tail call i8* @__cxa_begin_catch(i8* %7) #4
  tail call void @__cxa_end_catch()
  br label %9

9:                                                ; preds = %1, %5
  %10 = phi i32 [ 5, %5 ], [ 2, %1 ]
  ret i32 %10

11:                                               ; preds = %2
  unreachable
}

; Function Attrs: ssp uwtable
define i32 @_Z19throw_and_catch_valb(i1 zeroext) local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  br i1 %0, label %2, label %15

2:                                                ; preds = %1
  %3 = tail call i8* @__cxa_allocate_exception(i64 4) #4
  %4 = bitcast i8* %3 to i32*
  store i32 20, i32* %4, align 16, !tbaa !3
  invoke void @__cxa_throw(i8* %3, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %18 unwind label %5

5:                                                ; preds = %2
  %6 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIi to i8*)
  %7 = extractvalue { i8*, i32 } %6, 1
  %8 = tail call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIi to i8*)) #4
  %9 = icmp eq i32 %7, %8
  br i1 %9, label %10, label %17

10:                                               ; preds = %5
  %11 = extractvalue { i8*, i32 } %6, 0
  %12 = tail call i8* @__cxa_begin_catch(i8* %11) #4
  %13 = bitcast i8* %12 to i32*
  %14 = load i32, i32* %13, align 4, !tbaa !3
  tail call void @__cxa_end_catch() #4
  br label %15

15:                                               ; preds = %1, %10
  %16 = phi i32 [ %14, %10 ], [ 2, %1 ]
  ret i32 %16

17:                                               ; preds = %5
  resume { i8*, i32 } %6

18:                                               ; preds = %2
  unreachable
}

; Function Attrs: ssp uwtable
define i32 @_Z25throw_and_catch_in_callerb(i1 zeroext) local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3) #4
  store volatile i32 2, i32* %2, align 4, !tbaa !3
  br i1 %0, label %4, label %15

4:                                                ; preds = %1
  invoke void @_Z19throw_uncaught_voidPVi(i32* nonnull %2)
          to label %15 unwind label %5

5:                                                ; preds = %4
  %6 = landingpad { i8*, i32 }
          cleanup
          catch i8* bitcast (i8** @_ZTIi to i8*)
  %7 = extractvalue { i8*, i32 } %6, 1
  %8 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIi to i8*)) #4
  %9 = icmp eq i32 %7, %8
  br i1 %9, label %10, label %17

10:                                               ; preds = %5
  %11 = extractvalue { i8*, i32 } %6, 0
  %12 = call i8* @__cxa_begin_catch(i8* %11) #4
  %13 = bitcast i8* %12 to i32*
  %14 = load i32, i32* %13, align 4, !tbaa !3
  call void @__cxa_end_catch() #4
  br label %15

15:                                               ; preds = %4, %1, %10
  %16 = phi i32 [ %14, %10 ], [ 2, %1 ], [ 2, %4 ]
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3) #4
  ret i32 %16

17:                                               ; preds = %5
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3) #4
  resume { i8*, i32 } %6
}

; Function Attrs: ssp uwtable
define i32 @_Z27throw_and_rethrow_in_callerb(i1 zeroext) local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %2 = alloca i32, align 4
  %3 = bitcast i32* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3) #4
  store volatile i32 2, i32* %2, align 4, !tbaa !3
  br i1 %0, label %4, label %17

4:                                                ; preds = %1
  invoke void @_Z19throw_uncaught_voidPVi(i32* nonnull %2)
          to label %17 unwind label %5

5:                                                ; preds = %4
  %6 = landingpad { i8*, i32 }
          cleanup
          catch i8* bitcast (i8** @_ZTIi to i8*)
  %7 = extractvalue { i8*, i32 } %6, 0
  %8 = extractvalue { i8*, i32 } %6, 1
  %9 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIi to i8*)) #4
  %10 = icmp eq i32 %8, %9
  br i1 %10, label %11, label %18

11:                                               ; preds = %5
  %12 = call i8* @__cxa_begin_catch(i8* %7) #4
  invoke void @__cxa_rethrow() #5
          to label %23 unwind label %13

13:                                               ; preds = %11
  %14 = landingpad { i8*, i32 }
          cleanup
  %15 = extractvalue { i8*, i32 } %14, 0
  %16 = extractvalue { i8*, i32 } %14, 1
  call void @__cxa_end_catch() #4
  br label %18

17:                                               ; preds = %1, %4
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3) #4
  ret i32 2

18:                                               ; preds = %13, %5
  %19 = phi i32 [ %16, %13 ], [ %8, %5 ]
  %20 = phi i8* [ %15, %13 ], [ %7, %5 ]
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3) #4
  %21 = insertvalue { i8*, i32 } undef, i8* %20, 0
  %22 = insertvalue { i8*, i32 } %21, i32 %19, 1
  resume { i8*, i32 } %22

23:                                               ; preds = %11
  unreachable
}

declare void @__cxa_rethrow() local_unnamed_addr

attributes #0 = { ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind }
attributes #2 = { nounwind readnone }
attributes #3 = { noinline ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #4 = { nounwind }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.0 (tags/RELEASE_900/final)"}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C++ TBAA"}
!7 = !{!8, !8, i64 0}
!8 = !{!"bool", !5, i64 0}
