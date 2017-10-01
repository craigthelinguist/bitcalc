# bitcalc
This is a REPL for evaluating bitwise operations. It's a little bit easier to see what [some bitwise hacks are doing](https://stackoverflow.com/questions/1766535/bit-hack-round-off-to-multiple-of-8) when you see how the bits of a number are modified with each operation. For example, here's how you can round up to the next multiple of 8:

```
$ let x = 5
0000000000000101 (5)
$ (5 + 7)
0000000000001100 (12)
$ !7
1111111111111000 (65528)
$ (5 + 7) & !7
000000000000100 (8)
```

Run `cargo build` to compile the project.

