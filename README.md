# akin
> A zero-dependency crate for writing repetitive code easier and faster  

[![Tests](https://github.com/LyonSyonII/akin/actions/workflows/rust.yml/badge.svg)](https://github.com/LyonSyonII/akin/actions/workflows/rust.yml)         [![Crates.io](https://img.shields.io/crates/v/akin)](https://crates.io/crates/akin)

Check [Syntax](#syntax) for information on how to use it.  

1. [Why?](#why)
2. [Example](#example)
3. [Syntax](#syntax)
4. [NONE](#none)
5. [Joint modifier](#joint-modifier)
6. [Zero dependencies? Really?](#zero-dependencies-really)

## Why?
I've found myself having to write a lot of repetitive code (mostly when matching against enums in parsing).  
The fantastic [duplicate](https://crates.io/crates/duplicate) had an unintuitive syntax for me, so I decided to make my own tool.

## Example
```rust
trait Sqrt {
    fn dumb_sqrt(self) -> Result<f64, &'static str>;
}

akin! {
    let &int_type = [i64, u64];
    let &negative_check = [
        {
            if self < 0 {
                return Err("Sqrt of negative number")
            }
        }, 
        NONE
    ];
    
    let &num = [1,     2,    3,  4];
    let &res = [1., 1.41, 1.73,  2.];
    let &branch = {
        *num => Ok(*res),
    };
    
    impl Sqrt for *int_type {
        fn dumb_sqrt(self) -> Result<f64, &'static str> {
            *negative_check
            
            match self {
                *branch
                _ => Err("Sqrt of num not in [1, 4]")
            }
        }
    }
}
```
Is expanded to:
```rust
trait Sqrt {
    fn dumb_sqrt(self) -> Result<f64, &'static str>;
}

impl Sqrt for i64 {
    fn dumb_sqrt(self) -> Result<f64, &'static str> {
        if self < 0 {
            return Err("Sqrt of negative number")
        }
        
        match self {
            1 => Ok(1.),
            2 => Ok(1.41),
            3 => Ok(1.73),
            4 => Ok(2.),
            _ => Err("Sqrt of num not in [1, 4]")
        }
    }
}

impl Sqrt for u64 {
    fn dumb_sqrt(self) -> Result<f64, &'static str> {
        match self {
            1 => Ok(1.),
            2 => Ok(1.41),
            3 => Ok(1.73),
            4 => Ok(2.),
            _ => Err("Sqrt of num not in [1, 4]")
        }
    }
}
```

The good thing about **akin** is that it detects automatically the number of values of each variable *for each scope*, so for example "branch" will get copied 4 times (as "num" and "res" both have 4 values), but the main function will only be duplicated once, as all the variables it has have 2 values.

Check the [tests/](https://github.com/LyonSyonII/akin/tree/main/tests) folder of the repository for more examples.

## Syntax
The crate only provides one macro, `akin!`.
The syntax is as follows:

First, you declare the variables you'll use. 
A variable name must start with `&`, as it's the only way it can differentiate between macro or real declarations.  
Also notice that variables end with `;`

```rust
let &variable = [v1, v2, v3, ...];
let &variable2 = [...];
    
let &code = {
    ...
};
```

Then, when all variables have been declared, you can write the code snippet you want to duplicate.  
The amount of times it will be duplicated depends on the variables that are used.  

```rust
let &lhs = [1, 2, 3];
let &rhs = [4, 5, 6];
println!("*lhs + *rhs = {}", *lhs + *rhs);
```

Will get expanded to:

```rust
println!("1 + 4 = {}", 1 + 4);
println!("2 + 5 = {}", 2 + 5);
println!("3 + 6 = {}", 3 + 6);
```
Because the variables `&lhs` and `&rhs` both have 3 values.

As you can see, `&` is used to declare variables and `*` is used to "dereference" them to the current value.

As a matter of convenience, the range syntax is also accepted, when declaring a variable,
e.g. `0..3` and `0..=3`, which are equivalent to `[0,1,2]` and `[0,1,2,3]` respectively.
So the above example could also be written like

```rust
let &lhs = 1..=3;
let &rhs = 4..=6;
println!("*lhs + *rhs = {}", *lhs + *rhs);
```

Presently, only unsigned integers that can fit in `u64` are supported in ranges, i.e. ranges
like `-10..-1` or `'a'..'c'`, which are fine in regular Rust, aren't accepted by `akin`.

If a used variable has less values than another, the last one will be used.

```rust
akin! {
    let &v1 = [c];
    let &v2 = [a, b];
    println!("*v1*v2");
}
```
Expands to
```rust
println!("ca");
println!("cb");
```

All code in variables must be enclosed in brackets `{...}`.
```rust
akin! {
    let &var = [-1, 1];
    let &code = [
        {
            println!("true");
        },
        {
            println!("false");
        }
    ];
    
    if *var < 0 {
        *code
    }
}
```
Will expand to
```rust
if -1 < 0 {
    println!("true");
}
if 1 < 0 {
    println!("false")
}
```

Check the [tests/](https://github.com/LyonSyonII/akin/tree/main/tests) folder of the repository for more examples.

## NONE
`NONE` is the way you can tell `akin` to simply skip that value and not write anything.  
It is useful for when you want to have elements in a duplication that do not have to be in the others.
```rust

akin! {
    let &num = [1, 2, 3];
    let &code = [
        NONE,
        {
            .pow(2)
        }
    ];
    
    println!("*num^2 = {}", *num~u32*code);
    // *num~u32 is necessary to ensure the type is written correctly (it would be "1 u32" otherwise)
}
```

## Joint modifier
By default, `akin` places a space between all identifiers

```rust    
let name = 5; // 'name' is an identifier
    ^^^^
```
Sometimes, this is not desirable, for example, if trying to interpolate between a function name
```rust
let &name = [1];
fn _*name()...

// Will get wrongly expanded because '_' is an identifier
fn _ 1()
```
To avoid it, use the joint modifier `~`, making the next identifier not to be separated.
```rust    
let &name = [1];
fn _~*name()... // *name is affected by the modifier

// Will get correctly expanded to
fn _1()
```
Inside string literals `"..."` it is not necessary to use the modifier, as Rust does not count them as identifiers.

This is a limitation on proc-macro parsing, so I doubt it'll be fixed soon.

## Zero dependencies? Really?
Yes, this crate does not use `syn` nor `quote`, as parsing the syntax is pretty simple and both add a lot of overhead.  
For this reason, `akin` should not impact compile times as much as most proc-macros, try using it and see it by yourself!
