# bitcalc
This is a REPL for evaluating bitwise operations. It's a little bit easier to see what [some bitwise hacks are doing](https://stackoverflow.com/questions/1766535/bit-hack-round-off-to-multiple-of-8) when you see how each operation changes the bits on the underlying binary. For example, this one which rounds a number up to the nearest 8:

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

