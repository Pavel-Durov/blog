# Loops and Opcodes in Lua

## Abstract 

This article overviews the programming language Lua loops as well as the instructions used to implement them.
We will demonstrate Lua syntax and the appropriate instructions for for, while and repeat loops.
This topic might be a bit esoteric, but I find it fascinating - what happens behind the scenes in programming language implementations.


## Introduction

Programming languages we use are usually reduced into a sequence of instructions, aka opcodes, we'll use both terms interchangeably. These instructions are an essential part of the machine instruction set that tells the interpreter or the compiler what operation needs to be performed. Most instructions specify operands, the data used, it can be a register, direct value or memory address.
Next, we'll have a look at two kinds of for-loops, while and repeat-until statements.

# Lua for-loops

Lua has two types of for loops: numeric and generic.

## Why more than one type of for-loop?

We might wonder what's the difference, and why do we need two different for-loops. But it gets clearer once we have a look at the implementation of both loops.
The main difference is that numeric-loops are based on numeric condition, while generic for-loops allows us to perform enumeration-based loops; iterating over a list for example.
When numeric loops will check whether iteration index is for example not greater or less than a some number, generic loops will iterate until it encounter nil object, provided by the enumerator.

# Numeric for-loop

https://www.lua.org/pil/4.3.4.html

A numeric for loop identifies by two instructions:

FORPREP - sets up a for loop

FORLOOP - used for initial testing of the loop condition as well as conditional testing during the loop itself

Syntax:
```
FORPREP A sBx   R(A)-=R(A+2); pc+=sBx
FORLOOP A sBx   R(A)+=R(A+2);
                if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
```

Without going into too much depth of what each statement means here lets review some of it:

R(A) - loop internal index

R(A+1) - the limit

R(A+2) - stepping value

R(A+3) - loop variable, external index that is local to the for block


Loop iteration is performed by jumping back to the start of the loop body (pc+=sBx) if loop condition is valid, otherwise the loop terminates.

Numeric for-loop:
```lua
sum = 0
for i=0,100 do 
    sum  = sum + i
```

Lets have a look the instructions used to implement numeric loops:
```shell
$ ./src/luac -l -l -p ./example-numeric-loop.lua 

main <./example-numeric-loop.lua:0,0> (12 instructions at 0x1a1e700)
0+ params, 5 slots, 1 upvalue, 4 locals, 2 constants, 0 functions
        1       [1]     VARARGPREP      0
        2       [1]     SETTABUP        0 0 1k  ; _ENV "sum" 0
        3       [2]     LOADI           0 0
        4       [2]     LOADI           1 100
        5       [2]     LOADI           2 1
        6       [2]     FORPREP         0 4     ; exit to 12
        7       [3]     GETTABUP        4 0 0   ; _ENV "sum"
        8       [3]     ADD             4 4 3
        9       [3]     MMBIN           4 3 6   ; __add
        10      [3]     SETTABUP        0 0 4   ; _ENV "sum"
        11      [2]     FORLOOP         0 5     ; to 7
        12      [4]     RETURN          0 1 1   ; 0 out
constants (2) for 0x1a1e700:
        0       S       "sum"
        1       I       0
locals (4) for 0x1a1e700:
        0       (for state)     6       12
        1       (for state)     6       12
        2       (for state)     6       12
        3       i       7       11
upvalues (1) for 0x1a1e700:
        0       _ENV    1       0
```

## Generic for-loop

https://www.lua.org/pil/4.3.5.html

In addition to numeric for loop (implemented by `FORPREP` and `FORLOOP` instructions), Lua has a
generic for loop, implemented by `TFORCALL` and `TFORLOOP`. 
The generic for loop allows you to perform iteration-based loops. 

Syntax
```
TFORCALL A C R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2))
TFORLOOP A sBx if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }
```

Here, each time TFORCALL executes, the iterator function referenced by R(A) is called with two arguments: R(A+1) (the state) and R(A+2) the control variable. The results are returned in the local loop variables.
TFORLOOP tests the returned value, if it is nil, the loop terminates.
If it is not nil, there is another iteration, then the TFORLOOP instruction sends execution back to the beginning of the loop (specified by the sBx operand).


Generic for-loop:
```lua
for i,v in pairs({"your", "base", "is", "ours"}) do 
    print(i,v) 
end
```

Generic for-loop instructions:
```bash
$ ./src/luac -l -l -p ./example-generic-loop.lua 
main <./example-generic-loop.lua:0,0> (19 instructions at 0x2252700)
0+ params, 9 slots, 1 upvalue, 6 locals, 6 constants, 0 functions
        1       [1]     VARARGPREP      0
        2       [1]     GETTABUP        0 0 0   ; _ENV "pairs"
        3       [1]     NEWTABLE        1 0 4   ; 4
        4       [1]     EXTRAARG        0
        5       [1]     LOADK           2 1     ; "your"
        6       [1]     LOADK           3 2     ; "base"
        7       [1]     LOADK           4 3     ; "is"
        8       [1]     LOADK           5 4     ; "ours"
        9       [1]     SETLIST         1 4 0
        10      [1]     CALL            0 2 5   ; 1 in 4 out
        11      [1]     TFORPREP        0 4     ; to 16
        12      [2]     GETTABUP        6 0 5   ; _ENV "print"
        13      [2]     MOVE            7 4
        14      [2]     MOVE            8 5
        15      [2]     CALL            6 3 1   ; 2 in 0 out
        16      [1]     TFORCALL        0 2
        17      [1]     TFORLOOP        0 6     ; to 12
        18      [3]     CLOSE           0
        19      [3]     RETURN          0 1 1k  ; 0 out
constants (6) for 0x2252700:
        0       S       "pairs"
        1       S       "your"
        2       S       "base"
        3       S       "is"
        4       S       "ours"
        5       S       "print"
locals (6) for 0x2252700:
        0       (for state)     11      19
        1       (for state)     11      19
        2       (for state)     11      19
        3       (for state)     11      19
        4       i       12      16
        5       v       12      16
upvalues (1) for 0x2252700:
        0       _ENV    1       0
```


## Other loops 

In addition to for loops, Lua has other loops: while and repeat-until. Both are implemented using JMP instructions.

### While loop

https://www.lua.org/pil/4.3.2.html

While loop first tests the while condition. The loops ends once condition is false.

```lua 
$ cat ./example-while-loop.lua 
a = 10
while( a < 20 )
do
   a = a+1
```
While loop instructions:
```
main <./example-while-loop.lua:0,0> (11 instructions at 0x1721700)
0+ params, 2 slots, 1 upvalue, 0 locals, 2 constants, 0 functions
        1       [1]     VARARGPREP      0
        2       [1]     SETTABUP        0 0 1k  ; _ENV "a" 10
        3       [2]     GETTABUP        0 0 0   ; _ENV "a"
        4       [2]     LTI             0 20 0
        5       [2]     JMP             5       ; to 11
        6       [4]     GETTABUP        0 0 0   ; _ENV "a"
        7       [4]     ADDI            0 0 1
        8       [4]     MMBINI          0 1 6 0 ; __add
        9       [4]     SETTABUP        0 0 0   ; _ENV "a"
        10      [4]     JMP             -8      ; to 3
        11      [5]     RETURN          0 1 1   ; 0 out
constants (2) for 0x1721700:
        0       S       "a"
        1       I       10
locals (0) for 0x1721700:
upvalues (1) for 0x1721700:
        0       _ENV    1       0
```

Notice the 10th JMP instruction, it jumps to 3rd instruction - that's our loop!

### Repeat-until loop

https://www.lua.org/pil/4.3.3.html

A repeat-until statement repeats its loop body until the specified condition is true. 
The body is always executed at least once (similar to `do-while` statements in other programming languages).

```lua
a = 0;
repeat
	a = a +1
until ( a > 10)
print(a)
```
Repeat-until instructions:
```
$ ./src/luac -l -l -p ./ehtmlxample-repeat.lua 
main <./example-repeat.lua:0,0> (13 instructions at 0xe34700)
0+ params, 2 slots, 1 upvalue, 0 locals, 3 constants, 0 functions
        1       [1]     VARARGPREP      0
        2       [1]     SETTABUP        0 0 1k  ; _ENV "a" 0
        3       [3]     GETTABUP        0 0 0   ; _ENV "a"
        4       [3]     ADDI            0 0 1
        5       [3]     MMBINI          0 1 6 0 ; __add
        6       [3]     SETTABUP        0 0 0   ; _ENV "a"
        7       [4]     GETTABUP        0 0 0   ; _ENV "a"
        8       [4]     GTI             0 10 0
        9       [4]     JMP             -7      ; to 3
        10      [5]     GETTABUP        0 0 2   ; _ENV "print"
        11      [5]     GETTABUP        1 0 0   ; _ENV "a"
        12      [5]     CALL            0 2 1   ; 1 in 0 out
        13      [5]     RETURN          0 1 1   ; 0 out
constants (3) for 0xe34700:
        0       S       "a"
        1       I       0
        2       S       "print"
locals (0) for 0xe34700:
upvalues (1) for 0xe34700:
        0       _ENV    1       0
```

## Sammary

We covered here the 



This writing was for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing.
Code and tools were tested Lua 5.4.4 version.
This work relates to my PhD in Kings' College and the YK project - https://github.com/ykjit


Lua Opcodes can be found here: https://www.lua.org/source/5.1/lopcodes.h.html.