# akin
Crate for writing repetitive code easier and faster.
Check [Syntax](#syntax) for information about how to use it.

## Why?
I've found myself having to write a lot of repetitive code (mostly when matching against enums in parsing).  
The fantastic [duplicate](https://crates.io/crates/duplicate) had a sort of unintuitive syntax, so I decided to make my own tool.

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
            1 => Ok(1),
            2 => Ok(1.41),
            3 => Ok(1.73),
            4 => Ok(2),
            _ => Err("Sqrt of num not in [1, 4]")
        }
    }
}

impl Sqrt for u64 {
    fn dumb_sqrt(self) -> Result<f64, &'static str> {
        match self {
            1 => Ok(1),
            2 => Ok(1.41),
            3 => Ok(1.73),
            4 => Ok(2),
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

## Raw modifier
By default, `akin` places a space between all identifiers

```rust    
let name = 5; // 'name' is an identifier
    ^^^^
```
Sometimes, this is not desirable, for example, if trying to interpolate between a function name
```rust
    let &name = [1];
    fn _*name()...
    
    // Will get wrongly expanded to, because '_' is an identifier
    fn _ 1()
```
To avoid it, use the raw `#` modifier, making the identifier next to the one it affects to not be separated.
```rust    
let &name = [1];
fn #_*name()... // *name() is affected by the modifier
// Will get correctly expanded to
fn _1()
```
This is a limitation on proc_macro parsing, so I doubt it'll be fixed soon.