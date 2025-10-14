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

define void @factorial() noreturn {
enter:
  %offset = inttoptr i256 0 to ptr addrspace(2)
  %load = call i256 @llvm.evm.calldataload(ptr addrspace(2) %offset)
  %calldata = trunc i256 %load to i32
  br label %header

header:
  %phi = phi i32 [ %calldata, %enter ], [ %inc, %do ]
  %phi2 = phi i32 [ 1, %enter ], [ %mul, %do ]
  %cmp = icmp sgt i32 %phi, 0
  br i1 %cmp, label %do, label %exit

do:
  %mul = mul nsw i32 %phi2, %phi
  %inc = add nsw i32 %phi, -1
  br label %header

exit:
  %res = zext i32 %phi2 to i256
  store i256 %res, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

declare void @llvm.evm.return(ptr addrspace(1), i256)
declare i256 @llvm.evm.calldataload(ptr addrspace(2))
