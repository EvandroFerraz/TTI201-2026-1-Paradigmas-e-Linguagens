// Exemplos
fn main() {
    // Variáveis mutáveis e imutáveis
    let x = 5;          // imutável por padrão
    let mut y = 10;     // mut = mutável
    y = y + x;
    println!("y = {}", y); // y = 15
    
    // O sistema de ownership
    let s1 = String::from("olá");
    let s2 = s1; // s1 é "movido" para s2, s1 não existe mais!
    println!("{}", s2); // ok
    // println!("{}", s1); // ERRO de compilação!
}
