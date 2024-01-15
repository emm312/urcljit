; ModuleID = 'urcl'
source_filename = "urcl"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

define i32 @main() {
entry:
  %const = bitcast i64 4342180336 to i64
  %r0 = alloca i32, align 4
  %r1 = alloca i32, align 4
  %r2 = alloca i32, align 4
  %r3 = alloca i32, align 4
  %r4 = alloca i32, align 4
  %r5 = alloca i32, align 4
  %r6 = alloca i32, align 4
  %r7 = alloca i32, align 4
  %r8 = alloca i32, align 4
  %r9 = alloca i32, align 4
  %r10 = alloca i32, align 4
  %r11 = alloca i32, align 4
  %r12 = alloca i32, align 4
  %r13 = alloca i32, align 4
  %r14 = alloca i32, align 4
  %r15 = alloca i32, align 4
  %r16 = alloca i32, align 4
  %r17 = alloca i32, align 4
  %r18 = alloca i32, align 4
  %r19 = alloca i32, align 4
  %r20 = alloca i32, align 4
  %r21 = alloca i32, align 4
  %r22 = alloca i32, align 4
  %r23 = alloca i32, align 4
  %r24 = alloca i32, align 4
  %r25 = alloca i32, align 4
  %r26 = alloca i32, align 4
  %r27 = alloca i32, align 4
  %r28 = alloca i32, align 4
  %r29 = alloca i32, align 4
  %r30 = alloca i32, align 4
  %r31 = alloca i32, align 4
  %sp = alloca i32, align 4
  store i32 1, ptr %r2, align 4
  store i32 1, ptr %r3, align 4
  %loadreg = load i32, ptr %r2, align 4
  %const_mat = add i64 %const, 556
  %0 = inttoptr i64 %const_mat to ptr
  call void %0(i32 %loadreg)
  %1 = inttoptr i64 %const to ptr
  call void %1(i32 10)
  %loadreg1 = load i32, ptr %r3, align 4
  %const_mat19 = add i64 %const, 556
  %2 = inttoptr i64 %const_mat19 to ptr
  call void %2(i32 %loadreg1)
  %3 = inttoptr i64 %const to ptr
  call void %3(i32 10)
  %loadreg2 = load i32, ptr %r2, align 4
  %loadreg3 = load i32, ptr %r3, align 4
  %add = add i32 %loadreg2, %loadreg3
  store i32 %add, ptr %r2, align 4
  %loadreg4 = load i32, ptr %r2, align 4
  %const_mat20 = add i64 %const, 556
  %4 = inttoptr i64 %const_mat20 to ptr
  call void %4(i32 %loadreg4)
  %5 = inttoptr i64 %const to ptr
  call void %5(i32 10)
  %loadreg5 = load i32, ptr %r2, align 4
  %loadreg6 = load i32, ptr %r3, align 4
  %6 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %loadreg5, i32 %loadreg6)
  %math = extractvalue { i32, i1 } %6, 0
  %ov = extractvalue { i32, i1 } %6, 1
  %cmp2 = icmp ult i32 %math, %loadreg6
  %carry = or i1 %ov, %cmp2
  br i1 %carry, label %.L11673974989645628637, label %continuation.preheader

continuation.preheader:                           ; preds = %entry
  br label %continuation

.L11673974989645628637:                           ; preds = %continuation, %entry
  ret void
  ret void

continuation:                                     ; preds = %continuation.preheader, %continuation
  %loadreg8 = load i32, ptr %r2, align 4
  %loadreg9 = load i32, ptr %r3, align 4
  %add10 = add i32 %loadreg8, %loadreg9
  store i32 %add10, ptr %r3, align 4
  %loadreg11 = load i32, ptr %r3, align 4
  %const_mat21 = add i64 %const, 556
  %7 = inttoptr i64 %const_mat21 to ptr
  call void %7(i32 %loadreg11)
  %8 = inttoptr i64 %const to ptr
  call void %8(i32 10)
  %loadreg12 = load i32, ptr %r2, align 4
  %loadreg13 = load i32, ptr %r3, align 4
  %add14 = add i32 %loadreg12, %loadreg13
  %cmp115 = icmp uge i32 %add14, %loadreg12
  %cmp216 = icmp uge i32 %add14, %loadreg13
  %carry17 = or i1 %cmp115, %cmp216
  br i1 %carry17, label %continuation, label %.L11673974989645628637
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #0

attributes #0 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
