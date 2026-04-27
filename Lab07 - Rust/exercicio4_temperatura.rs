// ============================================================
// Exercício 4 — Conversor de Temperatura
// Converte uma temperatura em Celsius para Fahrenheit e Kelvin.
//
// Fórmulas utilizadas:
//   Fahrenheit : F = (C × 9/5) + 32
//   Kelvin     : K = C + 273.15
//
// Exemplo de entrada e saída:
//   Digite a temperatura em Celsius: 25
//   25.00°C = 77.00°F = 298.15K
// ============================================================

// Importa o módulo de entrada/saída para ler do teclado.
use std::io;

// ------------------------------------------------------------
// Função: para_fahrenheit
// Recebe uma temperatura em Celsius como f64 (ponto flutuante
// de 64 bits) e retorna o equivalente em Fahrenheit.
//
// Por que f64 e não i32?
// Temperaturas frequentemente têm casas decimais (ex: 36.5°C).
// f64 é o tipo de ponto flutuante padrão do Rust para precisão.
// ------------------------------------------------------------
fn para_fahrenheit(c: f64) -> f64 {
    // Os literais são 9.0 e 5.0 (não 9 e 5) porque Rust não converte
    // inteiro em float automaticamente — os tipos precisam ser iguais
    (c * 9.0 / 5.0) + 32.0
}

// ------------------------------------------------------------
// Função: para_kelvin
// Recebe uma temperatura em Celsius e retorna o equivalente
// em Kelvin. Função de expressão única — sem chaves nem return.
// ------------------------------------------------------------
fn para_kelvin(c: f64) -> f64 {
    // 273.15 é a diferença entre o zero absoluto (0K) e 0°C
    c + 273.15
}

// ------------------------------------------------------------
// Função principal: lê a temperatura e exibe as conversões
// ------------------------------------------------------------
fn main() {
    println!("Digite a temperatura em graus Celsius:");

    // Cria uma string vazia para armazenar o que o usuário digitar
    let mut entrada = String::new();

    // Lê uma linha do teclado e armazena em `entrada`
    // &mut passa referência mutável — exigido pelo borrow checker
    // expect define mensagem de erro caso a leitura falhe
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    // trim() remove espaços e a quebra de linha do final da string
    // parse() converte para f64 — aceita decimais como "36.5"
    // Celsius pode ser negativo, então não usamos u64 aqui
    let celsius: f64 = match entrada.trim().parse() {
        Ok(n) => n,  // Conversão bem-sucedida: usa o número
        Err(_) => {  // Falha na conversão: texto, vírgula no lugar de ponto, etc.
            println!("Por favor, digite um número válido.");
            return;  // Encerra o programa sem continuar
        }
    };

    // Chama as funções de conversão e armazena os resultados
    let fahrenheit = para_fahrenheit(celsius);
    let kelvin = para_kelvin(celsius);

    // Exibe os três valores com 2 casas decimais cada
    // {:.2} é o especificador de formato: . = casas decimais, 2 = quantidade
    println!("{:.2}°C = {:.2}°F = {:.2}K", celsius, fahrenheit, kelvin);
}
