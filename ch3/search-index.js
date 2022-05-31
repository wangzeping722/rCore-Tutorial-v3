var searchIndex = JSON.parse('{\
"os":{"doc":"The main module and entrypoint","t":[0,5,0,0,0,0,14,14,5,0,0,0,0,0,0,17,17,17,17,17,17,3,11,11,11,11,5,11,11,11,11,5,7,3,7,3,11,11,11,11,11,11,12,12,11,11,5,5,11,11,5,11,11,5,11,11,11,11,11,11,11,17,17,17,5,5,5,5,0,3,11,11,11,11,12,11,11,11,11,11,17,17,17,17,0,0,5,17,5,5,5,5,3,3,3,3,12,11,11,11,11,11,11,0,12,11,5,11,11,11,11,12,11,11,11,5,11,5,11,12,12,5,11,5,11,12,12,5,0,0,12,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,12,12,12,11,11,11,11,5,13,13,13,3,4,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,17,17,17,5,5,5,5,3,0,5,5,12,12,5,12,3,11,11,11,11,11,12,11,12,11,11,11,12],"n":["board","clear_bss","config","console","lang_items","loader","print","println","rust_main","sbi","sync","syscall","task","timer","trap","CLOCK_FREQ","APP_BASE_ADDRESS","APP_SIZE_LIMIT","KERNEL_STACK_SIZE","MAX_APP_NUM","USER_STACK_SIZE","Stdout","borrow","borrow_mut","from","into","print","try_from","try_into","type_id","write_str","panic","KERNEL_STACK","KernelStack","USER_STACK","UserStack","borrow","borrow","borrow_mut","borrow_mut","clone","clone","data","data","from","from","get_base_i","get_num_app","get_sp","get_sp","init_app_cx","into","into","load_apps","push_context","try_from","try_from","try_into","try_into","type_id","type_id","SBI_CONSOLE_PUTCHAR","SBI_SET_TIMER","SBI_SHUTDOWN","console_putchar","sbi_call","set_timer","shutdown","up","UPSafeCell","borrow","borrow_mut","exclusive_access","from","inner","into","new","try_from","try_into","type_id","SYSCALL_EXIT","SYSCALL_GET_TIME","SYSCALL_WRITE","SYSCALL_YIELD","fs","process","syscall","FD_STDOUT","sys_write","sys_exit","sys_get_time","sys_yield","TASK_MANAGER","TaskContext","TaskManager","TaskManagerInner","__private_field","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","context","current_task","deref","exit_current_and_run_next","find_next_task","from","from","from","inner","into","into","into","mark_current_exited","mark_current_exited","mark_current_suspended","mark_current_suspended","num_app","ra","run_first_task","run_first_task","run_next_task","run_next_task","s","sp","suspend_current_and_run_next","switch","task","tasks","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","TaskContext","borrow","borrow_mut","clone","from","goto_restore","into","ra","s","sp","try_from","try_into","type_id","zero_init","__switch","Exited","Ready","Running","TaskControlBlock","TaskStatus","UnInit","borrow","borrow","borrow_mut","borrow_mut","clone","clone","eq","from","from","into","into","task_cx","task_end_time","task_start_time","task_status","try_from","try_from","try_into","try_into","type_id","type_id","MICRO_PER_SEC","MSEC_PER_SEC","TICKS_PER_SEC","get_time","get_time_ms","get_time_us","set_next_trigger","TrapContext","context","enable_timer_interrupt","init","sepc","sstatus","trap_handler","x","TrapContext","app_init_context","borrow","borrow_mut","from","into","sepc","set_sp","sstatus","try_from","try_into","type_id","x"],"q":["os","","","","","","","","","","","","","","","os::board","os::config","","","","","os::console","","","","","","","","","","os::lang_items","os::loader","","","","","","","","","","","","","","","","","","","","","","","","","","","","","os::sbi","","","","","","","os::sync","os::sync::up","","","","","","","","","","","os::syscall","","","","","","","os::syscall::fs","","os::syscall::process","","","os::task","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","os::task::context","","","","","","","","","","","","","","os::task::switch","os::task::task","","","","","","","","","","","","","","","","","","","","","","","","","","","os::timer","","","","","","","os::trap","","","","","","","","os::trap::context","","","","","","","","","","","",""],"d":["Constants used in rCore for K210 devel board","clear BSS segment","Constants used in rCore","SBI console driver, for text output","The panic handler","Loading user applications into memory","print string macro","println string macro","the rust entry-point of os","SBI call wrappers","Synchronization and interior mutability primitives","Implementation of syscalls","Task management implementation","RISC-V timer-related functionality","Trap handling functionality","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Get base address of app i.","Get the total number of applications.","","","get app info with entry and sp and save <code>TrapContext</code> in …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Load nth user app at [APP_BASE_ADDRESS + n * …","","","","","","","","","","","use sbi call to putchar in console (qemu uart handler)","handle SBI call with <code>which</code> SBI_id and other arguments","use sbi call to set timer","use sbi call to getchar from console (qemu uart handler) …","Uniprocessor interior mutability primitives","Wrap a static data structure inside it so that we are able …","","","Exclusive access inner data in UPSafeCell. Panic if the …","Returns the argument unchanged.","inner data","Calls <code>U::from(self)</code>.","User is responsible to guarantee that inner struct is only …","","","","","","","","File and filesystem-related syscalls","Process management syscalls","handle syscall exception with <code>syscall_id</code> and other …","","write buf of length <code>len</code>  to a file with <code>fd</code>","task exits and submit an exit code","get time in milliseconds","current task gives up resources for other tasks","Global variable: TASK_MANAGER","Task Context","The task manager, where all the tasks are managed.","Inner of Task Manager","","","","","","","","Implementation of <code>TaskContext</code>","id of current <code>Running</code> task","","exit current task,  then run next task","Find next task to run and return task id.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","use inner value to get mutable access","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","exit current task","Change the status of current <code>Running</code> task into <code>Exited</code>.","suspend current task","Change the status of current <code>Running</code> task into <code>Ready</code>.","total number of tasks","return address ( e.g. __restore ) of __switch ASM function","run first task","Run the first task in task list.","rust next task","Switch current <code>Running</code> task to the task we have found, or …","callee saved registers:  s 0..11","kernel stack pointer of app","suspend current task, then run next task","Rust wrapper around <code>__switch</code>.","Types related to task management","task list","","","","","","","","","","Task Context","","","","Returns the argument unchanged.","set task context {__restore ASM funciton, kernel stack, …","Calls <code>U::from(self)</code>.","return address ( e.g. __restore ) of __switch ASM function","callee saved registers:  s 0..11","kernel stack pointer of app","","","","init task context","Switch to the context of <code>next_task_cx_ptr</code>, saving the …","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","read the <code>mtime</code> register","get current time in milliseconds","","set the next timer interrupt …","Trap Context","","timer interrupt enabled 开启时钟中断","initialize CSR <code>stvec</code> as the entry of <code>__alltraps</code>","CSR sepc","CSR sstatus      ","handle an interrupt, exception, or system call from user …","general regs[0..31]","Trap Context","init app context","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","CSR sepc","set stack pointer to x_2 reg (sp)","CSR sstatus      ","","","","general regs[0..31]"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,1,1,1,1,0,0,0,0,0,2,3,2,3,2,3,2,3,2,3,0,0,2,3,0,2,3,0,2,2,3,2,3,2,3,0,0,0,0,0,0,0,0,0,4,4,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,6,7,5,6,7,5,0,7,5,0,6,6,7,5,6,6,7,5,0,6,0,6,6,8,0,6,0,6,8,8,0,0,0,7,6,7,5,6,7,5,6,7,5,0,8,8,8,8,8,8,8,8,8,8,8,8,8,0,9,9,9,0,0,9,10,9,10,9,10,9,9,10,9,10,9,10,10,10,10,10,9,10,9,10,9,0,0,0,0,0,0,0,0,0,0,0,11,11,0,11,0,11,11,11,11,11,11,11,11,11,11,11,11],"f":[null,[[]],null,null,null,null,null,null,[[],["never",0]],null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[["arguments",3]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0],["str",0]],["result",6]],[[["panicinfo",3]],["never",0]],null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["kernelstack",3]],[[["",0]],["userstack",3]],null,null,[[]],[[]],[[["usize",0]],["usize",0]],[[],["usize",0]],[[["",0]],["usize",0]],[[["",0]],["usize",0]],[[["usize",0]],["usize",0]],[[]],[[]],[[]],[[["",0],["trapcontext",3]],["usize",0]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,[[["usize",0]]],[[["usize",0],["usize",0],["usize",0],["usize",0]],["usize",0]],[[["usize",0]]],[[],["never",0]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["refmut",3]],[[]],null,[[]],[[]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,[[["usize",0]],["isize",0]],null,[[["usize",0],["usize",0]],["isize",0]],[[["i32",0]],["never",0]],[[],["isize",0]],[[],["isize",0]],null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],null,null,[[["",0]],["taskmanager",3]],[[]],[[["",0]],["option",4,[["usize",0]]]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[["",0]]],[[]],[[["",0]]],null,null,[[]],[[["",0]],["never",0]],[[]],[[["",0]]],null,null,[[]],null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["taskcontext",3]],[[]],[[["usize",0]]],[[]],null,null,null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]],null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["taskcontrolblock",3]],[[["",0]],["taskstatus",4]],[[["",0],["taskstatus",4]],["bool",0]],[[]],[[]],[[]],[[]],null,null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,[[],["usize",0]],[[],["usize",0]],[[],["usize",0]],[[]],null,null,[[]],[[]],null,null,[[["trapcontext",3]],["trapcontext",3]],null,null,[[["usize",0],["usize",0]]],[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],null,[[["",0],["usize",0]]],null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null],"p":[[3,"Stdout"],[3,"KernelStack"],[3,"UserStack"],[3,"UPSafeCell"],[3,"TASK_MANAGER"],[3,"TaskManager"],[3,"TaskManagerInner"],[3,"TaskContext"],[4,"TaskStatus"],[3,"TaskControlBlock"],[3,"TrapContext"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};