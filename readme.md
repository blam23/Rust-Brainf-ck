# BF-CLI 

A simple Rust brainfuck interpreter written in rust.

## Usage Examples:

### Interpreting a bf file:

`bf-cli helloworld.bf`

or

`bf-cli -f helloworld.bf`

or 

`bf-cli --file helloworld.bf`

### Interpreting a string

`bf-cli -s +[,.]`

or

`bf-cli --str +[,.]`

## Compatibilty

It should be able to run all standard BF programs - It has an increased memory size of 60k bytes and starts at the 30k mark to support various programs that "go backwards". 
Each memory cell is a byte, the `+` and `-` routines wrap (pointer routines do not).

It will run mandlebrot and Towers of Hanoi, albeit quite slowly.

Comments in files are currently only supported via unused characters or a `[ ]` block at the start to skip over the contents if they contain any characters such as periods that need to be ignored.

## Missing Features

 - No real support for reading a file via `,`

### Improvements That Could Be Made

 - Mode for differentiating reading from a file or teminal
 - Storing loop start / end for quicker looping
 - JIT compilation (would fix above too)


