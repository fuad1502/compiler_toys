# JJIK Simple Calculator 🧮

This is an example project meant to show how to utilize [JJIK](https://github.com/fuad1502/compiler_toys/tree/master/jjik) for your parsing needs.

The binary simply prompts you to input an arithmetic expression, if the expression can be parsed, it prints the concrete syntax tree representation of your input and the result of evaluating the expression.

```
>>> 5 * (1 + 2)
E(3):
    E(6):
        5
    *
    E(5):
        (
        E(1):
            E(6):
                1
            +
            E(6):
                2
        )
Result = 15
```
If JJIK cannot parse the expression, it will print out a message describing why the parsing failed.

```
>>> 5 * +  
Line   1|5 * +
             ^
error: found Plus, expected: [Number, LeftParen]
```
Enter `CTRL-D` to terminate the program.

## Installation

```sh
cargo install jjik-simple-calculator
```
