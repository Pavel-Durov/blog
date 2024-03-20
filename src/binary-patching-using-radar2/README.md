# Binary Patching with Radar2

## Abstract

Binary patching is a crucial tool in software analysis and reverse engineering. This article provides an overview of the binary patching process using the Radare2 utility. The article begins by introducing the concept of binary patching.

Key concepts covered in the article include:
- Understanding binary analysis fundamentals
- Navigating and seeking specific memory addresses within binaries
- Writing assembly instructions to modify binary behavior

Throughout the article, step-by-step examples and commands are provided to guide readers through the process of binary patching using Radare2.
By the end of the article, readers will gain a fundamental understanding of how to navigate Radare2 tool and how to perform basic binary patching.

## Introduction

Binary patching is a process of modifying compiled binary executable files to change their behaviour or fix issues without access to the source code.

This technique is commonly used in software development, reverse engineering, and cybersecurity for various purposes such as bug fixing, feature enhancement, and vulnerability exploitation or mitigation.

In our examples, we will use a simple C application, compile it, run it and later on change its binary using the Radar2 utility to modify its behaviour.

Note: Assembly language and instruction codes are specific to CPU architecture. In the examples provided, we will be focusing on the Intel x86 architecture.

## Binary Analysis

Binary analysis is a process of examining and understanding compiled binary code (machine code) without access to the original source code. This analysis includes disassembling, decompiling, and analyzing the binary behaviour.
In our case, we will know exactly what the program intends to do, since we're going to be the ones who write it.
So the binary analysis will be straightforward.

## Wallet application

We're going to implement a simple wallet sample app in C where the wallet is unlocked only if the user enters the correct numeric password (1234 in our case).

```cpp
// file: wallet.c
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  int num = atoi(argv[1]);
  if (num == 1234) {
    printf("Unlocked!\n");
    return 0;
  }
  printf("Wrong passcode! This incident will be reported!\n");
  return 1;
}
```
Le's compile it:

```bash
$ clang wallet.c -o ./wallet
```

Lets see what it does.

If we give it any number other than 1234 we get an error message and error status code:

```shell
$ ./wallet 1
Wrong! This incident will be reported!
$ echo $?
1
```
If we guess the secret number we get success exit code and a message printed:

```shell
$ ./wallet 1234
Unlocked!
$ echo $?
0
```

## Patching the binary

Next, we're going to use Radar2 tool to do the binary patching.

Open it with Radar2:
```shell
$ r2 -w ./wallet # -w stands for write mode
[0x00001060]> aa
[0x00001060]> s main # seek main function
[0x00001150]> pdf
            ; DATA XREF from entry0 @ 0x1078(r)
┌ 107: int main (int argc, char **argv);
│           ; arg int argc @ rdi
│           ; arg char **argv @ rsi
│           ; var int64_t var_4h @ rbp-0x4
│           ; var int64_t var_8h @ rbp-0x8
│           ; var int64_t var_10h @ rbp-0x10
│           ; var int64_t var_14h @ rbp-0x14
│           0x00001150      55             push rbp
│           0x00001151      4889e5         mov rbp, rsp
│           0x00001154      4883ec20       sub rsp, 0x20
│           0x00001158      c745fc0000..   mov dword [var_4h], 0
│           0x0000115f      897df8         mov dword [var_8h], edi     ; argc
│           0x00001162      488975f0       mov qword [var_10h], rsi    ; argv
│           0x00001166      488b45f0       mov rax, qword [var_10h]
│           0x0000116a      488b7808       mov rdi, qword [rax + 8]
│           0x0000116e      e8cdfeffff     call sym.imp.atoi           ; int atoi(const char *str)
│           0x00001173      8945ec         mov dword [var_14h], eax
│           0x00001176      817decd204..   cmp dword [var_14h], 0x4d2
│       ┌─< 0x0000117d      0f851a000000   jne 0x119d
│       │   0x00001183      488d3d7a0e..   lea rdi, str.Unlocked__n    ; 0x2004 ; "Unlocked!\n"
│       │   0x0000118a      b000           mov al, 0
│       │   0x0000118c      e89ffeffff     call sym.imp.printf         ; int printf(const char *format)
│       │   0x00001191      c745fc0000..   mov dword [var_4h], 0
│      ┌──< 0x00001198      e915000000     jmp 0x11b2
│      │└─> 0x0000119d      488d3d6b0e..   lea rdi, str.Wrong_passcode__This_incident_will_be_reported__n ; 0x200f ; "Wrong passcode! This incident will be reported!\n"
│      │    0x000011a4      b000           mov al, 0
│      │    0x000011a6      e885feffff     call sym.imp.printf         ; int printf(const char *format)
│      │    0x000011ab      c745fc0100..   mov dword [var_4h], 1
│      │    ; CODE XREF from main @ 0x1198(x)
│      └──> 0x000011b2      8b45fc         mov eax, dword [var_4h]
│           0x000011b5      4883c420       add rsp, 0x20
│           0x000011b9      5d             pop rbp
└           0x000011ba      c3             ret
```

Let's break down the provided assembly code and understand its relation to C code:

There's a bunch of instructions related to args and string convertion to int value.
But we actually care about the if statement here.

In our code:
```c
 if (num == 1234) {
  ...
 }
```
Is mached to:
```asm
cmp dword [var_14h], 0x4d2
jne 0x119d
```
The instruction `jne` will jump to `0x119d` which is `0x0000119d` if the `cmp` returns false, which will print "Wrong passcode...".
What we want is to remove this condition or change it to be always true. One way of achieving this is by overriding it with a noop (no operation) instruction, and that's exactly what we're going to do next.


0x90 is a NOP instruction on Intel x86 CPU family. https://en.wikipedia.org/wiki/NOP_(code).

```shell
[0x00001150]> s 0x0000117d # navigate to jne
[0x0000117d]> px # print instruction as this location
- offset -  7D7E 7F80 8182 8384 8586 8788 898A 8B8C  DEF0123456789ABC
0x0000117d  0f85 1a00 0000 488d 3d7a 0e00 00b0 00e8  ......H.=z......
...
[0x000011de]> wx 909090909090 # override 6 bytes with noop instructions
```
Let's review the amended asm again:
```shell
[0x00001150]> pdf
            ; DATA XREF from entry0 @ 0x1078(r)
┌ 86: int main (int argc, char **argv);
│           ; arg int argc @ rdi
│           ; arg char **argv @ rsi
│           ; var int64_t var_4h @ rbp-0x4
│           ; var int64_t var_8h @ rbp-0x8
│           ; var char **var_10h @ rbp-0x10
│           ; var uint32_t var_14h @ rbp-0x14
│           0x00001150      55             push rbp
│           0x00001151      4889e5         mov rbp, rsp
│           0x00001154      4883ec20       sub rsp, 0x20
│           0x00001158      c745fc0000..   mov dword [var_4h], 0
│           0x0000115f      897df8         mov dword [var_8h], edi     ; argc
│           0x00001162      488975f0       mov qword [var_10h], rsi    ; argv
│           0x00001166      488b45f0       mov rax, qword [var_10h]
│           0x0000116a      488b7808       mov rdi, qword [rax + 8]    ; const char *str
│           0x0000116e      e8cdfeffff     call sym.imp.atoi           ; int atoi(const char *str)
│           0x00001173      8945ec         mov dword [var_14h], eax
│           0x00001176      817decd204..   cmp dword [var_14h], 0x4d2
│           0x0000117d      90             nop
│           0x0000117e      90             nop
│           0x0000117f      90             nop
│           0x00001180      90             nop
│           0x00001181      90             nop
│           0x00001182      90             nop
│           0x00001183      488d3d7a0e..   lea rdi, str.Unlocked__n    ; 0x2004 ; "Unlocked!\n" ; const char *format
│           0x0000118a      b000           mov al, 0
│           0x0000118c      e89ffeffff     call sym.imp.printf         ; int printf(const char *format)
│           0x00001191      c745fc0000..   mov dword [var_4h], 0
│       ┌─< 0x00001198      e915000000     jmp 0x11b2
..
│       │   ; CODE XREF from main @ 0x1198(x)
│       └─> 0x000011b2      8b45fc         mov eax, dword [var_4h]
│           0x000011b5      4883c420       add rsp, 0x20
│           0x000011b9      5d             pop rbp
└           0x000011ba      c3             ret
```
So why did we uesd `wx 909090909090` you might ask.

As we already mentioned, 90 is an opcode for nop and the jump instruction `0f851a000000` is a 6 bytes long:

`0F` - Primary jump instruction opcode.
`85` - Secondary jump instruction opcode - jump near if not zero/not equal (ZF=0)
`1a000000` - The relative address to jump to.

So we had to override all these bits inorder to maintain the correct flow of the program.

Now we can exit the radar2 and run the changed binary.

```shell
$ ./wallet 0
Correct!
$ echo $?
0
```

That's it!

We successfully patched the binary and we'll get a bypass no matter what integer we input to our wallet binary.

If we want to revert the changes, we can follow the same steps in reverse order. It's all about setting the right bits at the right place (which sounds easier than it is in practice).


## Summary

Binary patching is a powerful technique for modifying software behaviour without access to the original source code.

In this article, we demonstrated how to navigate, find, read and modify the behaviour of a simple binary using the Radare2 tool.

The main thing is to understand the binary itself, in our example we changed the program that we wrote, so obviously it was easier than modifying an external binary because we don't have access to its source.

I did this work as part of my Ph.D. research project where I intend to change interpreter behaviour at runtime by patching function instructions.

This article serves as a documentation of my own understanding and the organization of my thoughts, aiming to share knowledge with others.

I hope you find it valuable! If you have any further questions or wish to delve into more details, feel free to reach out.
