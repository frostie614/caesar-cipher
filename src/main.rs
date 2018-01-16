use std::env;

fn main(){


	let args:Vec<String> = env::args().collect();

	match args.len() {
		1 | 2 => {println!("Caesar requires at least 3 inputs of the form (caesar, msg, shift, decode)");},
		3 | 4 => {
				let m = &args[1];
				let k = &args[2];
				let mut decode = &String::from("false");
				if args.len() == 4 {
					decode = &args[3];
				}

				let msg:String = match m.parse(){
					Ok(m) => {m},
	    			Err(_) => {println!("Invalid input"); return;},
				};

				let shift:u8 = match k.parse() {
					Ok(k) => {k},
					Err(_) => {
						println!("The second input should be between 1:26"); return;
					},
				};

				let decode:bool = match decode.parse() {
					Ok(decode) => {decode},
					_ => {println!("Decode input needs to be a boolean (true or false)"); return;}
				};

				let output = caesar(&msg, shift, decode);

				println!("{:?}", msg);
				println!("{:?}", output);
	    },
		_ => {println!("Too many inputs. Should be at least 2, max 3, of the form (msg, shift, decode)");}
	}
}

fn caesar(msg:&String, mut shift:u8, decode:bool) -> String{

	if decode {
		shift = 26 - (shift % 26)  ;
	}

	let cipher:String = msg.chars().map(|c|{
		let offset:u8 = if c.is_uppercase() {'A'} else {'a'} as u8;
		if c.is_alphabetic() {(((c as u8 - offset + shift) % 26) + offset)  as char } else {c}
	})
	.collect();
	cipher
}






