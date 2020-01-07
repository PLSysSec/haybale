; ModuleID = 'panic.3a1fbbbh-cgu.0'
source_filename = "panic.3a1fbbbh-cgu.0"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::option::Option<usize>::Some" = type { [1 x i64], i64, [0 x i64] }
%"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [0 x i64], %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock", [0 x i64] }
%"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock" = type { [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64] }
%"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [4 x i64] }
%"core::marker::PhantomData<core::any::Any>" = type {}
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* } { void ({ i8*, i64 }*)* @_ZN4core3ptr18real_drop_in_place17h8d4737cc87bef060E, i64 16, i64 8, { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17h976b72f68f3a4ba9E", { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17he9402bbb616b91d0E" }, align 8
@0 = private unnamed_addr constant <{ [46 x i8] }> <{ [46 x i8] c"attempt to copy from unaligned or null pointer" }>, align 1
@1 = private unnamed_addr constant <{ [25 x i8] }> <{ [25 x i8] c"src/libcore/intrinsics.rs" }>, align 1
@2 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [46 x i8] }>, <{ [46 x i8] }>* @0, i32 0, i32 0, i32 0), [8 x i8] c".\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [25 x i8] }>, <{ [25 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\19\00\00\00\00\00\00\00\C9\05\00\00\05\00\00\00" }>, align 8
@3 = private unnamed_addr constant <{ [44 x i8] }> <{ [44 x i8] c"attempt to copy to unaligned or null pointer" }>, align 1
@4 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [44 x i8] }>, <{ [44 x i8] }>* @3, i32 0, i32 0, i32 0), [8 x i8] c",\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [25 x i8] }>, <{ [25 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\19\00\00\00\00\00\00\00\CA\05\00\00\05\00\00\00" }>, align 8
@5 = private unnamed_addr constant <{ [37 x i8] }> <{ [37 x i8] c"attempt to copy to overlapping memory" }>, align 1
@6 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [37 x i8] }>, <{ [37 x i8] }>* @5, i32 0, i32 0, i32 0), [8 x i8] c"%\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [25 x i8] }>, <{ [25 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\19\00\00\00\00\00\00\00\CB\05\00\00\05\00\00\00" }>, align 8
@str.1 = internal constant [73 x i8] c"/rustc/4560ea788cb760f0a34127156c78e2552949f734/src/libcore/intrinsics.rs"
@str.2 = internal constant [57 x i8] c"attempt to calculate the remainder with a divisor of zero"
@panic_loc.3 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([57 x i8]* @str.2 to [0 x i8]*), i64 57 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([73 x i8]* @str.1 to [0 x i8]*), i64 73 }, i32 1374, i32 23 }, align 8
@7 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Option::unwrap()` on a `None` value" }>, align 1
@8 = private unnamed_addr constant <{ [21 x i8] }> <{ [21 x i8] c"src/libcore/option.rs" }>, align 1
@9 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [43 x i8] }>, <{ [43 x i8] }>* @7, i32 0, i32 0, i32 0), [8 x i8] c"+\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [21 x i8] }>, <{ [21 x i8] }>* @8, i32 0, i32 0, i32 0), [16 x i8] c"\15\00\00\00\00\00\00\00z\01\00\00\15\00\00\00" }>, align 8
@vtable.4 = private unnamed_addr constant { void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* } { void ({ [0 x i8]*, i64 }*)* @_ZN4core3ptr18real_drop_in_place17h9514e60fbb172903E, i64 16, i64 8, i64 ({ [0 x i8]*, i64 }*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h8fff890bdf38c78dE" }, align 8
@vtable.5 = private unnamed_addr constant { void ({}*)*, i64, i64, i64 ({}*)* } { void ({}*)* @_ZN4core3ptr18real_drop_in_place17h4a75f11441a278b0E, i64 0, i64 1, i64 ({}*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hacfc9f1454cc02a5E" }, align 8
@10 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"panic.rs" }>, align 1
@11 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @10, i32 0, i32 0, i32 0), [16 x i8] c"\08\00\00\00\00\00\00\00\03\00\00\00\09\00\00\00" }>, align 8
@12 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"a > 2" }>, align 1

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17hbf4722bb0beb6bdfE"(i8* nonnull %unique.0, i64* noalias readonly align 8 dereferenceable(24) %unique.1) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %0 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hee29313ff7dc4286E"(i8* nonnull %unique.0, i64* noalias readonly align 8 dereferenceable(24) %unique.1)
  %1 = extractvalue { {}*, [3 x i64]* } %0, 0
  %2 = extractvalue { {}*, [3 x i64]* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %3 = call { i8*, i64* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h0d6a84545bc2897eE"({}* %1, [3 x i64]* noalias readonly align 8 dereferenceable(24) %2)
  %4 = extractvalue { i8*, i64* } %3, 0
  %5 = extractvalue { i8*, i64* } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { i8*, i64* } undef, i8* %4, 0
  %7 = insertvalue { i8*, i64* } %6, i64* %5, 1
  ret { i8*, i64* } %7
}

; <T as core::any::Any>::type_id
; Function Attrs: uwtable
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h8fff890bdf38c78dE"({ [0 x i8]*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17h2e10f19185efc463E()
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; <T as core::any::Any>::type_id
; Function Attrs: uwtable
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hacfc9f1454cc02a5E"({}* noalias nonnull readonly align 1 %self) unnamed_addr #1 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17h58617bc15d63018eE()
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn uwtable
define void @_ZN3std9panicking11begin_panic17h8273b1e5d825bcfaE([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) %file_line_col) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
  %_11 = alloca i8, align 1
  %_9 = alloca i64*, align 8
  %_7 = alloca { i8*, i64 }, align 8
  store i8 0, i8* %_11, align 1
  store i8 1, i8* %_11, align 1
  br i1 false, label %bb3, label %bb2

bb1:                                              ; preds = %bb6, %bb7
  %1 = bitcast { i8*, i32 }* %0 to i8**
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %4 = load i32, i32* %3, align 8
  %5 = insertvalue { i8*, i32 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i32 } %5, i32 %4, 1
  resume { i8*, i32 } %6

bb2:                                              ; preds = %start
  store i8 0, i8* %_11, align 1
; invoke std::panicking::begin_panic::PanicPayload<A>::new
  %7 = invoke { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h4f97964d7c002c26E"([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %start
  call void @llvm.trap()
  unreachable

bb4:                                              ; preds = %bb2
  store { i8*, i64 } %7, { i8*, i64 }* %_7, align 8
  %8 = bitcast { i8*, i64 }* %_7 to {}*
  %9 = bitcast i64** %_9 to {}**
  store {}* null, {}** %9, align 8
  %10 = load i64*, i64** %_9, align 8
; invoke std::panicking::rust_panic_with_hook
  invoke void @_ZN3std9panicking20rust_panic_with_hook17h0c4b67125f55410aE({}* nonnull align 1 %8, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* }* @vtable.0 to [3 x i64]*), i64* noalias readonly align 8 dereferenceable_or_null(48) %10, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) %file_line_col)
          to label %unreachable unwind label %cleanup1

bb5:                                              ; preds = %cleanup1
  br label %bb7

bb6:                                              ; preds = %bb7
  store i8 0, i8* %_11, align 1
  br label %bb1

bb7:                                              ; preds = %bb5, %cleanup
  %11 = load i8, i8* %_11, align 1, !range !0
  %12 = trunc i8 %11 to i1
  br i1 %12, label %bb6, label %bb1

cleanup:                                          ; preds = %bb2
  %13 = landingpad { i8*, i32 }
          cleanup
  %14 = extractvalue { i8*, i32 } %13, 0
  %15 = extractvalue { i8*, i32 } %13, 1
  %16 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %14, i8** %16, align 8
  %17 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %15, i32* %17, align 8
  br label %bb7

unreachable:                                      ; preds = %bb4
  unreachable

cleanup1:                                         ; preds = %bb4
  %18 = landingpad { i8*, i32 }
          cleanup
  %19 = extractvalue { i8*, i32 } %18, 0
  %20 = extractvalue { i8*, i32 } %18, 1
  %21 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %19, i8** %21, align 8
  %22 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %20, i32* %22, align 8
  br label %bb5
}

; std::panicking::begin_panic::PanicPayload<A>::new
; Function Attrs: uwtable
define { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h4f97964d7c002c26E"([0 x i8]* noalias nonnull readonly align 1 %inner.0, i64 %inner.1) unnamed_addr #1 {
start:
  %_2 = alloca { i8*, i64 }, align 8
  %_0 = alloca { i8*, i64 }, align 8
  %0 = bitcast { i8*, i64 }* %_2 to { [0 x i8]*, i64 }*
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %0, i32 0, i32 0
  store [0 x i8]* %inner.0, [0 x i8]** %1, align 8
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %0, i32 0, i32 1
  store i64 %inner.1, i64* %2, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_2, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_2, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0, i32 0, i32 0
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0, i32 0, i32 1
  %12 = load i64, i64* %11, align 8
  %13 = insertvalue { i8*, i64 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i64 } %13, i64 %12, 1
  ret { i8*, i64 } %14
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %src, i8* %dst, i64 %count) unnamed_addr #0 {
start:
  br i1 false, label %bb1, label %bb5

bb1:                                              ; preds = %start
; call core::intrinsics::is_aligned_and_not_null
  %0 = call zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17h754ce2e1ffc4ad22E(i8* %src)
  br label %bb2

bb2:                                              ; preds = %bb1
  %1 = xor i1 %0, true
  br i1 %1, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
  br label %bb5

bb4:                                              ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @2 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb5:                                              ; preds = %bb3, %start
  br i1 false, label %bb6, label %bb10

bb6:                                              ; preds = %bb5
; call core::intrinsics::is_aligned_and_not_null
  %2 = call zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17h754ce2e1ffc4ad22E(i8* %dst)
  br label %bb7

bb7:                                              ; preds = %bb6
  %3 = xor i1 %2, true
  br i1 %3, label %bb9, label %bb8

bb8:                                              ; preds = %bb7
  br label %bb10

bb9:                                              ; preds = %bb7
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @4 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb10:                                             ; preds = %bb8, %bb5
  br i1 false, label %bb11, label %bb15

bb11:                                             ; preds = %bb10
; call core::intrinsics::overlaps
  %4 = call zeroext i1 @_ZN4core10intrinsics8overlaps17h8ccb6fe4eb37eff0E(i8* %src, i8* %dst, i64 %count)
  br label %bb12

bb12:                                             ; preds = %bb11
  %5 = xor i1 %4, true
  %6 = xor i1 %5, true
  br i1 %6, label %bb14, label %bb13

bb13:                                             ; preds = %bb12
  br label %bb15

bb14:                                             ; preds = %bb12
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @6 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb15:                                             ; preds = %bb13, %bb10
  %7 = mul i64 1, %count
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %dst, i8* align 1 %src, i64 %7, i1 false)
  br label %bb16

bb16:                                             ; preds = %bb15
  ret void
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17h6feb3a6ae6660c8aE({ i8*, i64 }* %src, { i8*, i64 }* %dst, i64 %count) unnamed_addr #0 {
start:
  br i1 false, label %bb1, label %bb5

bb1:                                              ; preds = %start
; call core::intrinsics::is_aligned_and_not_null
  %0 = call zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17h733788fa8ba6cf68E({ i8*, i64 }* %src)
  br label %bb2

bb2:                                              ; preds = %bb1
  %1 = xor i1 %0, true
  br i1 %1, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
  br label %bb5

bb4:                                              ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @2 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb5:                                              ; preds = %bb3, %start
  br i1 false, label %bb6, label %bb10

bb6:                                              ; preds = %bb5
; call core::intrinsics::is_aligned_and_not_null
  %2 = call zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17h733788fa8ba6cf68E({ i8*, i64 }* %dst)
  br label %bb7

bb7:                                              ; preds = %bb6
  %3 = xor i1 %2, true
  br i1 %3, label %bb9, label %bb8

bb8:                                              ; preds = %bb7
  br label %bb10

bb9:                                              ; preds = %bb7
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @4 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb10:                                             ; preds = %bb8, %bb5
  br i1 false, label %bb11, label %bb15

bb11:                                             ; preds = %bb10
; call core::intrinsics::overlaps
  %4 = call zeroext i1 @_ZN4core10intrinsics8overlaps17hc76e2102b8c8c416E({ i8*, i64 }* %src, { i8*, i64 }* %dst, i64 %count)
  br label %bb12

bb12:                                             ; preds = %bb11
  %5 = xor i1 %4, true
  %6 = xor i1 %5, true
  br i1 %6, label %bb14, label %bb13

bb13:                                             ; preds = %bb12
  br label %bb15

bb14:                                             ; preds = %bb12
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @6 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb15:                                             ; preds = %bb13, %bb10
  %7 = mul i64 16, %count
  %8 = bitcast { i8*, i64 }* %dst to i8*
  %9 = bitcast { i8*, i64 }* %src to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %8, i8* align 8 %9, i64 %7, i1 false)
  br label %bb16

bb16:                                             ; preds = %bb15
  ret void
}

; core::intrinsics::is_aligned_and_not_null
; Function Attrs: uwtable
define zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17h733788fa8ba6cf68E({ i8*, i64 }* %ptr) unnamed_addr #1 {
start:
  %_0 = alloca i8, align 1
; call core::ptr::<impl *const T>::is_null
  %0 = call zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf2f323b98c2e5d70E"({ i8*, i64 }* %ptr)
  br label %bb5

bb1:                                              ; preds = %bb7
  store i8 1, i8* %_0, align 1
  br label %bb4

bb2:                                              ; preds = %bb7, %bb5
  store i8 0, i8* %_0, align 1
  br label %bb4

bb3:                                              ; preds = %bb5
  %1 = ptrtoint { i8*, i64 }* %ptr to i64
; call core::mem::align_of
  %2 = call i64 @_ZN4core3mem8align_of17h04d04943ccb5c5b1E()
  br label %bb6

bb4:                                              ; preds = %bb2, %bb1
  %3 = load i8, i8* %_0, align 1, !range !0
  %4 = trunc i8 %3 to i1
  ret i1 %4

bb5:                                              ; preds = %start
  %5 = xor i1 %0, true
  br i1 %5, label %bb3, label %bb2

bb6:                                              ; preds = %bb3
  %6 = icmp eq i64 %2, 0
  %7 = call i1 @llvm.expect.i1(i1 %6, i1 false)
  br i1 %7, label %panic, label %bb7

bb7:                                              ; preds = %bb6
  %8 = urem i64 %1, %2
  %9 = icmp eq i64 %8, 0
  br i1 %9, label %bb1, label %bb2

panic:                                            ; preds = %bb6
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.3 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; core::intrinsics::is_aligned_and_not_null
; Function Attrs: uwtable
define zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17h754ce2e1ffc4ad22E(i8* %ptr) unnamed_addr #1 {
start:
  %_0 = alloca i8, align 1
; call core::ptr::<impl *const T>::is_null
  %0 = call zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h9412bde71985e8f8E"(i8* %ptr)
  br label %bb5

bb1:                                              ; preds = %bb7
  store i8 1, i8* %_0, align 1
  br label %bb4

bb2:                                              ; preds = %bb7, %bb5
  store i8 0, i8* %_0, align 1
  br label %bb4

bb3:                                              ; preds = %bb5
  %1 = ptrtoint i8* %ptr to i64
; call core::mem::align_of
  %2 = call i64 @_ZN4core3mem8align_of17hb496263dfa6a12a1E()
  br label %bb6

bb4:                                              ; preds = %bb2, %bb1
  %3 = load i8, i8* %_0, align 1, !range !0
  %4 = trunc i8 %3 to i1
  ret i1 %4

bb5:                                              ; preds = %start
  %5 = xor i1 %0, true
  br i1 %5, label %bb3, label %bb2

bb6:                                              ; preds = %bb3
  %6 = icmp eq i64 %2, 0
  %7 = call i1 @llvm.expect.i1(i1 %6, i1 false)
  br i1 %7, label %panic, label %bb7

bb7:                                              ; preds = %bb6
  %8 = urem i64 %1, %2
  %9 = icmp eq i64 %8, 0
  br i1 %9, label %bb1, label %bb2

panic:                                            ; preds = %bb6
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.3 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; core::intrinsics::overlaps
; Function Attrs: uwtable
define zeroext i1 @_ZN4core10intrinsics8overlaps17h8ccb6fe4eb37eff0E(i8* %src, i8* %dst, i64 %count) unnamed_addr #1 {
start:
  %diff = alloca i64, align 8
  %src_usize = ptrtoint i8* %src to i64
  %dst_usize = ptrtoint i8* %dst to i64
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17hf18c4bf66e1c615cE()
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::checked_mul
  %1 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17he09525423c6155d6E"(i64 %0, i64 %count)
  %2 = extractvalue { i64, i64 } %1, 0
  %3 = extractvalue { i64, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::option::Option<T>::unwrap
  %4 = call i64 @"_ZN4core6option15Option$LT$T$GT$6unwrap17h03c37b25373178a2E"(i64 %2, i64 %3)
  br label %bb3

bb3:                                              ; preds = %bb2
  %5 = icmp ugt i64 %src_usize, %dst_usize
  br i1 %5, label %bb5, label %bb4

bb4:                                              ; preds = %bb3
  %6 = sub i64 %dst_usize, %src_usize
  store i64 %6, i64* %diff, align 8
  br label %bb6

bb5:                                              ; preds = %bb3
  %7 = sub i64 %src_usize, %dst_usize
  store i64 %7, i64* %diff, align 8
  br label %bb6

bb6:                                              ; preds = %bb4, %bb5
  %8 = load i64, i64* %diff, align 8
  %9 = icmp ugt i64 %4, %8
  ret i1 %9
}

; core::intrinsics::overlaps
; Function Attrs: uwtable
define zeroext i1 @_ZN4core10intrinsics8overlaps17hc76e2102b8c8c416E({ i8*, i64 }* %src, { i8*, i64 }* %dst, i64 %count) unnamed_addr #1 {
start:
  %diff = alloca i64, align 8
  %src_usize = ptrtoint { i8*, i64 }* %src to i64
  %dst_usize = ptrtoint { i8*, i64 }* %dst to i64
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17h9464861885e784a5E()
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::checked_mul
  %1 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17he09525423c6155d6E"(i64 %0, i64 %count)
  %2 = extractvalue { i64, i64 } %1, 0
  %3 = extractvalue { i64, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::option::Option<T>::unwrap
  %4 = call i64 @"_ZN4core6option15Option$LT$T$GT$6unwrap17h03c37b25373178a2E"(i64 %2, i64 %3)
  br label %bb3

bb3:                                              ; preds = %bb2
  %5 = icmp ugt i64 %src_usize, %dst_usize
  br i1 %5, label %bb5, label %bb4

bb4:                                              ; preds = %bb3
  %6 = sub i64 %dst_usize, %src_usize
  store i64 %6, i64* %diff, align 8
  br label %bb6

bb5:                                              ; preds = %bb3
  %7 = sub i64 %src_usize, %dst_usize
  store i64 %7, i64* %diff, align 8
  br label %bb6

bb6:                                              ; preds = %bb4, %bb5
  %8 = load i64, i64* %diff, align 8
  %9 = icmp ugt i64 %4, %8
  ret i1 %9
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17h2e10f19185efc463E() unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %_0 = alloca i64, align 8
  store i64 1229646359891580772, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %1, i64* %_0, align 8
  %2 = load i64, i64* %_0, align 8
  ret i64 %2
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17h58617bc15d63018eE() unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %_0 = alloca i64, align 8
  store i64 7549865886324542212, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %1, i64* %_0, align 8
  %2 = load i64, i64* %_0, align 8
  ret i64 %2
}

; core::mem::swap
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem4swap17h081aca4ce390846cE({ i8*, i64 }* align 8 dereferenceable(16) %x, { i8*, i64 }* align 8 dereferenceable(16) %y) unnamed_addr #0 {
start:
; call core::ptr::swap_nonoverlapping_one
  call void @_ZN4core3ptr23swap_nonoverlapping_one17h6a8f55afc8db658aE({ i8*, i64 }* %x, { i8*, i64 }* %y)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3mem4take17h7a417ec2384aa1b7E({ i8*, i64 }* align 8 dereferenceable(16) %dest) unnamed_addr #0 {
start:
; call <core::option::Option<T> as core::default::Default>::default
  %0 = call { i8*, i64 } @"_ZN72_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17hb57d367043dc0e9bE"()
  %1 = extractvalue { i8*, i64 } %0, 0
  %2 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::mem::replace
  %3 = call { i8*, i64 } @_ZN4core3mem7replace17he389d99eedc24d96E({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1 %1, i64 %2)
  %4 = extractvalue { i8*, i64 } %3, 0
  %5 = extractvalue { i8*, i64 } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { i8*, i64 } undef, i8* %4, 0
  %7 = insertvalue { i8*, i64 } %6, i64 %5, 1
  ret { i8*, i64 } %7
}

; core::mem::forget
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem6forget17hba650da2291da06cE({}* noalias nonnull align 1 %t.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %t.1) unnamed_addr #0 {
start:
  %_0.i = alloca { i8*, i64* }, align 8
  %0 = bitcast { i8*, i64* }* %_0.i to { {}*, [3 x i64]* }*
  %1 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %0, i32 0, i32 0
  store {}* %t.0, {}** %1, align 8, !noalias !1
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %0, i32 0, i32 1
  store [3 x i64]* %t.1, [3 x i64]** %2, align 8, !noalias !1
  %3 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %_0.i, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8, !noalias !1, !nonnull !5
  %5 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %_0.i, i32 0, i32 1
  %6 = load i64*, i64** %5, align 8, !noalias !1, !nonnull !5
  %7 = insertvalue { i8*, i64* } undef, i8* %4, 0
  %8 = insertvalue { i8*, i64* } %7, i64* %6, 1
  %9 = extractvalue { i8*, i64* } %8, 0
  %10 = extractvalue { i8*, i64* } %8, 1
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::replace
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3mem7replace17he389d99eedc24d96E({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1, i64) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %2 = alloca { i8*, i32 }, align 8
  %src = alloca { i8*, i64 }, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 0
  store i8* %0, i8** %3, align 8
  %4 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 1
  store i64 %1, i64* %4, align 8
; invoke core::mem::swap
  invoke void @_ZN4core3mem4swap17h081aca4ce390846cE({ i8*, i64 }* align 8 dereferenceable(16) %dest, { i8*, i64 }* align 8 dereferenceable(16) %src)
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

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17h32d0bd7cc53fbd27E() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 32, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17h9464861885e784a5E() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 16, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17hf18c4bf66e1c615cE() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 1, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::mem::align_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem8align_of17h04d04943ccb5c5b1E() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 8, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::mem::align_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem8align_of17hb496263dfa6a12a1E() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 1, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::num::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize13new_unchecked17he0e2f32a635eeee2E(i64 %n) unnamed_addr #0 {
start:
  %_0 = alloca i64, align 8
  store i64 %n, i64* %_0, align 8
  %0 = load i64, i64* %_0, align 8, !range !6
  ret i64 %0
}

; core::num::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize3get17h97645f35b46ed41aE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::num::<impl usize>::checked_mul
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17he09525423c6155d6E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %_0 = alloca { i64, i64 }, align 8
; call core::num::<impl usize>::overflowing_mul
  %0 = call { i64, i8 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_mul17hbfcd8418ef7bd2d2E"(i64 %self, i64 %rhs)
  %a = extractvalue { i64, i8 } %0, 0
  %1 = extractvalue { i64, i8 } %0, 1
  %b = trunc i8 %1 to i1
  br label %bb1

bb1:                                              ; preds = %start
  br i1 %b, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  %2 = bitcast { i64, i64 }* %_0 to %"core::option::Option<usize>::Some"*
  %3 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %2, i32 0, i32 1
  store i64 %a, i64* %3, align 8
  %4 = bitcast { i64, i64 }* %_0 to i64*
  store i64 1, i64* %4, align 8
  br label %bb4

bb3:                                              ; preds = %bb1
  %5 = bitcast { i64, i64 }* %_0 to i64*
  store i64 0, i64* %5, align 8
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 0
  %7 = load i64, i64* %6, align 8, !range !7
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
  %10 = insertvalue { i64, i64 } undef, i64 %7, 0
  %11 = insertvalue { i64, i64 } %10, i64 %9, 1
  ret { i64, i64 } %11
}

; core::num::<impl usize>::overflowing_mul
; Function Attrs: inlinehint uwtable
define internal { i64, i8 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_mul17hbfcd8418ef7bd2d2E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %0 = alloca { i64, i8 }, align 8
  %_0 = alloca { i64, i8 }, align 8
  %1 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %self, i64 %rhs)
  %2 = extractvalue { i64, i1 } %1, 0
  %3 = extractvalue { i64, i1 } %1, 1
  %4 = zext i1 %3 to i8
  %5 = bitcast { i64, i8 }* %0 to i64*
  store i64 %2, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  store i8 %4, i8* %6, align 8
  %7 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 0
  %a = load i64, i64* %7, align 8
  %8 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  %9 = load i8, i8* %8, align 8, !range !0
  %b = trunc i8 %9 to i1
  br label %bb1

bb1:                                              ; preds = %start
  %10 = bitcast { i64, i8 }* %_0 to i64*
  store i64 %a, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 1
  %12 = zext i1 %b to i8
  store i8 %12, i8* %11, align 8
  %13 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 0
  %14 = load i64, i64* %13, align 8
  %15 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 1
  %16 = load i8, i8* %15, align 8, !range !0
  %17 = trunc i8 %16 to i1
  %18 = zext i1 %17 to i8
  %19 = insertvalue { i64, i8 } undef, i64 %14, 0
  %20 = insertvalue { i64, i8 } %19, i8 %18, 1
  ret { i64, i8 } %20
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h4a75f11441a278b0E({}* nonnull align 1 %_1) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h594c2ebe64efcbf7E({}* nonnull align 1 %_1.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #1 {
start:
  %0 = bitcast [3 x i64]* %_1.1 to void ({}*)**
  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 0
  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !5, !nonnull !5
  call void %2({}* align 1 %_1.0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h8d4737cc87bef060E({ i8*, i64 }* align 8 dereferenceable(16) %_1) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h9514e60fbb172903E({ [0 x i8]*, i64 }* align 8 dereferenceable(16) %_1) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17hfbf77e21fbb099e6E({ {}*, [3 x i64]* }* align 8 dereferenceable(16)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_1 = alloca { {}*, [3 x i64]* }*, align 8
  store { {}*, [3 x i64]* }* %0, { {}*, [3 x i64]* }** %_1, align 8
  %2 = load { {}*, [3 x i64]* }*, { {}*, [3 x i64]* }** %_1, align 8, !nonnull !5
  %3 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %2, i32 0, i32 0
  %4 = load {}*, {}** %3, align 8, !nonnull !5
  %5 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %2, i32 0, i32 1
  %6 = load [3 x i64]*, [3 x i64]** %5, align 8, !nonnull !5
  %7 = bitcast [3 x i64]* %6 to void ({}*)**
  %8 = getelementptr inbounds void ({}*)*, void ({}*)** %7, i64 0
  %9 = load void ({}*)*, void ({}*)** %8, align 8, !invariant.load !5, !nonnull !5
  invoke void %9({}* align 1 %4)
          to label %bb3 unwind label %cleanup

bb1:                                              ; preds = %bb3
  ret void

bb2:                                              ; preds = %bb4
  %10 = bitcast { i8*, i32 }* %1 to i8**
  %11 = load i8*, i8** %10, align 8
  %12 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %13 = load i32, i32* %12, align 8
  %14 = insertvalue { i8*, i32 } undef, i8* %11, 0
  %15 = insertvalue { i8*, i32 } %14, i32 %13, 1
  resume { i8*, i32 } %15

bb3:                                              ; preds = %start
  %16 = load { {}*, [3 x i64]* }*, { {}*, [3 x i64]* }** %_1, align 8, !nonnull !5
  %17 = bitcast { {}*, [3 x i64]* }* %16 to { i8*, i64* }*
  %18 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %17, i32 0, i32 0
  %19 = load i8*, i8** %18, align 8, !nonnull !5
  %20 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %17, i32 0, i32 1
  %21 = load i64*, i64** %20, align 8, !nonnull !5
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h0e4c9a4d56147c0bE(i8* nonnull %19, i64* noalias readonly align 8 dereferenceable(24) %21)
  br label %bb1

bb4:                                              ; preds = %cleanup
  %22 = load { {}*, [3 x i64]* }*, { {}*, [3 x i64]* }** %_1, align 8, !nonnull !5
  %23 = bitcast { {}*, [3 x i64]* }* %22 to { i8*, i64* }*
  %24 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %23, i32 0, i32 0
  %25 = load i8*, i8** %24, align 8, !nonnull !5
  %26 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %23, i32 0, i32 1
  %27 = load i64*, i64** %26, align 8, !nonnull !5
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h0e4c9a4d56147c0bE(i8* nonnull %25, i64* noalias readonly align 8 dereferenceable(24) %27) #10
  br label %bb2

cleanup:                                          ; preds = %start
  %28 = landingpad { i8*, i32 }
          cleanup
  %29 = extractvalue { i8*, i32 } %28, 0
  %30 = extractvalue { i8*, i32 } %28, 1
  %31 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %29, i8** %31, align 8
  %32 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %30, i32* %32, align 8
  br label %bb4
}

; core::ptr::swap_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr19swap_nonoverlapping17h4ab4d0522774d0deE({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 %count) unnamed_addr #0 {
start:
  %x1 = bitcast { i8*, i64 }* %x to i8*
  %y2 = bitcast { i8*, i64 }* %y to i8*
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17h9464861885e784a5E()
  br label %bb1

bb1:                                              ; preds = %start
  %len = mul i64 %0, %count
; call core::ptr::swap_nonoverlapping_bytes
  call void @_ZN4core3ptr25swap_nonoverlapping_bytes17hd2aab92a7600e719E(i8* %x1, i8* %y2, i64 %len)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::ptr::swap_nonoverlapping_one
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr23swap_nonoverlapping_one17h6a8f55afc8db658aE({ i8*, i64 }* %x, { i8*, i64 }* %y) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
  %_18 = alloca i8, align 1
  store i8 0, i8* %_18, align 1
; call core::mem::size_of
  %1 = call i64 @_ZN4core3mem7size_of17h9464861885e784a5E()
  br label %bb2

bb1:                                              ; preds = %bb10, %bb11
  %2 = bitcast { i8*, i32 }* %0 to i8**
  %3 = load i8*, i8** %2, align 8
  %4 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %5 = load i32, i32* %4, align 8
  %6 = insertvalue { i8*, i32 } undef, i8* %3, 0
  %7 = insertvalue { i8*, i32 } %6, i32 %5, 1
  resume { i8*, i32 } %7

bb2:                                              ; preds = %start
  %8 = icmp ult i64 %1, 32
  br i1 %8, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
; call core::ptr::swap_nonoverlapping
  call void @_ZN4core3ptr19swap_nonoverlapping17h4ab4d0522774d0deE({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 1)
  br label %bb8

bb4:                                              ; preds = %bb2
  store i8 1, i8* %_18, align 1
; call core::ptr::read
  %9 = call { i8*, i64 } @_ZN4core3ptr4read17he3cbe5441ff3e780E({ i8*, i64 }* %x)
  %10 = extractvalue { i8*, i64 } %9, 0
  %11 = extractvalue { i8*, i64 } %9, 1
  br label %bb5

bb5:                                              ; preds = %bb4
; invoke core::intrinsics::copy_nonoverlapping
  invoke void @_ZN4core10intrinsics19copy_nonoverlapping17h6feb3a6ae6660c8aE({ i8*, i64 }* %y, { i8*, i64 }* %x, i64 1)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
  store i8 0, i8* %_18, align 1
; invoke core::ptr::write
  invoke void @_ZN4core3ptr5write17h2d414ec230d062b3E({ i8*, i64 }* %y, i8* noalias readonly align 1 %10, i64 %11)
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
  %12 = load i8, i8* %_18, align 1, !range !0
  %13 = trunc i8 %12 to i1
  br i1 %13, label %bb10, label %bb1

cleanup:                                          ; preds = %bb6, %bb5
  %14 = landingpad { i8*, i32 }
          cleanup
  %15 = extractvalue { i8*, i32 } %14, 0
  %16 = extractvalue { i8*, i32 } %14, 1
  %17 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %15, i8** %17, align 8
  %18 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %16, i32* %18, align 8
  br label %bb11
}

; core::ptr::swap_nonoverlapping_bytes
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ptr25swap_nonoverlapping_bytes17hd2aab92a7600e719E(i8* %x, i8* %y, i64 %len) unnamed_addr #0 {
start:
  %self.i.i4 = alloca %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, align 8
  %self.i5 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, align 8
  %self.i.i = alloca <4 x i64>*, align 8
  %self.i = alloca <4 x i64>*, align 8
  %t1 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>", align 8
  %t = alloca <4 x i64>, align 32
  %i = alloca i64, align 8
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17h32d0bd7cc53fbd27E()
  br label %bb1

bb1:                                              ; preds = %start
  store i64 0, i64* %i, align 8
  br label %bb2

bb2:                                              ; preds = %bb11, %bb1
  %1 = load i64, i64* %i, align 8
  %2 = add i64 %1, %0
  %3 = icmp ule i64 %2, %len
  br i1 %3, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
  %4 = load i64, i64* %i, align 8
  %5 = icmp ult i64 %4, %len
  br i1 %5, label %bb12, label %bb20

bb4:                                              ; preds = %bb2
  %6 = bitcast <4 x i64>* %t to {}*
  br label %bb5

bb5:                                              ; preds = %bb4
  store <4 x i64>* %t, <4 x i64>** %self.i, align 8
  %7 = load <4 x i64>*, <4 x i64>** %self.i, align 8, !nonnull !5
  store <4 x i64>* %7, <4 x i64>** %self.i.i, align 8
  %8 = load <4 x i64>*, <4 x i64>** %self.i.i, align 8, !nonnull !5
  br label %bb6

bb6:                                              ; preds = %bb5
  %t2 = bitcast <4 x i64>* %8 to i8*
  %9 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %10 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17ha16477e11cb653cdE"(i8* %x, i64 %9)
  br label %bb7

bb7:                                              ; preds = %bb6
  %11 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %12 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17ha16477e11cb653cdE"(i8* %y, i64 %11)
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %10, i8* %t2, i64 %0)
  br label %bb9

bb9:                                              ; preds = %bb8
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %12, i8* %10, i64 %0)
  br label %bb10

bb10:                                             ; preds = %bb9
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %t2, i8* %12, i64 %0)
  br label %bb11

bb11:                                             ; preds = %bb10
  %13 = load i64, i64* %i, align 8
  %14 = add i64 %13, %0
  store i64 %14, i64* %i, align 8
  br label %bb2

bb12:                                             ; preds = %bb3
  %15 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1 to {}*
  br label %bb13

bb13:                                             ; preds = %bb12
  %16 = load i64, i64* %i, align 8
  %rem = sub i64 %len, %16
  store %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1, %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i5, align 8
  %17 = load %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i5, align 8, !nonnull !5
  %18 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %17 to %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*
  store %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %18, %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i.i4, align 8
  %19 = load %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i.i4, align 8, !nonnull !5
  %20 = bitcast %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %19 to %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"*
  br label %bb14

bb14:                                             ; preds = %bb13
  %t3 = bitcast %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"* %20 to i8*
  %21 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %22 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17ha16477e11cb653cdE"(i8* %x, i64 %21)
  br label %bb15

bb15:                                             ; preds = %bb14
  %23 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %24 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17ha16477e11cb653cdE"(i8* %y, i64 %23)
  br label %bb16

bb16:                                             ; preds = %bb15
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %22, i8* %t3, i64 %rem)
  br label %bb17

bb17:                                             ; preds = %bb16
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %24, i8* %22, i64 %rem)
  br label %bb18

bb18:                                             ; preds = %bb17
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h08007be001a7047cE(i8* %t3, i8* %24, i64 %rem)
  br label %bb19

bb19:                                             ; preds = %bb18
  br label %bb20

bb20:                                             ; preds = %bb19, %bb3
  ret void
}

; core::ptr::<impl *mut T>::add
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17ha16477e11cb653cdE"(i8* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::<impl *mut T>::offset
  %0 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h3602c8d0a0f37401E"(i8* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; core::ptr::<impl *mut T>::offset
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h3602c8d0a0f37401E"(i8* %self, i64 %count) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  %1 = getelementptr inbounds i8, i8* %self, i64 %count
  store i8* %1, i8** %0, align 8
  %2 = load i8*, i8** %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %2
}

; core::ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h288e83a7f4ddb933E"(i8* %self) unnamed_addr #0 {
start:
; call core::ptr::null_mut
  %0 = call i8* @_ZN4core3ptr8null_mut17h618fd164138ad8ddE()
  br label %bb1

bb1:                                              ; preds = %start
  %1 = icmp eq i8* %self, %0
  ret i1 %1
}

; core::ptr::<impl *const T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h9412bde71985e8f8E"(i8* %self) unnamed_addr #0 {
start:
; call core::ptr::null
  %0 = call i8* @_ZN4core3ptr4null17hade47330ab062e29E()
  br label %bb1

bb1:                                              ; preds = %start
  %1 = icmp eq i8* %self, %0
  ret i1 %1
}

; core::ptr::<impl *const T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf2f323b98c2e5d70E"({ i8*, i64 }* %self) unnamed_addr #0 {
start:
  %0 = bitcast { i8*, i64 }* %self to i8*
; call core::ptr::null
  %1 = call i8* @_ZN4core3ptr4null17hade47330ab062e29E()
  br label %bb1

bb1:                                              ; preds = %start
  %2 = icmp eq i8* %0, %1
  ret i1 %2
}

; core::ptr::null
; Function Attrs: inlinehint uwtable
define i8* @_ZN4core3ptr4null17hade47330ab062e29E() unnamed_addr #0 {
start:
  ret i8* null
}

; core::ptr::read
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3ptr4read17he3cbe5441ff3e780E({ i8*, i64 }* %src) unnamed_addr #0 {
start:
  %self.i.i = alloca { i8*, i64 }*, align 8
  %self.i = alloca { i8*, i64 }*, align 8
  %_0.i = alloca { i8*, i64 }, align 8
  %tmp = alloca { i8*, i64 }, align 8
  %0 = bitcast { i8*, i64 }* %_0.i to {}*
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0.i, i32 0, i32 0
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0.i, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
  %5 = insertvalue { i8*, i64 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i64 } %5, i64 %4, 1
  store { i8*, i64 } %6, { i8*, i64 }* %tmp, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store { i8*, i64 }* %tmp, { i8*, i64 }** %self.i, align 8
  %7 = load { i8*, i64 }*, { i8*, i64 }** %self.i, align 8, !nonnull !5
  store { i8*, i64 }* %7, { i8*, i64 }** %self.i.i, align 8
  %8 = load { i8*, i64 }*, { i8*, i64 }** %self.i.i, align 8, !nonnull !5
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h6feb3a6ae6660c8aE({ i8*, i64 }* %src, { i8*, i64 }* %8, i64 1)
  br label %bb3

bb3:                                              ; preds = %bb2
  %9 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %tmp, i32 0, i32 0
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %tmp, i32 0, i32 1
  %12 = load i64, i64* %11, align 8
  %13 = insertvalue { i8*, i64 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i64 } %13, i64 %12, 1
  %15 = insertvalue { i8*, i64 } undef, i8* %10, 0
  %16 = insertvalue { i8*, i64 } %15, i64 %12, 1
  %17 = extractvalue { i8*, i64 } %16, 0
  %18 = extractvalue { i8*, i64 } %16, 1
  br label %bb4

bb4:                                              ; preds = %bb3
  %19 = insertvalue { i8*, i64 } undef, i8* %17, 0
  %20 = insertvalue { i8*, i64 } %19, i64 %18, 1
  ret { i8*, i64 } %20
}

; core::ptr::write
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr5write17h2d414ec230d062b3E({ i8*, i64 }* %dst, i8* noalias readonly align 1 %src.0, i64 %src.1) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %dst, i32 0, i32 0
  store i8* %src.0, i8** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %dst, i32 0, i32 1
  store i64 %src.1, i64* %1, align 8
  ret void
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h2d8d7f1628c0f790E"({}* %ptr.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %ptr.1) unnamed_addr #0 {
start:
  %_5 = alloca %"core::marker::PhantomData<core::any::Any>", align 1
  %_0 = alloca { i8*, i64* }, align 8
  %0 = bitcast { i8*, i64* }* %_0 to { {}*, [3 x i64]* }*
  %1 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %0, i32 0, i32 0
  store {}* %ptr.0, {}** %1, align 8
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %0, i32 0, i32 1
  store [3 x i64]* %ptr.1, [3 x i64]** %2, align 8
  %3 = bitcast { i8*, i64* }* %_0 to %"core::marker::PhantomData<core::any::Any>"*
  %4 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %_0, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8, !nonnull !5
  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %_0, i32 0, i32 1
  %7 = load i64*, i64** %6, align 8, !nonnull !5
  %8 = insertvalue { i8*, i64* } undef, i8* %5, 0
  %9 = insertvalue { i8*, i64* } %8, i64* %7, 1
  ret { i8*, i64* } %9
}

; core::ptr::unique::Unique<T>::as_mut
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_mut17he6792e0686dccd89E"({ i8*, i64* }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 0
  %1 = load i8*, i8** %0, align 8, !nonnull !5
  %2 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 1
  %3 = load i64*, i64** %2, align 8, !nonnull !5
; call core::ptr::unique::Unique<T>::as_ptr
  %4 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hee29313ff7dc4286E"(i8* nonnull %1, i64* noalias readonly align 8 dereferenceable(24) %3)
  %5 = extractvalue { {}*, [3 x i64]* } %4, 0
  %6 = extractvalue { {}*, [3 x i64]* } %4, 1
  br label %bb1

bb1:                                              ; preds = %start
  %7 = insertvalue { {}*, [3 x i64]* } undef, {}* %5, 0
  %8 = insertvalue { {}*, [3 x i64]* } %7, [3 x i64]* %6, 1
  ret { {}*, [3 x i64]* } %8
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hee29313ff7dc4286E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
  %0 = bitcast i8* %self.0 to {}*
  %1 = bitcast i64* %self.1 to [3 x i64]*
  %2 = insertvalue { {}*, [3 x i64]* } undef, {}* %0, 0
  %3 = insertvalue { {}*, [3 x i64]* } %2, [3 x i64]* %1, 1
  ret { {}*, [3 x i64]* } %3
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h0d6a84545bc2897eE"({}* %ptr.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %ptr.1) unnamed_addr #0 {
start:
  %_0 = alloca { i8*, i64* }, align 8
  %0 = bitcast { i8*, i64* }* %_0 to { {}*, [3 x i64]* }*
  %1 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %0, i32 0, i32 0
  store {}* %ptr.0, {}** %1, align 8
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %0, i32 0, i32 1
  store [3 x i64]* %ptr.1, [3 x i64]** %2, align 8
  %3 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %_0, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8, !nonnull !5
  %5 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %_0, i32 0, i32 1
  %6 = load i64*, i64** %5, align 8, !nonnull !5
  %7 = insertvalue { i8*, i64* } undef, i8* %4, 0
  %8 = insertvalue { i8*, i64* } %7, i64* %6, 1
  ret { i8*, i64* } %8
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hd9f649c0ef7f198cE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
  %0 = bitcast i8* %self.0 to {}*
  %1 = bitcast i64* %self.1 to [3 x i64]*
  %2 = insertvalue { {}*, [3 x i64]* } undef, {}* %0, 0
  %3 = insertvalue { {}*, [3 x i64]* } %2, [3 x i64]* %1, 1
  ret { {}*, [3 x i64]* } %3
}

; core::ptr::null_mut
; Function Attrs: inlinehint uwtable
define i8* @_ZN4core3ptr8null_mut17h618fd164138ad8ddE() unnamed_addr #0 {
start:
  ret i8* null
}

; core::alloc::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h6336d57b50a10cb1E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_0 = alloca { i64, i64 }, align 8
; call core::num::NonZeroUsize::new_unchecked
  %0 = call i64 @_ZN4core3num12NonZeroUsize13new_unchecked17he0e2f32a635eeee2E(i64 %align), !range !6
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %_0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 1
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !6
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6Layout4size17hd21a81e0c7cf525dE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6Layout5align17h6375923a653850f9E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %1 = load i64, i64* %0, align 8, !range !6
; call core::num::NonZeroUsize::get
  %2 = call i64 @_ZN4core3num12NonZeroUsize3get17h97645f35b46ed41aE(i64 %1)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %2
}

; core::option::Option<T>::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h3a9a694defa2be5cE"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call core::mem::take
  %0 = call { i8*, i64 } @_ZN4core3mem4take17h7a417ec2384aa1b7E({ i8*, i64 }* align 8 dereferenceable(16) %self)
  %1 = extractvalue { i8*, i64 } %0, 0
  %2 = extractvalue { i8*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i8*, i64 } undef, i8* %1, 0
  %4 = insertvalue { i8*, i64 } %3, i64 %2, 1
  ret { i8*, i64 } %4
}

; core::option::Option<T>::unwrap
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core6option15Option$LT$T$GT$6unwrap17h03c37b25373178a2E"(i64, i64) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %2 = alloca { i8*, i32 }, align 8
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  %5 = bitcast { i64, i64 }* %self to i64*
  %6 = load i64, i64* %5, align 8, !range !7
  switch i64 %6, label %bb3 [
    i64 0, label %bb2
    i64 1, label %bb4
  ]

bb1:                                              ; preds = %bb5
  %7 = bitcast { i8*, i32 }* %2 to i8**
  %8 = load i8*, i8** %7, align 8
  %9 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 1
  %10 = load i32, i32* %9, align 8
  %11 = insertvalue { i8*, i32 } undef, i8* %8, 0
  %12 = insertvalue { i8*, i32 } %11, i32 %10, 1
  resume { i8*, i32 } %12

bb2:                                              ; preds = %start
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @9 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
          to label %unreachable unwind label %cleanup

bb3:                                              ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  %13 = bitcast { i64, i64 }* %self to %"core::option::Option<usize>::Some"*
  %14 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %13, i32 0, i32 1
  %val = load i64, i64* %14, align 8
  %15 = bitcast { i64, i64 }* %self to i64*
  %16 = load i64, i64* %15, align 8, !range !7
  %17 = icmp eq i64 %16, 1
  br i1 %17, label %bb6, label %bb7

bb5:                                              ; preds = %cleanup
  br label %bb1

bb6:                                              ; preds = %bb7, %bb4
  ret i64 %val

bb7:                                              ; preds = %bb4
  br label %bb6

unreachable:                                      ; preds = %bb2
  unreachable

cleanup:                                          ; preds = %bb2
  %18 = landingpad { i8*, i32 }
          cleanup
  %19 = extractvalue { i8*, i32 } %18, 0
  %20 = extractvalue { i8*, i32 } %18, 1
  %21 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 0
  store i8* %19, i8** %21, align 8
  %22 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %2, i32 0, i32 1
  store i32 %20, i32* %22, align 8
  br label %bb5
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define { i8*, i64* } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h4a7cab8676df8bedE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call { i8*, i64* } @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17hbf4722bb0beb6bdfE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1)
  %1 = extractvalue { i8*, i64* } %0, 0
  %2 = extractvalue { i8*, i64* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = insertvalue { i8*, i64* } undef, i8* %1, 0
  %4 = insertvalue { i8*, i64* } %3, i64* %2, 1
  ret { i8*, i64* } %4
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc15exchange_malloc17hd8c43582226c1db7E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_0 = alloca i8*, align 8
  %0 = icmp eq i64 %size, 0
  br i1 %0, label %bb2, label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::from_size_align_unchecked
  %1 = call { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h6336d57b50a10cb1E(i64 %size, i64 %align)
  %2 = extractvalue { i64, i64 } %1, 0
  %3 = extractvalue { i64, i64 } %1, 1
  br label %bb3

bb2:                                              ; preds = %start
  %4 = inttoptr i64 %align to i8*
  store i8* %4, i8** %_0, align 8
  br label %bb8

bb3:                                              ; preds = %bb1
; call alloc::alloc::alloc
  %5 = call i8* @_ZN5alloc5alloc5alloc17h2dca5e7ffe7bec7fE(i64 %2, i64 %3)
  br label %bb4

bb4:                                              ; preds = %bb3
; call core::ptr::<impl *mut T>::is_null
  %6 = call zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h288e83a7f4ddb933E"(i8* %5)
  br label %bb5

bb5:                                              ; preds = %bb4
  %7 = xor i1 %6, true
  br i1 %7, label %bb7, label %bb6

bb6:                                              ; preds = %bb5
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17hc3523dbfff2fd73fE(i64 %2, i64 %3)
  unreachable

bb7:                                              ; preds = %bb5
  store i8* %5, i8** %_0, align 8
  br label %bb8

bb8:                                              ; preds = %bb7, %bb2
  %8 = load i8*, i8** %_0, align 8
  ret i8* %8
}

; alloc::alloc::alloc
; Function Attrs: inlinehint uwtable
define internal i8* @_ZN5alloc5alloc5alloc17h2dca5e7ffe7bec7fE(i64, i64) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::Layout::size
  %4 = call i64 @_ZN4core5alloc6Layout4size17hd21a81e0c7cf525dE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::align
  %5 = call i64 @_ZN4core5alloc6Layout5align17h6375923a653850f9E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = call i8* @__rust_alloc(i64 %4, i64 %5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %6
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17h7bcd30e5fe1e5609E(i8* %ptr, i64, i64) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::Layout::size
  %4 = call i64 @_ZN4core5alloc6Layout4size17hd21a81e0c7cf525dE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::align
  %5 = call i64 @_ZN4core5alloc6Layout5align17h6375923a653850f9E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %4, i64 %5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::alloc::box_free
; Function Attrs: inlinehint uwtable
define void @_ZN5alloc5alloc8box_free17h0e4c9a4d56147c0bE(i8* nonnull %ptr.0, i64* noalias readonly align 8 dereferenceable(24) %ptr.1) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
; call core::ptr::unique::Unique<T>::as_ptr
  %2 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hee29313ff7dc4286E"(i8* nonnull %ptr.0, i64* noalias readonly align 8 dereferenceable(24) %ptr.1)
  %3 = extractvalue { {}*, [3 x i64]* } %2, 0
  %4 = extractvalue { {}*, [3 x i64]* } %2, 1
  br label %bb1

bb1:                                              ; preds = %start
  %5 = bitcast [3 x i64]* %4 to i64*
  %6 = getelementptr inbounds i64, i64* %5, i64 1
  %7 = load i64, i64* %6, align 8, !invariant.load !5
  %8 = bitcast [3 x i64]* %4 to i64*
  %9 = getelementptr inbounds i64, i64* %8, i64 2
  %10 = load i64, i64* %9, align 8, !invariant.load !5
  store i64 %7, i64* %1, align 8
  %11 = load i64, i64* %1, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
  %12 = bitcast [3 x i64]* %4 to i64*
  %13 = getelementptr inbounds i64, i64* %12, i64 1
  %14 = load i64, i64* %13, align 8, !invariant.load !5
  %15 = bitcast [3 x i64]* %4 to i64*
  %16 = getelementptr inbounds i64, i64* %15, i64 2
  %17 = load i64, i64* %16, align 8, !invariant.load !5
  store i64 %17, i64* %0, align 8
  %18 = load i64, i64* %0, align 8
  br label %bb3

bb3:                                              ; preds = %bb2
  %19 = icmp ne i64 %11, 0
  br i1 %19, label %bb4, label %bb7

bb4:                                              ; preds = %bb3
; call core::alloc::Layout::from_size_align_unchecked
  %20 = call { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h6336d57b50a10cb1E(i64 %11, i64 %18)
  %21 = extractvalue { i64, i64 } %20, 0
  %22 = extractvalue { i64, i64 } %20, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %23 = bitcast {}* %3 to i8*
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17h7bcd30e5fe1e5609E(i8* %23, i64 %21, i64 %22)
  br label %bb6

bb6:                                              ; preds = %bb5
  br label %bb7

bb7:                                              ; preds = %bb6, %bb3
  ret void
}

; alloc::boxed::Box<T>::into_unique
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$11into_unique17h72517645f43691aeE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
  %unique = alloca { i8*, i64* }, align 8
  %0 = bitcast {}* %b.0 to i8*
  %1 = bitcast [3 x i64]* %b.1 to i64*
  %2 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %unique, i32 0, i32 0
  store i8* %0, i8** %2, align 8
  %3 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %unique, i32 0, i32 1
  store i64* %1, i64** %3, align 8
; call core::mem::forget
  call void @_ZN4core3mem6forget17hba650da2291da06cE({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::unique::Unique<T>::as_mut
  %4 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_mut17he6792e0686dccd89E"({ i8*, i64* }* align 8 dereferenceable(16) %unique)
  %5 = extractvalue { {}*, [3 x i64]* } %4, 0
  %6 = extractvalue { {}*, [3 x i64]* } %4, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::unique::Unique<T>::new_unchecked
  %7 = call { i8*, i64* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h2d8d7f1628c0f790E"({}* %5, [3 x i64]* noalias readonly align 8 dereferenceable(24) %6)
  %8 = extractvalue { i8*, i64* } %7, 0
  %9 = extractvalue { i8*, i64* } %7, 1
  br label %bb3

bb3:                                              ; preds = %bb2
  %10 = insertvalue { i8*, i64* } undef, i8* %8, 0
  %11 = insertvalue { i8*, i64* } %10, i64* %9, 1
  ret { i8*, i64* } %11
}

; alloc::boxed::Box<T>::into_raw_non_null
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$17into_raw_non_null17hf1151127514fee55E"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T>::into_unique
  %0 = call { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$11into_unique17h72517645f43691aeE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  %1 = extractvalue { i8*, i64* } %0, 0
  %2 = extractvalue { i8*, i64* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call <T as core::convert::Into<U>>::into
  %3 = call { i8*, i64* } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h4a7cab8676df8bedE"(i8* nonnull %1, i64* noalias readonly align 8 dereferenceable(24) %2)
  %4 = extractvalue { i8*, i64* } %3, 0
  %5 = extractvalue { i8*, i64* } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { i8*, i64* } undef, i8* %4, 0
  %7 = insertvalue { i8*, i64* } %6, i64* %5, 1
  ret { i8*, i64* } %7
}

; alloc::boxed::Box<T>::into_raw
; Function Attrs: inlinehint uwtable
define { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h3a9ab49181b6b1a4E"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T>::into_raw_non_null
  %0 = call { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$17into_raw_non_null17hf1151127514fee55E"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  %1 = extractvalue { i8*, i64* } %0, 0
  %2 = extractvalue { i8*, i64* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::as_ptr
  %3 = call { {}*, [3 x i64]* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hd9f649c0ef7f198cE"(i8* nonnull %1, i64* noalias readonly align 8 dereferenceable(24) %2)
  %4 = extractvalue { {}*, [3 x i64]* } %3, 0
  %5 = extractvalue { {}*, [3 x i64]* } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { {}*, [3 x i64]* } undef, {}* %4, 0
  %7 = insertvalue { {}*, [3 x i64]* } %6, [3 x i64]* %5, 1
  ret { {}*, [3 x i64]* } %7
}

; <core::option::Option<T> as core::default::Default>::default
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN72_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17hb57d367043dc0e9bE"() unnamed_addr #0 {
start:
  %_0 = alloca { i8*, i64 }, align 8
  %0 = bitcast { i8*, i64 }* %_0 to {}**
  store {}* null, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0, i32 0, i32 0
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_0, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
  %5 = insertvalue { i8*, i64 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i64 } %5, i64 %4, 1
  ret { i8*, i64 } %6
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17he9402bbb616b91d0E"({ i8*, i64 }* align 8 dereferenceable(16)) unnamed_addr #1 {
start:
  %_5 = alloca { {}*, [3 x i64]* }, align 8
  %self = alloca { i8*, i64 }*, align 8
  store { i8*, i64 }* %0, { i8*, i64 }** %self, align 8
  %1 = load { i8*, i64 }*, { i8*, i64 }** %self, align 8, !nonnull !5
  %2 = bitcast { i8*, i64 }* %1 to {}**
  %3 = load {}*, {}** %2, align 8
  %4 = icmp ule {}* %3, null
  %5 = select i1 %4, i64 0, i64 1
  switch i64 %5, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb1:                                              ; preds = %start
  %6 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 0
  store {}* inttoptr (i64 1 to {}*), {}** %6, align 8
  %7 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.5 to [3 x i64]*), [3 x i64]** %7, align 8
  br label %bb4

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %8 = load { i8*, i64 }*, { i8*, i64 }** %self, align 8, !nonnull !5
  %a = bitcast { i8*, i64 }* %8 to { [0 x i8]*, i64 }*
  %9 = bitcast { [0 x i8]*, i64 }* %a to {}*
  %10 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 0
  store {}* %9, {}** %10, align 8
  %11 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.4 to [3 x i64]*), [3 x i64]** %11, align 8
  br label %bb4

bb4:                                              ; preds = %bb1, %bb3
  %12 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 0
  %13 = load {}*, {}** %12, align 8, !nonnull !5
  %14 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 1
  %15 = load [3 x i64]*, [3 x i64]** %14, align 8, !nonnull !5
  %16 = insertvalue { {}*, [3 x i64]* } undef, {}* %13, 0
  %17 = insertvalue { {}*, [3 x i64]* } %16, [3 x i64]* %15, 1
  ret { {}*, [3 x i64]* } %17
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::box_me_up
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17h976b72f68f3a4ba9E"({ i8*, i64 }* align 8 dereferenceable(16)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_17 = alloca i8, align 1
  %_16 = alloca i8, align 1
  %_13 = alloca { {}*, [3 x i64]* }, align 8
  %_12 = alloca {}, align 1
  %_4 = alloca { i8*, i64 }, align 8
  %data = alloca { {}*, [3 x i64]* }, align 8
  %self = alloca { i8*, i64 }*, align 8
  store { i8*, i64 }* %0, { i8*, i64 }** %self, align 8
  store i8 0, i8* %_16, align 1
  store i8 0, i8* %_17, align 1
  %2 = load { i8*, i64 }*, { i8*, i64 }** %self, align 8, !nonnull !5
  store i8 1, i8* %_16, align 1
; call core::option::Option<T>::take
  %3 = call { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h3a9a694defa2be5cE"({ i8*, i64 }* align 8 dereferenceable(16) %2)
  store { i8*, i64 } %3, { i8*, i64 }* %_4, align 8
  br label %bb2

bb1:                                              ; preds = %bb19, %bb9, %bb12, %bb11, %bb13
  %4 = bitcast { i8*, i32 }* %1 to i8**
  %5 = load i8*, i8** %4, align 8
  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %7 = load i32, i32* %6, align 8
  %8 = insertvalue { i8*, i32 } undef, i8* %5, 0
  %9 = insertvalue { i8*, i32 } %8, i32 %7, 1
  resume { i8*, i32 } %9

bb2:                                              ; preds = %start
  %10 = bitcast { i8*, i64 }* %_4 to {}**
  %11 = load {}*, {}** %10, align 8
  %12 = icmp ule {}* %11, null
  %13 = select i1 %12, i64 0, i64 1
  switch i64 %13, label %bb4 [
    i64 0, label %bb3
    i64 1, label %bb5
  ]

bb3:                                              ; preds = %bb2
; invoke alloc::alloc::exchange_malloc
  %14 = invoke i8* @_ZN5alloc5alloc15exchange_malloc17hd8c43582226c1db7E(i64 0, i64 1)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h869c8c71c6323502E.exit" unwind label %cleanup

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h869c8c71c6323502E.exit": ; preds = %bb3
  %15 = bitcast i8* %14 to {}*
  br label %bb7

bb4:                                              ; preds = %bb2
  unreachable

bb5:                                              ; preds = %bb2
  store i8 0, i8* %_16, align 1
  %16 = bitcast { i8*, i64 }* %_4 to { [0 x i8]*, i64 }*
  %17 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %16, i32 0, i32 0
  %a.0 = load [0 x i8]*, [0 x i8]** %17, align 8, !nonnull !5
  %18 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %16, i32 0, i32 1
  %a.1 = load i64, i64* %18, align 8
; invoke alloc::alloc::exchange_malloc
  %19 = invoke i8* @_ZN5alloc5alloc15exchange_malloc17hd8c43582226c1db7E(i64 16, i64 8)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17hd74e55d4759d11ccE.exit" unwind label %cleanup

"_ZN5alloc5boxed12Box$LT$T$GT$3new17hd74e55d4759d11ccE.exit": ; preds = %bb5
  %20 = bitcast i8* %19 to { [0 x i8]*, i64 }*
  %21 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %20, i32 0, i32 0
  store [0 x i8]* %a.0, [0 x i8]** %21, align 8, !noalias !8
  %22 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %20, i32 0, i32 1
  store i64 %a.1, i64* %22, align 8
  br label %bb6

bb6:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17hd74e55d4759d11ccE.exit"
  %23 = bitcast { [0 x i8]*, i64 }* %20 to {}*
  store i8 1, i8* %_17, align 1
  %24 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  store {}* %23, {}** %24, align 8
  %25 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.4 to [3 x i64]*), [3 x i64]** %25, align 8
  br label %bb8

bb7:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h869c8c71c6323502E.exit"
  store i8 1, i8* %_17, align 1
  %26 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  store {}* %15, {}** %26, align 8
  %27 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.5 to [3 x i64]*), [3 x i64]** %27, align 8
  br label %bb8

bb8:                                              ; preds = %bb7, %bb6
  %28 = bitcast { i8*, i64 }* %_4 to {}**
  %29 = load {}*, {}** %28, align 8
  %30 = icmp ule {}* %29, null
  %31 = select i1 %30, i64 0, i64 1
  %32 = icmp eq i64 %31, 1
  br i1 %32, label %bb16, label %bb18

bb9:                                              ; preds = %cleanup1
  %33 = load i8, i8* %_17, align 1, !range !0
  %34 = trunc i8 %33 to i1
  br i1 %34, label %bb19, label %bb1

bb10:                                             ; preds = %bb15
  store { {}*, [3 x i64]* } %52, { {}*, [3 x i64]* }* %_13, align 8
  %35 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_13, i32 0, i32 0
  %36 = load {}*, {}** %35, align 8
  %37 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_13, i32 0, i32 1
  %38 = load [3 x i64]*, [3 x i64]** %37, align 8, !nonnull !5
  store i8 0, i8* %_17, align 1
  %39 = insertvalue { {}*, [3 x i64]* } undef, {}* %36, 0
  %40 = insertvalue { {}*, [3 x i64]* } %39, [3 x i64]* %38, 1
  ret { {}*, [3 x i64]* } %40

bb11:                                             ; preds = %bb14
  %41 = load i8, i8* %_16, align 1, !range !0
  %42 = trunc i8 %41 to i1
  br i1 %42, label %bb12, label %bb1

bb12:                                             ; preds = %bb11
  store i8 0, i8* %_16, align 1
  br label %bb1

bb13:                                             ; preds = %bb14
  br label %bb1

bb14:                                             ; preds = %cleanup
  %43 = bitcast { i8*, i64 }* %_4 to {}**
  %44 = load {}*, {}** %43, align 8
  %45 = icmp ule {}* %44, null
  %46 = select i1 %45, i64 0, i64 1
  %47 = icmp eq i64 %46, 1
  br i1 %47, label %bb11, label %bb13

bb15:                                             ; preds = %bb17, %bb16, %bb18
  store i8 0, i8* %_16, align 1
  store i8 0, i8* %_17, align 1
  %48 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  %49 = load {}*, {}** %48, align 8, !nonnull !5
  %50 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  %51 = load [3 x i64]*, [3 x i64]** %50, align 8, !nonnull !5
; invoke alloc::boxed::Box<T>::into_raw
  %52 = invoke { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h3a9ab49181b6b1a4E"({}* noalias nonnull align 1 %49, [3 x i64]* noalias readonly align 8 dereferenceable(24) %51)
          to label %bb10 unwind label %cleanup1

bb16:                                             ; preds = %bb8
  %53 = load i8, i8* %_16, align 1, !range !0
  %54 = trunc i8 %53 to i1
  br i1 %54, label %bb17, label %bb15

bb17:                                             ; preds = %bb16
  store i8 0, i8* %_16, align 1
  br label %bb15

bb18:                                             ; preds = %bb8
  br label %bb15

bb19:                                             ; preds = %bb9
  store i8 0, i8* %_17, align 1
; call core::ptr::real_drop_in_place
  call void @_ZN4core3ptr18real_drop_in_place17hfbf77e21fbb099e6E({ {}*, [3 x i64]* }* align 8 dereferenceable(16) %data) #10
  br label %bb1

cleanup:                                          ; preds = %bb5, %bb3
  %55 = landingpad { i8*, i32 }
          cleanup
  %56 = extractvalue { i8*, i32 } %55, 0
  %57 = extractvalue { i8*, i32 } %55, 1
  %58 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %56, i8** %58, align 8
  %59 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %57, i32* %59, align 8
  br label %bb14

cleanup1:                                         ; preds = %bb15
  %60 = landingpad { i8*, i32 }
          cleanup
  %61 = extractvalue { i8*, i32 } %60, 0
  %62 = extractvalue { i8*, i32 } %60, 1
  %63 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %61, i8** %63, align 8
  %64 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %62, i32* %64, align 8
  br label %bb9
}

; panic::may_panic
; Function Attrs: uwtable
define i32 @_ZN5panic9may_panic17ha5821793937ed625E(i32 %a) unnamed_addr #1 {
start:
  %0 = icmp sgt i32 %a, 2
  br i1 %0, label %bb2, label %bb1

bb1:                                              ; preds = %start
  ret i32 1

bb2:                                              ; preds = %start
; call std::panicking::begin_panic
  call void @_ZN3std9panicking11begin_panic17h8273b1e5d825bcfaE([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [5 x i8] }>* @12 to [0 x i8]*), i64 5, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @11 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #4

; std::panicking::rust_panic_with_hook
; Function Attrs: noreturn uwtable
declare void @_ZN3std9panicking20rust_panic_with_hook17h0c4b67125f55410aE({}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), i64* noalias readonly align 8 dereferenceable_or_null(48), { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24)) unnamed_addr #5

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hb4bc64e7f35c9151E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40)) unnamed_addr #2

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1 immarg) #6

; Function Attrs: nounwind readnone
declare i1 @llvm.expect.i1(i1, i1) #7

; Function Attrs: nounwind readnone speculatable
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #8

; alloc::alloc::handle_alloc_error
; Function Attrs: noreturn nounwind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17hc3523dbfff2fd73fE(i64, i64) unnamed_addr #9

; Function Attrs: nounwind uwtable
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #3

; Function Attrs: nounwind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #3

attributes #0 = { inlinehint uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { cold noinline noreturn uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #3 = { nounwind uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #4 = { cold noreturn nounwind }
attributes #5 = { noreturn uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #6 = { argmemonly nounwind }
attributes #7 = { nounwind readnone }
attributes #8 = { nounwind readnone speculatable }
attributes #9 = { noreturn nounwind uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #10 = { noinline }

!0 = !{i8 0, i8 2}
!1 = !{!2, !4}
!2 = distinct !{!2, !3, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hada88bff36d5848bE: %value.0"}
!3 = distinct !{!3, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hada88bff36d5848bE"}
!4 = distinct !{!4, !3, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hada88bff36d5848bE: %value.1"}
!5 = !{}
!6 = !{i64 1, i64 0}
!7 = !{i64 0, i64 2}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17hd74e55d4759d11ccE: %x.0"}
!10 = distinct !{!10, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17hd74e55d4759d11ccE"}
