; ModuleID = 'multithrow.cpp'
source_filename = "multithrow.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@_ZTIi = external constant i8*

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z14throw_multiplei(i32 %0) #0 !dbg !101 {
  %2 = alloca i32, align 4
  store volatile i32 %0, i32* %2, align 4
  call void @llvm.dbg.declare(metadata i32* %2, metadata !105, metadata !DIExpression()), !dbg !106
  %3 = load volatile i32, i32* %2, align 4, !dbg !107
  %4 = icmp sgt i32 %3, 15, !dbg !109
  br i1 %4, label %5, label %8, !dbg !110

5:                                                ; preds = %1
  %6 = call i8* @__cxa_allocate_exception(i64 4) #2, !dbg !111
  %7 = bitcast i8* %6 to i32*, !dbg !111
  store i32 10, i32* %7, align 16, !dbg !111
  call void @__cxa_throw(i8* %6, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #3, !dbg !111
  unreachable, !dbg !111

8:                                                ; preds = %1
  %9 = load volatile i32, i32* %2, align 4, !dbg !113
  %10 = icmp sgt i32 %9, 10, !dbg !115
  br i1 %10, label %11, label %14, !dbg !116

11:                                               ; preds = %8
  %12 = call i8* @__cxa_allocate_exception(i64 4) #2, !dbg !117
  %13 = bitcast i8* %12 to i32*, !dbg !117
  store i32 10, i32* %13, align 16, !dbg !117
  call void @__cxa_throw(i8* %12, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #3, !dbg !117
  unreachable, !dbg !117

14:                                               ; preds = %8
  %15 = load volatile i32, i32* %2, align 4, !dbg !119
  %16 = icmp sgt i32 %15, 2, !dbg !121
  br i1 %16, label %17, label %20, !dbg !122

17:                                               ; preds = %14
  %18 = call i8* @__cxa_allocate_exception(i64 4) #2, !dbg !123
  %19 = bitcast i8* %18 to i32*, !dbg !123
  store i32 2, i32* %19, align 16, !dbg !123
  call void @__cxa_throw(i8* %18, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #3, !dbg !123
  unreachable, !dbg !123

20:                                               ; preds = %14
  ret i32 0, !dbg !125
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

declare i8* @__cxa_allocate_exception(i64)

declare void @__cxa_throw(i8*, i8*, i8*)

attributes #0 = { mustprogress noinline optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nounwind }
attributes #3 = { noreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!93, !94, !95, !96, !97, !98, !99}
!llvm.ident = !{!100}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !1, producer: "clang version 13.0.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2, retainedTypes: !3, imports: !9, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "multithrow.cpp", directory: "/home/notroot/smbshare/CS530/haybale/tests/bcfiles")
!2 = !{}
!3 = !{!4}
!4 = !DIDerivedType(tag: DW_TAG_typedef, name: "int32_t", file: !5, line: 26, baseType: !6)
!5 = !DIFile(filename: "/usr/include/bits/stdint-intn.h", directory: "")
!6 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int32_t", file: !7, line: 41, baseType: !8)
!7 = !DIFile(filename: "/usr/include/bits/types.h", directory: "")
!8 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!9 = !{!10, !16, !20, !21, !25, !28, !30, !32, !34, !37, !40, !43, !46, !49, !51, !56, !60, !64, !68, !70, !72, !74, !76, !79, !82, !85, !88, !91}
!10 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !12, file: !15, line: 47)
!11 = !DINamespace(name: "std", scope: null)
!12 = !DIDerivedType(tag: DW_TAG_typedef, name: "int8_t", file: !5, line: 24, baseType: !13)
!13 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int8_t", file: !7, line: 37, baseType: !14)
!14 = !DIBasicType(name: "signed char", size: 8, encoding: DW_ATE_signed_char)
!15 = !DIFile(filename: "/usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/11.2.0/../../../../include/c++/11.2.0/cstdint", directory: "")
!16 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !17, file: !15, line: 48)
!17 = !DIDerivedType(tag: DW_TAG_typedef, name: "int16_t", file: !5, line: 25, baseType: !18)
!18 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int16_t", file: !7, line: 39, baseType: !19)
!19 = !DIBasicType(name: "short", size: 16, encoding: DW_ATE_signed)
!20 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !4, file: !15, line: 49)
!21 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !22, file: !15, line: 50)
!22 = !DIDerivedType(tag: DW_TAG_typedef, name: "int64_t", file: !5, line: 27, baseType: !23)
!23 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int64_t", file: !7, line: 44, baseType: !24)
!24 = !DIBasicType(name: "long int", size: 64, encoding: DW_ATE_signed)
!25 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !26, file: !15, line: 52)
!26 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast8_t", file: !27, line: 58, baseType: !14)
!27 = !DIFile(filename: "/usr/include/stdint.h", directory: "")
!28 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !29, file: !15, line: 53)
!29 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast16_t", file: !27, line: 60, baseType: !24)
!30 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !31, file: !15, line: 54)
!31 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast32_t", file: !27, line: 61, baseType: !24)
!32 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !33, file: !15, line: 55)
!33 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast64_t", file: !27, line: 62, baseType: !24)
!34 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !35, file: !15, line: 57)
!35 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least8_t", file: !27, line: 43, baseType: !36)
!36 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least8_t", file: !7, line: 52, baseType: !13)
!37 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !38, file: !15, line: 58)
!38 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least16_t", file: !27, line: 44, baseType: !39)
!39 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least16_t", file: !7, line: 54, baseType: !18)
!40 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !41, file: !15, line: 59)
!41 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least32_t", file: !27, line: 45, baseType: !42)
!42 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least32_t", file: !7, line: 56, baseType: !6)
!43 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !44, file: !15, line: 60)
!44 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least64_t", file: !27, line: 46, baseType: !45)
!45 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least64_t", file: !7, line: 58, baseType: !23)
!46 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !47, file: !15, line: 62)
!47 = !DIDerivedType(tag: DW_TAG_typedef, name: "intmax_t", file: !27, line: 101, baseType: !48)
!48 = !DIDerivedType(tag: DW_TAG_typedef, name: "__intmax_t", file: !7, line: 72, baseType: !24)
!49 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !50, file: !15, line: 63)
!50 = !DIDerivedType(tag: DW_TAG_typedef, name: "intptr_t", file: !27, line: 87, baseType: !24)
!51 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !52, file: !15, line: 65)
!52 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint8_t", file: !53, line: 24, baseType: !54)
!53 = !DIFile(filename: "/usr/include/bits/stdint-uintn.h", directory: "")
!54 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint8_t", file: !7, line: 38, baseType: !55)
!55 = !DIBasicType(name: "unsigned char", size: 8, encoding: DW_ATE_unsigned_char)
!56 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !57, file: !15, line: 66)
!57 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint16_t", file: !53, line: 25, baseType: !58)
!58 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint16_t", file: !7, line: 40, baseType: !59)
!59 = !DIBasicType(name: "unsigned short", size: 16, encoding: DW_ATE_unsigned)
!60 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !61, file: !15, line: 67)
!61 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint32_t", file: !53, line: 26, baseType: !62)
!62 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint32_t", file: !7, line: 42, baseType: !63)
!63 = !DIBasicType(name: "unsigned int", size: 32, encoding: DW_ATE_unsigned)
!64 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !65, file: !15, line: 68)
!65 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint64_t", file: !53, line: 27, baseType: !66)
!66 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint64_t", file: !7, line: 45, baseType: !67)
!67 = !DIBasicType(name: "long unsigned int", size: 64, encoding: DW_ATE_unsigned)
!68 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !69, file: !15, line: 70)
!69 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast8_t", file: !27, line: 71, baseType: !55)
!70 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !71, file: !15, line: 71)
!71 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast16_t", file: !27, line: 73, baseType: !67)
!72 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !73, file: !15, line: 72)
!73 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast32_t", file: !27, line: 74, baseType: !67)
!74 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !75, file: !15, line: 73)
!75 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast64_t", file: !27, line: 75, baseType: !67)
!76 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !77, file: !15, line: 75)
!77 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least8_t", file: !27, line: 49, baseType: !78)
!78 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least8_t", file: !7, line: 53, baseType: !54)
!79 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !80, file: !15, line: 76)
!80 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least16_t", file: !27, line: 50, baseType: !81)
!81 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least16_t", file: !7, line: 55, baseType: !58)
!82 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !83, file: !15, line: 77)
!83 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least32_t", file: !27, line: 51, baseType: !84)
!84 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least32_t", file: !7, line: 57, baseType: !62)
!85 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !86, file: !15, line: 78)
!86 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least64_t", file: !27, line: 52, baseType: !87)
!87 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least64_t", file: !7, line: 59, baseType: !66)
!88 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !89, file: !15, line: 80)
!89 = !DIDerivedType(tag: DW_TAG_typedef, name: "uintmax_t", file: !27, line: 102, baseType: !90)
!90 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uintmax_t", file: !7, line: 73, baseType: !67)
!91 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !92, file: !15, line: 81)
!92 = !DIDerivedType(tag: DW_TAG_typedef, name: "uintptr_t", file: !27, line: 90, baseType: !67)
!93 = !{i32 7, !"Dwarf Version", i32 4}
!94 = !{i32 2, !"Debug Info Version", i32 3}
!95 = !{i32 1, !"wchar_size", i32 4}
!96 = !{i32 7, !"PIC Level", i32 2}
!97 = !{i32 7, !"PIE Level", i32 2}
!98 = !{i32 7, !"uwtable", i32 1}
!99 = !{i32 7, !"frame-pointer", i32 2}
!100 = !{!"clang version 13.0.1"}
!101 = distinct !DISubprogram(name: "throw_multiple", linkageName: "_Z14throw_multiplei", scope: !1, file: !1, line: 4, type: !102, scopeLine: 4, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!102 = !DISubroutineType(types: !103)
!103 = !{!8, !104}
!104 = !DIDerivedType(tag: DW_TAG_volatile_type, baseType: !8)
!105 = !DILocalVariable(name: "a", arg: 1, scope: !101, file: !1, line: 4, type: !104)
!106 = !DILocation(line: 4, column: 33, scope: !101)
!107 = !DILocation(line: 5, column: 9, scope: !108)
!108 = distinct !DILexicalBlock(scope: !101, file: !1, line: 5, column: 9)
!109 = !DILocation(line: 5, column: 11, scope: !108)
!110 = !DILocation(line: 5, column: 9, scope: !101)
!111 = !DILocation(line: 6, column: 9, scope: !112)
!112 = distinct !DILexicalBlock(scope: !108, file: !1, line: 5, column: 17)
!113 = !DILocation(line: 7, column: 16, scope: !114)
!114 = distinct !DILexicalBlock(scope: !108, file: !1, line: 7, column: 16)
!115 = !DILocation(line: 7, column: 18, scope: !114)
!116 = !DILocation(line: 7, column: 16, scope: !108)
!117 = !DILocation(line: 8, column: 9, scope: !118)
!118 = distinct !DILexicalBlock(scope: !114, file: !1, line: 7, column: 24)
!119 = !DILocation(line: 9, column: 16, scope: !120)
!120 = distinct !DILexicalBlock(scope: !114, file: !1, line: 9, column: 16)
!121 = !DILocation(line: 9, column: 18, scope: !120)
!122 = !DILocation(line: 9, column: 16, scope: !114)
!123 = !DILocation(line: 10, column: 9, scope: !124)
!124 = distinct !DILexicalBlock(scope: !120, file: !1, line: 9, column: 23)
!125 = !DILocation(line: 12, column: 7, scope: !126)
!126 = distinct !DILexicalBlock(scope: !120, file: !1, line: 11, column: 12)
