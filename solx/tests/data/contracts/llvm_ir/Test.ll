; ModuleID = 'solx/tests/data/contracts/solidity/Test.sol:Test.runtime'
source_filename = "solx/tests/data/contracts/solidity/Test.sol:Test.runtime"
target datalayout = "E-p:256:256-i256:256:256-S256-a:256:256"
target triple = "evm-unknown-unknown"

; Function Attrs: nofree nosync nounwind memory(none)
declare i256 @llvm.evm.calldatasize() #0

; Function Attrs: noreturn nounwind
declare void @llvm.evm.revert(ptr addrspace(1), i256) #1

; Function Attrs: nofree noinline noreturn null_pointer_is_valid
define private fastcc void @main() unnamed_addr #2 {
entry:
  store i256 128, ptr addrspace(1) inttoptr (i256 64 to ptr addrspace(1)), align 64
  %calldatasize = tail call i256 @llvm.evm.calldatasize()
  tail call void @llvm.evm.revert(ptr addrspace(1) noalias nocapture nofree noundef nonnull align 32 null, i256 0)
  unreachable
}

; Function Attrs: nofree noinline noreturn null_pointer_is_valid
define void @__entry() local_unnamed_addr #2 {
entry:
  tail call fastcc void @main()
  unreachable
}

attributes #0 = { nofree nosync nounwind memory(none) }
attributes #1 = { noreturn nounwind }
attributes #2 = { nofree noinline noreturn null_pointer_is_valid }

!llvm.dbg.cu = !{!0}

!0 = distinct !DICompileUnit(language: DW_LANG_C, file: !1, isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!1 = !DIFile(filename: "solx/tests/data/contracts/solidity/Test.sol:Test.runtime", directory: "")
