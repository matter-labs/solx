;! { "cases": [ {
;!     "name": "egcd",
;!     "inputs": [
;!         {
;!             "method": "#fallback",
;!             "calldata": [
;!                 "0x0000000000000000000000000000000000000000000000000000000000000168",
;!                 "0x00000000000000000000000000000000000000000000000000000000000000d2"
;!             ]
;!         }
;!     ],
;!     "expected": [
;!         "0x000000000000000000000000000000000000000000000000000000000000001e",
;!         "0x0000000000000000000000000000000000000000000000000000000000000003",
;!         "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb"
;!     ]
;! } ] }


target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define void @egcd() noreturn {
bb:
  %addr_arg = inttoptr i256 0 to ptr addrspace(2)
  %arg = call i256 @llvm.evm.calldataload(ptr addrspace(2) %addr_arg)
  %addr_arg1 = inttoptr i256 32 to ptr addrspace(2)
  %arg1 = call i256 @llvm.evm.calldataload(ptr addrspace(2) %addr_arg1)
  %addr1 = inttoptr i256 32 to ptr addrspace(1)
  store i256 0, ptr addrspace(1) %addr1, align 4
  %addr2 = inttoptr i256 64 to ptr addrspace(1)
  store i256 0, ptr addrspace(1) %addr2, align 4
  %i = icmp eq i256 %arg1, 0
  br i1 %i, label %bb20, label %bb4

bb4:                                              ; preds = %bb4, %bb
  %i5 = phi i256 [ %i9, %bb4 ], [ 1, %bb ]
  %i6 = phi i256 [ %i7, %bb4 ], [ %arg, %bb ]
  %i7 = phi i256 [ %i13, %bb4 ], [ %arg1, %bb ]
  %i8 = phi i256 [ %i17, %bb4 ], [ 1, %bb ]
  %i9 = phi i256 [ %i15, %bb4 ], [ 0, %bb ]
  %i10 = phi i256 [ %i8, %bb4 ], [ 0, %bb ]
  %i11 = sdiv i256 %i6, %i7
  %i12 = mul nsw i256 %i11, %i7
  %i13 = sub nsw i256 %i6, %i12
  %i14 = mul nsw i256 %i11, %i9
  %i15 = sub nsw i256 %i5, %i14
  %i16 = mul nsw i256 %i11, %i8
  %i17 = sub nsw i256 %i10, %i16
  %i18 = icmp eq i256 %i13, 0
  br i1 %i18, label %bb19, label %bb4

bb19:                                             ; preds = %bb4
  store i256 %i9, ptr addrspace(1) %addr1, align 4
  store i256 %i8, ptr addrspace(1) %addr2, align 4
  br label %bb20

bb20:                                             ; preds = %bb19, %bb
  %i21 = phi i256 [ %i7, %bb19 ], [ %arg, %bb ]
  store i256 %i21, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 96)
  unreachable
}

declare void @llvm.evm.return(ptr addrspace(1), i256)
declare i256 @llvm.evm.calldataload(ptr addrspace(2))
