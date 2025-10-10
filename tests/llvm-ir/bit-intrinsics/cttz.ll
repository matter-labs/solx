;! { "cases": [ {
;!     "name": "1",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                "0x0000000000000000000000000000000000000000000000000000000000000000"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!                "0x100"
;!     ]
;! }, {
;!     "name": "2",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!                "0x0"
;!     ]
;! }, {
;!     "name": "3",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                "0xFF00000000000000000000000000000000000000000000000000000000000000"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!                "0xf8"
;!     ]
;! } ] }


target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define i256 @__entry() noreturn {
runtime:
  %val = load i256, ptr addrspace(2) null, align 32
  %res = call i256 @llvm.cttz.i256(i256 %val, i1 false)
  store i256 %res, ptr addrspace(1) null, align 32
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

declare i256 @llvm.cttz.i256(i256, i1)
declare void @llvm.evm.return(ptr addrspace(1), i256)
