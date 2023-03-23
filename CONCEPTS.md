# Concepts

## Rust prelude
"The Prelude" is a list of things that Rust automatically imports in each program. Its kept as small as possible and is focussed on traits.

"The Prelude" vs "Other Preludes" : distinction is that other preludes need to imported before being used. 

## Rust variables
In Rust, variables are immutable by default. 
Based on this concept, when you state that a value won’t change, it really won’t change, so you don’t have to keep track of it yourself. Your code is thus easier to reason through.he Rust compiler makes sure that when a value when you state that a value won’t change, it really won’t change, so you don’t have to keep track of it yourself. Your code is thus easier to reason through.

```
let myvar = 10; // The variable myvar is immutable and cannot be assigned a new val
```

In order to define variables whose value can be changed later on we use the mut keyword

`let mut myvar = 10;`

Similar to variables ,references in Rust also need to be associated with the mut keyword in order to change the underlying value associated with the reference.

## Rust Strings
Rust String types are stored with UTF-8 encoding under the hood. In UTF-8 each character can be represented between 1 to 4 bytes. 
So, if declare a string "Four" it will be stored as 46 6f 75 72 where each byte represents the hexadecimal representation of each character.

1 byte can represent 00 to FF in hex

## Rust crates
Crate is a collection of Rust source files.
Rust crates can be either binary crates or library crates. Library crates are intended to be used in other programs and are not exceutable on their own. 

## Rust Scalar Types and Compond types
Scalar variable type is associated with a single value. Rust has 4 primary scalar types : integers, floating-point numbers, booleans and characters.

Compound types can group multiple values into one type. Rust has 2 primitive compound types : tupples and arrays.

## Rust tuples
A tuple is a general way of grouping together a number of values of variety of types into one compound type. 
```
fn main() {
    let tup: (i32, f64, u38) = (500, 6.4, 1);   
}
```
To get individual values out of a tuple we can use pattern matching to destructure a tuple value
```
fn main() {
    let tup: (i32, f64, u38) = (500, 6.4, 1);   
    let (x, y, z) = tup;
    println!("The value of y is {y}");
}
```
We can also access the tuple element directly by using a . and the index of the value we want to access
```
fn main() {
    let tup: (i32, f64, u38) = (500, 6.4, 1);   
    let five_hundred = tup.0;
    let six_point_found = tup.1;    
    
}
```

## Rust Arrays
If we want to represent a collection of values of the same type then we can use an Array. 

Arrays are useful when you want your data to be allocated on the stack rather than the heap or when you want to ensure that you always have a fixed number of elements. 

A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you are unsure whether to use array or vector , chances are you should use a vector. 

Arrays are more useful when you know the number of elements will not change.

## Rust if else
Simple example of if else in Rust
```
fn main() {
    let number = 2;
    if number < 2 {
        println!("I dont get it");
    }
    else {
        println!("I get it");
    }
}

```
The condition in the if and else if statements should always be a boolean condition. 
Using too many else if statements can clutter your code. Consider using match for such cases.

`if` is an expression and we can use it at the right side of a `let` statement to assign the outcome to a variable

```
fn main() {
    let condition = true;
    let x = if condition { 5 } else { 6 };
}

```

## Rust loop
The `loop` keyword tells Rust to execute a block of code over and over again or until you explicitly tell it to stop. `break` can be used to exit the loop and `continue` can be used to continue the loop execution by skipping the statements in the current iteration and beginning from the start of the loop. 

```
fn main() {
    loop {
        println!("print till stopped !!");
    }
}

```

Returning values from Loops
```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

```
If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a loop label on a loop and use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. 

```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
         loop {
            println("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            
            if count == 2 {
                break 'counting_up;
            }
            remaining = -1;        
         
         }
    
        count += 1;
    
    }


}


```

## Rust Vectors

Vectors allow us to store more than one value of the same type in a single data structure so that all the values are put next to each other in the memory.

Creating mutable vectors allows you to add elements to the vector after is has been defined (reason why you would use vectors over arrays).

Create an empty vector to hold values of type i32. 
Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
Vec<T> type provided by the standard library can hold any type.
Vec<T> in v will hold elements of the i32 type.

```
let v: Vec<i32> = Vec::new();
```

More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation. Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.    

```
   let v = v![1, 2, 3];
```
Updating a vector
```
let mut v = vector![1, 2, 3, 4, 5];
v.push(6);
v.push(7);    
    
```    
