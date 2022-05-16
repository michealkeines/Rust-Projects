# CHIP-8 EMULATOR

## Current functionalities:

-> 16 registers, 4kb Memory

-> ADD, AND, OR, XOR instruction

-> Run Loop that can execute mulitple instuctions sequentially

-> Stack with overflow check

-> CALL & RET for functions

-> JMP, SE, SNE

## EXAMPLE

```
call add_twice;
call add_twice;
0x00 ; exit

add_twice:
    add reg1, reg2 
    add reg1, reg2
    ret
```

## RUN

```
cargo run
```