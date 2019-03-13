//does not work
//^ help: consider giving it a 'static lifetime: `&'static` 
//this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//fn terong()-> &str {
//	 "terong" 
//}

fn jamur(instr: & str)-> &str {
	instr
}

fn kol() -> &'static str {
	"kol"
	
}

fn sawi<'a>() -> &'a str {
	"sawi"
}

fn bayam() -> String {
	"bayam".to_string()
}

fn daunsingkong() -> String{
	let ret:  String  = "daun singkong".to_string();
	ret
}

fn gori(instr: String) -> String {
	let a = "gori";
	let b = format!("{} {}", a , instr);
	let c = format!("{} {}", a , instr);
	b
} 

fn goreng(instr1: &str, instr2: &str) {
	println!("{} goreng {}", instr1, instr2);
}

fn main() {
	let b = "jamur";

	//let trg = terong();
	let jmr = jamur(b);
	let kll = kol();
	let c = sawi();
	let d = sawi();
	let bym = bayam();
	let dns = daunsingkong();
	let gri = gori("bakar".to_string());

	println!("jmr {} kol {} sawi {} sawi {} ", jmr, kll, c, d);
	println!("bym {} dns {} gri{} ", bym, dns, gri);
	println!("jmr lagi {}", jmr);
	let jjmr = jmr;
	let jjjmr = jmr;
	println!("jjjmr {}", jjjmr);
}
