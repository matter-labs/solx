; ModuleID = '/Users/hedgarmac/src/era-compiler-tester//tests/solidity/simple/default.sol:Test'
source_filename = "/Users/hedgarmac/src/era-compiler-tester//tests/solidity/simple/default.sol:Test"
target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.exp(i256, i256) #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.signextend(i256, i256) #0

; Function Attrs: nounwind willreturn memory(argmem: read)
declare i256 @llvm.evm.sha3(ptr addrspace(1), i256) #1

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.addmod(i256, i256, i256) #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.mulmod(i256, i256, i256) #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.byte(i256, i256) #0

; Function Attrs: nounwind memory(argmem: write)
declare void @llvm.evm.mstore8(ptr addrspace(1), i256) #2

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.msize() #3

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.calldatasize() #0

; Function Attrs: nounwind willreturn memory(inaccessiblemem: read)
declare i256 @llvm.evm.returndatasize() #4

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.codesize() #0

; Function Attrs: nounwind willreturn memory(inaccessiblemem: read)
declare i256 @llvm.evm.extcodesize(i256) #4

; Function Attrs: nounwind willreturn
declare void @llvm.evm.extcodecopy(i256, ptr addrspace(1), ptr addrspace(4), i256) #3

; Function Attrs: nounwind willreturn memory(inaccessiblemem: read)
declare i256 @llvm.evm.extcodehash(i256) #4

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i256 @llvm.evm.datasize(metadata) #5

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i256 @llvm.evm.dataoffset(metadata) #5

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i256 @llvm.evm.linkersymbol(metadata) #5

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i256 @llvm.evm.loadimmutable(metadata) #5

; Function Attrs: nocallback noduplicate nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare i256 @llvm.evm.pushdeployaddress() #6

; Function Attrs: nounwind willreturn
declare void @llvm.evm.log0(ptr addrspace(1), i256) #3

; Function Attrs: nounwind willreturn
declare void @llvm.evm.log1(ptr addrspace(1), i256, i256) #3

; Function Attrs: nounwind willreturn
declare void @llvm.evm.log2(ptr addrspace(1), i256, i256, i256) #3

; Function Attrs: nounwind willreturn
declare void @llvm.evm.log3(ptr addrspace(1), i256, i256, i256, i256) #3

; Function Attrs: nounwind willreturn
declare void @llvm.evm.log4(ptr addrspace(1), i256, i256, i256, i256, i256) #3

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.call(i256, i256, i256, ptr addrspace(1), i256, ptr addrspace(1), i256) #3

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.staticcall(i256, i256, ptr addrspace(1), i256, ptr addrspace(1), i256) #3

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.delegatecall(i256, i256, ptr addrspace(1), i256, ptr addrspace(1), i256) #3

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.callcode(i256, i256, i256, ptr addrspace(1), i256, ptr addrspace(1), i256) #3

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.create(i256, ptr addrspace(1), i256) #3

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.create2(i256, ptr addrspace(1), i256, i256) #3

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.address() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.caller() #0

; Function Attrs: nounwind willreturn memory(inaccessiblemem: read)
declare i256 @llvm.evm.balance(i256) #4

; Function Attrs: nounwind willreturn memory(inaccessiblemem: read)
declare i256 @llvm.evm.selfbalance() #4

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.callvalue() #0

; Function Attrs: nounwind willreturn
declare i256 @llvm.evm.gas() #3

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.gasprice() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.gaslimit() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.blockhash(i256) #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.coinbase() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.basefee() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.timestamp() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.number() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.chainid() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.origin() #0

; Function Attrs: nounwind willreturn memory(none)
declare i256 @llvm.evm.difficulty() #0

; Function Attrs: noreturn nounwind
declare void @llvm.evm.return(ptr addrspace(1), i256) #7

; Function Attrs: noreturn nounwind
declare void @llvm.evm.revert(ptr addrspace(1), i256) #7

; Function Attrs: noreturn nounwind
declare void @llvm.evm.stop() #7

; Function Attrs: noreturn nounwind
declare void @llvm.evm.invalid() #7

; Function Attrs: nounwind
declare void @llvm.evm.selfdestruct(i256) #8

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p1.p1.i256(ptr addrspace(1) nocapture writeonly, ptr addrspace(1) nocapture readonly, i256, i1 immarg) #9

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p1.p2.i256(ptr addrspace(1) noalias nocapture writeonly, ptr addrspace(2) noalias nocapture readonly, i256, i1 immarg) #9

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p1.p3.i256(ptr addrspace(1) noalias nocapture writeonly, ptr addrspace(3) noalias nocapture readonly, i256, i1 immarg) #9

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p1.p4.i256(ptr addrspace(1) noalias nocapture writeonly, ptr addrspace(4) noalias nocapture readonly, i256, i1 immarg) #9

; Function Attrs: minsize nofree null_pointer_is_valid optsize
define private void @main() #10 {
entry:
  %stack_var_000 = alloca i256, align 32
  store i256 0, ptr %stack_var_000, align 32
  %stack_var_001 = alloca i256, align 32
  store i256 0, ptr %stack_var_001, align 32
  %stack_var_002 = alloca i256, align 32
  store i256 0, ptr %stack_var_002, align 32
  %stack_var_003 = alloca i256, align 32
  store i256 0, ptr %stack_var_003, align 32
  %stack_var_004 = alloca i256, align 32
  store i256 0, ptr %stack_var_004, align 32
  br label %"block_dt_0/0"

return:                                           ; No predecessors!
  ret void

"block_dt_0/0":                                   ; preds = %entry
  store i256 128, ptr %stack_var_000, align 32
  store i256 64, ptr %stack_var_001, align 32
  %argument_0 = load i256, ptr %stack_var_001, align 32
  %argument_1 = load i256, ptr %stack_var_000, align 32
  %memory_store_pointer = inttoptr i256 %argument_0 to ptr addrspace(1)
  store i256 %argument_1, ptr addrspace(1) %memory_store_pointer, align 1
  %callvalue = call i256 @llvm.evm.callvalue()
  store i256 %callvalue, ptr %stack_var_000, align 32
  %dup1 = load i256, ptr %stack_var_000, align 32
  store i256 %dup1, ptr %stack_var_001, align 32
  %argument_01 = load i256, ptr %stack_var_001, align 32
  %comparison_result = icmp eq i256 %argument_01, 0
  %comparison_result_extended = zext i1 %comparison_result to i256
  store i256 %comparison_result_extended, ptr %stack_var_001, align 32
  store i256 1, ptr %stack_var_002, align 32
  %conditional_dt_1_condition = load i256, ptr %stack_var_001, align 32
  %conditional_dt_1_condition_compared = icmp ne i256 %conditional_dt_1_condition, 0
  br i1 %conditional_dt_1_condition_compared, label %"block_dt_1/0", label %conditional_dt_1_join_block

"block_dt_1/0":                                   ; preds = %"block_dt_0/0"
  store i256 2, ptr %stack_var_000, align 32
  br label %"block_dt_2/0"

"block_dt_2/0":                                   ; preds = %"block_dt_1/0"
  %datasize = call i256 @llvm.evm.datasize(metadata !2)
  store i256 %datasize, ptr %stack_var_000, align 32
  %dup14 = load i256, ptr %stack_var_000, align 32
  store i256 %dup14, ptr %stack_var_001, align 32
  %dataoffset = call i256 @llvm.evm.dataoffset(metadata !2)
  store i256 %dataoffset, ptr %stack_var_002, align 32
  store i256 0, ptr %stack_var_003, align 32
  %argument_05 = load i256, ptr %stack_var_003, align 32
  %argument_16 = load i256, ptr %stack_var_002, align 32
  %argument_2 = load i256, ptr %stack_var_001, align 32
  %codecopy_destination_pointer = inttoptr i256 %argument_05 to ptr addrspace(1)
  %codecopy_source_pointer = inttoptr i256 %argument_16 to ptr addrspace(4)
  call void @llvm.memcpy.p1.p4.i256(ptr addrspace(1) align 1 %codecopy_destination_pointer, ptr addrspace(4) align 1 %codecopy_source_pointer, i256 %argument_2, i1 false)
  store i256 0, ptr %stack_var_001, align 32
  %argument_07 = load i256, ptr %stack_var_001, align 32
  %argument_18 = load i256, ptr %stack_var_000, align 32
  %revert_offset_pointer9 = inttoptr i256 %argument_07 to ptr addrspace(1)
  call void @llvm.evm.return(ptr addrspace(1) noalias nocapture nofree noundef nonnull align 32 %revert_offset_pointer9, i256 %argument_18)
  unreachable

conditional_dt_1_join_block:                      ; preds = %"block_dt_0/0"
  store i256 0, ptr %stack_var_001, align 32
  store i256 0, ptr %stack_var_002, align 32
  %argument_02 = load i256, ptr %stack_var_002, align 32
  %argument_13 = load i256, ptr %stack_var_001, align 32
  %revert_offset_pointer = inttoptr i256 %argument_02 to ptr addrspace(1)
  call void @llvm.evm.revert(ptr addrspace(1) noalias nocapture nofree noundef nonnull align 32 %revert_offset_pointer, i256 %argument_13)
  unreachable
}

; Function Attrs: minsize nofree null_pointer_is_valid optsize
define void @__entry() #10 {
entry:
  call void @main()
  br label %return

return:                                           ; preds = %entry
  call void @llvm.evm.stop()
  unreachable
}

attributes #0 = { nounwind willreturn memory(none) }
attributes #1 = { nounwind willreturn memory(argmem: read) }
attributes #2 = { nounwind memory(argmem: write) }
attributes #3 = { nounwind willreturn }
attributes #4 = { nounwind willreturn memory(inaccessiblemem: read) }
attributes #5 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #6 = { nocallback noduplicate nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #7 = { noreturn nounwind }
attributes #8 = { nounwind }
attributes #9 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #10 = { minsize nofree null_pointer_is_valid optsize }

!llvm.dbg.cu = !{!0}

!0 = distinct !DICompileUnit(language: DW_LANG_C, file: !1, isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!1 = !DIFile(filename: "/Users/hedgarmac/src/era-compiler-tester//tests/solidity/simple/default.sol:Test", directory: "")
!2 = !{!"/Users/hedgarmac/src/era-compiler-tester//tests/solidity/simple/default.sol:Test.runtime"}
