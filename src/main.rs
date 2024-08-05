use std::io;

fn main() {
    println!("Digite um número para ver sua tabuada:");

    let mut numero = String::new();

    io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler a entrada do usuário");

    let numero: u32 = numero.trim().parse().expect("Por favor digite um número inteiro");

    for i in 1..11 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}