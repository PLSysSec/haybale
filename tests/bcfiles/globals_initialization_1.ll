; ModuleID = 'globals_initialization_1.c'
source_filename = "globals_initialization_1.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

%struct.SomeStruct = type { i32, i32, i32 }
%struct.StructWithPointers = type { i32, i32*, %struct.SomeStruct*, %struct.StructWithPointers* }
%struct.StructWithFunctionPointer = type { i32, i32 (i32, i32)*, i8* }

@a = local_unnamed_addr constant i32 2, align 4
@b = local_unnamed_addr constant i32 2, align 4
@c = local_unnamed_addr constant i32 6, align 4
@ss0 = local_unnamed_addr constant %struct.SomeStruct zeroinitializer, align 4
@ss1 = constant %struct.SomeStruct { i32 2, i32 6, i32 2 }, align 4
@x = external constant i32, align 4
@swp1 = constant %struct.StructWithPointers { i32 6, i32* getelementptr inbounds (%struct.StructWithPointers, %struct.StructWithPointers* @swp0, i32 0, i32 0), %struct.SomeStruct* @ss2, %struct.StructWithPointers* @swp0 }, align 8
@swp0 = constant %struct.StructWithPointers { i32 2, i32* @x, %struct.SomeStruct* @ss1, %struct.StructWithPointers* @swp1 }, align 8
@ss2 = external constant %struct.SomeStruct, align 4
@crossMod1 = external constant %struct.StructWithPointers, align 8
@crossMod0 = local_unnamed_addr constant %struct.StructWithPointers { i32 2, i32* getelementptr inbounds (%struct.StructWithPointers, %struct.StructWithPointers* @crossMod1, i32 0, i32 0), %struct.SomeStruct* @ss1, %struct.StructWithPointers* @crossMod1 }, align 8
@swfp1 = local_unnamed_addr constant %struct.StructWithFunctionPointer { i32 21, i32 (i32, i32)* @bar, i8* bitcast (i32 ()* @foo to i8*) }, align 8

; Function Attrs: norecurse nounwind readnone ssp uwtable
define i32 @bar(i32, i32) #0 {
  %3 = add nsw i32 %1, %0
  ret i32 %3
}

; Function Attrs: norecurse nounwind readonly ssp uwtable
define i32 @foo() #1 {
  %1 = load i32, i32* getelementptr inbounds (%struct.SomeStruct, %struct.SomeStruct* @ss2, i64 0, i32 2), align 4, !tbaa !3
  %2 = load i32, i32* @x, align 4, !tbaa !8
  %3 = load i32, i32* getelementptr inbounds (%struct.SomeStruct, %struct.SomeStruct* @ss2, i64 0, i32 1), align 4, !tbaa !9
  %4 = load %struct.StructWithPointers*, %struct.StructWithPointers** getelementptr inbounds (%struct.StructWithPointers, %struct.StructWithPointers* @crossMod1, i64 0, i32 3), align 8, !tbaa !10
  %5 = getelementptr inbounds %struct.StructWithPointers, %struct.StructWithPointers* %4, i64 0, i32 1
  %6 = load i32*, i32** %5, align 8, !tbaa !13
  %7 = load i32, i32* %6, align 4, !tbaa !8
  %8 = add i32 %1, 23
  %9 = add i32 %8, %2
  %10 = add i32 %9, %3
  %11 = add i32 %10, %7
  ret i32 %11
}

attributes #0 = { norecurse nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { norecurse nounwind readonly ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
!3 = !{!4, !5, i64 8}
!4 = !{!"SomeStruct", !5, i64 0, !5, i64 4, !5, i64 8}
!5 = !{!"int", !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
!8 = !{!5, !5, i64 0}
!9 = !{!4, !5, i64 4}
!10 = !{!11, !12, i64 24}
!11 = !{!"StructWithPointers", !5, i64 0, !12, i64 8, !12, i64 16, !12, i64 24}
!12 = !{!"any pointer", !6, i64 0}
!13 = !{!11, !12, i64 8}
