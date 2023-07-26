; ModuleID = 'probe5.bacfabe5f65090c8-cgu.0'
source_filename = "probe5.bacfabe5f65090c8-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17ha3c6c9cfcbd08eabE"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca i8, align 1
; call core::f64::<impl f64>::classify
  %0 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17h2e0619aec5c22df0E"(double %self) #2, !range !1
  store i8 %0, ptr %_2, align 1
  %1 = load i8, ptr %_2, align 1, !range !1, !noundef !2
  %_3 = zext i8 %1 to i32
  %_0 = icmp eq i32 %_3, 3
  ret i1 %_0
}

; probe5::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe55probe17hbb8c1bb77ec88d6cE() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17ha3c6c9cfcbd08eabE"(double 1.000000e+00) #2
  ret void
}

; core::f64::<impl f64>::classify
; Function Attrs: nounwind
declare dso_local i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17h2e0619aec5c22df0E"(double) unnamed_addr #1

attributes #0 = { inlinehint nounwind "target-cpu"="mvp" }
attributes #1 = { nounwind "target-cpu"="mvp" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.73.0-nightly (8771282d4 2023-07-23)"}
!1 = !{i8 0, i8 5}
!2 = !{}
