use std::io;

fn main() {
    loop {
        let mut numero = String::new();

        match io::stdin().read_line(&mut numero) {
            Err(_) => {
                println!("Erro au tentar ler o valor, tente novamente ");
                continue;
            },
            _ => {}
        }
        let numero: i32 = match numero.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Digite um valor numerico!!!");
                continue;
            }
        };

        if numero < 0 {
            println!("Digite um valor positivo!!!");
            continue;
        }

        let soma = calcular_soma(numero);
        println!("{soma}");
        break;
    }
    // Calcular o valor
}

fn calcular_soma(valor: i32) -> i32 {
    let n = valor * 2;
    let mut soma = 0;
    let mut count = 0;
    while count <= n {
        if count % 2 == 0 {
            soma += count;
        }
        count += 1;
    }

    soma
}
