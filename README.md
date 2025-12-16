# rlox

An implementation of the bytecode virtual machine for Lox, the programming language in [Crafting Interpreters](https://craftinginterpreters.com/). This project follows the book's `clox` interpreter but is implemented in Rust instead of C.

At this moment it can handle simple programs like:

```java
var x = 10;

if (x > 5 and x < 15) {
  print "x is in range";
} else {
  print "x is out of range";
}
```
