target datalayout = "E-p:256:256-i256:256:256-S32-a:256:256"  
declare void @foo()                                              
define void @glob() nounwind {                                    
  call void @foo()                                                
  ret void
}