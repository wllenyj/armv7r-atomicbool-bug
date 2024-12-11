# Arm Atomicbool Bug

```
cargo build --release
ar -x target/armv7r-none-eabihf/release/libarmv7r_atomicbool_align.rlib
objdump -d ./armv7r_atomicbool_align-xxxxxxx.o
```

We get the following assembly code:
```
00000000 <_ZN23armv7r_atomicbool_align10mutex_bool17h72ab2ff002ba6a72E>:
       0: 10 40 2d e9   push    {r4, lr}
       4: 10 d0 4d e2   sub     sp, sp, #16
       8: 00 40 a0 e1   mov     r4, r0
       c: 00 00 a0 e3   mov     r0, #0
      10: 08 00 cd e5   strb    r0, [sp, #8]
      14: 0d 10 a0 e1   mov     r1, sp
      18: 04 00 cd e5   strb    r0, [sp, #4]
      1c: 04 00 81 e3   orr     r0, r1, #4
      20: fe ff ff eb   bl      0x20 <_ZN23armv7r_atomicbool_align10mutex_bool17h72ab2ff002ba6a72E+0x20> @ imm = #-8
      24: 07 00 9d e8   ldm     sp, {r0, r1, r2}
      28: 07 00 84 e8   stm     r4, {r0, r1, r2}
      2c: 10 d0 8d e2   add     sp, sp, #16
      30: 10 80 bd e8   pop     {r4, pc}
```
I think the `orr` instruction is wrong. It should be `add r0, r1, #4` like the `normal` function.
```
00000000 <_ZN23armv7r_atomicbool_align6normal17hd8d5a641764e76d7E>:
       0: 00 48 2d e9   push    {r11, lr}
       4: 10 d0 4d e2   sub     sp, sp, #16
       8: 00 00 a0 e3   mov     r0, #0
       c: 04 10 8d e2   add     r1, sp, #4
      10: 0c 00 cd e5   strb    r0, [sp, #12]
      14: 08 00 cd e5   strb    r0, [sp, #8]
      18: 04 00 81 e2   add     r0, r1, #4
      1c: fe ff ff eb   bl      0x1c <_ZN23armv7r_atomicbool_align6normal17hd8d5a641764e76d7E+0x1c> @ imm = #-8
      20: 10 d0 8d e2   add     sp, sp, #16
      24: 00 88 bd e8   pop     {r11, pc}
```

