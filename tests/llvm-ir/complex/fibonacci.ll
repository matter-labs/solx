;! { "cases": [ {
;!     "name": "fibonacci",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "0x000000000000000000000000000000000000000000000000000000000000000A"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "0x0000000000000000000000000000000000000000000000000000000000000037"
;!     ]
;! } ] }


target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define void @test() noreturn {
bb:
  %offset = inttoptr i256 0 to ptr addrspace(2)
  %calldata = call i256 @llvm.evm.calldataload(ptr addrspace(2) %offset)
  %res = call i256 @fib(i256 noundef %calldata)
  store i256 %res, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

; Function Attrs: noinline
define i256 @fib(i256 noundef %arg) noinline {
bb:
  %i = icmp ult i256 %arg, 2
  br i1 %i, label %bb9, label %bb1

bb1:                                              ; preds = %bb1, %bb
  %i2 = phi i256 [ %i6, %bb1 ], [ %arg, %bb ]
  %i3 = phi i256 [ %i7, %bb1 ], [ 0, %bb ]
  %i4 = add i256 %i2, -1
  %i5 = call i256 @fib(i256 noundef %i4)
  %i6 = add i256 %i2, -2
  %i7 = add i256 %i5, %i3
  %i8 = icmp ult i256 %i6, 2
  br i1 %i8, label %bb9, label %bb1

bb9:                                              ; preds = %bb1, %bb
  %i10 = phi i256 [ 0, %bb ], [ %i7, %bb1 ]
  %i11 = phi i256 [ %arg, %bb ], [ %i6, %bb1 ]
  %i12 = add i256 %i11, %i10
  ret i256 %i12
}

declare void @llvm.evm.return(ptr addrspace(1), i256)
declare i256 @llvm.evm.calldataload(ptr addrspace(2))
