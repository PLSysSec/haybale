; ModuleID = 'basic_rust.3a1fbbbh-cgu.0'
source_filename = "basic_rust.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128"
target triple = "i686-unknown-linux-gnu"

%"core::fmt::Formatter" = type { [0 x i32], i32, [0 x i32], i32, [0 x i32], { i32, i32 }, [0 x i32], { i32, i32 }, [0 x i32], { {}*, [3 x i32]* }, [0 x i8], i8, [3 x i8] }
%"core::fmt::::Opaque" = type {}
%"core::fmt::Arguments" = type { [0 x i32], { [0 x { [0 x i8]*, i32 }]*, i32 }, [0 x i32], { i32*, i32 }, [0 x i32], { [0 x { i8*, i8* }]*, i32 }, [0 x i32] }
%"core::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc8 = private unnamed_addr constant <{ [13 x i8] }> <{ [13 x i8] c"basic_rust.rs" }>, align 1
@alloc9 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [13 x i8] }>, <{ [13 x i8] }>* @alloc8, i32 0, i32 0, i32 0), [12 x i8] c"\0D\00\00\00\04\00\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [33 x i8] c"attempt to multiply with overflow"
@alloc2 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"out: " }>, align 1
@alloc3 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc4 = private unnamed_addr constant <{ i8*, [4 x i8], i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [5 x i8] }>, <{ [5 x i8] }>* @alloc2, i32 0, i32 0, i32 0), [4 x i8] c"\05\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00" }>, align 4
@0 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* bitcast (<{ i8*, [4 x i8], i8*, [4 x i8] }>* @alloc4 to i8*), [0 x i8] zeroinitializer }>, align 4

; core::fmt::ArgumentV1::new
; Function Attrs: nonlazybind uwtable
define { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h73cf485f640832c9E(i32* noalias readonly align 4 dereferenceable(4) %x, i1 (i32*, %"core::fmt::Formatter"*)* nonnull %f) unnamed_addr #0 {
start:
  %0 = alloca %"core::fmt::::Opaque"*, align 4
  %1 = alloca i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, align 4
  %2 = alloca { i8*, i8* }, align 4
  %3 = bitcast i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1 to i1 (i32*, %"core::fmt::Formatter"*)**
  store i1 (i32*, %"core::fmt::Formatter"*)* %f, i1 (i32*, %"core::fmt::Formatter"*)** %3, align 4
  %_3 = load i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1, align 4, !nonnull !2
  br label %bb1

bb1:                                              ; preds = %start
  %4 = bitcast %"core::fmt::::Opaque"** %0 to i32**
  store i32* %x, i32** %4, align 4
  %_5 = load %"core::fmt::::Opaque"*, %"core::fmt::::Opaque"** %0, align 4, !nonnull !2
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i8* }* %2 to %"core::fmt::::Opaque"**
  store %"core::fmt::::Opaque"* %_5, %"core::fmt::::Opaque"** %5, align 4
  %6 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %7 = bitcast i8** %6 to i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)**
  store i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %7, align 4
  %8 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 4, !nonnull !2
  %10 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %11 = load i8*, i8** %10, align 4, !nonnull !2
  %12 = insertvalue { i8*, i8* } undef, i8* %9, 0
  %13 = insertvalue { i8*, i8* } %12, i8* %11, 1
  ret { i8*, i8* } %13
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hdfc69fab17c9400aE(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(24) %0, [0 x { [0 x i8]*, i32 }]* noalias nonnull readonly align 4 %pieces.0, i32 %pieces.1, [0 x { i8*, i8* }]* noalias nonnull readonly align 4 %args.0, i32 %args.1) unnamed_addr #1 {
start:
  %_4 = alloca { i32*, i32 }, align 4
  %1 = bitcast { i32*, i32 }* %_4 to {}**
  store {}* null, {}** %1, align 4
  %2 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i32 }]*, i32 }*
  %3 = getelementptr inbounds { [0 x { [0 x i8]*, i32 }]*, i32 }, { [0 x { [0 x i8]*, i32 }]*, i32 }* %2, i32 0, i32 0
  store [0 x { [0 x i8]*, i32 }]* %pieces.0, [0 x { [0 x i8]*, i32 }]** %3, align 4
  %4 = getelementptr inbounds { [0 x { [0 x i8]*, i32 }]*, i32 }, { [0 x { [0 x i8]*, i32 }]*, i32 }* %2, i32 0, i32 1
  store i32 %pieces.1, i32* %4, align 4
  %5 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3
  %6 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %_4, i32 0, i32 0
  %7 = load i32*, i32** %6, align 4
  %8 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %_4, i32 0, i32 1
  %9 = load i32, i32* %8, align 4
  %10 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %5, i32 0, i32 0
  store i32* %7, i32** %10, align 4
  %11 = getelementptr inbounds { i32*, i32 }, { i32*, i32 }* %5, i32 0, i32 1
  store i32 %9, i32* %11, align 4
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5
  %13 = getelementptr inbounds { [0 x { i8*, i8* }]*, i32 }, { [0 x { i8*, i8* }]*, i32 }* %12, i32 0, i32 0
  store [0 x { i8*, i8* }]* %args.0, [0 x { i8*, i8* }]** %13, align 4
  %14 = getelementptr inbounds { [0 x { i8*, i8* }]*, i32 }, { [0 x { i8*, i8* }]*, i32 }* %12, i32 0, i32 1
  store i32 %args.1, i32* %14, align 4
  ret void
}

; basic_rust::ez
; Function Attrs: nonlazybind uwtable
define i32 @_ZN10basic_rust2ez17hf9be465885e49920E(i32 %input) unnamed_addr #0 {
start:
  %0 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %input, i32 2)
  %_3.0 = extractvalue { i32, i1 } %0, 0
  %_3.1 = extractvalue { i32, i1 } %0, 1
  %1 = call i1 @llvm.expect.i1(i1 %_3.1, i1 false)
  br i1 %1, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i32 %_3.0

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hdaab655da8250769E([0 x i8]* noalias nonnull readonly align 1 bitcast ([33 x i8]* @str.0 to [0 x i8]*), i32 33, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc9 to %"core::panic::Location"*))
  unreachable
}

; basic_rust::main
; Function Attrs: nonlazybind uwtable
define void @_ZN10basic_rust4main17h9124f02438e9f370E() unnamed_addr #0 {
start:
  %_11 = alloca i32*, align 4
  %_10 = alloca [1 x { i8*, i8* }], align 4
  %_3 = alloca %"core::fmt::Arguments", align 4
  %out = alloca i32, align 4
; call basic_rust::ez
  %0 = call i32 @_ZN10basic_rust2ez17hf9be465885e49920E(i32 1)
  store i32 %0, i32* %out, align 4
  br label %bb1

bb1:                                              ; preds = %start
  %_17 = load [2 x { [0 x i8]*, i32 }]*, [2 x { [0 x i8]*, i32 }]** bitcast (<{ i8*, [0 x i8] }>* @0 to [2 x { [0 x i8]*, i32 }]**), align 4, !nonnull !2
  %_4.0 = bitcast [2 x { [0 x i8]*, i32 }]* %_17 to [0 x { [0 x i8]*, i32 }]*
  store i32* %out, i32** %_11, align 4
  %arg0 = load i32*, i32** %_11, align 4, !nonnull !2
; call core::fmt::ArgumentV1::new
  %1 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h73cf485f640832c9E(i32* noalias readonly align 4 dereferenceable(4) %arg0, i1 (i32*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h2720722ff93c563bE")
  %_14.0 = extractvalue { i8*, i8* } %1, 0
  %_14.1 = extractvalue { i8*, i8* } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %2 = bitcast [1 x { i8*, i8* }]* %_10 to { i8*, i8* }*
  %3 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 0
  store i8* %_14.0, i8** %3, align 4
  %4 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  store i8* %_14.1, i8** %4, align 4
  %_7.0 = bitcast [1 x { i8*, i8* }]* %_10 to [0 x { i8*, i8* }]*
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hdfc69fab17c9400aE(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(24) %_3, [0 x { [0 x i8]*, i32 }]* noalias nonnull readonly align 4 %_4.0, i32 2, [0 x { i8*, i8* }]* noalias nonnull readonly align 4 %_7.0, i32 1)
  br label %bb3

bb3:                                              ; preds = %bb2
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h2812396d3cb60f3cE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24) %_3)
  br label %bb4

bb4:                                              ; preds = %bb3
  ret void
}

; Function Attrs: nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #2

; Function Attrs: nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hdaab655da8250769E([0 x i8]* noalias nonnull readonly align 1, i32, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #4

; core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h2720722ff93c563bE"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 4 dereferenceable(36)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h2812396d3cb60f3cE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24)) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }
attributes #2 = { nounwind readnone speculatable willreturn }
attributes #3 = { nounwind readnone willreturn }
attributes #4 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{}
