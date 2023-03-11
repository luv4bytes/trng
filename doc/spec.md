# TRNG language specification

TRNG is a very close relative to Brainfuck.
It works based on the same principle with a few modifications to make writing programs a bit less fucky.

# Table of Contents
1. [Instruction codes](#instruction-codes)  
    1.1 [Cell instructions](#cell-instructions)    
    1.2 [I/O instructions](#io-instructions)     
    1.3 [Loops](#loops)

## Instruction codes

### Cell instructions

#### PFW
PFW (Pointer forward) is used when the cell pointer (read/write head) should move forward i.e. increment the current pointer index.

Syntax:

    pfw n

    where n is a non-negative integer value.

Example:

    pfw 23

#### PBW
PBW (Pointer backward) is used when the cell pointer (read/write head) should move backward i.e. decrement the current pointer index.

Syntax:

    pbw n

    where n is a non-negative integer value.
    

Example:

    pbw 10


#### INC
INC (increment) is used to increment the value of the current cell by n.

Syntax:

    inc n

    where n is a non-negative integer value.
    

Example:

    inc 10

#### DEC
DEC (decrement) is used to decrement the value of the current cell by n.

Syntax:

    dec n

    where n is a non-negative integer value.
    

Example:

    dec 10

#### SET

SET (set following value) is used to set the following consecutive value. This means that the bytes of the passed value will be stored each byte a cell. The pointer will be set accordingly.

Syntax:
    
    set [value]

Example:

    set Hello
    pfw 1
    inc 32
    pfw 1
    set World

### I/O instructions

#### WRT
WRT (write) is used to write the value of the current cell to the standard output.

Syntax:

    wrt

Example:

    inc 72
    wrt

#### WRTI

WRTI (write current as integer) is used to read the current cell value as an integer and write it to the standard output.

Syntax:

    wrti

Example:

    inc 123
    wrti

    Output => "123"

    ====

    inc 123
    wrt

    Output => "{"

#### RDI
RDI (read input) is used to read a byte from the standard input and store it in the current cell.

Syntax:

    rdi
    

Example:

    inc 72
    pfw 1
    rdi

#### WRA

WRA (write all) is used to write the current cell and all following cell to stdout until a null byte inside a cell is encountered.

Syntax:

    wra

Example:

    set Hello
    pbw 5
    wra

#### RDA

RDA (read all) is used to read all bytes from standard input and store them on the tape until LF is encountered.

Syntax:

    rda

Example:

    rda
    pbw 5
    wra    

#### CLR
CLR (clear) is used to write null byte to the current cell and all following cells until a null byte is encountered.

Syntax:
    
    clr

Example:

    set Hello
    pbw 5
    wra
    pbw 5
    clr

### Loops

#### LOP
LOP (loop) starts a loop. All following instructions are executed until a POL is encountered. At this point the value of the current cell is checked if it is null. If the current cell value is null the interpreter jumps back to the start of the loop. If it is not, it proceeds with the following instructions after the end of the loop.

Syntax:

    lop

Example:

    inc 10
    lop
    dec 1
    pol
    set Hello
    pbw 5
    wra

#### POL
POL (end loop) ends the current loop if the current cell value is null.

For an example check [LOP](#lop).