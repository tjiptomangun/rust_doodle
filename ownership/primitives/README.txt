From the book it is stated that all primitive types implement Copy trait.
That is why we can assign the variable to another unconditioanlly.
//https://doc.rust-lang.org/1.8.0/book/ownership.html 

The reason that we cannot use a binding after some objects is moved is that
those objects have references both in the stack and in heap.

Binding only copy stack value, not the heap values.
(please see simple.rs for some prove)


Primitive types in rust are:
1. booleans

2. char

3. numeric types (signed, unsigned, fixed, variable, 
   floating point and integer)
  which are: 
  i6, i16, i32, i64, 
  u8, u16, u32, u64,
  isize, usize
  f32, f64

  numeric types consists of two parts: the category and the size
  - the categories are signed and unsigned
  - the size are fixed and variable
    fixed size have specific number of bits in their representation
    variable size types depends on the size of pointer on the underlying machine.

4. Arrays (fixed-size list of elements of the same type). 
   By default arrays, are immutable

5. Slices. A slice is reference (or view into) another data structure. 
   They are useful for allowing safe, efficient access to a portion of 
   an array wihtout copying. You can use a combo of & and [] to create
   slice from various things.

6. str. str is the most primitive string types. It is become useful
   when placed behind reference, like &str. (strange , why is it in the 
   stack ?)

7. Tuples
   A tuple is an ordered list of fixed size.
   let x  = (1, "hello") which can be translated to
   let x: (i32, &str) = (1, "hello");
   &str is called string slice

8. Functions
   Functions also have a type and can be copied like below:

   fn foo(x: i32) -> i32 {x}
  
   let x: fn(i32) -> i32 = foo;

   It this case, x is a `function pointer` to a function that takes an i32
   and returns an i32
