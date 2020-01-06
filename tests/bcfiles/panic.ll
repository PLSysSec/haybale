; ModuleID = 'panic.3a1fbbbh-cgu.0'
source_filename = "panic.3a1fbbbh-cgu.0"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [0 x i64], %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock", [0 x i64] }
%"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock" = type { [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64] }
%"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [4 x i64] }
%"core::marker::PhantomData<core::any::Any>" = type {}
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* } { void ({ i8*, i64 }*)* @_ZN4core3ptr18real_drop_in_place17hbb20fd2076a802dbE, i64 16, i64 8, { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17h5a9a0f40839bb94cE", { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hdc84d3ba2e8bffa6E" }, align 8
@vtable.1 = private unnamed_addr constant { void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* } { void ({ [0 x i8]*, i64 }*)* @_ZN4core3ptr18real_drop_in_place17h84582bf9c863cd5bE, i64 16, i64 8, i64 ({ [0 x i8]*, i64 }*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hbb70a3e5c97dd526E" }, align 8
@0 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 1
@vtable.2 = private unnamed_addr constant { void ({}*)*, i64, i64, i64 ({}*)* } { void ({}*)* @_ZN4core3ptr18real_drop_in_place17h9a0d339ecb001978E, i64 0, i64 1, i64 ({}*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h1ffc2044a97e8bbeE" }, align 8
@1 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"panic.rs" }>, align 1
@2 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\08\00\00\00\00\00\00\00\03\00\00\00\09\00\00\00" }>, align 8
@3 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"a > 2" }>, align 1

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h9cfaf8c419904282E"(i8* nonnull %unique.0, i64* noalias readonly align 8 dereferenceable(24) %unique.1) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %0 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hfc456c05a7f514c8E"(i8* nonnull %unique.0, i64* noalias readonly align 8 dereferenceable(24) %unique.1)
  %1 = extractvalue { {}*, [3 x i64]* } %0, 0
  %2 = extractvalue { {}*, [3 x i64]* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %3 = call { i8*, i64* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h04c451c98b8d7358E"({}* %1, [3 x i64]* noalias readonly align 8 dereferenceable(24) %2)
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
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h1ffc2044a97e8bbeE"({}* noalias nonnull readonly align 1 %self) unnamed_addr #1 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17hb3d77a5626f81f27E()
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; <T as core::any::Any>::type_id
; Function Attrs: uwtable
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hbb70a3e5c97dd526E"({ [0 x i8]*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17h50fd7088c687d9d3E()
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn uwtable
define void @_ZN3std9panicking11begin_panic17h39256d503986386eE([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) %file_line_col) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_11 = alloca i8, align 1
  %_9 = alloca i64*, align 8
  %_7 = alloca { i8*, i64 }, align 8
  store i8 0, i8* %_11, align 1
  store i8 1, i8* %_11, align 1
  br i1 false, label %bb3, label %bb2

bb1:                                              ; preds = %bb6, %bb7
  %0 = bitcast { i8*, i32 }* %personalityslot to i8**
  %1 = load i8*, i8** %0, align 8
  %2 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %3 = load i32, i32* %2, align 8
  %4 = insertvalue { i8*, i32 } undef, i8* %1, 0
  %5 = insertvalue { i8*, i32 } %4, i32 %3, 1
  resume { i8*, i32 } %5

bb2:                                              ; preds = %start
  store i8 0, i8* %_11, align 1
; invoke std::panicking::begin_panic::PanicPayload<A>::new
  %6 = invoke { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h7a81fb81b023e487E"([0 x i8]* noalias nonnull readonly align 1 %msg.0, i64 %msg.1)
          to label %bb5 unwind label %cleanup

bb3:                                              ; preds = %start
  call void @llvm.trap()
  unreachable

bb4:                                              ; preds = %cleanup1
  br label %bb7

bb5:                                              ; preds = %bb2
  store { i8*, i64 } %6, { i8*, i64 }* %_7, align 8
  %7 = bitcast { i8*, i64 }* %_7 to {}*
  %8 = bitcast i64** %_9 to {}**
  store {}* null, {}** %8, align 8
  %9 = load i64*, i64** %_9, align 8
; invoke std::panicking::rust_panic_with_hook
  invoke void @_ZN3std9panicking20rust_panic_with_hook17h7d6a669f1a899680E({}* nonnull align 1 %7, [3 x i64]* noalias readonly align 8 dereferenceable(24) bitcast ({ void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* }* @vtable.0 to [3 x i64]*), i64* noalias readonly align 8 dereferenceable_or_null(48) %9, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) %file_line_col)
          to label %unreachable unwind label %cleanup1

bb6:                                              ; preds = %bb7
  store i8 0, i8* %_11, align 1
  br label %bb1

bb7:                                              ; preds = %bb4, %cleanup
  %10 = load i8, i8* %_11, align 1, !range !0
  %11 = trunc i8 %10 to i1
  br i1 %11, label %bb6, label %bb1

cleanup:                                          ; preds = %bb2
  %12 = landingpad { i8*, i32 }
          cleanup
  %13 = extractvalue { i8*, i32 } %12, 0
  %14 = extractvalue { i8*, i32 } %12, 1
  %15 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %13, i8** %15, align 8
  %16 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %14, i32* %16, align 8
  br label %bb7

unreachable:                                      ; preds = %bb5
  unreachable

cleanup1:                                         ; preds = %bb5
  %17 = landingpad { i8*, i32 }
          cleanup
  %18 = extractvalue { i8*, i32 } %17, 0
  %19 = extractvalue { i8*, i32 } %17, 1
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %18, i8** %20, align 8
  %21 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %19, i32* %21, align 8
  br label %bb4
}

; std::panicking::begin_panic::PanicPayload<A>::new
; Function Attrs: uwtable
define { i8*, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17h7a81fb81b023e487E"([0 x i8]* noalias nonnull readonly align 1 %inner.0, i64 %inner.1) unnamed_addr #1 {
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
define void @_ZN4core10intrinsics19copy_nonoverlapping17h16536a193b0964f5E({ i8*, i64 }* %src, { i8*, i64 }* %dst, i64 %count) unnamed_addr #0 {
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
define void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %src, i8* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 1, %count
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %dst, i8* align 1 %src, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17h50fd7088c687d9d3E() unnamed_addr #1 {
start:
  %tmp_ret = alloca i64, align 8
  %_0 = alloca i64, align 8
  store i64 1229646359891580772, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %0, i64* %_0, align 8
  %1 = load i64, i64* %_0, align 8
  ret i64 %1
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17hb3d77a5626f81f27E() unnamed_addr #1 {
start:
  %tmp_ret = alloca i64, align 8
  %_0 = alloca i64, align 8
  store i64 7549865886324542212, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %0, i64* %_0, align 8
  %1 = load i64, i64* %_0, align 8
  ret i64 %1
}

; core::mem::swap
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem4swap17hf75d3ef97f4589f4E({ i8*, i64 }* align 8 dereferenceable(16) %x, { i8*, i64 }* align 8 dereferenceable(16) %y) unnamed_addr #0 {
start:
; call core::ptr::swap_nonoverlapping_one
  call void @_ZN4core3ptr23swap_nonoverlapping_one17hd992ae766cf402f8E({ i8*, i64 }* %x, { i8*, i64 }* %y)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::forget
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem6forget17h7d89cbcefdb4336fE({}* noalias nonnull align 1 %t.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %t.1) unnamed_addr #0 {
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
define { i8*, i64 } @_ZN4core3mem7replace17hb6aec7f1adcc5d7fE({ i8*, i64 }* align 8 dereferenceable(16) %dest, i8* noalias readonly align 1, i64) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %src = alloca { i8*, i64 }, align 8
  %2 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 0
  store i8* %0, i8** %2, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; invoke core::mem::swap
  invoke void @_ZN4core3mem4swap17hf75d3ef97f4589f4E({ i8*, i64 }* align 8 dereferenceable(16) %dest, { i8*, i64 }* align 8 dereferenceable(16) %src)
          to label %bb2 unwind label %cleanup

bb1:                                              ; preds = %bb3
  %4 = bitcast { i8*, i32 }* %personalityslot to i8**
  %5 = load i8*, i8** %4, align 8
  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %7 = load i32, i32* %6, align 8
  %8 = insertvalue { i8*, i32 } undef, i8* %5, 0
  %9 = insertvalue { i8*, i32 } %8, i32 %7, 1
  resume { i8*, i32 } %9

bb2:                                              ; preds = %start
  %10 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 0
  %11 = load i8*, i8** %10, align 8
  %12 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %src, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { i8*, i64 } undef, i8* %11, 0
  %15 = insertvalue { i8*, i64 } %14, i64 %13, 1
  ret { i8*, i64 } %15

bb3:                                              ; preds = %cleanup
  br label %bb1

cleanup:                                          ; preds = %start
  %16 = landingpad { i8*, i32 }
          cleanup
  %17 = extractvalue { i8*, i32 } %16, 0
  %18 = extractvalue { i8*, i32 } %16, 1
  %19 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %17, i8** %19, align 8
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %18, i32* %20, align 8
  br label %bb3
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17h514e4c67b817495cE() unnamed_addr #0 {
start:
  %tmp_ret = alloca i64, align 8
  store i64 32, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17hc3b2a53f16495aa1E() unnamed_addr #0 {
start:
  %tmp_ret = alloca i64, align 8
  store i64 16, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::num::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize13new_unchecked17heca65d9b55465c49E(i64 %n) unnamed_addr #0 {
start:
  %_0 = alloca i64, align 8
  store i64 %n, i64* %_0, align 8
  %0 = load i64, i64* %_0, align 8, !range !6
  ret i64 %0
}

; core::num::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize3get17h6307bbc49f9bcb2fE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h63de9f8b4fce6658E({}* nonnull align 1 %arg0.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %arg0.1) unnamed_addr #1 {
start:
  %0 = bitcast [3 x i64]* %arg0.1 to void ({}*)**
  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 0
  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !5, !nonnull !5
  call void %2({}* align 1 %arg0.0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h84582bf9c863cd5bE({ [0 x i8]*, i64 }* align 8 dereferenceable(16) %arg0) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17h9a0d339ecb001978E({}* nonnull align 1 %arg0) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17hbb20fd2076a802dbE({ i8*, i64 }* align 8 dereferenceable(16) %arg0) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::real_drop_in_place
; Function Attrs: uwtable
define internal void @_ZN4core3ptr18real_drop_in_place17he062aa2ca9621ca6E({ {}*, [3 x i64]* }* align 8 dereferenceable(16)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %arg0 = alloca { {}*, [3 x i64]* }*, align 8
  store { {}*, [3 x i64]* }* %0, { {}*, [3 x i64]* }** %arg0, align 8
  %1 = load { {}*, [3 x i64]* }*, { {}*, [3 x i64]* }** %arg0, align 8, !nonnull !5
  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %1, i32 0, i32 0
  %3 = load {}*, {}** %2, align 8, !nonnull !5
  %4 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %1, i32 0, i32 1
  %5 = load [3 x i64]*, [3 x i64]** %4, align 8, !nonnull !5
  %6 = bitcast [3 x i64]* %5 to void ({}*)**
  %7 = getelementptr inbounds void ({}*)*, void ({}*)** %6, i64 0
  %8 = load void ({}*)*, void ({}*)** %7, align 8, !invariant.load !5, !nonnull !5
  invoke void %8({}* align 1 %3)
          to label %bb3 unwind label %cleanup

bb1:                                              ; preds = %bb3
  ret void

bb2:                                              ; preds = %bb4
  %9 = bitcast { i8*, i32 }* %personalityslot to i8**
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %12 = load i32, i32* %11, align 8
  %13 = insertvalue { i8*, i32 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i32 } %13, i32 %12, 1
  resume { i8*, i32 } %14

bb3:                                              ; preds = %start
  %15 = load { {}*, [3 x i64]* }*, { {}*, [3 x i64]* }** %arg0, align 8, !nonnull !5
  %16 = bitcast { {}*, [3 x i64]* }* %15 to { i8*, i64* }*
  %17 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %16, i32 0, i32 0
  %18 = load i8*, i8** %17, align 8, !nonnull !5
  %19 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %16, i32 0, i32 1
  %20 = load i64*, i64** %19, align 8, !nonnull !5
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h2f074b68af6a8c4cE(i8* nonnull %18, i64* noalias readonly align 8 dereferenceable(24) %20)
  br label %bb1

bb4:                                              ; preds = %cleanup
  %21 = load { {}*, [3 x i64]* }*, { {}*, [3 x i64]* }** %arg0, align 8, !nonnull !5
  %22 = bitcast { {}*, [3 x i64]* }* %21 to { i8*, i64* }*
  %23 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %22, i32 0, i32 0
  %24 = load i8*, i8** %23, align 8, !nonnull !5
  %25 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %22, i32 0, i32 1
  %26 = load i64*, i64** %25, align 8, !nonnull !5
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h2f074b68af6a8c4cE(i8* nonnull %24, i64* noalias readonly align 8 dereferenceable(24) %26) #8
  br label %bb2

cleanup:                                          ; preds = %start
  %27 = landingpad { i8*, i32 }
          cleanup
  %28 = extractvalue { i8*, i32 } %27, 0
  %29 = extractvalue { i8*, i32 } %27, 1
  %30 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %28, i8** %30, align 8
  %31 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %29, i32* %31, align 8
  br label %bb4
}

; core::ptr::swap_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr19swap_nonoverlapping17h6a167cbb495015fdE({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 %count) unnamed_addr #0 {
start:
  %0 = bitcast { i8*, i64 }* %x to i8*
  %1 = bitcast { i8*, i64 }* %y to i8*
; call core::mem::size_of
  %2 = call i64 @_ZN4core3mem7size_of17hc3b2a53f16495aa1E()
  br label %bb1

bb1:                                              ; preds = %start
  %3 = mul i64 %2, %count
; call core::ptr::swap_nonoverlapping_bytes
  call void @_ZN4core3ptr25swap_nonoverlapping_bytes17ha54edffc065ef658E(i8* %0, i8* %1, i64 %3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::ptr::swap_nonoverlapping_one
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr23swap_nonoverlapping_one17hd992ae766cf402f8E({ i8*, i64 }* %x, { i8*, i64 }* %y) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_18 = alloca i8, align 1
  store i8 0, i8* %_18, align 1
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17hc3b2a53f16495aa1E()
  br label %bb2

bb1:                                              ; preds = %bb10, %bb11
  %1 = bitcast { i8*, i32 }* %personalityslot to i8**
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %4 = load i32, i32* %3, align 8
  %5 = insertvalue { i8*, i32 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i32 } %5, i32 %4, 1
  resume { i8*, i32 } %6

bb2:                                              ; preds = %start
  %7 = icmp ult i64 %0, 32
  br i1 %7, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
; call core::ptr::swap_nonoverlapping
  call void @_ZN4core3ptr19swap_nonoverlapping17h6a167cbb495015fdE({ i8*, i64 }* %x, { i8*, i64 }* %y, i64 1)
  br label %bb8

bb4:                                              ; preds = %bb2
  store i8 1, i8* %_18, align 1
; call core::ptr::read
  %8 = call { i8*, i64 } @_ZN4core3ptr4read17h1bd84008752b2502E({ i8*, i64 }* %x)
  %9 = extractvalue { i8*, i64 } %8, 0
  %10 = extractvalue { i8*, i64 } %8, 1
  br label %bb5

bb5:                                              ; preds = %bb4
; invoke core::intrinsics::copy_nonoverlapping
  invoke void @_ZN4core10intrinsics19copy_nonoverlapping17h16536a193b0964f5E({ i8*, i64 }* %y, { i8*, i64 }* %x, i64 1)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
  store i8 0, i8* %_18, align 1
; invoke core::ptr::write
  invoke void @_ZN4core3ptr5write17h59739f37ea207121E({ i8*, i64 }* %y, i8* noalias readonly align 1 %9, i64 %10)
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
  %11 = load i8, i8* %_18, align 1, !range !0
  %12 = trunc i8 %11 to i1
  br i1 %12, label %bb10, label %bb1

cleanup:                                          ; preds = %bb6, %bb5
  %13 = landingpad { i8*, i32 }
          cleanup
  %14 = extractvalue { i8*, i32 } %13, 0
  %15 = extractvalue { i8*, i32 } %13, 1
  %16 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %14, i8** %16, align 8
  %17 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %15, i32* %17, align 8
  br label %bb11
}

; core::ptr::swap_nonoverlapping_bytes
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ptr25swap_nonoverlapping_bytes17ha54edffc065ef658E(i8* %x, i8* %y, i64 %len) unnamed_addr #0 {
start:
  %self.i.i2 = alloca %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, align 8
  %self.i3 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, align 8
  %self.i.i = alloca <4 x i64>*, align 8
  %self.i = alloca <4 x i64>*, align 8
  %t1 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>", align 8
  %t = alloca <4 x i64>, align 32
  %i = alloca i64, align 8
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17h514e4c67b817495cE()
  br label %bb1

bb1:                                              ; preds = %start
  store i64 0, i64* %i, align 8
  br label %bb2

bb2:                                              ; preds = %bb11, %bb1
  %1 = load i64, i64* %i, align 8
  %2 = add i64 %1, %0
  %3 = icmp ule i64 %2, %len
  br i1 %3, label %bb3, label %bb4

bb3:                                              ; preds = %bb2
  %4 = bitcast <4 x i64>* %t to {}*
  br label %bb5

bb4:                                              ; preds = %bb2
  %5 = load i64, i64* %i, align 8
  %6 = icmp ult i64 %5, %len
  br i1 %6, label %bb12, label %bb20

bb5:                                              ; preds = %bb3
  store <4 x i64>* %t, <4 x i64>** %self.i, align 8
  %7 = load <4 x i64>*, <4 x i64>** %self.i, align 8, !nonnull !5
  store <4 x i64>* %7, <4 x i64>** %self.i.i, align 8
  %8 = load <4 x i64>*, <4 x i64>** %self.i.i, align 8, !nonnull !5
  br label %bb6

bb6:                                              ; preds = %bb5
  %9 = bitcast <4 x i64>* %8 to i8*
  %10 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %11 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h962a877cd3c22d71E"(i8* %x, i64 %10)
  br label %bb7

bb7:                                              ; preds = %bb6
  %12 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %13 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h962a877cd3c22d71E"(i8* %y, i64 %12)
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %11, i8* %9, i64 %0)
  br label %bb9

bb9:                                              ; preds = %bb8
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %13, i8* %11, i64 %0)
  br label %bb10

bb10:                                             ; preds = %bb9
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %9, i8* %13, i64 %0)
  br label %bb11

bb11:                                             ; preds = %bb10
  %14 = load i64, i64* %i, align 8
  %15 = add i64 %14, %0
  store i64 %15, i64* %i, align 8
  br label %bb2

bb12:                                             ; preds = %bb4
  %16 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1 to {}*
  br label %bb13

bb13:                                             ; preds = %bb12
  %17 = load i64, i64* %i, align 8
  %18 = sub i64 %len, %17
  store %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1, %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i3, align 8
  %19 = load %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i3, align 8, !nonnull !5
  %20 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %19 to %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*
  store %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %20, %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i.i2, align 8
  %21 = load %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i.i2, align 8, !nonnull !5
  %22 = bitcast %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %21 to %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"*
  br label %bb14

bb14:                                             ; preds = %bb13
  %23 = bitcast %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"* %22 to i8*
  %24 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %25 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h962a877cd3c22d71E"(i8* %x, i64 %24)
  br label %bb15

bb15:                                             ; preds = %bb14
  %26 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %27 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h962a877cd3c22d71E"(i8* %y, i64 %26)
  br label %bb16

bb16:                                             ; preds = %bb15
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %25, i8* %23, i64 %18)
  br label %bb17

bb17:                                             ; preds = %bb16
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %27, i8* %25, i64 %18)
  br label %bb18

bb18:                                             ; preds = %bb17
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17hb005ef098c06ecf0E(i8* %23, i8* %27, i64 %18)
  br label %bb19

bb19:                                             ; preds = %bb18
  br label %bb20

bb20:                                             ; preds = %bb19, %bb4
  ret void
}

; core::ptr::<impl *mut T>::add
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h962a877cd3c22d71E"(i8* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::<impl *mut T>::offset
  %0 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h1fa39a11828af721E"(i8* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; core::ptr::<impl *mut T>::offset
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h1fa39a11828af721E"(i8* %self, i64 %count) unnamed_addr #0 {
start:
  %tmp_ret = alloca i8*, align 8
  %0 = getelementptr inbounds i8, i8* %self, i64 %count
  store i8* %0, i8** %tmp_ret, align 8
  %1 = load i8*, i8** %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; core::ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hc5eef318cb1f18adE"(i8* %self) unnamed_addr #0 {
start:
; call core::ptr::null_mut
  %0 = call i8* @_ZN4core3ptr8null_mut17hb6110281383d514cE()
  br label %bb1

bb1:                                              ; preds = %start
  %1 = icmp eq i8* %self, %0
  ret i1 %1
}

; core::ptr::read
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @_ZN4core3ptr4read17h1bd84008752b2502E({ i8*, i64 }* %src) unnamed_addr #0 {
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
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h16536a193b0964f5E({ i8*, i64 }* %src, { i8*, i64 }* %8, i64 1)
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
define void @_ZN4core3ptr5write17h59739f37ea207121E({ i8*, i64 }* %dst, i8* noalias readonly align 1 %src.0, i64 %src.1) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %dst, i32 0, i32 0
  store i8* %src.0, i8** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %dst, i32 0, i32 1
  store i64 %src.1, i64* %1, align 8
  ret void
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h1a464bf292351aa8E"({}* %ptr.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %ptr.1) unnamed_addr #0 {
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
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_mut17h062c6a843588adafE"({ i8*, i64* }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 0
  %1 = load i8*, i8** %0, align 8, !nonnull !5
  %2 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %self, i32 0, i32 1
  %3 = load i64*, i64** %2, align 8, !nonnull !5
; call core::ptr::unique::Unique<T>::as_ptr
  %4 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hfc456c05a7f514c8E"(i8* nonnull %1, i64* noalias readonly align 8 dereferenceable(24) %3)
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
define { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hfc456c05a7f514c8E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
  %0 = bitcast i8* %self.0 to {}*
  %1 = bitcast i64* %self.1 to [3 x i64]*
  %2 = insertvalue { {}*, [3 x i64]* } undef, {}* %0, 0
  %3 = insertvalue { {}*, [3 x i64]* } %2, [3 x i64]* %1, 1
  ret { {}*, [3 x i64]* } %3
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h04c451c98b8d7358E"({}* %ptr.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %ptr.1) unnamed_addr #0 {
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
define { {}*, [3 x i64]* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h3066c126da80d8efE"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #0 {
start:
  %0 = bitcast i8* %self.0 to {}*
  %1 = bitcast i64* %self.1 to [3 x i64]*
  %2 = insertvalue { {}*, [3 x i64]* } undef, {}* %0, 0
  %3 = insertvalue { {}*, [3 x i64]* } %2, [3 x i64]* %1, 1
  ret { {}*, [3 x i64]* } %3
}

; core::ptr::null_mut
; Function Attrs: inlinehint uwtable
define i8* @_ZN4core3ptr8null_mut17hb6110281383d514cE() unnamed_addr #0 {
start:
  ret i8* null
}

; core::alloc::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h7df9b8de862ced3eE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_0 = alloca { i64, i64 }, align 8
; call core::num::NonZeroUsize::new_unchecked
  %0 = call i64 @_ZN4core3num12NonZeroUsize13new_unchecked17heca65d9b55465c49E(i64 %align), !range !6
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
define internal i64 @_ZN4core5alloc6Layout4size17h913e505ce214fca8E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6Layout5align17ha0bdf2f1dc33a41cE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %1 = load i64, i64* %0, align 8, !range !6
; call core::num::NonZeroUsize::get
  %2 = call i64 @_ZN4core3num12NonZeroUsize3get17h6307bbc49f9bcb2fE(i64 %1)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %2
}

; core::option::Option<T>::take
; Function Attrs: inlinehint uwtable
define { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h6720285cec1b77d6E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_3 = alloca { i8*, i64 }, align 8
  %0 = bitcast { i8*, i64 }* %_3 to {}**
  store {}* null, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_3, i32 0, i32 0
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_3, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
; call core::mem::replace
  %5 = call { i8*, i64 } @_ZN4core3mem7replace17hb6aec7f1adcc5d7fE({ i8*, i64 }* align 8 dereferenceable(16) %self, i8* noalias readonly align 1 %2, i64 %4)
  %6 = extractvalue { i8*, i64 } %5, 0
  %7 = extractvalue { i8*, i64 } %5, 1
  br label %bb1

bb1:                                              ; preds = %start
  %8 = insertvalue { i8*, i64 } undef, i8* %6, 0
  %9 = insertvalue { i8*, i64 } %8, i64 %7, 1
  ret { i8*, i64 } %9
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define { i8*, i64* } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h715518c92fbe7e50E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call { i8*, i64* } @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h9cfaf8c419904282E"(i8* nonnull %self.0, i64* noalias readonly align 8 dereferenceable(24) %self.1)
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
define internal i8* @_ZN5alloc5alloc15exchange_malloc17hbcc72b6a2a88f249E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_0 = alloca i8*, align 8
  %0 = icmp eq i64 %size, 0
  br i1 %0, label %bb2, label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::from_size_align_unchecked
  %1 = call { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h7df9b8de862ced3eE(i64 %size, i64 %align)
  %2 = extractvalue { i64, i64 } %1, 0
  %3 = extractvalue { i64, i64 } %1, 1
  br label %bb3

bb2:                                              ; preds = %start
  %4 = inttoptr i64 %align to i8*
  store i8* %4, i8** %_0, align 8
  br label %bb8

bb3:                                              ; preds = %bb1
; call alloc::alloc::alloc
  %5 = call i8* @_ZN5alloc5alloc5alloc17hd782af5423e4e2abE(i64 %2, i64 %3)
  br label %bb4

bb4:                                              ; preds = %bb3
; call core::ptr::<impl *mut T>::is_null
  %6 = call zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hc5eef318cb1f18adE"(i8* %5)
  br label %bb5

bb5:                                              ; preds = %bb4
  %7 = xor i1 %6, true
  br i1 %7, label %bb7, label %bb6

bb6:                                              ; preds = %bb5
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h45c54f24deea91a5E(i64 %2, i64 %3)
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
define internal i8* @_ZN5alloc5alloc5alloc17hd782af5423e4e2abE(i64, i64) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::Layout::size
  %4 = call i64 @_ZN4core5alloc6Layout4size17h913e505ce214fca8E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::align
  %5 = call i64 @_ZN4core5alloc6Layout5align17ha0bdf2f1dc33a41cE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = call i8* @__rust_alloc(i64 %4, i64 %5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %6
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17haf5396612adfc201E(i8* %ptr, i64, i64) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::Layout::size
  %4 = call i64 @_ZN4core5alloc6Layout4size17h913e505ce214fca8E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::align
  %5 = call i64 @_ZN4core5alloc6Layout5align17ha0bdf2f1dc33a41cE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %4, i64 %5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::alloc::box_free
; Function Attrs: inlinehint uwtable
define void @_ZN5alloc5alloc8box_free17h2f074b68af6a8c4cE(i8* nonnull %ptr.0, i64* noalias readonly align 8 dereferenceable(24) %ptr.1) unnamed_addr #0 {
start:
  %tmp_ret1 = alloca i64, align 8
  %tmp_ret = alloca i64, align 8
; call core::ptr::unique::Unique<T>::as_ptr
  %0 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17hfc456c05a7f514c8E"(i8* nonnull %ptr.0, i64* noalias readonly align 8 dereferenceable(24) %ptr.1)
  %1 = extractvalue { {}*, [3 x i64]* } %0, 0
  %2 = extractvalue { {}*, [3 x i64]* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = bitcast [3 x i64]* %2 to i64*
  %4 = getelementptr inbounds i64, i64* %3, i64 1
  %5 = load i64, i64* %4, align 8, !invariant.load !5
  %6 = bitcast [3 x i64]* %2 to i64*
  %7 = getelementptr inbounds i64, i64* %6, i64 2
  %8 = load i64, i64* %7, align 8, !invariant.load !5
  store i64 %5, i64* %tmp_ret, align 8
  %9 = load i64, i64* %tmp_ret, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
  %10 = bitcast [3 x i64]* %2 to i64*
  %11 = getelementptr inbounds i64, i64* %10, i64 1
  %12 = load i64, i64* %11, align 8, !invariant.load !5
  %13 = bitcast [3 x i64]* %2 to i64*
  %14 = getelementptr inbounds i64, i64* %13, i64 2
  %15 = load i64, i64* %14, align 8, !invariant.load !5
  store i64 %15, i64* %tmp_ret1, align 8
  %16 = load i64, i64* %tmp_ret1, align 8
  br label %bb3

bb3:                                              ; preds = %bb2
  %17 = icmp ne i64 %9, 0
  br i1 %17, label %bb4, label %bb7

bb4:                                              ; preds = %bb3
; call core::alloc::Layout::from_size_align_unchecked
  %18 = call { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h7df9b8de862ced3eE(i64 %9, i64 %16)
  %19 = extractvalue { i64, i64 } %18, 0
  %20 = extractvalue { i64, i64 } %18, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %21 = bitcast {}* %1 to i8*
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17haf5396612adfc201E(i8* %21, i64 %19, i64 %20)
  br label %bb6

bb6:                                              ; preds = %bb5
  br label %bb7

bb7:                                              ; preds = %bb6, %bb3
  ret void
}

; alloc::boxed::Box<T>::into_unique
; Function Attrs: inlinehint uwtable
define { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$11into_unique17hcc96cf3ef268096dE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
  %unique = alloca { i8*, i64* }, align 8
  %0 = bitcast {}* %b.0 to i8*
  %1 = bitcast [3 x i64]* %b.1 to i64*
  %2 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %unique, i32 0, i32 0
  store i8* %0, i8** %2, align 8
  %3 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %unique, i32 0, i32 1
  store i64* %1, i64** %3, align 8
; call core::mem::forget
  call void @_ZN4core3mem6forget17h7d89cbcefdb4336fE({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::unique::Unique<T>::as_mut
  %4 = call { {}*, [3 x i64]* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_mut17h062c6a843588adafE"({ i8*, i64* }* align 8 dereferenceable(16) %unique)
  %5 = extractvalue { {}*, [3 x i64]* } %4, 0
  %6 = extractvalue { {}*, [3 x i64]* } %4, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::unique::Unique<T>::new_unchecked
  %7 = call { i8*, i64* } @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17h1a464bf292351aa8E"({}* %5, [3 x i64]* noalias readonly align 8 dereferenceable(24) %6)
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
define { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$17into_raw_non_null17ha46ec4fe8ced99eaE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T>::into_unique
  %0 = call { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$11into_unique17hcc96cf3ef268096dE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  %1 = extractvalue { i8*, i64* } %0, 0
  %2 = extractvalue { i8*, i64* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call <T as core::convert::Into<U>>::into
  %3 = call { i8*, i64* } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h715518c92fbe7e50E"(i8* nonnull %1, i64* noalias readonly align 8 dereferenceable(24) %2)
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
define { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h67a291623a911e06E"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T>::into_raw_non_null
  %0 = call { i8*, i64* } @"_ZN5alloc5boxed12Box$LT$T$GT$17into_raw_non_null17ha46ec4fe8ced99eaE"({}* noalias nonnull align 1 %b.0, [3 x i64]* noalias readonly align 8 dereferenceable(24) %b.1)
  %1 = extractvalue { i8*, i64* } %0, 0
  %2 = extractvalue { i8*, i64* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::as_ptr
  %3 = call { {}*, [3 x i64]* } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h3066c126da80d8efE"(i8* nonnull %1, i64* noalias readonly align 8 dereferenceable(24) %2)
  %4 = extractvalue { {}*, [3 x i64]* } %3, 0
  %5 = extractvalue { {}*, [3 x i64]* } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { {}*, [3 x i64]* } undef, {}* %4, 0
  %7 = insertvalue { {}*, [3 x i64]* } %6, [3 x i64]* %5, 1
  ret { {}*, [3 x i64]* } %7
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hdc84d3ba2e8bffa6E"({ i8*, i64 }* align 8 dereferenceable(16)) unnamed_addr #1 {
start:
  %_5 = alloca { {}*, [3 x i64]* }, align 8
  %self = alloca { i8*, i64 }*, align 8
  store { i8*, i64 }* %0, { i8*, i64 }** %self, align 8
  %1 = load { i8*, i64 }*, { i8*, i64 }** %self, align 8, !nonnull !5
  %2 = bitcast { i8*, i64 }* %1 to {}**
  %3 = load {}*, {}** %2, align 8
  %4 = icmp eq {}* %3, null
  %5 = select i1 %4, i64 0, i64 1
  switch i64 %5, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb1:                                              ; preds = %start
  %6 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 0
  store {}* bitcast (<{ [0 x i8] }>* @0 to {}*), {}** %6, align 8
  %7 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.2 to [3 x i64]*), [3 x i64]** %7, align 8
  br label %bb4

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %8 = load { i8*, i64 }*, { i8*, i64 }** %self, align 8, !nonnull !5
  %9 = bitcast { i8*, i64 }* %8 to { [0 x i8]*, i64 }*
  %10 = bitcast { [0 x i8]*, i64 }* %9 to {}*
  %11 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 0
  store {}* %10, {}** %11, align 8
  %12 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.1 to [3 x i64]*), [3 x i64]** %12, align 8
  br label %bb4

bb4:                                              ; preds = %bb1, %bb3
  %13 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 0
  %14 = load {}*, {}** %13, align 8, !nonnull !5
  %15 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_5, i32 0, i32 1
  %16 = load [3 x i64]*, [3 x i64]** %15, align 8, !nonnull !5
  %17 = insertvalue { {}*, [3 x i64]* } undef, {}* %14, 0
  %18 = insertvalue { {}*, [3 x i64]* } %17, [3 x i64]* %16, 1
  ret { {}*, [3 x i64]* } %18
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::box_me_up
; Function Attrs: uwtable
define { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17h5a9a0f40839bb94cE"({ i8*, i64 }* align 8 dereferenceable(16)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
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
  %1 = load { i8*, i64 }*, { i8*, i64 }** %self, align 8, !nonnull !5
  store i8 1, i8* %_16, align 1
; call core::option::Option<T>::take
  %2 = call { i8*, i64 } @"_ZN4core6option15Option$LT$T$GT$4take17h6720285cec1b77d6E"({ i8*, i64 }* align 8 dereferenceable(16) %1)
  store { i8*, i64 } %2, { i8*, i64 }* %_4, align 8
  br label %bb2

bb1:                                              ; preds = %bb19, %bb9, %bb12, %bb11, %bb13
  %3 = bitcast { i8*, i32 }* %personalityslot to i8**
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %6 = load i32, i32* %5, align 8
  %7 = insertvalue { i8*, i32 } undef, i8* %4, 0
  %8 = insertvalue { i8*, i32 } %7, i32 %6, 1
  resume { i8*, i32 } %8

bb2:                                              ; preds = %start
  %9 = bitcast { i8*, i64 }* %_4 to {}**
  %10 = load {}*, {}** %9, align 8
  %11 = icmp eq {}* %10, null
  %12 = select i1 %11, i64 0, i64 1
  switch i64 %12, label %bb4 [
    i64 0, label %bb3
    i64 1, label %bb5
  ]

bb3:                                              ; preds = %bb2
; invoke alloc::alloc::exchange_malloc
  %13 = invoke i8* @_ZN5alloc5alloc15exchange_malloc17hbcc72b6a2a88f249E(i64 0, i64 1)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h5a802d1ad180d886E.exit" unwind label %cleanup

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h5a802d1ad180d886E.exit": ; preds = %bb3
  %14 = bitcast i8* %13 to {}*
  br label %bb7

bb4:                                              ; preds = %bb2
  unreachable

bb5:                                              ; preds = %bb2
  store i8 0, i8* %_16, align 1
  %15 = bitcast { i8*, i64 }* %_4 to { [0 x i8]*, i64 }*
  %16 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %15, i32 0, i32 0
  %17 = load [0 x i8]*, [0 x i8]** %16, align 8, !nonnull !5
  %18 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %15, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
; invoke alloc::alloc::exchange_malloc
  %20 = invoke i8* @_ZN5alloc5alloc15exchange_malloc17hbcc72b6a2a88f249E(i64 16, i64 8)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h953f627916b56f0fE.exit" unwind label %cleanup

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h953f627916b56f0fE.exit": ; preds = %bb5
  %21 = bitcast i8* %20 to { [0 x i8]*, i64 }*
  %22 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %21, i32 0, i32 0
  store [0 x i8]* %17, [0 x i8]** %22, align 8, !noalias !7
  %23 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %21, i32 0, i32 1
  store i64 %19, i64* %23, align 8
  br label %bb6

bb6:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h953f627916b56f0fE.exit"
  %24 = bitcast { [0 x i8]*, i64 }* %21 to {}*
  store i8 1, i8* %_17, align 1
  %25 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  store {}* %24, {}** %25, align 8
  %26 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.1 to [3 x i64]*), [3 x i64]** %26, align 8
  br label %bb8

bb7:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h5a802d1ad180d886E.exit"
  store i8 1, i8* %_17, align 1
  %27 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  store {}* %14, {}** %27, align 8
  %28 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  store [3 x i64]* bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.2 to [3 x i64]*), [3 x i64]** %28, align 8
  br label %bb8

bb8:                                              ; preds = %bb7, %bb6
  %29 = bitcast { i8*, i64 }* %_4 to {}**
  %30 = load {}*, {}** %29, align 8
  %31 = icmp eq {}* %30, null
  %32 = select i1 %31, i64 0, i64 1
  %33 = icmp eq i64 %32, 1
  br i1 %33, label %bb16, label %bb18

bb9:                                              ; preds = %cleanup1
  %34 = load i8, i8* %_17, align 1, !range !0
  %35 = trunc i8 %34 to i1
  br i1 %35, label %bb19, label %bb1

bb10:                                             ; preds = %bb15
  store { {}*, [3 x i64]* } %53, { {}*, [3 x i64]* }* %_13, align 8
  %36 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_13, i32 0, i32 0
  %37 = load {}*, {}** %36, align 8
  %38 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_13, i32 0, i32 1
  %39 = load [3 x i64]*, [3 x i64]** %38, align 8, !nonnull !5
  store i8 0, i8* %_17, align 1
  %40 = insertvalue { {}*, [3 x i64]* } undef, {}* %37, 0
  %41 = insertvalue { {}*, [3 x i64]* } %40, [3 x i64]* %39, 1
  ret { {}*, [3 x i64]* } %41

bb11:                                             ; preds = %bb14
  %42 = load i8, i8* %_16, align 1, !range !0
  %43 = trunc i8 %42 to i1
  br i1 %43, label %bb12, label %bb1

bb12:                                             ; preds = %bb11
  store i8 0, i8* %_16, align 1
  br label %bb1

bb13:                                             ; preds = %bb14
  br label %bb1

bb14:                                             ; preds = %cleanup
  %44 = bitcast { i8*, i64 }* %_4 to {}**
  %45 = load {}*, {}** %44, align 8
  %46 = icmp eq {}* %45, null
  %47 = select i1 %46, i64 0, i64 1
  %48 = icmp eq i64 %47, 1
  br i1 %48, label %bb11, label %bb13

bb15:                                             ; preds = %bb17, %bb16, %bb18
  store i8 0, i8* %_16, align 1
  store i8 0, i8* %_17, align 1
  %49 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 0
  %50 = load {}*, {}** %49, align 8, !nonnull !5
  %51 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %data, i32 0, i32 1
  %52 = load [3 x i64]*, [3 x i64]** %51, align 8, !nonnull !5
; invoke alloc::boxed::Box<T>::into_raw
  %53 = invoke { {}*, [3 x i64]* } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h67a291623a911e06E"({}* noalias nonnull align 1 %50, [3 x i64]* noalias readonly align 8 dereferenceable(24) %52)
          to label %bb10 unwind label %cleanup1

bb16:                                             ; preds = %bb8
  %54 = load i8, i8* %_16, align 1, !range !0
  %55 = trunc i8 %54 to i1
  br i1 %55, label %bb17, label %bb15

bb17:                                             ; preds = %bb16
  store i8 0, i8* %_16, align 1
  br label %bb15

bb18:                                             ; preds = %bb8
  br label %bb15

bb19:                                             ; preds = %bb9
  store i8 0, i8* %_17, align 1
; call core::ptr::real_drop_in_place
  call void @_ZN4core3ptr18real_drop_in_place17he062aa2ca9621ca6E({ {}*, [3 x i64]* }* align 8 dereferenceable(16) %data) #8
  br label %bb1

cleanup:                                          ; preds = %bb5, %bb3
  %56 = landingpad { i8*, i32 }
          cleanup
  %57 = extractvalue { i8*, i32 } %56, 0
  %58 = extractvalue { i8*, i32 } %56, 1
  %59 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %57, i8** %59, align 8
  %60 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %58, i32* %60, align 8
  br label %bb14

cleanup1:                                         ; preds = %bb15
  %61 = landingpad { i8*, i32 }
          cleanup
  %62 = extractvalue { i8*, i32 } %61, 0
  %63 = extractvalue { i8*, i32 } %61, 1
  %64 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %62, i8** %64, align 8
  %65 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %63, i32* %65, align 8
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
  call void @_ZN3std9panicking11begin_panic17h39256d503986386eE([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [5 x i8] }>* @3 to [0 x i8]*), i64 5, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @2 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #3

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #4

; std::panicking::rust_panic_with_hook
; Function Attrs: noreturn uwtable
declare void @_ZN3std9panicking20rust_panic_with_hook17h7d6a669f1a899680E({}* nonnull align 1, [3 x i64]* noalias readonly align 8 dereferenceable(24), i64* noalias readonly align 8 dereferenceable_or_null(48), { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24)) unnamed_addr #5

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1) #6

; alloc::alloc::handle_alloc_error
; Function Attrs: noreturn nounwind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h45c54f24deea91a5E(i64, i64) unnamed_addr #7

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
attributes #7 = { noreturn nounwind uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #8 = { noinline }

!0 = !{i8 0, i8 2}
!1 = !{!2, !4}
!2 = distinct !{!2, !3, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hc1404bd126599c44E: %value.0"}
!3 = distinct !{!3, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hc1404bd126599c44E"}
!4 = distinct !{!4, !3, !"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hc1404bd126599c44E: %value.1"}
!5 = !{}
!6 = !{i64 1, i64 0}
!7 = !{!8}
!8 = distinct !{!8, !9, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h953f627916b56f0fE: %x.0"}
!9 = distinct !{!9, !"_ZN5alloc5boxed12Box$LT$T$GT$3new17h953f627916b56f0fE"}
