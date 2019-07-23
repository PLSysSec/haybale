; ModuleID = 'struct.c'
source_filename = "struct.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

%struct.OneInt = type { i32 }
%struct.TwoInts = type { i32, i32 }
%struct.ThreeInts = type { i32, i32, i32 }
%struct.Mismatched = type { i8, i32, i8 }
%struct.Nested = type { %struct.TwoInts, %struct.Mismatched }
%struct.WithArray = type { %struct.Mismatched, [10 x i32], %struct.Mismatched }

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @one_int(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.OneInt, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.OneInt* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 4, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.OneInt, %struct.OneInt* %3, i32 0, i32 0
  store volatile i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds %struct.OneInt, %struct.OneInt* %3, i32 0, i32 0
  %8 = load volatile i32, i32* %7, align 4
  %9 = sub nsw i32 %8, 3
  ret i32 %9
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1) #1

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @two_ints_first(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.TwoInts, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.TwoInts* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 8, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 0
  store volatile i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 0
  %8 = load volatile i32, i32* %7, align 4
  %9 = sub nsw i32 %8, 3
  ret i32 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @two_ints_second(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.TwoInts, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.TwoInts* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 8, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 1
  store volatile i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 1
  %8 = load volatile i32, i32* %7, align 4
  %9 = sub nsw i32 %8, 3
  ret i32 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @two_ints_both(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.TwoInts, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.TwoInts* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 8, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = add nsw i32 %5, 2
  %7 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 0
  store volatile i32 %6, i32* %7, align 4
  %8 = load i32, i32* %2, align 4
  %9 = add nsw i32 %8, 3
  %10 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 1
  store volatile i32 %9, i32* %10, align 4
  %11 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 1
  %12 = load volatile i32, i32* %11, align 4
  %13 = sub nsw i32 %12, 10
  %14 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 0
  store volatile i32 %13, i32* %14, align 4
  %15 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 0
  %16 = load volatile i32, i32* %15, align 4
  %17 = add nsw i32 %16, 7
  %18 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 1
  store volatile i32 %17, i32* %18, align 4
  %19 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %3, i32 0, i32 1
  %20 = load volatile i32, i32* %19, align 4
  %21 = sub nsw i32 %20, 3
  ret i32 %21
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @three_ints(i32, i32) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca %struct.ThreeInts, align 4
  store i32 %0, i32* %3, align 4
  store i32 %1, i32* %4, align 4
  %6 = bitcast %struct.ThreeInts* %5 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %6, i8 0, i64 12, i1 true)
  %7 = load i32, i32* %3, align 4
  %8 = load i32, i32* %4, align 4
  %9 = add nsw i32 %7, %8
  %10 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 0
  store volatile i32 %9, i32* %10, align 4
  %11 = load i32, i32* %3, align 4
  %12 = load i32, i32* %4, align 4
  %13 = sub nsw i32 %11, %12
  %14 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 1
  store volatile i32 %13, i32* %14, align 4
  %15 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 0
  %16 = load volatile i32, i32* %15, align 4
  %17 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 1
  %18 = load volatile i32, i32* %17, align 4
  %19 = add nsw i32 %16, %18
  %20 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 2
  store volatile i32 %19, i32* %20, align 4
  %21 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 2
  %22 = load volatile i32, i32* %21, align 4
  %23 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 0
  %24 = load volatile i32, i32* %23, align 4
  %25 = mul nsw i32 2, %24
  %26 = sub nsw i32 %22, %25
  %27 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 1
  store volatile i32 %26, i32* %27, align 4
  %28 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 2
  %29 = load volatile i32, i32* %28, align 4
  %30 = load i32, i32* %3, align 4
  %31 = sub nsw i32 %29, %30
  %32 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 0
  store volatile i32 %31, i32* %32, align 4
  %33 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %5, i32 0, i32 0
  %34 = load volatile i32, i32* %33, align 4
  %35 = sub nsw i32 %34, 3
  ret i32 %35
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @zero_initialize(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.ThreeInts, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %7 = bitcast %struct.ThreeInts* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %7, i8 0, i64 12, i1 true)
  %8 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %3, i32 0, i32 0
  %9 = load volatile i32, i32* %8, align 4
  %10 = add nsw i32 %9, 2
  store i32 %10, i32* %4, align 4
  %11 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %3, i32 0, i32 1
  %12 = load volatile i32, i32* %11, align 4
  %13 = add nsw i32 %12, 4
  store i32 %13, i32* %5, align 4
  %14 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %3, i32 0, i32 2
  %15 = load volatile i32, i32* %14, align 4
  %16 = add nsw i32 %15, 6
  store i32 %16, i32* %6, align 4
  %17 = load i32, i32* %4, align 4
  %18 = load i32, i32* %5, align 4
  %19 = add nsw i32 %17, %18
  %20 = load i32, i32* %6, align 4
  %21 = add nsw i32 %19, %20
  %22 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %3, i32 0, i32 1
  store volatile i32 %21, i32* %22, align 4
  %23 = load i32, i32* %2, align 4
  %24 = getelementptr inbounds %struct.ThreeInts, %struct.ThreeInts* %3, i32 0, i32 1
  %25 = load volatile i32, i32* %24, align 4
  %26 = sub nsw i32 %23, %25
  ret i32 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define zeroext i8 @mismatched_first(i8 zeroext) #0 {
  %2 = alloca i8, align 1
  %3 = alloca %struct.Mismatched, align 4
  store i8 %0, i8* %2, align 1
  %4 = bitcast %struct.Mismatched* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 12, i1 true)
  %5 = load i8, i8* %2, align 1
  %6 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %3, i32 0, i32 0
  store volatile i8 %5, i8* %6, align 4
  %7 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %3, i32 0, i32 0
  %8 = load volatile i8, i8* %7, align 4
  %9 = zext i8 %8 to i32
  %10 = sub nsw i32 %9, 3
  %11 = trunc i32 %10 to i8
  ret i8 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @mismatched_second(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.Mismatched, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.Mismatched* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 12, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %3, i32 0, i32 1
  store volatile i32 %5, i32* %6, align 4
  %7 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %3, i32 0, i32 1
  %8 = load volatile i32, i32* %7, align 4
  %9 = sub i32 %8, 3
  ret i32 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define zeroext i8 @mismatched_third(i8 zeroext) #0 {
  %2 = alloca i8, align 1
  %3 = alloca %struct.Mismatched, align 4
  store i8 %0, i8* %2, align 1
  %4 = bitcast %struct.Mismatched* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 12, i1 true)
  %5 = load i8, i8* %2, align 1
  %6 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %3, i32 0, i32 2
  store volatile i8 %5, i8* %6, align 4
  %7 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %3, i32 0, i32 2
  %8 = load volatile i8, i8* %7, align 4
  %9 = zext i8 %8 to i32
  %10 = sub nsw i32 %9, 3
  %11 = trunc i32 %10 to i8
  ret i8 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @mismatched_all(i8 zeroext, i32) #0 {
  %3 = alloca i8, align 1
  %4 = alloca i32, align 4
  %5 = alloca %struct.Mismatched, align 4
  store i8 %0, i8* %3, align 1
  store i32 %1, i32* %4, align 4
  %6 = bitcast %struct.Mismatched* %5 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %6, i8 0, i64 12, i1 true)
  %7 = load i8, i8* %3, align 1
  %8 = zext i8 %7 to i32
  %9 = add nsw i32 %8, 3
  %10 = trunc i32 %9 to i8
  %11 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 0
  store volatile i8 %10, i8* %11, align 4
  %12 = load i32, i32* %4, align 4
  %13 = sub nsw i32 %12, 3
  %14 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 1
  store volatile i32 %13, i32* %14, align 4
  %15 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 0
  %16 = load volatile i8, i8* %15, align 4
  %17 = zext i8 %16 to i32
  %18 = load i8, i8* %3, align 1
  %19 = zext i8 %18 to i32
  %20 = sub nsw i32 %17, %19
  %21 = trunc i32 %20 to i8
  %22 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 2
  store volatile i8 %21, i8* %22, align 4
  %23 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 2
  %24 = load volatile i8, i8* %23, align 4
  %25 = zext i8 %24 to i32
  %26 = load i8, i8* %3, align 1
  %27 = zext i8 %26 to i32
  %28 = sub nsw i32 %25, %27
  %29 = trunc i32 %28 to i8
  %30 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 0
  store volatile i8 %29, i8* %30, align 4
  %31 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 1
  %32 = load volatile i32, i32* %31, align 4
  %33 = add i32 %32, 4
  %34 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 1
  store volatile i32 %33, i32* %34, align 4
  %35 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 0
  %36 = load volatile i8, i8* %35, align 4
  %37 = zext i8 %36 to i32
  %38 = load i8, i8* %3, align 1
  %39 = zext i8 %38 to i32
  %40 = sub nsw i32 %37, %39
  %41 = trunc i32 %40 to i8
  %42 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 0
  store volatile i8 %41, i8* %42, align 4
  %43 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 2
  %44 = load volatile i8, i8* %43, align 4
  %45 = zext i8 %44 to i32
  %46 = sub nsw i32 %45, 5
  %47 = trunc i32 %46 to i8
  %48 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 2
  store volatile i8 %47, i8* %48, align 4
  %49 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 1
  %50 = load volatile i32, i32* %49, align 4
  %51 = load i32, i32* %4, align 4
  %52 = add i32 %50, %51
  %53 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 1
  store volatile i32 %52, i32* %53, align 4
  %54 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 0
  %55 = load volatile i8, i8* %54, align 4
  %56 = zext i8 %55 to i32
  %57 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 1
  %58 = load volatile i32, i32* %57, align 4
  %59 = add i32 %56, %58
  %60 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %5, i32 0, i32 2
  %61 = load volatile i8, i8* %60, align 4
  %62 = zext i8 %61 to i32
  %63 = add i32 %59, %62
  ret i32 %63
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @nested_first(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.Nested, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.Nested* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 20, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.Nested, %struct.Nested* %3, i32 0, i32 0
  %7 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %6, i32 0, i32 0
  store volatile i32 %5, i32* %7, align 4
  %8 = getelementptr inbounds %struct.Nested, %struct.Nested* %3, i32 0, i32 0
  %9 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %8, i32 0, i32 1
  store volatile i32 3, i32* %9, align 4
  %10 = getelementptr inbounds %struct.Nested, %struct.Nested* %3, i32 0, i32 0
  %11 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %10, i32 0, i32 0
  %12 = load volatile i32, i32* %11, align 4
  %13 = getelementptr inbounds %struct.Nested, %struct.Nested* %3, i32 0, i32 0
  %14 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %13, i32 0, i32 1
  %15 = load volatile i32, i32* %14, align 4
  %16 = sub nsw i32 %12, %15
  ret i32 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @nested_second(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.Nested, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.Nested* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 20, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.Nested, %struct.Nested* %3, i32 0, i32 1
  %7 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %6, i32 0, i32 1
  store volatile i32 %5, i32* %7, align 4
  %8 = getelementptr inbounds %struct.Nested, %struct.Nested* %3, i32 0, i32 1
  %9 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %8, i32 0, i32 1
  %10 = load volatile i32, i32* %9, align 4
  %11 = sub i32 %10, 3
  ret i32 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @nested_all(i8 zeroext, i32) #0 {
  %3 = alloca i8, align 1
  %4 = alloca i32, align 4
  %5 = alloca %struct.Nested, align 4
  store i8 %0, i8* %3, align 1
  store i32 %1, i32* %4, align 4
  %6 = bitcast %struct.Nested* %5 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %6, i8 0, i64 20, i1 true)
  %7 = load i32, i32* %4, align 4
  %8 = add nsw i32 %7, 3
  %9 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 0
  %10 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %9, i32 0, i32 1
  store volatile i32 %8, i32* %10, align 4
  %11 = load i8, i8* %3, align 1
  %12 = zext i8 %11 to i32
  %13 = sub nsw i32 %12, 4
  %14 = trunc i32 %13 to i8
  %15 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %16 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %15, i32 0, i32 0
  store volatile i8 %14, i8* %16, align 4
  %17 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %18 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %17, i32 0, i32 1
  %19 = load volatile i32, i32* %18, align 4
  %20 = load i32, i32* %4, align 4
  %21 = add i32 %19, %20
  %22 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 0
  %23 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %22, i32 0, i32 0
  store volatile i32 %21, i32* %23, align 4
  %24 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %25 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %24, i32 0, i32 0
  %26 = load volatile i8, i8* %25, align 4
  %27 = zext i8 %26 to i32
  %28 = add nsw i32 %27, 10
  %29 = trunc i32 %28 to i8
  %30 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %31 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %30, i32 0, i32 2
  store volatile i8 %29, i8* %31, align 4
  %32 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %33 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %32, i32 0, i32 2
  %34 = load volatile i8, i8* %33, align 4
  %35 = zext i8 %34 to i32
  %36 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %37 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %36, i32 0, i32 0
  %38 = load volatile i8, i8* %37, align 4
  %39 = zext i8 %38 to i32
  %40 = add nsw i32 %35, %39
  %41 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %42 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %41, i32 0, i32 1
  store volatile i32 %40, i32* %42, align 4
  %43 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 1
  %44 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %43, i32 0, i32 2
  %45 = load volatile i8, i8* %44, align 4
  %46 = zext i8 %45 to i32
  %47 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 0
  %48 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %47, i32 0, i32 0
  %49 = load volatile i32, i32* %48, align 4
  %50 = add nsw i32 %46, %49
  %51 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 0
  %52 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %51, i32 0, i32 1
  store volatile i32 %50, i32* %52, align 4
  %53 = getelementptr inbounds %struct.Nested, %struct.Nested* %5, i32 0, i32 0
  %54 = getelementptr inbounds %struct.TwoInts, %struct.TwoInts* %53, i32 0, i32 1
  %55 = load volatile i32, i32* %54, align 4
  %56 = load i32, i32* %4, align 4
  %57 = sub nsw i32 %55, %56
  ret i32 %57
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @with_array(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.WithArray, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.WithArray* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 64, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %7 = getelementptr inbounds [10 x i32], [10 x i32]* %6, i64 0, i64 4
  store volatile i32 %5, i32* %7, align 4
  %8 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %9 = getelementptr inbounds [10 x i32], [10 x i32]* %8, i64 0, i64 7
  store volatile i32 3, i32* %9, align 4
  %10 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %11 = getelementptr inbounds [10 x i32], [10 x i32]* %10, i64 0, i64 4
  %12 = load volatile i32, i32* %11, align 4
  %13 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %14 = getelementptr inbounds [10 x i32], [10 x i32]* %13, i64 0, i64 7
  %15 = load volatile i32, i32* %14, align 4
  %16 = sub nsw i32 %12, %15
  ret i32 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @with_array_all(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.WithArray, align 4
  store i32 %0, i32* %2, align 4
  %4 = bitcast %struct.WithArray* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %4, i8 0, i64 64, i1 true)
  %5 = load i32, i32* %2, align 4
  %6 = sub nsw i32 %5, 4
  %7 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %8 = getelementptr inbounds [10 x i32], [10 x i32]* %7, i64 0, i64 2
  store volatile i32 %6, i32* %8, align 4
  %9 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %10 = getelementptr inbounds [10 x i32], [10 x i32]* %9, i64 0, i64 5
  %11 = load volatile i32, i32* %10, align 4
  %12 = sub nsw i32 %11, 3
  %13 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %14 = getelementptr inbounds [10 x i32], [10 x i32]* %13, i64 0, i64 4
  store volatile i32 %12, i32* %14, align 4
  %15 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %16 = getelementptr inbounds [10 x i32], [10 x i32]* %15, i64 0, i64 2
  %17 = load volatile i32, i32* %16, align 4
  %18 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 0
  %19 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %18, i32 0, i32 1
  store volatile i32 %17, i32* %19, align 4
  %20 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %21 = getelementptr inbounds [10 x i32], [10 x i32]* %20, i64 0, i64 2
  %22 = load volatile i32, i32* %21, align 4
  %23 = load i32, i32* %2, align 4
  %24 = add nsw i32 %22, %23
  %25 = add nsw i32 %24, 1
  %26 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 2
  %27 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %26, i32 0, i32 1
  store volatile i32 %25, i32* %27, align 4
  %28 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %29 = getelementptr inbounds [10 x i32], [10 x i32]* %28, i64 0, i64 4
  %30 = load volatile i32, i32* %29, align 4
  %31 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 2
  %32 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %31, i32 0, i32 1
  %33 = load volatile i32, i32* %32, align 4
  %34 = add i32 %30, %33
  ret i32 %34
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @structptr(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.Mismatched, align 4
  %4 = alloca %struct.Mismatched*, align 8
  store i32 %0, i32* %2, align 4
  %5 = bitcast %struct.Mismatched* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %5, i8 0, i64 12, i1 true)
  store %struct.Mismatched* %3, %struct.Mismatched** %4, align 8
  %6 = load i32, i32* %2, align 4
  %7 = add nsw i32 %6, 4
  %8 = load %struct.Mismatched*, %struct.Mismatched** %4, align 8
  %9 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %8, i32 0, i32 1
  store volatile i32 %7, i32* %9, align 4
  %10 = load %struct.Mismatched*, %struct.Mismatched** %4, align 8
  %11 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %10, i32 0, i32 2
  %12 = load volatile i8, i8* %11, align 4
  %13 = zext i8 %12 to i32
  %14 = load i32, i32* %2, align 4
  %15 = add nsw i32 %13, %14
  %16 = trunc i32 %15 to i8
  %17 = load %struct.Mismatched*, %struct.Mismatched** %4, align 8
  %18 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %17, i32 0, i32 0
  store volatile i8 %16, i8* %18, align 4
  %19 = load %struct.Mismatched*, %struct.Mismatched** %4, align 8
  %20 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %19, i32 0, i32 1
  %21 = load volatile i32, i32* %20, align 4
  %22 = load %struct.Mismatched*, %struct.Mismatched** %4, align 8
  %23 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %22, i32 0, i32 0
  %24 = load volatile i8, i8* %23, align 4
  %25 = zext i8 %24 to i32
  %26 = add i32 %21, %25
  ret i32 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @ptrs(i32) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.WithArray, align 4
  %4 = alloca %struct.WithArray, align 4
  %5 = alloca %struct.WithArray*, align 8
  %6 = alloca i32*, align 8
  %7 = alloca i32*, align 8
  store i32 %0, i32* %2, align 4
  %8 = bitcast %struct.WithArray* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %8, i8 0, i64 64, i1 true)
  %9 = bitcast %struct.WithArray* %4 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %9, i8 0, i64 64, i1 true)
  store %struct.WithArray* %3, %struct.WithArray** %5, align 8
  %10 = load i32, i32* %2, align 4
  %11 = add nsw i32 %10, 4
  %12 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %13 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %12, i32 0, i32 1
  %14 = getelementptr inbounds [10 x i32], [10 x i32]* %13, i64 0, i64 3
  store volatile i32 %11, i32* %14, align 4
  store %struct.WithArray* %4, %struct.WithArray** %5, align 8
  %15 = load i32, i32* %2, align 4
  %16 = add nsw i32 %15, 7
  %17 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %18 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %17, i32 0, i32 1
  %19 = getelementptr inbounds [10 x i32], [10 x i32]* %18, i64 0, i64 4
  store volatile i32 %16, i32* %19, align 4
  %20 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 0
  %21 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %20, i32 0, i32 1
  %22 = load volatile i32, i32* %21, align 4
  %23 = add i32 %22, 3
  %24 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %25 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %24, i32 0, i32 2
  %26 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %25, i32 0, i32 1
  store volatile i32 %23, i32* %26, align 4
  %27 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %28 = getelementptr inbounds [10 x i32], [10 x i32]* %27, i64 0, i64 0
  store i32* %28, i32** %6, align 8
  %29 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %30 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %29, i32 0, i32 1
  %31 = getelementptr inbounds [10 x i32], [10 x i32]* %30, i64 0, i64 4
  %32 = load volatile i32, i32* %31, align 4
  %33 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %34 = getelementptr inbounds [10 x i32], [10 x i32]* %33, i64 0, i64 3
  %35 = load volatile i32, i32* %34, align 4
  %36 = add nsw i32 %32, %35
  %37 = load i32*, i32** %6, align 8
  %38 = getelementptr inbounds i32, i32* %37, i64 7
  store volatile i32 %36, i32* %38, align 4
  %39 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %40 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %39, i32 0, i32 1
  %41 = getelementptr inbounds [10 x i32], [10 x i32]* %40, i64 0, i64 0
  store i32* %41, i32** %7, align 8
  %42 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %43 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %42, i32 0, i32 1
  %44 = getelementptr inbounds [10 x i32], [10 x i32]* %43, i64 0, i64 7
  %45 = load volatile i32, i32* %44, align 4
  %46 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %4, i32 0, i32 2
  %47 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %46, i32 0, i32 1
  %48 = load volatile i32, i32* %47, align 4
  %49 = sub i32 %45, %48
  %50 = load i32*, i32** %7, align 8
  %51 = getelementptr inbounds i32, i32* %50, i64 1
  store volatile i32 %49, i32* %51, align 4
  %52 = load i32*, i32** %6, align 8
  store i32* %52, i32** %7, align 8
  %53 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 0
  %54 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %53, i32 0, i32 1
  %55 = load volatile i32, i32* %54, align 4
  %56 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %57 = getelementptr inbounds [10 x i32], [10 x i32]* %56, i64 0, i64 3
  %58 = load volatile i32, i32* %57, align 4
  %59 = add i32 %55, %58
  %60 = load i32*, i32** %7, align 8
  %61 = getelementptr inbounds i32, i32* %60, i64 5
  store volatile i32 %59, i32* %61, align 4
  %62 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %63 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %62, i32 0, i32 2
  %64 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %63, i32 0, i32 1
  %65 = load volatile i32, i32* %64, align 4
  %66 = add i32 %65, 3
  %67 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %4, i32 0, i32 0
  %68 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %67, i32 0, i32 1
  store volatile i32 %66, i32* %68, align 4
  %69 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %4, i32 0, i32 0
  %70 = getelementptr inbounds %struct.Mismatched, %struct.Mismatched* %69, i32 0, i32 1
  %71 = load volatile i32, i32* %70, align 4
  %72 = load %struct.WithArray*, %struct.WithArray** %5, align 8
  %73 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %72, i32 0, i32 1
  %74 = getelementptr inbounds [10 x i32], [10 x i32]* %73, i64 0, i64 1
  %75 = load volatile i32, i32* %74, align 4
  %76 = add i32 %71, %75
  %77 = load i32*, i32** %7, align 8
  %78 = getelementptr inbounds i32, i32* %77, i64 5
  %79 = load volatile i32, i32* %78, align 4
  %80 = add i32 %76, %79
  %81 = getelementptr inbounds %struct.WithArray, %struct.WithArray* %3, i32 0, i32 1
  %82 = getelementptr inbounds [10 x i32], [10 x i32]* %81, i64 0, i64 5
  %83 = load volatile i32, i32* %82, align 4
  %84 = add i32 %80, %83
  ret i32 %84
}

attributes #0 = { noinline nounwind optnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 8.0.0 (tags/RELEASE_800/final)"}
