fn main() {
    println!("Bem Vindo a Calculadora");
    let result: f64;
    result = operar(0.5, '/', 0.9);
    print!("{}", result)
}

fn operar(first: f64, operador: char, second: f64) -> f64 {
    match operador {
        '+' => first + second,
        '-' => first - second,
        '/' => first / second,
        '*' | 'x' | 'X' => first * second,
        _ => panic!("Operador Invalido"),
    }
}
