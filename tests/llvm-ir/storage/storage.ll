;! { "cases": [ {
;!     "name": "gcd",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "0",
;!                 "0xDE"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "0xDE"
;!     ]
;! } ] }


target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define i32 @gcd() noreturn {
entry:
  %off = inttoptr i256 0 to ptr addrspace(2)
  %off2 = inttoptr i256 32 to ptr addrspace(2)
  %key = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off)
  %val = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off2)
  %key_ptr = inttoptr i256 %key to ptr addrspace(5)
  store i256 %val, ptr addrspace(5) %key_ptr, align 32
  store i256 255, ptr addrspace(1) null, align 32
  %load_val = load i256, ptr addrspace(5) %key_ptr, align 32
  store i256 %load_val, ptr addrspace(1) null, align 32
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

declare i256 @llvm.evm.calldataload(ptr addrspace(2))
declare void @llvm.evm.return(ptr addrspace(1), i256)
