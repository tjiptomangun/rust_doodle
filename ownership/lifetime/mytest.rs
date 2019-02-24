fn terong()-> &str {
	instr
}

fn jamur(instr: & str)-> &str {
	instr
}

fn goreng(instr1: &str, instr2: &str) {
	println!("terong  {} goreng {}", instr1, instr2);
}

fn main() {
	let b = "belanda";
	let c = "mentega";

	let trg = terong(b);
	goreng(trg, c);
	let jmr = jamur(b);
	goreng(jmr, c);
}
