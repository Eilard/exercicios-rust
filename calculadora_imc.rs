/* 
peso kg = float f32
altura cm = float f32
calculo peso / (altura * altura)

*/

use std::io;

fn ler() -> String{
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("erro");
	input.trim().to_string()
}

fn calculadora_imc(n1: f32, n2: f32) -> f32{
	let resultado = n1 / (n2 * n2);
	resultado
}
fn main() {
	println!("Digite seu peso:");
	let peso = ler();
	let n1: f32 = peso.parse().expect("erro!");

	println!("Digite sua altura:");
	let altura = ler();
	let n2: f32 = altura.parse().expect("erro!");

	let resultado: f32 = calculadora_imc(n1, n2);
	println!("{resultado:.2}");

  
}
