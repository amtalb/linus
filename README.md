# Linus
A silly programming language named after a silly cat.

### Introduction
Linus is the newest addition to the hallowed pantheon of useless programming languages. It mostly serves as an exercise for me to 1) learn how to code in Rust and 2) learn how compilers work. Did I take on too much when I tried to accomplish these two tasks at the same time? Probably! Even though I have barely scratched the surface, it has been an awesome project so far and I am excited to keep working on it.

The name Linus comes from my cat, Linus. He is a strange cat that is not very good at being a cat. He is clumsy, overweight and a little odd. A perfect metaphor for this programming language!

![Linus the cat](linus.jpg)

### Syntax
#### Literals
- `num`: Numbers -- represented as `f64` in Rust
- `str`: Strings -- represented as `String` in Rust
- `bool`: Booleans -- represented as `bool` in Rust
- `none`: None -- represented as `None` in Rust
- Comments start with `#` and run until the end of the line

#### Operators
These are pretty self-explanatory so I will just list them:
- Arithmetic: `+`, `-`, `*`, `/`
- Comparison: `>`, `<`, `>=`, `<=`, `=` *note: this is "equals" in Linus, not assignment!
- Logical: `and`, `or`, `not`
- Linus does not have operator precedence as all expression are in prefix notation. In order to specify a different precedence, one can use parentheses, the application operator (`$`), or indentation.
```
+ 1 2                   # Evaluates to 3
and (not true) false    # Evaluates to true
and not true false      # Error!
+ 1 * 2 - 4 3           # Evaluates to 3
+ 1
    * 2
        - 4 3           # Evaluates to 3
> 1 $ + 1 2             # Evaluates to false
```
#### Variables (WIP)
- Variables must have type specified
```
# Assign a variable
def x: num -> 1
```

#### Functions (WIP)
- Linus is similar to Lisp languages in that just about everything is an expression and every expression is prefix. Because it's fun!
```
#  Define a function that takes two nums as arguments and returns the sum
def add_nums: num
    x: num y: num ->
    + x y 
```

### Development Status/Roadmap
Linus is in a *very* early stage at the moment, please do not expect to be able to write real software with it. This is just something I am doing for fun (and I have no idea what I am doing) so I will add features as I can. If you have any suggestions, please let me know! I would love to hear them.
- Variables
- Scoping/blocks
- Control Flow
- Functions
- Collection Types
- Structs/Enums
- Compile to LLVM
- Synchronize on parser when hitting an error
- Better error messages
- TESTING!