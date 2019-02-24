//does not work
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
}
