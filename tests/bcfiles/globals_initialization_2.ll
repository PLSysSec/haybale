; ModuleID = 'globals_initialization_2.c'
source_filename = "globals_initialization_2.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.14.0"

%struct.SomeStruct = type { i32, i32, i32 }
%struct.StructWithPointers = type { i32, i32*, %struct.SomeStruct*, %struct.StructWithPointers* }
%struct.StructWithFunctionPointer = type { i32, i32 (i32, i32)*, i8* }

@x = local_unnamed_addr constant i32 511, align 4
@ss2 = constant %struct.SomeStruct { i32 1533, i32 515, i32 1 }, align 4
@c = external constant i32, align 4
@swp0 = external constant %struct.StructWithPointers, align 8
@swp2 = local_unnamed_addr constant %struct.StructWithPointers { i32 511, i32* @c, %struct.SomeStruct* @ss2, %struct.StructWithPointers* @swp0 }, align 8
@swp1 = external constant %struct.StructWithPointers, align 8
@swp3 = local_unnamed_addr constant %struct.StructWithPointers { i32 509, i32* getelementptr inbounds (%struct.StructWithPointers, %struct.StructWithPointers* @swp0, i32 0, i32 0), %struct.SomeStruct* @ss2, %struct.StructWithPointers* @swp1 }, align 8
@crossMod0 = external constant %struct.StructWithPointers, align 8
@ss0 = external constant %struct.SomeStruct, align 4
@crossMod1 = local_unnamed_addr constant %struct.StructWithPointers { i32 2, i32* getelementptr inbounds (%struct.StructWithPointers, %struct.StructWithPointers* @crossMod0, i32 0, i32 0), %struct.SomeStruct* @ss0, %struct.StructWithPointers* @crossMod0 }, align 8
@swfp2 = local_unnamed_addr constant %struct.StructWithFunctionPointer { i32 21, i32 (i32, i32)* @bar, i8* bitcast (i32 (...)* @foo to i8*) }, align 8

declare i32 @bar(i32, i32) #0

declare i32 @foo(...) #0

attributes #0 = { "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.0 (tags/RELEASE_900/final)"}
