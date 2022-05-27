.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp  // 交换 sp 和 sscratch 的值，交换后，sp指向内核栈，sscratch指向用户栈
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8
    # save general-purpose registers  分配34*8字节的栈帧, 用于保存通用寄存器
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)     # sstatus,存储当前处于那个特权级, S or U
    sd t1, 33*8(sp)     # sepc 返回用户态后执行的第一条指令的指针
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch   # sscratch 目前指向的是用户栈
    sd t2, 2*8(sp)      # 空间利用，把这个存储到 原本应该存放 x2 寄存器的内存地址
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp   # 让寄存器 a0 指向内核栈的栈指针, 因为函数的第一个参数从a0获取
    call trap_handler   # 函数返回后，后调用下一条指令，也就是 __restore中的指令开始执行

__restore:
    # case1: start running app by __restore, 开始运行app
    # case2: back to U after handling trap, 从中断处理程序返回
    mv sp, a0 # 为啥要这个, a0是返回值，有可能内核栈发生了变化，所以需要恢复内核栈的位置
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1   // 恢复spec
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack 释放内核栈内存
    addi sp, sp, 34*8
    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp # 交换值,sp指向用户栈
    sret    # sret 从S特权级返回到U特权级
