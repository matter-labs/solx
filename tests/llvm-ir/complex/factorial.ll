;! { "cases": [ {
;!     "name": "fact1",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "0x000000000000000000000000000000000000000000000000000000000000000a"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "0x0000000000000000000000000000000000000000000000000000000000375f00"
;!     ]
;! },
;! {
;!     "name": "fact2",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "0x0000000000000000000000000000000000000000000000000000000000000001"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "0x0000000000000000000000000000000000000000000000000000000000000001"
;!     ]
;! } ] }


target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define i256 @factorial() {
entry:
  %offset = inttoptr i256 0 to ptr addrspace(2)
  %calldata = call i256 @llvm.evm.calldataload(ptr addrspace(2) %offset)
  %cmp = icmp sgt i256 %calldata, 0
  br i1 %cmp, label %loop, label %exit

loop:
  %phi = phi i256 [ %mul, %loop ], [ 1, %entry ]
  %phi2 = phi i256 [ %dec, %loop ], [ %calldata, %entry ]
  %mul = mul nsw i256 %phi, %phi2
  %dec = add nsw i256 %phi2, -1
  %cmp2 = icmp ugt i256 %phi2, 1
  br i1 %cmp2, label %loop, label %exit

exit:
  %phi3 = phi i256 [ 1, %entry ], [ %mul, %loop ]
  store i256 %phi3, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

declare void @llvm.evm.return(ptr addrspace(1), i256)
declare i256 @llvm.evm.calldataload(ptr addrspace(2))
