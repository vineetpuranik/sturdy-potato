# Concepts

## Rust prelude
"The Prelude" is a list of things that Rust automatically imports in each program. Its kept as small as possible and is focussed on traits.

"The Prelude" vs "Other Preludes" : distinction is that other preludes need to imported before being used. 

## Rust variables
In Rust, variables are immutable by default. 
let myvar = 10; // The variable myvar is immutable and cannot be assigned a new val

In order to define variables whose value can be changed later on we use the mut keyword
let mut myvar = 10;

Similar to variables , references in Rust also need to be associated with the mut keyword in order to change the underlying value associated with the reference.

## Rust Strings
Rust String types are stored with UTF-8 encoding under the hood. In UTF-8 each character can be represented between 1 to 4 bytes. 
So, if declare a string "Four" it will be stored as 46 6f 75 72 where each byte represents the hexadecimal representation of each character.

1 byte can represent 00 to FF in hex

## Rust crates
Crate is a collection of Rust source files.
Rust crates can be either binary crates or library crates. Library crates are intended to be used in other programs and are not exceutable on their own. 

