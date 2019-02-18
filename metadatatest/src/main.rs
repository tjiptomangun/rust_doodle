use std::fs;
use std::env;
use std::io::Read;
use std::io::Write;

fn main()  {
/*
	let arguments: Vec<_> = env::args().collect();
	let fname = &arguments[1];
	let metadata = fs::metadata(fname)?;
*/
	if let Some(arg1) = env::args().nth(1) {
		let metadata = fs::metadata(&arg1).unwrap();
		println!("file type {:?} ", metadata.file_type());
		println!("file is file {:?} ", metadata.is_file());
		println!("file is dir {:?} ", metadata.is_dir());
		println!("file length {:?} ", metadata.len()); 

		let mut file = fs::File::open(&arg1).unwrap();
		let mut buffer = Vec::new();

		buffer.clear(); 
		file.read_to_end(&mut buffer).expect("cannot read buffer into file"); 
        println!("buff length {:?}", buffer.len()); 
	    let mut outp = fs::File::create("output.dat").unwrap(); 
        outp.write(&buffer).expect("cannot write buffer to file");
    }
	else {
		println!("Need filename");
	}

}
