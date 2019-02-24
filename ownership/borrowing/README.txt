Instead of moving that cause ownership of a resource moved, we can borrow 
ownership. 
Borrowing will not cause an object change ownership. 
Borrowing ownership does not deallocate resource when it out of scope 
like moving does.
This means after call to function that accept borrow reference, we
can use our original binding again.
Common borrow method are using type references (&T) and pass this 
reference to method or other scope.

On &mut references
The rules about borrowing in Rust:
1. A borrow must last for a scope no greater than the owner.
	//Also true to me: Ownership must live longer than borrowship
2. You may have one or the other of these two kinds of borrows,
   but not both at the same time:
   a. One or more references (&T) to a resource
   b. exactly one mutable reference

This is very similar to definition on a data race:
There is a data race when to or more pointers access the same
memory location at the same time, where at least one of them
is writing, and the operations are not synchronized.

In rust borrowing is tied to the scope that the borrow is valid for
    

	let mut x = 5; 
	{
		let y = &mut x; //<-- borrow starts here
		...
		...
	}                   //<-- borrow ends here
	


See borrowing.rs
https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html


