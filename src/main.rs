use std::io::stdin;

/*
macro_rules! scan {
	($i:tt) => (Some($i))
}
*/

fn main() -> std::io::Result<()> {
	/*
	let handle = stdin();
	
	let mut buf = String::new();
	
	handle.read_line(&mut buf).unwrap();
	
	let linha: Vec<&str> = buf.split(" ").collect();
	
	let a = linha.get(0)
		.unwrap()
		.trim()
		.parse::<isize>()
		.unwrap_or_default();
	
	let b = linha.get(1)
		.unwrap()
		.trim()
		.parse::<isize>()
		.unwrap();
	
	let q = a/b;
	
	let r = a%b;
	
	println!("{} = {}x{}+{}", a, b, q, r);
	
	println!("{}", b*q+r);
	*/
	
    /*
	let i = scan!(1);

	println!("{:#?}", i);
	*/
	
	Ok(())
}































/*

let mut handle = handle.lines();
	
	while let Some(linha) = handle.next() {
		let n = linha.unwrap().trim().parse::<usize>().unwrap_or_default();
		
		if n != 0 {
			println!("vai ter duas!");
		}else{
			println!("vai ter copa!");
		}
	}

*/



/*
const TAM: usize = 15;

const INICALIZACAO_MATRIZ: usize = 1;

/*
let string: Vec<&str> = buf.split(" ").collect();

let x = string.get(0)
	.unwrap()
	.trim()
	.parse::<isize>()
	.unwrap_or_default();
	
	 .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();


fn media(s: [[Tipo; N]; N]) -> Tipo {
	(0..N).map(|i| s[i][i]).sum::<Tipo>()/N as Tipo
}
*/


fn formatar(matriz: &mut [[usize; TAM]; TAM], size: usize){
	let mut aux:usize = 1;
	
	for i in 0..size {
		let mut linha = aux;
		
		for j in 0..size {
			matriz[i][j] = linha;
			linha += linha;
		}
		
		aux += aux;
	}
}

fn printar(matriz: &[[usize; TAM]; TAM], size: usize) {
	let mut buf = String::new();
	
	let s: usize = match size {
		1 => {
			let t = format!("{}", matriz[size-1][size-1]);
			
			let tam: usize = t.chars().count();
			
			tam
		},
		_ => {
			let t = format!("{}", matriz[size-1][size-1]);
			
			let tam: usize = t.chars().count() + 1;
			
			tam
		},
	};
	
	for i in 0..size {
		for j in 0..size {
			
			let s = match  (j, s) {
				(_, 1) => format!("{:>1}", matriz[i][j]),
				
				(0, 2) => format!("{:>1}", matriz[i][j]),
				(_, 2) => format!("{:>2}", matriz[i][j]),
				
				(0, 3) => format!("{:>2}", matriz[i][j]),
				(_, 3) => format!("{:>3}", matriz[i][j]),
				
				(0, 4) => format!("{:>3}", matriz[i][j]),
				(_, 4) => format!("{:>4}", matriz[i][j]),
				
				(0, 5) => format!("{:>4}", matriz[i][j]),
				(_, 5) => format!("{:>5}", matriz[i][j]),
				
				(0, 6) => format!("{:>5}", matriz[i][j]),
				(_, 6) => format!("{:>6}", matriz[i][j]),
				
				(0, 7) => format!("{:>6}", matriz[i][j]),
				(_, 7) => format!("{:>7}", matriz[i][j]),
				
				(0, 8) => format!("{:>7}", matriz[i][j]),
				(_, 8) => format!("{:>8}", matriz[i][j]),
				
				(0, 9) => format!("{:>8}", matriz[i][j]),
				(_, 9) => format!("{:>9}", matriz[i][j]),
				
				(0, 10) => format!("{:>9}", matriz[i][j]),
				(_, 10) => format!("{:>10}", matriz[i][j]),
				
				(_, _) => format!("{:>2}", matriz[i][j]),
			};
			
			buf.push_str(s.as_str());
			
		}
		buf.push_str("\n");
	}
	buf.push_str("\n");
	print!("{}", buf);
}

/*
fn m(n: usize) -> usize {
	let mut aux: usize = 1;
	
	let mut maior = 1;
	
	for i in 0..n {
		let mut linha = aux;
		for j in 0..n {
			linha += linha;
			maior = linha;
		}
		aux += aux;
	}
	maior
}
*/
*/


