# Rust-Brainfuck

A simple Rust brainfuck interpreter written in rust.

## Brainfuck

Brainfuck is a very simple language, the best description of which I think can be found [here](https://esolangs.org/wiki/Brainfuck).

A cool list of programs written in Brainfuck is available [here](http://esoteric.sange.fi/brainfuck/bf-source/prog/).
I reccomend the hanoi.bf script or the mandelbrot.bf script! 

![hanoi](https://cloud.githubusercontent.com/assets/1008996/24750668/98c476ec-1abe-11e7-9008-1919fecc499d.png)
![mandelbrot](https://cloud.githubusercontent.com/assets/1008996/24751578/0bbd3bfe-1ac2-11e7-970b-2f5652aac4d6.png)

## Usage Examples

### Interpreting a bf file

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
Each memory cell is a signed byte, the `+` and `-` routines wrap (pointer routines do not).

It will run mandlebrot and Towers of Hanoi, albeit quite slowly.

Comments in files are currently only supported via unused characters or a `[ ]` block at the start to skip over the contents if they contain any characters such as periods that need to be ignored.

## Missing Features

 - No real support for reading a file via `,`

### Improvements That Could Be Made

 - JIT compilation


