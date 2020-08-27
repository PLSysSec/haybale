; ModuleID = 'panic.3a1fbbbh-cgu.0'
source_filename = "panic.3a1fbbbh-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::panic::Location" = type { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
%"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [4 x i64] }
%"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [0 x i64], %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock", [0 x i64] }
%"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock" = type { [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64] }
%"core::marker::PhantomData<u8>" = type {}
%"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocErr>::Err" = type { [0 x i8], %"core::alloc::AllocErr", [0 x i8] }
%"core::alloc::AllocErr" = type {}
%"alloc::alloc::Global" = type {}
%"core::result::Result<core::alloc::MemoryBlock, core::alloc::AllocErr>::Err" = type { [0 x i8], %"core::alloc::AllocErr", [0 x i8] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* } { void ({ i8*, i64 }*)* @_ZN4core3ptr13drop_in_place17h30521acf87699e27E, i64 16, i64 8, { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h059f6afe427c0ae6E", { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hc02f6d5d8b3bc05cE" }, align 8
@vtable.1 = private unnamed_addr constant { void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* } { void ({ [0 x i8]*, i64 }*)* @_ZN4core3ptr13drop_in_place17h770c911d3e2ab738E, i64 16, i64 8, i64 ({ [0 x i8]*, i64 }*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h0005824ec84d3f86E" }, align 8
@alloc19 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"a > 2" }>, align 1
@alloc20 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"panic.rs" }>, align 1
@alloc21 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @alloc20, i32 0, i32 0, i32 0), [16 x i8] c"\08\00\00\00\00\00\00\00\03\00\00\00\09\00\00\00" }>, align 8

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17ha588496513836548E"(i8* nonnull %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h1aafa3f293363b25E"(i8* nonnull %unique)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h6627a5ab4ea7064dE"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; <T as core::any::Any>::type_id
; Function Attrs: uwtable
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h0005824ec84d3f86E"({ [0 x i8]*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17h827867d63b4b4e6bE()
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn uwtable
define void @_ZN3std9panicking11begin_panic17h5ae0871c3ba84f98E([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) %0) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_10 = alloca i8, align 1
  %_5 = alloca { i8*, i64 }, align 8
  store i8 0, i8* %_10, align 1
  store i8 1, i8* %_10, align 1
  store i8 0, i8* %_10, align 1
; invoke std::panicking::begin_panic::PanicPayload<A>::new
  %2 = invoke { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h120501dac8746813E"([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1)
          to label %bb2 unwind label %cleanup

bb1:                                              ; preds = %bb5, %bb6
  %3 = bitcast { i8*, i32 }* %1 to i8**
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %6 = load i32, i32* %5, align 8
  %7 = insertvalue { i8*, i32 } undef, i8* %4, 0
  %8 = insertvalue { i8*, i32 } %7, i32 %6, 1
  resume { i8*, i32 } %8

bb2:                                              ; preds = %start
  store { i8*, i64 } %2, { i8*, i64 }* %_5, align 8
  %_2.0 = bitcast { i8*, i64 }* %_5 to {}*
; invoke core::panic::Location::caller
  %_9 = invoke align 8 dereferenceable(24) %"core::panic::Location"* @_ZN4core5panic8Location6caller17h0ea59da85abecfa1E(%"core::panic::Location"* noalias readonly align 8 dereferenceable(24) %0)
          to label %bb4 unwind label %cleanup1

bb3:                                              ; preds = %cleanup1
  br label %bb6

bb4:                                              ; preds = %bb2
; invoke std::panicking::rust_panic_with_hook
  invoke void @_ZN3std9panicking20rust_panic_with_hook17h3fc8a110bc9d166fE({}* nonnull align 1 %_2.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* }* @vtable.0 to [3 x i64]*), i64* noalias readonly align 8 dereferenceable_or_null(48) null, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) %_9)
          to label %unreachable unwind label %cleanup1

bb5:                                              ; preds = %bb6
  store i8 0, i8* %_10, align 1
  br label %bb1

bb6:                                              ; preds = %bb3, %cleanup
  %9 = load i8, i8* %_10, align 1, !range !1
  %10 = trunc i8 %9 to i1
  br i1 %10, label %bb5, label %bb1

cleanup:                                          ; preds = %start
  %11 = landingpad { i8*, i32 }
          cleanup
  %12 = extractvalue { i8*, i32 } %11, 0
  %13 = extractvalue { i8*, i32 } %11, 1
  %14 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %12, i8** %14, align 8
  %15 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %13, i32* %15, align 8
  br label %bb6

cleanup1:                                         ; preds = %bb4, %bb2
  %16 = landingpad { i8*, i32 }
          cleanup
  %17 = extractvalue { i8*, i32 } %16, 0
  %18 = extractvalue { i8*, i32 } %16, 1
  %19 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %17, i8** %19, align 8
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %18, i32* %20, align 8
  br label %bb3

unreachable:                                      ; preds = %bb4
  unreachable
}

; std::panicking::begin_panic::PanicPayload<A>::new
; Function Attrs: uwtable
define { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h120501dac8746813E"([0 x i8]* noalias nonnull readonly align 1 %inner.0, i64 %inner.1) unnamed_addr #1 {
start:
  %_2 = alloca { i8*, i64 }, align 8
  %0 = alloca { i8*, i64 }, align 8
  %1 = bitcast { i8*, i64 }* %_2 to { [0 x i8]*, i64 }*
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %1, i32 0, i32 0
  store [0 x i8]* %inner.0, [0 x i8]** %2, align 8
  %3 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %1, i32 0, i32 1
  store i64 %inner.1, i64* %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_2, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8
  %6 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_2, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 0
  store i8* %5, i8** %8, align 8
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 1
  store i64 %7, i64* %9, align 8
  %10 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 0
  %11 = load i8*, i8** %10, align 8
  %12 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { i8*, i64 } undef, i8* %11, 0
  %15 = insertvalue { i8*, i64 } %14, i64 %13, 1
  ret { i8*, i64 } %15
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %src, i8* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 1, %count
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %dst, i8* align 1 %src, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17hded36a0cdfa854e6E({ i8*, i64 }* %src, { i8*, i64 }* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 16, %count
  %1 = bitcast { i8*, i64 }* %dst to i8*
  %2 = bitcast { i8*, i64 }* %src to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17h827867d63b4b4e6bE() unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  store i64 9147559743429524724, i64* %0, align 8
  %_1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %_1, i64* %1, align 8
  %2 = load i64, i64* %1, align 8
  ret i64 %2
}

; core::mem::swap
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem4swap17ha1c1538299dc7a85E({ i8*, i64 }* align 8 dereferenceable(16) %x, { i8*, i64 }* align 8 dereferenceable(16) %y) unnamed_addr #0 {
start:
; call core::ptr::swap_nonoverlapping_one
  call void @_ZN4core3ptr23swap_nonoverlapping_one17hd748c4ad652f3a54E({ i8*, i64 }* %x, { i8*, i64 }* %y)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3mem4take17h91657bb9670099edE({ i8*, i64 }* align 8 dereferenceable(16) %dest) unnamed_addr #0 {
start:
; call <core::option::Option<T> as core::default::Default>::default
  %0 = call { i8*, i64 } @"_ZN72_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h6d26e62efc4f54a7E"()
  %_3.0 = extractvalue { i8*, i64 } %0, 0
  %_3.1 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::mem::replace
  %1 = call { i8*, i64 } @_ZN4core3mem7replace17h7fa30ebaa35d6e64E({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1 %_3.0, i64 %_3.1)
  %2 = extractvalue { i8*, i64 } %1, 0
  %3 = extractvalue { i8*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = insertvalue { i8*, i64 } undef, i8* %2, 0
  %5 = insertvalue { i8*, i64 } %4, i64 %3, 1
  ret { i8*, i64 } %5
}

; core::mem::replace
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3mem7replace17h7fa30ebaa35d6e64E({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1 %0, i64 %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %2 = alloca { i8*, i32 }, align 8
  %src = alloca { i8*, i64 }, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 0
  store i8* %0, i8** %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; invoke core::mem::swap
  invoke void @_ZN4core3mem4swap17ha1c1538299dc7a85E({ i8*, i64 }* align 8 dereferenceable(16) %dest, { i8*, i64 }* align 8 dereferenceable(16) %src)
          to label %bb2 unwind label %cleanup

bb1:                                              ; preds = %bb3
  %5 = bitcast { i8*, i32 }* %2 to i8**
  %6 = load i8*, i8** %5, align 8
  %7 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 1
  %8 = load i32, i32* %7, align 8
  %9 = insertvalue { i8*, i32 } undef, i8* %6, 0
  %10 = insertvalue { i8*, i32 } %9, i32 %8, 1
  resume { i8*, i32 } %10

bb2:                                              ; preds = %start
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 0
  %12 = load i8*, i8** %11, align 8
  %13 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
  %15 = insertvalue { i8*, i64 } undef, i8* %12, 0
  %16 = insertvalue { i8*, i64 } %15, i64 %14, 1
  ret { i8*, i64 } %16

bb3:                                              ; preds = %cleanup
  br label %bb1

cleanup:                                          ; preds = %start
  %17 = landingpad { i8*, i32 }
          cleanup
  %18 = extractvalue { i8*, i32 } %17, 0
  %19 = extractvalue { i8*, i32 } %17, 1
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 0
  store i8* %18, i8** %20, align 8
  %21 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 1
  store i32 %19, i32* %21, align 8
  br label %bb3
}

; core::num::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h5beda99855ca8475E(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !2
  ret i64 %1
}

; core::num::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize3get17h6fb979e371d431ccE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17h233094890efa2ce5E({}* %_1.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  %1 = bitcast [3 x i64]* %_1.1 to void ({}*)**
  %2 = getelementptr inbounds void ({}*)*, void ({}*)** %1, i64 0
  %3 = load void ({}*)*, void ({}*)** %2, align 8, !invariant.load !3, !nonnull !3
  call void %3({}* %_1.0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr13drop_in_place17h30521acf87699e27E({ i8*, i64 }* %_1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17h3f8de45cb7a779d6E({ {}*, [3 x i64]* }* %_1) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
  %1 = alloca {}, align 1
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 0
  %3 = load {}*, {}** %2, align 8, !nonnull !3
  %4 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 1
  %5 = load [3 x i64]*, [3 x i64]** %4, align 8, !nonnull !3
  %6 = bitcast [3 x i64]* %5 to void ({}*)**
  %7 = getelementptr inbounds void ({}*)*, void ({}*)** %6, i64 0
  %8 = load void ({}*)*, void ({}*)** %7, align 8, !invariant.load !3, !nonnull !3
  invoke void %8({}* %3)
          to label %bb3 unwind label %cleanup

bb1:                                              ; preds = %bb3
  ret void

bb2:                                              ; preds = %bb4
  %9 = bitcast { i8*, i32 }* %0 to i8**
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %12 = load i32, i32* %11, align 8
  %13 = insertvalue { i8*, i32 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i32 } %13, i32 %12, 1
  resume { i8*, i32 } %14

bb3:                                              ; preds = %start
  %15 = bitcast { {}*, [3 x i64]* }* %_1 to { i8*, i64* }*
  %16 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %15, i32 0, i32 0
  %17 = load i8*, i8** %16, align 8, !nonnull !3
  %18 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %15, i32 0, i32 1
  %19 = load i64*, i64** %18, align 8, !nonnull !3
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h0dad36ae68ddb938E(i8* nonnull %17, i64* noalias readonly align 8 dereferenceable(24) %19)
  br label %bb1

bb4:                                              ; preds = %cleanup
  %20 = bitcast { {}*, [3 x i64]* }* %_1 to { i8*, i64* }*
  %21 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %20, i32 0, i32 0
  %22 = load i8*, i8** %21, align 8, !nonnull !3
  %23 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %20, i32 0, i32 1
  %24 = load i64*, i64** %23, align 8, !nonnull !3
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h0dad36ae68ddb938E(i8* nonnull %22, i64* noalias readonly align 8 dereferenceable(24) %24) #7
  br label %bb2

cleanup:                                          ; preds = %start
  %25 = landingpad { i8*, i32 }
          cleanup
  %26 = extractvalue { i8*, i32 } %25, 0
  %27 = extractvalue { i8*, i32 } %25, 1
  %28 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %26, i8** %28, align 8
  %29 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %27, i32* %29, align 8
  br label %bb4
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr13drop_in_place17h770c911d3e2ab738E({ [0 x i8]*, i64 }* %_1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  ret void
}

; core::ptr::swap_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr19swap_nonoverlapping17h9d345d8b9374353dE({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 %count) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %x1 = bitcast { i8*, i64 }* %x to i8*
  %y2 = bitcast { i8*, i64 }* %y to i8*
  store i64 16, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %len = mul i64 %1, %count
; call core::ptr::swap_nonoverlapping_bytes
  call void @_ZN4core3ptr25swap_nonoverlapping_bytes17h619d15c1d3f196e4E(i8* %x1, i8* %y2, i64 %len)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::ptr::swap_nonoverlapping_one
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr23swap_nonoverlapping_one17hd748c4ad652f3a54E({ i8*, i64 }* %x, { i8*, i64 }* %y) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca i64, align 8
  %1 = alloca { i8*, i32 }, align 8
  %_18 = alloca i8, align 1
  %2 = alloca {}, align 1
  store i8 0, i8* %_18, align 1
  store i64 16, i64* %0, align 8
  %3 = load i64, i64* %0, align 8
  br label %bb2

bb1:                                              ; preds = %bb10, %bb11
  %4 = bitcast { i8*, i32 }* %1 to i8**
  %5 = load i8*, i8** %4, align 8
  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %7 = load i32, i32* %6, align 8
  %8 = insertvalue { i8*, i32 } undef, i8* %5, 0
  %9 = insertvalue { i8*, i32 } %8, i32 %7, 1
  resume { i8*, i32 } %9

bb2:                                              ; preds = %start
  %_3 = icmp ult i64 %3, 32
  br i1 %_3, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
; call core::ptr::swap_nonoverlapping
  call void @_ZN4core3ptr19swap_nonoverlapping17h9d345d8b9374353dE({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 1)
  br label %bb8

bb4:                                              ; preds = %bb2
  store i8 1, i8* %_18, align 1
; call core::ptr::read
  %10 = call { i8*, i64 } @_ZN4core3ptr4read17h39f36972cd6f7088E({ i8*, i64 }* %x)
  %z.0 = extractvalue { i8*, i64 } %10, 0
  %z.1 = extractvalue { i8*, i64 } %10, 1
  br label %bb5

bb5:                                              ; preds = %bb4
; invoke core::intrinsics::copy_nonoverlapping
  invoke void @_ZN4core10intrinsics19copy_nonoverlapping17hded36a0cdfa854e6E({ i8*, i64 }* %y, { i8*, i64 }* %x, i64 1)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
  store i8 0, i8* %_18, align 1
; invoke core::ptr::write
  invoke void @_ZN4core3ptr5write17h542a49247cbb9d3fE({ i8*, i64 }* %y, i8* noalias readonly align 1 %z.0, i64 %z.1)
          to label %bb7 unwind label %cleanup

bb7:                                              ; preds = %bb6
  store i8 0, i8* %_18, align 1
  br label %bb9

bb8:                                              ; preds = %bb3
  br label %bb9

bb9:                                              ; preds = %bb8, %bb7
  ret void

bb10:                                             ; preds = %bb11
  store i8 0, i8* %_18, align 1
  br label %bb1

bb11:                                             ; preds = %cleanup
  %11 = load i8, i8* %_18, align 1, !range !1
  %12 = trunc i8 %11 to i1
  br i1 %12, label %bb10, label %bb1

cleanup:                                          ; preds = %bb6, %bb5
  %13 = landingpad { i8*, i32 }
          cleanup
  %14 = extractvalue { i8*, i32 } %13, 0
  %15 = extractvalue { i8*, i32 } %13, 1
  %16 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %14, i8** %16, align 8
  %17 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %15, i32* %17, align 8
  br label %bb11
}

; core::ptr::swap_nonoverlapping_bytes
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ptr25swap_nonoverlapping_bytes17h619d15c1d3f196e4E(i8* %x, i8* %y, i64 %len) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %t1 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>", align 8
  %t = alloca <4 x i64>, align 32
  %i = alloca i64, align 8
  %1 = alloca {}, align 1
  store i64 32, i64* %0, align 8
  %2 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64 0, i64* %i, align 8
  br label %bb2

bb2:                                              ; preds = %bb11, %bb1
  %_8 = load i64, i64* %i, align 8
  %_7 = add i64 %_8, %2
  %_6 = icmp ule i64 %_7, %len
  br i1 %_6, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
  %_38 = load i64, i64* %i, align 8
  %_37 = icmp ult i64 %_38, %len
  br i1 %_37, label %bb13, label %bb12

bb4:                                              ; preds = %bb2
  %3 = bitcast <4 x i64>* %t to {}*
  br label %bb5

bb5:                                              ; preds = %bb4
  br label %bb6

bb6:                                              ; preds = %bb5
  %t2 = bitcast <4 x i64>* %t to i8*
  %_17 = load i64, i64* %i, align 8
; call core::ptr::mut_ptr::<impl *mut T>::add
  %x3 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h60de6656046e39caE"(i8* %x, i64 %_17)
  br label %bb7

bb7:                                              ; preds = %bb6
  %_20 = load i64, i64* %i, align 8
; call core::ptr::mut_ptr::<impl *mut T>::add
  %y4 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h60de6656046e39caE"(i8* %y, i64 %_20)
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %x3, i8* %t2, i64 %2)
  br label %bb9

bb9:                                              ; preds = %bb8
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %y4, i8* %x3, i64 %2)
  br label %bb10

bb10:                                             ; preds = %bb9
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %t2, i8* %y4, i64 %2)
  br label %bb11

bb11:                                             ; preds = %bb10
  %4 = load i64, i64* %i, align 8
  %5 = add i64 %4, %2
  store i64 %5, i64* %i, align 8
  br label %bb2

bb12:                                             ; preds = %bb3
  br label %bb21

bb13:                                             ; preds = %bb3
  %6 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1 to {}*
  br label %bb14

bb14:                                             ; preds = %bb13
  %_43 = load i64, i64* %i, align 8
  %rem = sub i64 %len, %_43
  %_4.i = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1 to %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*
  %_2.i.i = bitcast %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %_4.i to %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"*
  br label %bb15

bb15:                                             ; preds = %bb14
  %t5 = bitcast %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"* %_2.i.i to i8*
  %_49 = load i64, i64* %i, align 8
; call core::ptr::mut_ptr::<impl *mut T>::add
  %x6 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h60de6656046e39caE"(i8* %x, i64 %_49)
  br label %bb16

bb16:                                             ; preds = %bb15
  %_52 = load i64, i64* %i, align 8
; call core::ptr::mut_ptr::<impl *mut T>::add
  %y7 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h60de6656046e39caE"(i8* %y, i64 %_52)
  br label %bb17

bb17:                                             ; preds = %bb16
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %x6, i8* %t5, i64 %rem)
  br label %bb18

bb18:                                             ; preds = %bb17
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %y7, i8* %x6, i64 %rem)
  br label %bb19

bb19:                                             ; preds = %bb18
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h24df7b4ba27e05b1E(i8* %t5, i8* %y7, i64 %rem)
  br label %bb20

bb20:                                             ; preds = %bb19
  br label %bb21

bb21:                                             ; preds = %bb12, %bb20
  ret void
}

; core::ptr::read
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3ptr4read17h39f36972cd6f7088E({ i8*, i64 }* %src) unnamed_addr #0 {
start:
  %0 = alloca { i8*, i64 }, align 8
  %tmp = alloca { i8*, i64 }, align 8
  %1 = bitcast { i8*, i64 }* %0 to {}*
  %2 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 0
  %3 = load i8*, i8** %2, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = insertvalue { i8*, i64 } undef, i8* %3, 0
  %7 = insertvalue { i8*, i64 } %6, i64 %5, 1
  store { i8*, i64 } %7, { i8*, i64 }* %tmp, align 8
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hded36a0cdfa854e6E({ i8*, i64 }* %src, { i8*, i64 }* %tmp, i64 1)
  br label %bb3

bb3:                                              ; preds = %bb2
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %tmp, i32 0, i32 0
  %_7.0 = load i8*, i8** %8, align 8
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %tmp, i32 0, i32 1
  %_7.1 = load i64, i64* %9, align 8
  %10 = insertvalue { i8*, i64 } undef, i8* %_7.0, 0
  %11 = insertvalue { i8*, i64 } %10, i64 %_7.1, 1
  %12 = insertvalue { i8*, i64 } undef, i8* %_7.0, 0
  %13 = insertvalue { i8*, i64 } %12, i64 %_7.1, 1
  %14 = extractvalue { i8*, i64 } %13, 0
  %15 = extractvalue { i8*, i64 } %13, 1
  br label %bb4

bb4:                                              ; preds = %bb3
  %16 = insertvalue { i8*, i64 } undef, i8* %14, 0
  %17 = insertvalue { i8*, i64 } %16, i64 %15, 1
  ret { i8*, i64 } %17
}

; core::ptr::write
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr5write17h542a49247cbb9d3fE({ i8*, i64 }* %dst, i8* noalias readonly align 1 %src.0, i64 %src.1) unnamed_addr #0 {
start:
  %0 = alloca {}, align 1
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %dst, i32 0, i32 0
  store i8* %src.0, i8** %1, align 8
  %2 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %dst, i32 0, i32 1
  store i64 %src.1, i64* %2, align 8
  ret void
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hb7692bcb401d3524E"(i8* %ptr) unnamed_addr #0 {
start:
  %_5 = alloca %"core::marker::PhantomData<u8>", align 1
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = bitcast i8** %0 to %"core::marker::PhantomData<u8>"*
  %2 = load i8*, i8** %0, align 8, !nonnull !3
  ret i8* %2
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h8d8bde9108e7e01dE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %0 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h44eadf0be90493adE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1)
  %_3.0 = extractvalue { {}*, [3 x i64]* } %0, 0
  %_3.1 = extractvalue { {}*, [3 x i64]* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = bitcast {}* %_3.0 to i8*
; call core::ptr::unique::Unique<T>::new_unchecked
  %1 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hb7692bcb401d3524E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %1
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h1aafa3f293363b25E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h44eadf0be90493adE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
  %_2.0 = bitcast i8* %self.0 to {}*
  %_2.1 = bitcast i64* %self.1 to [3 x i64]*
  %0 = insertvalue { {}*, [3 x i64]* } undef, {}* %_2.0, 0
  %1 = insertvalue { {}*, [3 x i64]* } %0, [3 x i64]* %_2.1, 1
  ret { {}*, [3 x i64]* } %1
}

; core::ptr::unique::Unique<T>::as_ref
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ref17h69cfdf28aedb38fbE"({ i8*, i64* }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 0
  %_3.0 = load i8*, i8** %0, align 8, !nonnull !3
  %1 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 1
  %_3.1 = load i64*, i64** %1, align 8, !nonnull !3
; call core::ptr::unique::Unique<T>::as_ptr
  %2 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h44eadf0be90493adE"(i8* nonnull %_3.0, i64* noalias readonly align 8 dereferenceable(24) %_3.1)
  %_2.0 = extractvalue { {}*, [3 x i64]* } %2, 0
  %_2.1 = extractvalue { {}*, [3 x i64]* } %2, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { {}*, [3 x i64]* } undef, {}* %_2.0, 0
  %4 = insertvalue { {}*, [3 x i64]* } %3, [3 x i64]* %_2.1, 1
  ret { {}*, [3 x i64]* } %4
}

; core::ptr::mut_ptr::<impl *mut T>::add
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h60de6656046e39caE"(i8* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::mut_ptr::<impl *mut T>::offset
  %0 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h808ddbb39b74a830E"(i8* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; core::ptr::mut_ptr::<impl *mut T>::offset
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h808ddbb39b74a830E"(i8* %self, i64 %count) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  %1 = getelementptr inbounds i8, i8* %self, i64 %count
  store i8* %1, i8** %0, align 8
  %_3 = load i8*, i8** %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %_3
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h2105e283956fa61bE"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
  %0 = icmp eq i8* %self, null
  ret i1 %0
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h6627a5ab4ea7064dE"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !3
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::new
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h9fe8a6d22efa2658E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_3 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h2105e283956fa61bE"(i8* %ptr)
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = xor i1 %_3, true
  br i1 %_2, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  %1 = bitcast i8** %0 to {}**
  store {}* null, {}** %1, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %_5 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h6627a5ab4ea7064dE"(i8* %ptr)
  br label %bb4

bb4:                                              ; preds = %bb3
  store i8* %_5, i8** %0, align 8
  br label %bb5

bb5:                                              ; preds = %bb2, %bb4
  %2 = load i8*, i8** %0, align 8
  ret i8* %2
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hfa211bdaa98fc7bfE"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h9d792496738602d3E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h5beda99855ca8475E(i64 %align), !range !2
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !2
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout4size17hc7e0ae9506674cedE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout5align17h5fc25def5f287ce1E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !2
; call core::num::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num12NonZeroUsize3get17h6fb979e371d431ccE(i64 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::alloc::layout::Layout::dangling
; Function Attrs: inlinehint uwtable
define internal nonnull i8* @_ZN4core5alloc6layout6Layout8dangling17h862ce3e5670203b2E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call core::alloc::layout::Layout::align
  %_3 = call i64 @_ZN4core5alloc6layout6Layout5align17h5fc25def5f287ce1E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = inttoptr i64 %_3 to i8*
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h6627a5ab4ea7064dE"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::option::Option<T>::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h9b29bcc05ac5f7beE"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call core::mem::take
  %0 = call { i8*, i64 } @_ZN4core3mem4take17h91657bb9670099edE({ i8*, i64 }* align 8 dereferenceable(16) %self)
  %1 = extractvalue { i8*, i64 } %0, 0
  %2 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i8*, i64 } undef, i8* %1, 0
  %4 = insertvalue { i8*, i64 } %3, i64 %2, 1
  ret { i8*, i64 } %4
}

; core::option::Option<T>::ok_or
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h5d2145ea7a7407f0E"(i8* %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_7 = alloca i8, align 1
  %2 = alloca i8*, align 8
  %self = alloca i8*, align 8
  store i8* %0, i8** %self, align 8
  store i8 0, i8* %_7, align 1
  store i8 1, i8* %_7, align 1
  %3 = bitcast i8** %self to {}**
  %4 = load {}*, {}** %3, align 8
  %5 = icmp ule {}* %4, null
  %_3 = select i1 %5, i64 0, i64 1
  switch i64 %_3, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb1:                                              ; preds = %start
  store i8 0, i8* %_7, align 1
  %6 = bitcast i8** %2 to %"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocErr>::Err"*
  %7 = bitcast %"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocErr>::Err"* %6 to %"core::alloc::AllocErr"*
  %8 = bitcast i8** %2 to {}**
  store {}* null, {}** %8, align 8
  br label %bb5

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %v = load i8*, i8** %self, align 8, !nonnull !3
  store i8* %v, i8** %2, align 8
  br label %bb5

bb4:                                              ; No predecessors!
  %9 = bitcast { i8*, i32 }* %1 to i8**
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %12 = load i32, i32* %11, align 8
  %13 = insertvalue { i8*, i32 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i32 } %13, i32 %12, 1
  resume { i8*, i32 } %14

bb5:                                              ; preds = %bb1, %bb3
  %15 = load i8, i8* %_7, align 1, !range !1
  %16 = trunc i8 %15 to i1
  br i1 %16, label %bb7, label %bb6

bb6:                                              ; preds = %bb7, %bb5
  %17 = load i8*, i8** %2, align 8
  ret i8* %17

bb7:                                              ; preds = %bb5
  store i8 0, i8* %_7, align 1
  br label %bb6
}

; <T as core::convert::From<T>>::from
; Function Attrs: uwtable
define void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h9f380d2db4cb7032E"() unnamed_addr #1 {
start:
  ret void
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h4ca1a6692518ff8cE"(i8* nonnull %self) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17ha588496513836548E"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; alloc::alloc::alloc_zeroed
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc12alloc_zeroed17hb0fe3378e8a643afE(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_2 = call i64 @_ZN4core5alloc6layout6Layout4size17hc7e0ae9506674cedE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_4 = call i64 @_ZN4core5alloc6layout6Layout5align17h5fc25def5f287ce1E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_alloc_zeroed(i64 %_2, i64 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc15exchange_malloc17hbe3192f114f9051fE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_8 = alloca %"alloc::alloc::Global", align 1
  %_6 = alloca { i8*, i64 }, align 8
; call core::alloc::layout::Layout::from_size_align_unchecked
  %0 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h9d792496738602d3E(i64 %size, i64 %align)
  %layout.0 = extractvalue { i64, i64 } %0, 0
  %layout.1 = extractvalue { i64, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call <alloc::alloc::Global as core::alloc::AllocRef>::alloc
  %1 = call { i8*, i64 } @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h521d2fd70d6a82b0E"(%"alloc::alloc::Global"* nonnull align 1 %_8, i64 %layout.0, i64 %layout.1, i1 zeroext false)
  store { i8*, i64 } %1, { i8*, i64 }* %_6, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
  %2 = bitcast { i8*, i64 }* %_6 to {}**
  %3 = load {}*, {}** %2, align 8
  %4 = icmp ule {}* %3, null
  %_11 = select i1 %4, i64 1, i64 0
  switch i64 %_11, label %bb4 [
    i64 0, label %bb5
    i64 1, label %bb3
  ]

bb3:                                              ; preds = %bb2
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h3f1dc1d2a9116445E(i64 %layout.0, i64 %layout.1)
  unreachable

bb4:                                              ; preds = %bb2
  unreachable

bb5:                                              ; preds = %bb2
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_6, i32 0, i32 0
  %memory.0 = load i8*, i8** %5, align 8, !nonnull !3
  %6 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_6, i32 0, i32 1
  %memory.1 = load i64, i64* %6, align 8
; call core::ptr::non_null::NonNull<T>::as_ptr
  %7 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hfa211bdaa98fc7bfE"(i8* nonnull %memory.0)
  br label %bb6

bb6:                                              ; preds = %bb5
  ret i8* %7
}

; alloc::alloc::alloc
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc5alloc17h89edc7931e539108E(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_2 = call i64 @_ZN4core5alloc6layout6Layout4size17hc7e0ae9506674cedE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_4 = call i64 @_ZN4core5alloc6layout6Layout5align17h5fc25def5f287ce1E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_alloc(i64 %_2, i64 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17h48b4607135848f71E(i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17hc7e0ae9506674cedE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_6 = call i64 @_ZN4core5alloc6layout6Layout5align17h5fc25def5f287ce1E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::alloc::box_free
; Function Attrs: inlinehint uwtable
define void @_ZN5alloc5alloc8box_free17h0dad36ae68ddb938E(i8* nonnull %0, i64* noalias readonly align 8 dereferenceable(24) %1) unnamed_addr #0 {
start:
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %_14 = alloca %"alloc::alloc::Global", align 1
  %ptr = alloca { i8*, i64* }, align 8
  %4 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %ptr, i32 0, i32 0
  store i8* %0, i8** %4, align 8
  %5 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %ptr, i32 0, i32 1
  store i64* %1, i64** %5, align 8
; call core::ptr::unique::Unique<T>::as_ref
  %6 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ref17h69cfdf28aedb38fbE"({ i8*, i64* }* noalias readonly align 8 dereferenceable(16) %ptr)
  %_4.0 = extractvalue { {}*, [3 x i64]* } %6, 0
  %_4.1 = extractvalue { {}*, [3 x i64]* } %6, 1
  br label %bb1

bb1:                                              ; preds = %start
  %7 = bitcast [3 x i64]* %_4.1 to i64*
  %8 = getelementptr inbounds i64, i64* %7, i64 1
  %9 = load i64, i64* %8, align 8, !invariant.load !3
  %10 = bitcast [3 x i64]* %_4.1 to i64*
  %11 = getelementptr inbounds i64, i64* %10, i64 2
  %12 = load i64, i64* %11, align 8, !invariant.load !3
  store i64 %9, i64* %3, align 8
  %size = load i64, i64* %3, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::unique::Unique<T>::as_ref
  %13 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ref17h69cfdf28aedb38fbE"({ i8*, i64* }* noalias readonly align 8 dereferenceable(16) %ptr)
  %_8.0 = extractvalue { {}*, [3 x i64]* } %13, 0
  %_8.1 = extractvalue { {}*, [3 x i64]* } %13, 1
  br label %bb3

bb3:                                              ; preds = %bb2
  %14 = bitcast [3 x i64]* %_8.1 to i64*
  %15 = getelementptr inbounds i64, i64* %14, i64 1
  %16 = load i64, i64* %15, align 8, !invariant.load !3
  %17 = bitcast [3 x i64]* %_8.1 to i64*
  %18 = getelementptr inbounds i64, i64* %17, i64 2
  %19 = load i64, i64* %18, align 8, !invariant.load !3
  store i64 %19, i64* %2, align 8
  %align = load i64, i64* %2, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
; call core::alloc::layout::Layout::from_size_align_unchecked
  %20 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h9d792496738602d3E(i64 %size, i64 %align)
  %layout.0 = extractvalue { i64, i64 } %20, 0
  %layout.1 = extractvalue { i64, i64 } %20, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %21 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %ptr, i32 0, i32 0
  %_17.0 = load i8*, i8** %21, align 8, !nonnull !3
  %22 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %ptr, i32 0, i32 1
  %_17.1 = load i64*, i64** %22, align 8, !nonnull !3
; call core::ptr::unique::Unique<T>::cast
  %_16 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h8d8bde9108e7e01dE"(i8* nonnull %_17.0, i64* noalias readonly align 8 dereferenceable(24) %_17.1)
  br label %bb6

bb6:                                              ; preds = %bb5
; call <T as core::convert::Into<U>>::into
  %_15 = call nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h4ca1a6692518ff8cE"(i8* nonnull %_16)
  br label %bb7

bb7:                                              ; preds = %bb6
; call <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
  call void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hfc383c86d43addfbE"(%"alloc::alloc::Global"* nonnull align 1 %_14, i8* nonnull %_15, i64 %layout.0, i64 %layout.1)
  br label %bb8

bb8:                                              ; preds = %bb7
  ret void
}

; alloc::boxed::Box<T>::leak
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$4leak17h59dccf457a83474aE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
  %0 = alloca { i8*, i64* }, align 8
  %_8 = alloca { i8*, i64* }, align 8
  %1 = bitcast { i8*, i64* }* %0 to { {}*, [3 x i64]* }*
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %1, i32 0, i32 0
  store {}* %b.0, {}** %2, align 8, !noalias !4
  %3 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %1, i32 0, i32 1
  store [3 x i64]* %b.1, [3 x i64]** %3, align 8, !noalias !4
  %4 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %0, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8, !noalias !4, !nonnull !3
  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %0, i32 0, i32 1
  %7 = load i64*, i64** %6, align 8, !noalias !4, !nonnull !3
  %8 = insertvalue { i8*, i64* } undef, i8* %5, 0
  %9 = insertvalue { i8*, i64* } %8, i64* %7, 1
  store { i8*, i64* } %9, { i8*, i64* }* %_8, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %_2.i = bitcast { i8*, i64* }* %_8 to { {}*, [3 x i64]* }*
  br label %bb2

bb2:                                              ; preds = %bb1
  %10 = bitcast { {}*, [3 x i64]* }* %_2.i to { i8*, i64* }*
  %11 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %10, i32 0, i32 0
  %_5.0 = load i8*, i8** %11, align 8, !nonnull !3
  %12 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %10, i32 0, i32 1
  %_5.1 = load i64*, i64** %12, align 8, !nonnull !3
; call core::ptr::unique::Unique<T>::as_ptr
  %13 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h44eadf0be90493adE"(i8* nonnull %_5.0, i64* noalias readonly align 8 dereferenceable(24) %_5.1)
  %_4.0 = extractvalue { {}*, [3 x i64]* } %13, 0
  %_4.1 = extractvalue { {}*, [3 x i64]* } %13, 1
  br label %bb3

bb3:                                              ; preds = %bb2
  %14 = insertvalue { {}*, [3 x i64]* } undef, {}* %_4.0, 0
  %15 = insertvalue { {}*, [3 x i64]* } %14, [3 x i64]* %_4.1, 1
  ret { {}*, [3 x i64]* } %15
}

; alloc::boxed::Box<T>::into_raw
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h33139acf28084453E"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T>::leak
  %0 = call { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$4leak17h59dccf457a83474aE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  %_2.0 = extractvalue { {}*, [3 x i64]* } %0, 0
  %_2.1 = extractvalue { {}*, [3 x i64]* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = insertvalue { {}*, [3 x i64]* } undef, {}* %_2.0, 0
  %2 = insertvalue { {}*, [3 x i64]* } %1, [3 x i64]* %_2.1, 1
  ret { {}*, [3 x i64]* } %2
}

; <alloc::alloc::Global as core::alloc::AllocRef>::alloc
; Function Attrs: inlinehint uwtable
define internal { i8*, i64 } @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h521d2fd70d6a82b0E"(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %0, i64 %1, i1 zeroext %2) unnamed_addr #0 {
start:
  %_26 = alloca { i8*, i64 }, align 8
  %_16 = alloca i8*, align 8
  %raw_ptr = alloca i8*, align 8
  %_8 = alloca { i8*, i64 }, align 8
  %3 = alloca { i8*, i64 }, align 8
  %init = alloca i8, align 1
  %layout = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %5, align 8
  %6 = zext i1 %2 to i8
  store i8 %6, i8* %init, align 1
; call core::alloc::layout::Layout::size
  %size = call i64 @_ZN4core5alloc6layout6Layout4size17hc7e0ae9506674cedE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %_6 = icmp eq i64 %size, 0
  br i1 %_6, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  %7 = load i8, i8* %init, align 1, !range !1
  %8 = trunc i8 %7 to i1
  %_12 = zext i1 %8 to i64
  switch i64 %_12, label %bb6 [
    i64 0, label %bb7
    i64 1, label %bb5
  ]

bb3:                                              ; preds = %bb1
; call core::alloc::layout::Layout::dangling
  %_9 = call nonnull i8* @_ZN4core5alloc6layout6Layout8dangling17h862ce3e5670203b2E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb4

bb4:                                              ; preds = %bb3
  %9 = bitcast { i8*, i64 }* %_8 to i8**
  store i8* %_9, i8** %9, align 8
  %10 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_8, i32 0, i32 1
  store i64 0, i64* %10, align 8
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_8, i32 0, i32 0
  %12 = load i8*, i8** %11, align 8, !nonnull !3
  %13 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_8, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
  %15 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 0
  store i8* %12, i8** %15, align 8
  %16 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 1
  store i64 %14, i64* %16, align 8
  br label %bb20

bb5:                                              ; preds = %bb2
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_14.0 = load i64, i64* %17, align 8
  %18 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_14.1 = load i64, i64* %18, align 8, !range !2
; call alloc::alloc::alloc_zeroed
  %19 = call i8* @_ZN5alloc5alloc12alloc_zeroed17hb0fe3378e8a643afE(i64 %_14.0, i64 %_14.1)
  store i8* %19, i8** %raw_ptr, align 8
  br label %bb9

bb6:                                              ; preds = %bb2
  unreachable

bb7:                                              ; preds = %bb2
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_13.0 = load i64, i64* %20, align 8
  %21 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_13.1 = load i64, i64* %21, align 8, !range !2
; call alloc::alloc::alloc
  %22 = call i8* @_ZN5alloc5alloc5alloc17h89edc7931e539108E(i64 %_13.0, i64 %_13.1)
  store i8* %22, i8** %raw_ptr, align 8
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb10

bb9:                                              ; preds = %bb5
  br label %bb10

bb10:                                             ; preds = %bb8, %bb9
  %_19 = load i8*, i8** %raw_ptr, align 8
; call core::ptr::non_null::NonNull<T>::new
  %_18 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h9fe8a6d22efa2658E"(i8* %_19)
  br label %bb11

bb11:                                             ; preds = %bb10
; call core::option::Option<T>::ok_or
  %_17 = call i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h5d2145ea7a7407f0E"(i8* %_18)
  br label %bb12

bb12:                                             ; preds = %bb11
; call <core::result::Result<T,E> as core::ops::try::Try>::into_result
  %23 = call i8* @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$11into_result17h7d6943d96cfd17abE"(i8* %_17)
  store i8* %23, i8** %_16, align 8
  br label %bb13

bb13:                                             ; preds = %bb12
  %24 = bitcast i8** %_16 to {}**
  %25 = load {}*, {}** %24, align 8
  %26 = icmp ule {}* %25, null
  %_21 = select i1 %26, i64 1, i64 0
  switch i64 %_21, label %bb15 [
    i64 0, label %bb14
    i64 1, label %bb16
  ]

bb14:                                             ; preds = %bb13
  %val = load i8*, i8** %_16, align 8, !nonnull !3
  %27 = bitcast { i8*, i64 }* %_26 to i8**
  store i8* %val, i8** %27, align 8
  %28 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_26, i32 0, i32 1
  store i64 %size, i64* %28, align 8
  %29 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_26, i32 0, i32 0
  %30 = load i8*, i8** %29, align 8, !nonnull !3
  %31 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_26, i32 0, i32 1
  %32 = load i64, i64* %31, align 8
  %33 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 0
  store i8* %30, i8** %33, align 8
  %34 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 1
  store i64 %32, i64* %34, align 8
  br label %bb20

bb15:                                             ; preds = %bb13
  unreachable

bb16:                                             ; preds = %bb13
; call <T as core::convert::From<T>>::from
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h9f380d2db4cb7032E"()
  br label %bb18

bb17:                                             ; preds = %bb20, %bb19
  %35 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 0
  %36 = load i8*, i8** %35, align 8
  %37 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %3, i32 0, i32 1
  %38 = load i64, i64* %37, align 8
  %39 = insertvalue { i8*, i64 } undef, i8* %36, 0
  %40 = insertvalue { i8*, i64 } %39, i64 %38, 1
  ret { i8*, i64 } %40

bb18:                                             ; preds = %bb16
; call <core::result::Result<T,E> as core::ops::try::Try>::from_error
  %41 = call { i8*, i64 } @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$10from_error17hbf2971c6984c513fE"()
  store { i8*, i64 } %41, { i8*, i64 }* %3, align 8
  br label %bb19

bb19:                                             ; preds = %bb18
  br label %bb17

bb20:                                             ; preds = %bb14, %bb4
  br label %bb17
}

; <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
; Function Attrs: inlinehint uwtable
define internal void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hfc383c86d43addfbE"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %2 = alloca {}, align 1
  %layout = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; call core::alloc::layout::Layout::size
  %_5 = call i64 @_ZN4core5alloc6layout6Layout4size17hc7e0ae9506674cedE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %_4 = icmp ne i64 %_5, 0
  br i1 %_4, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  br label %bb6

bb3:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_7 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hfa211bdaa98fc7bfE"(i8* nonnull %ptr)
  br label %bb4

bb4:                                              ; preds = %bb3
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_9.0 = load i64, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_9.1 = load i64, i64* %6, align 8, !range !2
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17h48b4607135848f71E(i8* %_7, i64 %_9.0, i64 %_9.1)
  br label %bb5

bb5:                                              ; preds = %bb4
  br label %bb6

bb6:                                              ; preds = %bb2, %bb5
  ret void
}

; <core::option::Option<T> as core::default::Default>::default
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN72_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h6d26e62efc4f54a7E"() unnamed_addr #0 {
start:
  %0 = alloca { i8*, i64 }, align 8
  %1 = bitcast { i8*, i64 }* %0 to {}**
  store {}* null, {}** %1, align 8
  %2 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 0
  %3 = load i8*, i8** %2, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = insertvalue { i8*, i64 } undef, i8* %3, 0
  %7 = insertvalue { i8*, i64 } %6, i64 %5, 1
  ret { i8*, i64 } %7
}

; <core::result::Result<T,E> as core::ops::try::Try>::from_error
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$10from_error17hbf2971c6984c513fE"() unnamed_addr #0 {
start:
  %0 = alloca { i8*, i64 }, align 8
  %1 = bitcast { i8*, i64 }* %0 to %"core::result::Result<core::alloc::MemoryBlock, core::alloc::AllocErr>::Err"*
  %2 = bitcast %"core::result::Result<core::alloc::MemoryBlock, core::alloc::AllocErr>::Err"* %1 to %"core::alloc::AllocErr"*
  %3 = bitcast { i8*, i64 }* %0 to {}**
  store {}* null, {}** %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8
  %6 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %0, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = insertvalue { i8*, i64 } undef, i8* %5, 0
  %9 = insertvalue { i8*, i64 } %8, i64 %7, 1
  ret { i8*, i64 } %9
}

; <core::result::Result<T,E> as core::ops::try::Try>::into_result
; Function Attrs: inlinehint uwtable
define i8* @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$11into_result17h7d6943d96cfd17abE"(i8* %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hc02f6d5d8b3bc05cE"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = bitcast { i8*, i64 }* %self to {}**
  %1 = load {}*, {}** %0, align 8
  %2 = icmp ule {}* %1, null
  %_6 = select i1 %2, i64 0, i64 1
  switch i64 %_6, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb1:                                              ; preds = %start
; call std::process::abort
  call void @_ZN3std7process5abort17hc47c8bc271845c6fE()
  unreachable

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %a = bitcast { i8*, i64 }* %self to { [0 x i8]*, i64 }*
  %_5.0 = bitcast { [0 x i8]*, i64 }* %a to {}*
  %3 = insertvalue { {}*, [3 x i64]* } undef, {}* %_5.0, 0
  %4 = insertvalue { {}*, [3 x i64]* } %3, [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.1 to [3 x i64]*), 1
  ret { {}*, [3 x i64]* } %4
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::take_box
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h059f6afe427c0ae6E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
  %_14 = alloca i8, align 1
  %_4 = alloca { i8*, i64 }, align 8
  %data = alloca { {}*, [3 x i64]* }, align 8
  store i8 0, i8* %_14, align 1
; call core::option::Option<T>::take
  %1 = call { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h9b29bcc05ac5f7beE"({ i8*, i64 }* align 8 dereferenceable(16) %self)
  store { i8*, i64 } %1, { i8*, i64 }* %_4, align 8
  br label %bb2

bb1:                                              ; preds = %bb9, %bb10, %bb7
  %2 = bitcast { i8*, i32 }* %0 to i8**
  %3 = load i8*, i8** %2, align 8
  %4 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %5 = load i32, i32* %4, align 8
  %6 = insertvalue { i8*, i32 } undef, i8* %3, 0
  %7 = insertvalue { i8*, i32 } %6, i32 %5, 1
  resume { i8*, i32 } %7

bb2:                                              ; preds = %start
  %8 = bitcast { i8*, i64 }* %_4 to {}**
  %9 = load {}*, {}** %8, align 8
  %10 = icmp ule {}* %9, null
  %_6 = select i1 %10, i64 0, i64 1
  switch i64 %_6, label %bb4 [
    i64 0, label %bb3
    i64 1, label %bb5
  ]

bb3:                                              ; preds = %bb2
; invoke std::process::abort
  invoke void @_ZN3std7process5abort17hc47c8bc271845c6fE()
          to label %unreachable unwind label %cleanup

bb4:                                              ; preds = %bb2
  unreachable

bb5:                                              ; preds = %bb2
  %11 = bitcast { i8*, i64 }* %_4 to { [0 x i8]*, i64 }*
  %12 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %11, i32 0, i32 0
  %a.0 = load [0 x i8]*, [0 x i8]** %12, align 8, !nonnull !3
  %13 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %11, i32 0, i32 1
  %a.1 = load i64, i64* %13, align 8
; invoke alloc::alloc::exchange_malloc
  %14 = invoke i8* @_ZN5alloc5alloc15exchange_malloc17hbe3192f114f9051fE(i64 16, i64 8)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h2ac8ef232e67a55eE.exit" unwind label %cleanup

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h2ac8ef232e67a55eE.exit": ; preds = %bb5
  %15 = bitcast i8* %14 to { [0 x i8]*, i64 }*
  %16 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %15, i32 0, i32 0
  store [0 x i8]* %a.0, [0 x i8]** %16, align 8, !noalias !8
  %17 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %15, i32 0, i32 1
  store i64 %a.1, i64* %17, align 8
  br label %bb6

bb6:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h2ac8ef232e67a55eE.exit"
  %18 = bitcast { [0 x i8]*, i64 }* %15 to {}*
  %19 = bitcast {}* %18 to i8*
  %_8.0 = bitcast i8* %19 to {}*
  store i8 1, i8* %_14, align 1
  %20 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  store {}* %_8.0, {}** %20, align 8
  %21 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.1 to [3 x i64]*), [3 x i64]** %21, align 8
  store i8 0, i8* %_14, align 1
  %22 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  %_13.0 = load {}*, {}** %22, align 8, !nonnull !3
  %23 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  %_13.1 = load [3 x i64]*, [3 x i64]** %23, align 8, !nonnull !3
; invoke alloc::boxed::Box<T>::into_raw
  %24 = invoke { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h33139acf28084453E"({}* noalias nonnull align 1 %_13.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %_13.1)
          to label %bb8 unwind label %cleanup1

bb7:                                              ; preds = %cleanup1
  %25 = load i8, i8* %_14, align 1, !range !1
  %26 = trunc i8 %25 to i1
  br i1 %26, label %bb10, label %bb1

bb8:                                              ; preds = %bb6
  %_11.0 = extractvalue { {}*, [3 x i64]* } %24, 0
  %_11.1 = extractvalue { {}*, [3 x i64]* } %24, 1
  store i8 0, i8* %_14, align 1
  %27 = insertvalue { {}*, [3 x i64]* } undef, {}* %_11.0, 0
  %28 = insertvalue { {}*, [3 x i64]* } %27, [3 x i64]* %_11.1, 1
  ret { {}*, [3 x i64]* } %28

bb9:                                              ; preds = %cleanup
  br label %bb1

bb10:                                             ; preds = %bb7
  store i8 0, i8* %_14, align 1
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h3f8de45cb7a779d6E({ {}*, [3 x i64]* }* %data) #7
  br label %bb1

cleanup:                                          ; preds = %bb5, %bb3
  %29 = landingpad { i8*, i32 }
          cleanup
  %30 = extractvalue { i8*, i32 } %29, 0
  %31 = extractvalue { i8*, i32 } %29, 1
  %32 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %30, i8** %32, align 8
  %33 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %31, i32* %33, align 8
  br label %bb9

cleanup1:                                         ; preds = %bb6
  %34 = landingpad { i8*, i32 }
          cleanup
  %35 = extractvalue { i8*, i32 } %34, 0
  %36 = extractvalue { i8*, i32 } %34, 1
  %37 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %35, i8** %37, align 8
  %38 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %36, i32* %38, align 8
  br label %bb7

unreachable:                                      ; preds = %bb3
  unreachable
}

; panic::may_panic
; Function Attrs: uwtable
define i32 @_ZN5panic9may_panic17h044e5a8a5c34bdceE(i32 %a) unnamed_addr #1 {
start:
  %_2 = icmp sgt i32 %a, 2
  br i1 %_2, label %bb2, label %bb1

bb1:                                              ; preds = %start
  ret i32 1

bb2:                                              ; preds = %start
; call std::panicking::begin_panic
  call void @_ZN3std9panicking11begin_panic17h5ae0871c3ba84f98E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [5 x i8] }>* @alloc19 to [0 x i8]*), i64 5, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc21 to %"core::panic::Location"*))
  unreachable
}

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; core::panic::Location::caller
; Function Attrs: uwtable
declare align 8 dereferenceable(24) %"core::panic::Location"* @_ZN4core5panic8Location6caller17h0ea59da85abecfa1E(%"core::panic::Location"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #1

; std::panicking::rust_panic_with_hook
; Function Attrs: noreturn uwtable
declare void @_ZN3std9panicking20rust_panic_with_hook17h3fc8a110bc9d166fE({}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), i64* noalias readonly align 8 dereferenceable_or_null(48), %"core::panic::Location"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #4

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #5

; Function Attrs: nounwind uwtable
declare i8* @__rust_alloc_zeroed(i64, i64) unnamed_addr #3

; alloc::alloc::handle_alloc_error
; Function Attrs: noreturn nounwind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h3f1dc1d2a9116445E(i64, i64) unnamed_addr #6

; Function Attrs: nounwind uwtable
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #3

; Function Attrs: nounwind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #3

; std::process::abort
; Function Attrs: noreturn uwtable
declare void @_ZN3std7process5abort17hc47c8bc271845c6fE() unnamed_addr #4

attributes #0 = { inlinehint uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #3 = { nounwind uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #4 = { noreturn uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #5 = { argmemonly nounwind willreturn }
attributes #6 = { noreturn nounwind uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #7 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i8 0, i8 2}
!2 = !{i64 1, i64 0}
!3 = !{}
!4 = !{!5, !7}
!5 = distinct !{!5, !6, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h6459b12fc5b20f17E: %value.0"}
!6 = distinct !{!6, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h6459b12fc5b20f17E"}
!7 = distinct !{!7, !6, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h6459b12fc5b20f17E: %value.1"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h2ac8ef232e67a55eE: %x.0"}
!10 = distinct !{!10, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h2ac8ef232e67a55eE"}
