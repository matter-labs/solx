target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

define i32 @gcd() noreturn {
entry:
  %off = inttoptr i256 0 to ptr addrspace(2)
  %off2 = inttoptr i256 32 to ptr addrspace(2)
  %load = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off)
  %load2 = call i256 @llvm.evm.calldataload(ptr addrspace(2) %off2)
  %a = trunc i256 %load to i32
  %b = trunc i256 %load2 to i32
  %cmp_eq0 = icmp eq i32 %a, %b
  br i1 %cmp_eq0, label %exit, label %body

body:                                                ; preds = %body, %body
  %phi1 = phi i32 [ %sub2, %body ], [ %b, %entry ]
  %phi2 = phi i32 [ %sub1, %body ], [ %a, %entry ]
  %cmp_slt = icmp slt i32 %phi1, %phi2
  %sel = select i1 %cmp_slt, i32 %phi1, i32 0
  %sub1 = sub nsw i32 %phi2, %sel
  %sel2 = select i1 %cmp_slt, i32 0, i32 %phi2
  %sub2 = sub nsw i32 %phi1, %sel2
  %cmp_eq = icmp eq i32 %sub1, %sub2
  br i1 %cmp_eq, label %exit, label %body

exit:                                               ; preds = %body, %body
  %phi3 = phi i32 [ %a, %entry ], [ %sub1, %body ]
  %res = zext i32 %phi3 to i256
  store i256 %res, ptr addrspace(1) null, align 4
  call void @llvm.evm.return(ptr addrspace(1) null, i256 32)
  unreachable
}

declare i256 @llvm.evm.calldataload(ptr addrspace(2))
declare void @llvm.evm.return(ptr addrspace(1), i256)