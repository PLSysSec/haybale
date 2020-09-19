; ModuleID = 'issue_10.3a1fbbbh-cgu.0'
source_filename = "issue_10.3a1fbbbh-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::::Opaque" = type {}
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"core::panic::Location" = type { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc1 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 4
@0 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [0 x i8] zeroinitializer }>, align 8
@alloc5 = private unnamed_addr constant <{ [45 x i8] }> <{ [45 x i8] c"assertion failed: `(left == right)`\0A  left: `" }>, align 1
@alloc6 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"`,\0A right: `" }>, align 1
@alloc7 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"`" }>, align 1
@alloc8 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [45 x i8] }>, <{ [45 x i8] }>* @alloc5, i32 0, i32 0, i32 0), [8 x i8] c"-\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [12 x i8] }>, <{ [12 x i8] }>* @alloc6, i32 0, i32 0, i32 0), [8 x i8] c"\0C\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc7, i32 0, i32 0, i32 0), [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@1 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* bitcast (<{ i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8] }>* @alloc8 to i8*), [0 x i8] zeroinitializer }>, align 8
@alloc13 = private unnamed_addr constant <{ [11 x i8] }> <{ [11 x i8] c"issue_10.rs" }>, align 1
@alloc14 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [11 x i8] }>, <{ [11 x i8] }>* @alloc13, i32 0, i32 0, i32 0), [16 x i8] c"\0B\00\00\00\00\00\00\00\02\00\00\00\05\00\00\00" }>, align 8

; <&T as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcf85089dc8155d07E"(i32** noalias readonly align 8 dereferenceable(8) %self, %"core::fmt::Formatter"* align 8 dereferenceable(64) %f) unnamed_addr #0 {
start:
  %_4 = load i32*, i32** %self, align 8, !nonnull !1
; call core::fmt::num::<impl core::fmt::Debug for u32>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9849c002a5128e9dE"(i32* noalias readonly align 4 dereferenceable(4) %_4, %"core::fmt::Formatter"* align 8 dereferenceable(64) %f)
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %0
}

; core::fmt::ArgumentV1::new
; Function Attrs: uwtable
define { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h236efc43f4cd131eE(i32** noalias readonly align 8 dereferenceable(8) %x, i1 (i32**, %"core::fmt::Formatter"*)* nonnull %f) unnamed_addr #0 {
start:
  %0 = alloca %"core::fmt::::Opaque"*, align 8
  %1 = alloca i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, align 8
  %2 = alloca { i8*, i8* }, align 8
  %3 = bitcast i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1 to i1 (i32**, %"core::fmt::Formatter"*)**
  store i1 (i32**, %"core::fmt::Formatter"*)* %f, i1 (i32**, %"core::fmt::Formatter"*)** %3, align 8
  %_3 = load i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1, align 8, !nonnull !1
  br label %bb1

bb1:                                              ; preds = %start
  %4 = bitcast %"core::fmt::::Opaque"** %0 to i32***
  store i32** %x, i32*** %4, align 8
  %_5 = load %"core::fmt::::Opaque"*, %"core::fmt::::Opaque"** %0, align 8, !nonnull !1
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i8* }* %2 to %"core::fmt::::Opaque"**
  store %"core::fmt::::Opaque"* %_5, %"core::fmt::::Opaque"** %5, align 8
  %6 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %7 = bitcast i8** %6 to i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)**
  store i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %7, align 8
  %8 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8, !nonnull !1
  %10 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %11 = load i8*, i8** %10, align 8, !nonnull !1
  %12 = insertvalue { i8*, i8* } undef, i8* %9, 0
  %13 = insertvalue { i8*, i8* } %12, i8* %11, 1
  ret { i8*, i8* } %13
}

; core::fmt::num::<impl core::fmt::Debug for u32>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9849c002a5128e9dE"(i32* noalias readonly align 4 dereferenceable(4) %self, %"core::fmt::Formatter"* align 8 dereferenceable(64) %f) unnamed_addr #1 {
start:
  %0 = alloca i8, align 1
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h86bed372f12c8068E(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(64) %f)
  br label %bb1

bb1:                                              ; preds = %start
  br i1 %_3, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
; call core::fmt::Formatter::debug_upper_hex
  %_7 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hef9b5660ea2c0e9fE(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(64) %f)
  br label %bb5

bb3:                                              ; preds = %bb1
; call core::fmt::num::<impl core::fmt::LowerHex for u32>::fmt
  %1 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h1e429e8f8d097c32E"(i32* noalias readonly align 4 dereferenceable(4) %self, %"core::fmt::Formatter"* align 8 dereferenceable(64) %f)
  %2 = zext i1 %1 to i8
  store i8 %2, i8* %0, align 1
  br label %bb4

bb4:                                              ; preds = %bb3
  br label %bb11

bb5:                                              ; preds = %bb2
  br i1 %_7, label %bb7, label %bb6

bb6:                                              ; preds = %bb5
; call core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
  %3 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h9718cd0454163961E"(i32* noalias readonly align 4 dereferenceable(4) %self, %"core::fmt::Formatter"* align 8 dereferenceable(64) %f)
  %4 = zext i1 %3 to i8
  store i8 %4, i8* %0, align 1
  br label %bb9

bb7:                                              ; preds = %bb5
; call core::fmt::num::<impl core::fmt::UpperHex for u32>::fmt
  %5 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u32$GT$3fmt17hbae43f4b8f732509E"(i32* noalias readonly align 4 dereferenceable(4) %self, %"core::fmt::Formatter"* align 8 dereferenceable(64) %f)
  %6 = zext i1 %5 to i8
  store i8 %6, i8* %0, align 1
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb10

bb9:                                              ; preds = %bb6
  br label %bb10

bb10:                                             ; preds = %bb9, %bb8
  br label %bb11

bb11:                                             ; preds = %bb10, %bb4
  %7 = load i8, i8* %0, align 1, !range !2
  %8 = trunc i8 %7 to i1
  ret i1 %8
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117ha85784472e376694E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %0, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %args.0, i64 %args.1) unnamed_addr #1 {
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

; issue_10::panic_if_not_zero
; Function Attrs: uwtable
define void @_ZN8issue_1017panic_if_not_zero17h50d5521753bc521fE(i32 %0) unnamed_addr #0 {
start:
  %_25 = alloca i32*, align 8
  %_23 = alloca i32*, align 8
  %_21 = alloca { i64*, i64* }, align 8
  %_20 = alloca [2 x { i8*, i8* }], align 8
  %_13 = alloca %"core::fmt::Arguments", align 8
  %_2 = alloca { i32*, i32* }, align 8
  %x = alloca i32, align 4
  store i32 %0, i32* %x, align 4
  %_35 = load i32*, i32** bitcast (<{ i8*, [0 x i8] }>* @0 to i32**), align 8, !nonnull !1
  %1 = bitcast { i32*, i32* }* %_2 to i32**
  store i32* %x, i32** %1, align 8
  %2 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %_2, i32 0, i32 1
  store i32* %_35, i32** %2, align 8
  %3 = bitcast { i32*, i32* }* %_2 to i32**
  %left_val = load i32*, i32** %3, align 8, !nonnull !1
  %4 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %_2, i32 0, i32 1
  %right_val = load i32*, i32** %4, align 8, !nonnull !1
  %_9 = load i32, i32* %left_val, align 4
  %_10 = load i32, i32* %right_val, align 4
  %_8 = icmp eq i32 %_9, %_10
  %_7 = xor i1 %_8, true
  br i1 %_7, label %bb2, label %bb1

bb1:                                              ; preds = %start
  ret void

bb2:                                              ; preds = %start
  %_34 = load [3 x { [0 x i8]*, i64 }]*, [3 x { [0 x i8]*, i64 }]** bitcast (<{ i8*, [0 x i8] }>* @1 to [3 x { [0 x i8]*, i64 }]**), align 8, !nonnull !1
  %_14.0 = bitcast [3 x { [0 x i8]*, i64 }]* %_34 to [0 x { [0 x i8]*, i64 }]*
  store i32* %left_val, i32** %_23, align 8
  store i32* %right_val, i32** %_25, align 8
  %5 = bitcast { i64*, i64* }* %_21 to i32***
  store i32** %_23, i32*** %5, align 8
  %6 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_21, i32 0, i32 1
  %7 = bitcast i64** %6 to i32***
  store i32** %_25, i32*** %7, align 8
  %8 = bitcast { i64*, i64* }* %_21 to i32***
  %arg0 = load i32**, i32*** %8, align 8, !nonnull !1
  %9 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_21, i32 0, i32 1
  %10 = bitcast i64** %9 to i32***
  %arg1 = load i32**, i32*** %10, align 8, !nonnull !1
; call core::fmt::ArgumentV1::new
  %11 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h236efc43f4cd131eE(i32** noalias readonly align 8 dereferenceable(8) %arg0, i1 (i32**, %"core::fmt::Formatter"*)* nonnull @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcf85089dc8155d07E")
  %_28.0 = extractvalue { i8*, i8* } %11, 0
  %_28.1 = extractvalue { i8*, i8* } %11, 1
  br label %bb3

bb3:                                              ; preds = %bb2
; call core::fmt::ArgumentV1::new
  %12 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h236efc43f4cd131eE(i32** noalias readonly align 8 dereferenceable(8) %arg1, i1 (i32**, %"core::fmt::Formatter"*)* nonnull @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcf85089dc8155d07E")
  %_31.0 = extractvalue { i8*, i8* } %12, 0
  %_31.1 = extractvalue { i8*, i8* } %12, 1
  br label %bb4

bb4:                                              ; preds = %bb3
  %13 = bitcast [2 x { i8*, i8* }]* %_20 to { i8*, i8* }*
  %14 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %13, i32 0, i32 0
  store i8* %_28.0, i8** %14, align 8
  %15 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %13, i32 0, i32 1
  store i8* %_28.1, i8** %15, align 8
  %16 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_20, i32 0, i32 1
  %17 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %16, i32 0, i32 0
  store i8* %_31.0, i8** %17, align 8
  %18 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %16, i32 0, i32 1
  store i8* %_31.1, i8** %18, align 8
  %_17.0 = bitcast [2 x { i8*, i8* }]* %_20 to [0 x { i8*, i8* }]*
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117ha85784472e376694E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_13, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %_14.0, i64 3, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %_17.0, i64 2)
  br label %bb5

bb5:                                              ; preds = %bb4
; call std::panicking::begin_panic_fmt
  call void @_ZN3std9panicking15begin_panic_fmt17he5e9d28c7f7ec820E(%"core::fmt::Arguments"* noalias readonly align 8 dereferenceable(48) %_13, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc14 to %"core::panic::Location"*))
  unreachable
}

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h86bed372f12c8068E(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(64)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for u32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17h1e429e8f8d097c32E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #0

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hef9b5660ea2c0e9fE(%"core::fmt::Formatter"* noalias readonly align 8 dereferenceable(64)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for u32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u32$GT$3fmt17hbae43f4b8f732509E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #0

; core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h9718cd0454163961E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #0

; std::panicking::begin_panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN3std9panicking15begin_panic_fmt17he5e9d28c7f7ec820E(%"core::fmt::Arguments"* noalias readonly align 8 dereferenceable(48), %"core::panic::Location"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { inlinehint uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{i8 0, i8 2}
