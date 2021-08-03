addi t0, zero, 1
add t1, t0, t0
sub t2, t0, t1
add t3, t0, t2
beq t3, zero, A
addi t1, zero, 100
beq zero, zero SAIR


A:
addi t1, zero, 200
beq zero, zero, SAIR

SAIR:
nop