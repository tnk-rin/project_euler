; ModuleID = 'probe2.fa491717-cgu.0'
source_filename = "probe2.fa491717-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; probe2::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe25probe17heea75c687b4027cdE() unnamed_addr #0 {
start:
  %0 = alloca i32, align 4
  store i32 -2147483648, i32* %0, align 4
  %1 = load i32, i32* %0, align 4
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.bitreverse.i32(i32) #1

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
