// ============================================================
// Exercício 1 — Verificador de Número Feliz
// Um número é "feliz" se a soma repetida dos quadrados
// dos seus dígitos eventualmente chega a 1.
// Caso entre em ciclo infinito sem chegar a 1, é "infeliz".
// Exemplo feliz:  19 → 82 → 68 → 100 → 1 ✓
// Exemplo infeliz: 17 → entra em ciclo sem chegar a 1 ✗
// ============================================================

// Importa HashSet: estrutura que armazena valores únicos.
// Usada para detectar se um número já foi processado (ciclo).
use std::collections::HashSet;

// Importa o módulo de entrada/saída para ler do teclado.
use std::io;

// ------------------------------------------------------------
// Função: soma_quadrados
// Recebe um número inteiro (mut = pode ser modificado internamente)
// e retorna a soma dos quadrados de cada um dos seus dígitos.
// Exemplo: 82 → 8² + 2² = 64 + 4 = 68
// ------------------------------------------------------------
fn soma_quadrados(mut n: i32) -> i32 {
    let mut soma = 0;

    // Percorre cada dígito do número da direita para a esquerda
    while n > 0 {
        let digito = n % 10;   // Pega o último dígito (ex: 82 % 10 = 2)
        soma += digito * digito; // Acumula o quadrado do dígito
        n /= 10;               // Remove o último dígito (ex: 82 / 10 = 8)
    }

    soma // Retorna a soma (última expressão sem ; = valor de retorno)
}

// ------------------------------------------------------------
// Função: eh_feliz
// Verifica se um número é feliz repetindo a soma dos quadrados.
// Retorna true se chegar a 1, ou false se detectar ciclo.
// ------------------------------------------------------------
fn eh_feliz(mut n: i32) -> bool {
    // HashSet guarda os números já vistos para detectar ciclos
    let mut vistos = HashSet::new();

    // Continua enquanto não chegar a 1 e o número ainda não foi visto
    while n != 1 && !vistos.contains(&n) {
        vistos.insert(n);        // Registra o número atual como visitado
        n = soma_quadrados(n);   // Calcula o próximo valor da sequência
    }

    // Se o loop parou porque n == 1, é feliz; senão entrou em ciclo
    n == 1
}

// ------------------------------------------------------------
// Função principal: lê um número do usuário e exibe o resultado
// ------------------------------------------------------------
fn main() {
    println!("Digite um número inteiro positivo:");

    // String::new() cria uma string vazia para receber a entrada
    let mut entrada = String::new();

    // read_line lê o que o usuário digitou e armazena em `entrada`
    // O & passa uma referência mutável (necessário para o borrow checker)
    // expect interrompe o programa com mensagem se a leitura falhar
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    // trim() remove espaços e a quebra de linha (\n) do final
    // parse() tenta converter a string para i32
    // match trata os dois possíveis resultados: Ok (sucesso) ou erro
    let numero: i32 = match entrada.trim().parse() {
        Ok(n) if n > 0 => n, // Aceita apenas se for número positivo
        _ => {
            // O _ captura qualquer outro caso (negativo, zero, texto...)
            println!("Por favor, digite um número inteiro positivo válido.");
            return; // Encerra o programa antecipadamente
        }
    };

    // Chama a função e exibe o resultado com base no retorno (true/false)
    if eh_feliz(numero) {
        println!("{} é um número feliz!", numero);
    } else {
        println!("{} não é um número feliz.", numero);
    }
}
