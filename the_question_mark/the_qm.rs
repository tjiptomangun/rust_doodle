use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{Read, Write, Error};

fn main() {
        //The ? Operator Can Only Be Used in Functions That Return Result
        //https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html
        fn terong() -> Result<(), Error> { 
            let mut f = File::create("foo.txt")?;
            f.write_all(b"Hello World!");
            Ok(())
        }

        //is equivalent to
        fn wortel() -> Result<(), Error> { 
            let mut file = try!(File::create("my_best_friends.txt"));
            try!(file.write_all(b"This is a list of my best friends."));
            Ok(())
        }

        //let mut f = File::create("foo.txt")?;
        //let mut f = try!(File::create("foo.txt"));
        //f.write_all(b"Hello World!");
        terong();
        wortel();
}
