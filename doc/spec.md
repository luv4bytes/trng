# TRNG language specification

TRNG is a very close relative to Brainfuck.
It works based on the same principle with a few modifications to make writing programs a bit less fucky.

# Table of Contents
1. [Instruction codes](#instruction-codes)  
    1.1 [Cell instructions](#cell-instructions)    
    1.2 [I/O instructions](#io-instructions)     
    1.3 [Loops](#loops)

## Instruction codes

### Instruction table

|Op code          |Arguments|Description  |
------------------|---------|-------------|
|[pfw](#pfw)      |[integer]|Moves the cell pointer one step forward.
|[pbw](#pbw)      |[integer]|Moves the cell pointer one step back.
|[inc](#inc)      |[integer]|Increments the value of the current cell by [integer].
|[dec](#dec)      |[integer]|Decrements the value of the current cell by [integer].
|[set](#set)      |[value]  |Sets the given [value], placing each byte in a separate cell and incrementing the pointer accordingly.
|[wrt](#wrt)      |         |Write the value of the current cell to the standard output.
|[wrti8](#wrti8)  |         |Write the value of the current cell to the standard output. The value is interpreted as an 8-bit integer in LE byte order. 
|[wrti16](#wrti16)|         |Write the value of the current cell and the next one together interpreted as an 16-bit integer in LE byte order.
|[wrti32](#wrti32)|         |Write the value of the current cell and the next one together interpreted as an 32-bit integer in LE byte order.
|[wrti64](#wrti64)|         |Write the value of the current cell and the next one together interpreted as an 64-bit integer in LE byte order.
|[wra](#wra)      |         |Write all bytes from the current cell on to the standard output until a null byte is encountered.
|[rdi](#rdi)      |         |Reads the next byte from standard input and stores it in the current cell.
|[rda](#rda)      |         |Reads all bytes from standard input until LF is encountered and stores them in separate cells.
|[clr](#clr)      |         |Writes a null byte to the current cell and all following cells while the current cell is not a null byte. The pointer is moved accordingly.


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