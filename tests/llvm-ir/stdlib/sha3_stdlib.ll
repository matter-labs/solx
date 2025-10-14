;! { "cases": [ {
;!     "name": "egcd",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "304594385234",
;!                 "56457598675863654"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "-53675409633959416604748946233496653964072736789863655143901645101595015023086"
;!     ]
;! } ] }


target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define void @test() noreturn {
bb:
  %off1 = inttoptr i256 0 to ptr addrspace(2)
  %off2 = inttoptr i256 32 to ptr addrspace(2)
  %arg1 = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off1)
  %arg2 = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off2)
  store i256 %arg1, ptr addrspace(1) null, align 32
  store i256 %arg2, ptr addrspace(1) inttoptr (i256 32 to ptr addrspace(1)), align 32
  %res = tail call i256 @__sha3(ptr addrspace(1) null, i256 64, i1 true)
  store i256 %res, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

declare i256 @__sha3(ptr addrspace(1), i256, i1) #0
declare void @llvm.evm.return(ptr addrspace(1), i256)
declare i256 @llvm.evm.calldataload(ptr addrspace(2))

attributes #0 = { alwaysinline mustprogress nofree null_pointer_is_valid norecurse nosync nounwind readnone willreturn }
