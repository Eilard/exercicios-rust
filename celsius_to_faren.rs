use std::io;

fn ler() -> String{
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Erro!");
	input.trim().to_string()
}

fn temperatura(numero: f32) ->f32{
	let resultado = numero * 9.0/5.0 + 32.0;
	resultado

}

fn main() {
    let input = ler();
    let numero: f32 = input.parse().expect("Digite a temperatura");
    let faren: f32 = temperatura(numero);
    println!("{faren}");

}
