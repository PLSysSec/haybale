; ModuleID = 'globals.c'
source_filename = "globals.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.15.0"

@global1 = global i32 3, align 4
@global2 = global i32 5, align 4
@global3 = common global i32 0, align 4

; Function Attrs: nofree noinline norecurse nounwind ssp uwtable
define i32 @read_global() local_unnamed_addr #0 {
  %1 = load volatile i32, i32* @global1, align 4, !tbaa !3
  ret i32 %1
}

; Function Attrs: nofree noinline norecurse nounwind ssp uwtable
define i32 @modify_global(i32) local_unnamed_addr #0 {
  store volatile i32 %0, i32* @global3, align 4, !tbaa !3
  %2 = load volatile i32, i32* @global3, align 4, !tbaa !3
  ret i32 %2
}

; Function Attrs: nofree noinline norecurse nounwind ssp uwtable
define i32 @modify_global_with_call(i32) local_unnamed_addr #0 {
  %2 = tail call i32 @modify_global(i32 %0)
  %3 = load volatile i32, i32* @global3, align 4, !tbaa !3
  ret i32 %3
}

; Function Attrs: nofree norecurse nounwind ssp uwtable
define i32 @dont_confuse_globals(i32) local_unnamed_addr #1 {
  store volatile i32 100, i32* @global1, align 4, !tbaa !3
  store volatile i32 95, i32* @global2, align 4, !tbaa !3
  store volatile i32 %0, i32* @global3, align 4, !tbaa !3
  %2 = load volatile i32, i32* @global2, align 4, !tbaa !3
  %3 = add nsw i32 %2, -200
  store volatile i32 %3, i32* @global1, align 4, !tbaa !3
  %4 = load volatile i32, i32* @global3, align 4, !tbaa !3
  ret i32 %4
}

attributes #0 = { nofree noinline norecurse nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nofree norecurse nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
