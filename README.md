# TRNG - Brainfucks pretty sister

## What is TRNG?

TRNG is a very close relative of [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck). Meaning it works based on the same principles as Brainfuck but with a lot more instructions and features to make writing programs a bit more understandable.

The TRNG interpreter is written in [Rust](https://www.rust-lang.org/).

## Examples

The following demonstrates a simple "Hello World" program written in TRNG.

    set Hello
    pbw 5
    wra
    pbw 5
    clr
    pbw 5
    inc 32
    wrt
    dec 32
    set World
    pbw 5
    wra
    pbw 5
    clr
    inc 13
    wrt
    dec 3
    wrt
