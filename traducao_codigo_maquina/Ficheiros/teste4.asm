auipc t0, 0x0000fc10
addi t4, zero, 0x552
sw t4, 0(t0)
lb t2, 0(t0)
add t1, t2, t2
auipc t3, 0x0000fc10
sw t1, 4(t0)