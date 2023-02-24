# Macros

This is a set of proc macros that I use.

- ```#[no_unwrap]```
This proc macro detects unwraps in the function body and returns error if there is any unwrap.

- ```#[derive(Builder)]```
This proc macro creates a builder struct for the given struct. Panics if there is no fields in the struct

- ```#[optinal]```
This proc macro creates an Optional struct which makes all the fields option.The new struct includes all the attributes
from parent struct. Useful while updating a struct in the Database.

- ```ternary!(2==2?"Yes":"No")```
This proc macro makes you to use ternary operators like it is in java in rust.
