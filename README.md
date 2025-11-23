# rlox

An implementation of the bytecode virtual machine for Lox, the programming language in [Crafting Interpreters](https://craftinginterpreters.com/). This project follows the book's `clox` interpreter but is implemented in Rust instead of C.

At this moment it can handle simple programs like: 

```java
{
  var a = 1;
  print a;
  a = 2;
  print a;
  {
    var b = a + 1;
    print b;
  }
}
```
