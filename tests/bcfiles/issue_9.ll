; ModuleID = 'issue_9.3a1fbbbh-cgu.0'
source_filename = "issue_9.3a1fbbbh-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::panic::Location" = type { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::::Opaque" = type {}
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [4 x i64] }
%"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [0 x i64], %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock", [0 x i64] }
%"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock" = type { [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64] }
%"core::marker::PhantomData<u8>" = type {}
%"core::ptr::Repr<u32>" = type { [2 x i64] }
%"core::result::Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocErr>::Err" = type { [0 x i8], %"core::alloc::AllocErr", [0 x i8] }
%"core::alloc::AllocErr" = type {}
%"alloc::alloc::Global" = type {}
%"core::result::Result<core::alloc::MemoryBlock, core::alloc::AllocErr>::Err" = type { [0 x i8], %"core::alloc::AllocErr", [0 x i8] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* } { void ({ i8*, i64 }*)* @_ZN4core3ptr13drop_in_place17hbd43b2ce6d7f832fE, i64 16, i64 8, { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h82c28fc02bab35edE", { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hf4a84d088d75cc42E" }, align 8
@vtable.1 = private unnamed_addr constant { void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* } { void ({ [0 x i8]*, i64 }*)* @_ZN4core3ptr13drop_in_place17h0bfae6d147091688E, i64 16, i64 8, i64 ({ [0 x i8]*, i64 }*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5f38ccb545fd6d78E" }, align 8
@alloc14 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 4
@0 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @alloc14, i32 0, i32 0, i32 0), [0 x i8] zeroinitializer }>, align 8
@alloc34 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"abort" }>, align 1
@alloc35 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"issue_9.rs" }>, align 1
@alloc36 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [10 x i8] }>, <{ [10 x i8] }>* @alloc35, i32 0, i32 0, i32 0), [16 x i8] c"\0A\00\00\00\00\00\00\00\12\00\00\00\0D\00\00\00" }>, align 8
@_ZN7issue_93BUF17hcc9c6437bcff6fd1E = internal global <{ [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\02\00\00\00" }>, align 4
@alloc4 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"out: " }>, align 1
@alloc5 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc6 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [5 x i8] }>, <{ [5 x i8] }>* @alloc4, i32 0, i32 0, i32 0), [8 x i8] c"\05\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc5, i32 0, i32 0, i32 0), [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@1 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @alloc6 to i8*), [0 x i8] zeroinitializer }>, align 8

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h754b519c573d06eeE"(i8* nonnull %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h7894e68fc0578af6E"(i8* nonnull %unique)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h90c0f25537f85227E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; <T as core::any::Any>::type_id
; Function Attrs: uwtable
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h5f38ccb545fd6d78E"({ [0 x i8]*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17h1f49e0d94ab583f7E()
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn uwtable
define void @_ZN3std9panicking11begin_panic17h647a347b27423f53E([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) %0) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_10 = alloca i8, align 1
  %_5 = alloca { i8*, i64 }, align 8
  store i8 0, i8* %_10, align 1
  store i8 1, i8* %_10, align 1
  store i8 0, i8* %_10, align 1
; invoke std::panicking::begin_panic::PanicPayload<A>::new
  %2 = invoke { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h03b85867241c2459E"([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1)
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
define { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h03b85867241c2459E"([0 x i8]* noalias nonnull readonly align 1 %inner.0, i64 %inner.1) unnamed_addr #1 {
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
define void @_ZN4core10intrinsics19copy_nonoverlapping17ha834922761a347ceE({ i8*, i64 }* %src, { i8*, i64 }* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 16, %count
  %1 = bitcast { i8*, i64 }* %dst to i8*
  %2 = bitcast { i8*, i64 }* %src to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %src, i8* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 1, %count
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %dst, i8* align 1 %src, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17h1f49e0d94ab583f7E() unnamed_addr #1 {
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

; core::fmt::ArgumentV1::new
; Function Attrs: uwtable
define { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17hf9394e7024b31c51E(i64* noalias readonly align 8 dereferenceable(8) %x, i1 (i64*, %"core::fmt::Formatter"*)* nonnull %f) unnamed_addr #1 {
start:
  %0 = alloca %"core::fmt::::Opaque"*, align 8
  %1 = alloca i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, align 8
  %2 = alloca { i8*, i8* }, align 8
  %3 = bitcast i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1 to i1 (i64*, %"core::fmt::Formatter"*)**
  store i1 (i64*, %"core::fmt::Formatter"*)* %f, i1 (i64*, %"core::fmt::Formatter"*)** %3, align 8
  %_3 = load i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1, align 8, !nonnull !2
  br label %bb1

bb1:                                              ; preds = %start
  %4 = bitcast %"core::fmt::::Opaque"** %0 to i64**
  store i64* %x, i64** %4, align 8
  %_5 = load %"core::fmt::::Opaque"*, %"core::fmt::::Opaque"** %0, align 8, !nonnull !2
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i8* }* %2 to %"core::fmt::::Opaque"**
  store %"core::fmt::::Opaque"* %_5, %"core::fmt::::Opaque"** %5, align 8
  %6 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %7 = bitcast i8** %6 to i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)**
  store i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %7, align 8
  %8 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8, !nonnull !2
  %10 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %11 = load i8*, i8** %10, align 8, !nonnull !2
  %12 = insertvalue { i8*, i8* } undef, i8* %9, 0
  %13 = insertvalue { i8*, i8* } %12, i8* %11, 1
  ret { i8*, i8* } %13
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h097ac936ced356f4E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %0, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_4 = alloca { i64*, i64 }, align 8
  %1 = bitcast { i64*, i64 }* %_4 to {}**
  store {}* null, {}** %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*
  %3 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %3, align 8
  %4 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 1
  store i64 %pieces.1, i64* %4, align 8
  %5 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3
  %6 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0
  %7 = load i64*, i64** %6, align 8
  %8 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 0
  store i64* %7, i64** %10, align 8
  %11 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 1
  store i64 %9, i64* %11, align 8
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5
  %13 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %12, i32 0, i32 0
  store [0 x { i8*, i8* }]* %args.0, [0 x { i8*, i8* }]** %13, align 8
  %14 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %12, i32 0, i32 1
  store i64 %args.1, i64* %14, align 8
  ret void
}

; core::mem::swap
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem4swap17h1e88abcb6ce296f8E({ i8*, i64 }* align 8 dereferenceable(16) %x, { i8*, i64 }* align 8 dereferenceable(16) %y) unnamed_addr #0 {
start:
; call core::ptr::swap_nonoverlapping_one
  call void @_ZN4core3ptr23swap_nonoverlapping_one17hd7091da7c377be02E({ i8*, i64 }* %x, { i8*, i64 }* %y)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3mem4take17h07cfb3bd67ef11ffE({ i8*, i64 }* align 8 dereferenceable(16) %dest) unnamed_addr #0 {
start:
; call <core::option::Option<T> as core::default::Default>::default
  %0 = call { i8*, i64 } @"_ZN72_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h36117822c9ac85a1E"()
  %_3.0 = extractvalue { i8*, i64 } %0, 0
  %_3.1 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::mem::replace
  %1 = call { i8*, i64 } @_ZN4core3mem7replace17he280b9ea3403bab6E({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1 %_3.0, i64 %_3.1)
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
define { i8*, i64 } @_ZN4core3mem7replace17he280b9ea3403bab6E({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1 %0, i64 %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %2 = alloca { i8*, i32 }, align 8
  %src = alloca { i8*, i64 }, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 0
  store i8* %0, i8** %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; invoke core::mem::swap
  invoke void @_ZN4core3mem4swap17h1e88abcb6ce296f8E({ i8*, i64 }* align 8 dereferenceable(16) %dest, { i8*, i64 }* align 8 dereferenceable(16) %src)
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
define internal i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h49d381afadfa51f8E(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !3
  ret i64 %1
}

; core::num::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize3get17h9d2488e4cc600e4aE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr13drop_in_place17h0bfae6d147091688E({ [0 x i8]*, i64 }* %_1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17h57f0b149c0190461E({ {}*, [3 x i64]* }* %_1) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
  %1 = alloca {}, align 1
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 0
  %3 = load {}*, {}** %2, align 8, !nonnull !2
  %4 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 1
  %5 = load [3 x i64]*, [3 x i64]** %4, align 8, !nonnull !2
  %6 = bitcast [3 x i64]* %5 to void ({}*)**
  %7 = getelementptr inbounds void ({}*)*, void ({}*)** %6, i64 0
  %8 = load void ({}*)*, void ({}*)** %7, align 8, !invariant.load !2, !nonnull !2
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
  %17 = load i8*, i8** %16, align 8, !nonnull !2
  %18 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %15, i32 0, i32 1
  %19 = load i64*, i64** %18, align 8, !nonnull !2
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h0989123a145fc6aaE(i8* nonnull %17, i64* noalias readonly align 8 dereferenceable(24) %19)
  br label %bb1

bb4:                                              ; preds = %cleanup
  %20 = bitcast { {}*, [3 x i64]* }* %_1 to { i8*, i64* }*
  %21 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %20, i32 0, i32 0
  %22 = load i8*, i8** %21, align 8, !nonnull !2
  %23 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %20, i32 0, i32 1
  %24 = load i64*, i64** %23, align 8, !nonnull !2
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h0989123a145fc6aaE(i8* nonnull %22, i64* noalias readonly align 8 dereferenceable(24) %24) #7
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
define void @_ZN4core3ptr13drop_in_place17h97375c81ce9810b7E({}* %_1.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  %1 = bitcast [3 x i64]* %_1.1 to void ({}*)**
  %2 = getelementptr inbounds void ({}*)*, void ({}*)** %1, i64 0
  %3 = load void ({}*)*, void ({}*)** %2, align 8, !invariant.load !2, !nonnull !2
  call void %3({}* %_1.0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr13drop_in_place17hbd43b2ce6d7f832fE({ i8*, i64 }* %_1) unnamed_addr #1 {
start:
  %0 = alloca {}, align 1
  ret void
}

; core::ptr::swap_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr19swap_nonoverlapping17h3503292344414a75E({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 %count) unnamed_addr #0 {
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
  call void @_ZN4core3ptr25swap_nonoverlapping_bytes17hba9c0c213bd11c46E(i8* %x1, i8* %y2, i64 %len)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::ptr::swap_nonoverlapping_one
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr23swap_nonoverlapping_one17hd7091da7c377be02E({ i8*, i64 }* %x, { i8*, i64 }* %y) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
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
  call void @_ZN4core3ptr19swap_nonoverlapping17h3503292344414a75E({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 1)
  br label %bb8

bb4:                                              ; preds = %bb2
  store i8 1, i8* %_18, align 1
; call core::ptr::read
  %10 = call { i8*, i64 } @_ZN4core3ptr4read17h4886c54c60d04e34E({ i8*, i64 }* %x)
  %z.0 = extractvalue { i8*, i64 } %10, 0
  %z.1 = extractvalue { i8*, i64 } %10, 1
  br label %bb5

bb5:                                              ; preds = %bb4
; invoke core::intrinsics::copy_nonoverlapping
  invoke void @_ZN4core10intrinsics19copy_nonoverlapping17ha834922761a347ceE({ i8*, i64 }* %y, { i8*, i64 }* %x, i64 1)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
  store i8 0, i8* %_18, align 1
; invoke core::ptr::write
  invoke void @_ZN4core3ptr5write17h7f030cb187137130E({ i8*, i64 }* %y, i8* noalias readonly align 1 %z.0, i64 %z.1)
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
define internal void @_ZN4core3ptr25swap_nonoverlapping_bytes17hba9c0c213bd11c46E(i8* %x, i8* %y, i64 %len) unnamed_addr #0 {
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
  %x3 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17he2bb317df3be6019E"(i8* %x, i64 %_17)
  br label %bb7

bb7:                                              ; preds = %bb6
  %_20 = load i64, i64* %i, align 8
; call core::ptr::mut_ptr::<impl *mut T>::add
  %y4 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17he2bb317df3be6019E"(i8* %y, i64 %_20)
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %x3, i8* %t2, i64 %2)
  br label %bb9

bb9:                                              ; preds = %bb8
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %y4, i8* %x3, i64 %2)
  br label %bb10

bb10:                                             ; preds = %bb9
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %t2, i8* %y4, i64 %2)
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
  %x6 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17he2bb317df3be6019E"(i8* %x, i64 %_49)
  br label %bb16

bb16:                                             ; preds = %bb15
  %_52 = load i64, i64* %i, align 8
; call core::ptr::mut_ptr::<impl *mut T>::add
  %y7 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17he2bb317df3be6019E"(i8* %y, i64 %_52)
  br label %bb17

bb17:                                             ; preds = %bb16
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %x6, i8* %t5, i64 %rem)
  br label %bb18

bb18:                                             ; preds = %bb17
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %y7, i8* %x6, i64 %rem)
  br label %bb19

bb19:                                             ; preds = %bb18
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb1b0c8987f4b0d16E(i8* %t5, i8* %y7, i64 %rem)
  br label %bb20

bb20:                                             ; preds = %bb19
  br label %bb21

bb21:                                             ; preds = %bb12, %bb20
  ret void
}

; core::ptr::read
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3ptr4read17h4886c54c60d04e34E({ i8*, i64 }* %src) unnamed_addr #0 {
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
  call void @_ZN4core10intrinsics19copy_nonoverlapping17ha834922761a347ceE({ i8*, i64 }* %src, { i8*, i64 }* %tmp, i64 1)
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
define void @_ZN4core3ptr5write17h7f030cb187137130E({ i8*, i64 }* %dst, i8* noalias readonly align 1 %src.0, i64 %src.1) unnamed_addr #0 {
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
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h615a4e7a947e9f4aE"(i8* %ptr) unnamed_addr #0 {
start:
  %_5 = alloca %"core::marker::PhantomData<u8>", align 1
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = bitcast i8** %0 to %"core::marker::PhantomData<u8>"*
  %2 = load i8*, i8** %0, align 8, !nonnull !2
  ret i8* %2
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h34187990c4aed1e7E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %0 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h1a5dfd32c28aaa62E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1)
  %_3.0 = extractvalue { {}*, [3 x i64]* } %0, 0
  %_3.1 = extractvalue { {}*, [3 x i64]* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = bitcast {}* %_3.0 to i8*
; call core::ptr::unique::Unique<T>::new_unchecked
  %1 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h615a4e7a947e9f4aE"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %1
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h1a5dfd32c28aaa62E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
  %_2.0 = bitcast i8* %self.0 to {}*
  %_2.1 = bitcast i64* %self.1 to [3 x i64]*
  %0 = insertvalue { {}*, [3 x i64]* } undef, {}* %_2.0, 0
  %1 = insertvalue { {}*, [3 x i64]* } %0, [3 x i64]* %_2.1, 1
  ret { {}*, [3 x i64]* } %1
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h7894e68fc0578af6E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::unique::Unique<T>::as_ref
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ref17hbce7dbd9f7968908E"({ i8*, i64* }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 0
  %_3.0 = load i8*, i8** %0, align 8, !nonnull !2
  %1 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 1
  %_3.1 = load i64*, i64** %1, align 8, !nonnull !2
; call core::ptr::unique::Unique<T>::as_ptr
  %2 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h1a5dfd32c28aaa62E"(i8* nonnull %_3.0, i64* noalias readonly align 8 dereferenceable(24) %_3.1)
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
define i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17he2bb317df3be6019E"(i8* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::mut_ptr::<impl *mut T>::offset
  %0 = call i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h0f645c1965bc591dE"(i8* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; core::ptr::mut_ptr::<impl *mut T>::offset
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h0f645c1965bc591dE"(i8* %self, i64 %count) unnamed_addr #0 {
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
define zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hdbf35946a04d3b48E"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
  %0 = icmp eq i8* %self, null
  ret i1 %0
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h90c0f25537f85227E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !2
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::new
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h03c63ab9a711f45eE"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_3 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hdbf35946a04d3b48E"(i8* %ptr)
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
  %_5 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h90c0f25537f85227E"(i8* %ptr)
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
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h5ff24208150a7370E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hbcc7d3320ea013caE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h49d381afadfa51f8E(i64 %align), !range !3
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !3
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout4size17habdc867cc00cae17E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout5align17hcdf6fbc707910731E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !3
; call core::num::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num12NonZeroUsize3get17h9d2488e4cc600e4aE(i64 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::alloc::layout::Layout::dangling
; Function Attrs: inlinehint uwtable
define internal nonnull i8* @_ZN4core5alloc6layout6Layout8dangling17h6a289f95946f0577E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call core::alloc::layout::Layout::align
  %_3 = call i64 @_ZN4core5alloc6layout6Layout5align17hcdf6fbc707910731E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = inttoptr i64 %_3 to i8*
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h90c0f25537f85227E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::slice::<impl [T]>::len
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17h52e39e7be40354c4E"([0 x i32]* noalias nonnull readonly align 4 %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %_2 = alloca %"core::ptr::Repr<u32>", align 8
  %0 = bitcast %"core::ptr::Repr<u32>"* %_2 to { [0 x i32]*, i64 }*
  %1 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %0, i32 0, i32 0
  store [0 x i32]* %self.0, [0 x i32]** %1, align 8
  %2 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %0, i32 0, i32 1
  store i64 %self.1, i64* %2, align 8
  %3 = bitcast %"core::ptr::Repr<u32>"* %_2 to { i32*, i64 }*
  %4 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %3, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  ret i64 %5
}

; core::option::Option<T>::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h7e2f2ae6b4128e85E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call core::mem::take
  %0 = call { i8*, i64 } @_ZN4core3mem4take17h07cfb3bd67ef11ffE({ i8*, i64 }* align 8 dereferenceable(16) %self)
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
define i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h08e7dc1c2ce2436fE"(i8* %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
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
  %v = load i8*, i8** %self, align 8, !nonnull !2
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
define void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h3bda895a95e4640fE"() unnamed_addr #1 {
start:
  ret void
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17ha430ee81eca1bceaE"(i8* nonnull %self) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h754b519c573d06eeE"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; alloc::alloc::alloc_zeroed
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc12alloc_zeroed17hbb1f60a990ab3af2E(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_2 = call i64 @_ZN4core5alloc6layout6Layout4size17habdc867cc00cae17E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_4 = call i64 @_ZN4core5alloc6layout6Layout5align17hcdf6fbc707910731E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_alloc_zeroed(i64 %_2, i64 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc15exchange_malloc17h6fa384c42032eb26E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_8 = alloca %"alloc::alloc::Global", align 1
  %_6 = alloca { i8*, i64 }, align 8
; call core::alloc::layout::Layout::from_size_align_unchecked
  %0 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hbcc7d3320ea013caE(i64 %size, i64 %align)
  %layout.0 = extractvalue { i64, i64 } %0, 0
  %layout.1 = extractvalue { i64, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call <alloc::alloc::Global as core::alloc::AllocRef>::alloc
  %1 = call { i8*, i64 } @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h136f91b2468b55daE"(%"alloc::alloc::Global"* nonnull align 1 %_8, i64 %layout.0, i64 %layout.1, i1 zeroext false)
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
  %memory.0 = load i8*, i8** %5, align 8, !nonnull !2
  %6 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_6, i32 0, i32 1
  %memory.1 = load i64, i64* %6, align 8
; call core::ptr::non_null::NonNull<T>::as_ptr
  %7 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h5ff24208150a7370E"(i8* nonnull %memory.0)
  br label %bb6

bb6:                                              ; preds = %bb5
  ret i8* %7
}

; alloc::alloc::alloc
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc5alloc17h6353cc1079f046d1E(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_2 = call i64 @_ZN4core5alloc6layout6Layout4size17habdc867cc00cae17E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_4 = call i64 @_ZN4core5alloc6layout6Layout5align17hcdf6fbc707910731E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = call i8* @__rust_alloc(i64 %_2, i64 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %4
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17h822afe5426a324baE(i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17habdc867cc00cae17E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_6 = call i64 @_ZN4core5alloc6layout6Layout5align17hcdf6fbc707910731E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::alloc::box_free
; Function Attrs: inlinehint uwtable
define void @_ZN5alloc5alloc8box_free17h0989123a145fc6aaE(i8* nonnull %0, i64* noalias readonly align 8 dereferenceable(24) %1) unnamed_addr #0 {
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
  %6 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ref17hbce7dbd9f7968908E"({ i8*, i64* }* noalias readonly align 8 dereferenceable(16) %ptr)
  %_4.0 = extractvalue { {}*, [3 x i64]* } %6, 0
  %_4.1 = extractvalue { {}*, [3 x i64]* } %6, 1
  br label %bb1

bb1:                                              ; preds = %start
  %7 = bitcast [3 x i64]* %_4.1 to i64*
  %8 = getelementptr inbounds i64, i64* %7, i64 1
  %9 = load i64, i64* %8, align 8, !invariant.load !2
  %10 = bitcast [3 x i64]* %_4.1 to i64*
  %11 = getelementptr inbounds i64, i64* %10, i64 2
  %12 = load i64, i64* %11, align 8, !invariant.load !2
  store i64 %9, i64* %3, align 8
  %size = load i64, i64* %3, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::unique::Unique<T>::as_ref
  %13 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ref17hbce7dbd9f7968908E"({ i8*, i64* }* noalias readonly align 8 dereferenceable(16) %ptr)
  %_8.0 = extractvalue { {}*, [3 x i64]* } %13, 0
  %_8.1 = extractvalue { {}*, [3 x i64]* } %13, 1
  br label %bb3

bb3:                                              ; preds = %bb2
  %14 = bitcast [3 x i64]* %_8.1 to i64*
  %15 = getelementptr inbounds i64, i64* %14, i64 1
  %16 = load i64, i64* %15, align 8, !invariant.load !2
  %17 = bitcast [3 x i64]* %_8.1 to i64*
  %18 = getelementptr inbounds i64, i64* %17, i64 2
  %19 = load i64, i64* %18, align 8, !invariant.load !2
  store i64 %19, i64* %2, align 8
  %align = load i64, i64* %2, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
; call core::alloc::layout::Layout::from_size_align_unchecked
  %20 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hbcc7d3320ea013caE(i64 %size, i64 %align)
  %layout.0 = extractvalue { i64, i64 } %20, 0
  %layout.1 = extractvalue { i64, i64 } %20, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %21 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %ptr, i32 0, i32 0
  %_17.0 = load i8*, i8** %21, align 8, !nonnull !2
  %22 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %ptr, i32 0, i32 1
  %_17.1 = load i64*, i64** %22, align 8, !nonnull !2
; call core::ptr::unique::Unique<T>::cast
  %_16 = call nonnull i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h34187990c4aed1e7E"(i8* nonnull %_17.0, i64* noalias readonly align 8 dereferenceable(24) %_17.1)
  br label %bb6

bb6:                                              ; preds = %bb5
; call <T as core::convert::Into<U>>::into
  %_15 = call nonnull i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17ha430ee81eca1bceaE"(i8* nonnull %_16)
  br label %bb7

bb7:                                              ; preds = %bb6
; call <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
  call void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17h8f6150e04adc5817E"(%"alloc::alloc::Global"* nonnull align 1 %_14, i8* nonnull %_15, i64 %layout.0, i64 %layout.1)
  br label %bb8

bb8:                                              ; preds = %bb7
  ret void
}

; alloc::boxed::Box<T>::leak
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$4leak17heb279647e6fcd9bdE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
  %0 = alloca { i8*, i64* }, align 8
  %_8 = alloca { i8*, i64* }, align 8
  %1 = bitcast { i8*, i64* }* %0 to { {}*, [3 x i64]* }*
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %1, i32 0, i32 0
  store {}* %b.0, {}** %2, align 8, !noalias !4
  %3 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %1, i32 0, i32 1
  store [3 x i64]* %b.1, [3 x i64]** %3, align 8, !noalias !4
  %4 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %0, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8, !noalias !4, !nonnull !2
  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %0, i32 0, i32 1
  %7 = load i64*, i64** %6, align 8, !noalias !4, !nonnull !2
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
  %_5.0 = load i8*, i8** %11, align 8, !nonnull !2
  %12 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %10, i32 0, i32 1
  %_5.1 = load i64*, i64** %12, align 8, !nonnull !2
; call core::ptr::unique::Unique<T>::as_ptr
  %13 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h1a5dfd32c28aaa62E"(i8* nonnull %_5.0, i64* noalias readonly align 8 dereferenceable(24) %_5.1)
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
define { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h337d87a7256e556fE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T>::leak
  %0 = call { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$4leak17heb279647e6fcd9bdE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
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
define internal { i8*, i64 } @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h136f91b2468b55daE"(%"alloc::alloc::Global"* nonnull align 1 %self, i64 %0, i64 %1, i1 zeroext %2) unnamed_addr #0 {
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
  %size = call i64 @_ZN4core5alloc6layout6Layout4size17habdc867cc00cae17E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
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
  %_9 = call nonnull i8* @_ZN4core5alloc6layout6Layout8dangling17h6a289f95946f0577E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb4

bb4:                                              ; preds = %bb3
  %9 = bitcast { i8*, i64 }* %_8 to i8**
  store i8* %_9, i8** %9, align 8
  %10 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_8, i32 0, i32 1
  store i64 0, i64* %10, align 8
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_8, i32 0, i32 0
  %12 = load i8*, i8** %11, align 8, !nonnull !2
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
  %_14.1 = load i64, i64* %18, align 8, !range !3
; call alloc::alloc::alloc_zeroed
  %19 = call i8* @_ZN5alloc5alloc12alloc_zeroed17hbb1f60a990ab3af2E(i64 %_14.0, i64 %_14.1)
  store i8* %19, i8** %raw_ptr, align 8
  br label %bb9

bb6:                                              ; preds = %bb2
  unreachable

bb7:                                              ; preds = %bb2
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_13.0 = load i64, i64* %20, align 8
  %21 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_13.1 = load i64, i64* %21, align 8, !range !3
; call alloc::alloc::alloc
  %22 = call i8* @_ZN5alloc5alloc5alloc17h6353cc1079f046d1E(i64 %_13.0, i64 %_13.1)
  store i8* %22, i8** %raw_ptr, align 8
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb10

bb9:                                              ; preds = %bb5
  br label %bb10

bb10:                                             ; preds = %bb8, %bb9
  %_19 = load i8*, i8** %raw_ptr, align 8
; call core::ptr::non_null::NonNull<T>::new
  %_18 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h03c63ab9a711f45eE"(i8* %_19)
  br label %bb11

bb11:                                             ; preds = %bb10
; call core::option::Option<T>::ok_or
  %_17 = call i8* @"_ZN4core6option15Option$LT$T$GT$5ok_or17h08e7dc1c2ce2436fE"(i8* %_18)
  br label %bb12

bb12:                                             ; preds = %bb11
; call <core::result::Result<T,E> as core::ops::try::Try>::into_result
  %23 = call i8* @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$11into_result17h988a188ce9308aa8E"(i8* %_17)
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
  %val = load i8*, i8** %_16, align 8, !nonnull !2
  %27 = bitcast { i8*, i64 }* %_26 to i8**
  store i8* %val, i8** %27, align 8
  %28 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_26, i32 0, i32 1
  store i64 %size, i64* %28, align 8
  %29 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_26, i32 0, i32 0
  %30 = load i8*, i8** %29, align 8, !nonnull !2
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
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h3bda895a95e4640fE"()
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
  %41 = call { i8*, i64 } @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$10from_error17h497eb65ef9d92fd0E"()
  store { i8*, i64 } %41, { i8*, i64 }* %3, align 8
  br label %bb19

bb19:                                             ; preds = %bb18
  br label %bb17

bb20:                                             ; preds = %bb14, %bb4
  br label %bb17
}

; <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
; Function Attrs: inlinehint uwtable
define internal void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17h8f6150e04adc5817E"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %2 = alloca {}, align 1
  %layout = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; call core::alloc::layout::Layout::size
  %_5 = call i64 @_ZN4core5alloc6layout6Layout4size17habdc867cc00cae17E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %_4 = icmp ne i64 %_5, 0
  br i1 %_4, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  br label %bb6

bb3:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_7 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h5ff24208150a7370E"(i8* nonnull %ptr)
  br label %bb4

bb4:                                              ; preds = %bb3
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_9.0 = load i64, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_9.1 = load i64, i64* %6, align 8, !range !3
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17h822afe5426a324baE(i8* %_7, i64 %_9.0, i64 %_9.1)
  br label %bb5

bb5:                                              ; preds = %bb4
  br label %bb6

bb6:                                              ; preds = %bb2, %bb5
  ret void
}

; <core::option::Option<T> as core::default::Default>::default
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN72_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h36117822c9ac85a1E"() unnamed_addr #0 {
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
define { i8*, i64 } @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$10from_error17h497eb65ef9d92fd0E"() unnamed_addr #0 {
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
define i8* @"_ZN73_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try..Try$GT$11into_result17h988a188ce9308aa8E"(i8* %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hf4a84d088d75cc42E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
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
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h82c28fc02bab35edE"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
  %_14 = alloca i8, align 1
  %_4 = alloca { i8*, i64 }, align 8
  %data = alloca { {}*, [3 x i64]* }, align 8
  store i8 0, i8* %_14, align 1
; call core::option::Option<T>::take
  %1 = call { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h7e2f2ae6b4128e85E"({ i8*, i64 }* align 8 dereferenceable(16) %self)
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
  %a.0 = load [0 x i8]*, [0 x i8]** %12, align 8, !nonnull !2
  %13 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %11, i32 0, i32 1
  %a.1 = load i64, i64* %13, align 8
; invoke alloc::alloc::exchange_malloc
  %14 = invoke i8* @_ZN5alloc5alloc15exchange_malloc17h6fa384c42032eb26E(i64 16, i64 8)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17had29f65c0bc9b1a6E.exit" unwind label %cleanup

"_ZN5alloc5boxed12Box$LT$T$GT$3new17had29f65c0bc9b1a6E.exit": ; preds = %bb5
  %15 = bitcast i8* %14 to { [0 x i8]*, i64 }*
  %16 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %15, i32 0, i32 0
  store [0 x i8]* %a.0, [0 x i8]** %16, align 8, !noalias !8
  %17 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %15, i32 0, i32 1
  store i64 %a.1, i64* %17, align 8
  br label %bb6

bb6:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17had29f65c0bc9b1a6E.exit"
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
  %_13.0 = load {}*, {}** %22, align 8, !nonnull !2
  %23 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  %_13.1 = load [3 x i64]*, [3 x i64]** %23, align 8, !nonnull !2
; invoke alloc::boxed::Box<T>::into_raw
  %24 = invoke { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h337d87a7256e556fE"({}* noalias nonnull align 1 %_13.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %_13.1)
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
  call void @_ZN4core3ptr13drop_in_place17h57f0b149c0190461E({ {}*, [3 x i64]* }* %data) #7
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

; issue_9::Foo::ez2
; Function Attrs: uwtable
define internal { [0 x i32]*, i64 } @_ZN7issue_93Foo3ez217h44d0776d18aafa78E({ i32*, i64 }* align 8 dereferenceable(16) %self, i32 %input) unnamed_addr #1 {
start:
  %_3 = alloca { [0 x i32]*, i64 }, align 8
  %_4 = icmp ult i32 %input, 5
  br i1 %_4, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_10 = load [0 x i32]*, [0 x i32]** bitcast (<{ i8*, [0 x i8] }>* @0 to [0 x i32]**), align 8, !nonnull !2
  %0 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %_3, i32 0, i32 0
  store [0 x i32]* %_10, [0 x i32]** %0, align 8
  %1 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %_3, i32 0, i32 1
  store i64 0, i64* %1, align 8
  br label %bb3

bb2:                                              ; preds = %start
  %2 = bitcast { i32*, i64 }* %self to { [0 x i32]*, i64 }*
  %3 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %2, i32 0, i32 0
  %_6.0 = load [0 x i32]*, [0 x i32]** %3, align 8, !nonnull !2
  %4 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %2, i32 0, i32 1
  %_6.1 = load i64, i64* %4, align 8
  %5 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %_3, i32 0, i32 0
  store [0 x i32]* %_6.0, [0 x i32]** %5, align 8
  %6 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %_3, i32 0, i32 1
  store i64 %_6.1, i64* %6, align 8
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  %7 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %_3, i32 0, i32 0
  %8 = load [0 x i32]*, [0 x i32]** %7, align 8, !nonnull !2
  %9 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %_3, i32 0, i32 1
  %10 = load i64, i64* %9, align 8
  %11 = insertvalue { [0 x i32]*, i64 } undef, [0 x i32]* %8, 0
  %12 = insertvalue { [0 x i32]*, i64 } %11, i64 %10, 1
  ret { [0 x i32]*, i64 } %12
}

; issue_9::Foo::ez3
; Function Attrs: uwtable
define internal i64 @_ZN7issue_93Foo3ez317hc88270a14359ff66E({ i32*, i64 }* align 8 dereferenceable(16) %self, i32 %input) unnamed_addr #1 {
start:
; call issue_9::Foo::ez2
  %0 = call { [0 x i32]*, i64 } @_ZN7issue_93Foo3ez217h44d0776d18aafa78E({ i32*, i64 }* align 8 dereferenceable(16) %self, i32 %input)
  %_6.0 = extractvalue { [0 x i32]*, i64 } %0, 0
  %_6.1 = extractvalue { [0 x i32]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::slice::<impl [T]>::len
  %_4 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17h52e39e7be40354c4E"([0 x i32]* noalias nonnull readonly align 4 %_6.0, i64 %_6.1)
  br label %bb2

bb2:                                              ; preds = %bb1
  %_3 = icmp ugt i64 %_4, 0
  br i1 %_3, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
; call std::panicking::begin_panic
  call void @_ZN3std9panicking11begin_panic17h647a347b27423f53E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [5 x i8] }>* @alloc34 to [0 x i8]*), i64 5, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc36 to %"core::panic::Location"*))
  unreachable

bb4:                                              ; preds = %bb2
  ret i64 1
}

; issue_9::main
; Function Attrs: uwtable
define void @_ZN7issue_94main17h18f4fe5077b5cb8bE() unnamed_addr #1 {
start:
  %_17 = alloca i64*, align 8
  %_16 = alloca [1 x { i8*, i8* }], align 8
  %_9 = alloca %"core::fmt::Arguments", align 8
  %out = alloca i64, align 8
  %foo = alloca { i32*, i64 }, align 8
  %0 = bitcast { i32*, i64 }* %foo to { [0 x i32]*, i64 }*
  %1 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %0, i32 0, i32 0
  store [0 x i32]* bitcast (<{ [8 x i8] }>* @_ZN7issue_93BUF17hcc9c6437bcff6fd1E to [0 x i32]*), [0 x i32]** %1, align 8
  %2 = getelementptr inbounds { [0 x i32]*, i64 }, { [0 x i32]*, i64 }* %0, i32 0, i32 1
  store i64 2, i64* %2, align 8
; call issue_9::Foo::ez3
  %3 = call i64 @_ZN7issue_93Foo3ez317hc88270a14359ff66E({ i32*, i64 }* align 8 dereferenceable(16) %foo, i32 2)
  store i64 %3, i64* %out, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %_23 = load [2 x { [0 x i8]*, i64 }]*, [2 x { [0 x i8]*, i64 }]** bitcast (<{ i8*, [0 x i8] }>* @1 to [2 x { [0 x i8]*, i64 }]**), align 8, !nonnull !2
  %_10.0 = bitcast [2 x { [0 x i8]*, i64 }]* %_23 to [0 x { [0 x i8]*, i64 }]*
  store i64* %out, i64** %_17, align 8
  %arg0 = load i64*, i64** %_17, align 8, !nonnull !2
; call core::fmt::ArgumentV1::new
  %4 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17hf9394e7024b31c51E(i64* noalias readonly align 8 dereferenceable(8) %arg0, i1 (i64*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h2bd91e40b91ccaf6E")
  %_20.0 = extractvalue { i8*, i8* } %4, 0
  %_20.1 = extractvalue { i8*, i8* } %4, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast [1 x { i8*, i8* }]* %_16 to { i8*, i8* }*
  %6 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %5, i32 0, i32 0
  store i8* %_20.0, i8** %6, align 8
  %7 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %5, i32 0, i32 1
  store i8* %_20.1, i8** %7, align 8
  %_13.0 = bitcast [1 x { i8*, i8* }]* %_16 to [0 x { i8*, i8* }]*
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h097ac936ced356f4E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_9, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %_10.0, i64 2, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %_13.0, i64 1)
  br label %bb3

bb3:                                              ; preds = %bb2
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he30fba25a77fc277E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_9)
  br label %bb4

bb4:                                              ; preds = %bb3
  ret void
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

; core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h2bd91e40b91ccaf6E"(i64* noalias readonly align 8 dereferenceable(8), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17he30fba25a77fc277E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #1

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
!2 = !{}
!3 = !{i64 1, i64 0}
!4 = !{!5, !7}
!5 = distinct !{!5, !6, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h47326501a9781028E: %value.0"}
!6 = distinct !{!6, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h47326501a9781028E"}
!7 = distinct !{!7, !6, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17h47326501a9781028E: %value.1"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17had29f65c0bc9b1a6E: %x.0"}
!10 = distinct !{!10, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17had29f65c0bc9b1a6E"}
