;! { "cases": [ {
;!     "name": "test1",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "0x0000000000000000000000000000000000000000000000000000000000000007",
;!                 "0x0000000000000000000000000000000000000000000000000000000000000003"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "0x000000000000000000000000000000000000000000000000000000000000000a"
;!     ]
;! } ] }


define void @tworet_test() noreturn {
  %res = call {i256, i256} @tworet()
  %x1 = extractvalue {i256, i256} %res, 0
  %x2 = extractvalue {i256, i256} %res, 1
  %sum = add i256 %x1, %x2
  store i256 %sum, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

define {i256, i256} @tworet() noinline {
  %off1 = inttoptr i256 0 to ptr addrspace(2)
  %off2 = inttoptr i256 32 to ptr addrspace(2)
  %arg1 = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off1)
  %arg2 = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off2)
  %res.t = insertvalue {i256, i256} undef, i256 %arg1, 0
  %res = insertvalue {i256, i256} %res.t, i256 %arg2, 1
  ret {i256, i256} %res
}

declare void @llvm.evm.return(ptr addrspace(1), i256)
declare i256 @llvm.evm.calldataload(ptr addrspace(2))
