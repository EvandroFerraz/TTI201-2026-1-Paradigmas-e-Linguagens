// ============================================================
// Exercício 2 — Verificador de Número Primo
// Um número é primo se for maior que 1 e divisível
// apenas por 1 e por ele mesmo.
// Exemplos primos:    2, 3, 5, 7, 11, 13...
// Exemplos não primos: 1, 4, 6, 8, 9, 10...
// ============================================================

// Importa o módulo de entrada/saída para ler do teclado.
use std::io;

// ------------------------------------------------------------
// Função: eh_primo
// Recebe um número sem sinal de 32 bits (u32 — nunca negativo)
// e retorna true se for primo, false caso contrário.
//
// Otimização: só testamos divisores até a raiz quadrada de n.
// Se n não tem divisor até √n, não terá nenhum além disso.
// Exemplo: para n=36, √36=6 — basta testar até 6.
// ------------------------------------------------------------
fn eh_primo(n: u32) -> bool {
    // Números menores ou iguais a 1 não são primos por definição
    if n <= 1 {
        return false;
    }

    // 2 é o único número primo par — tratado como caso especial
    if n == 2 {
        return true;
    }

    // Todo número par maior que 2 é divisível por 2, logo não é primo
    if n % 2 == 0 {
        return false;
    }

    // Calcula o limite de verificação: raiz quadrada de n
    // "as f64" converte u32 para ponto flutuante para usar sqrt()
    // "as u32" converte o resultado de volta para inteiro
    let limite = (n as f64).sqrt() as u32;

    // Testa divisores ímpares de 3 até √n
    // O ..= indica range inclusivo (inclui o valor de `limite`)
    for i in 3..=limite {
        // Se n for divisível por qualquer i, não é primo
        if n % i == 0 {
            return false;
        }
    }

    // Se passou por todos os testes sem retornar false, é primo
    true
}

// ------------------------------------------------------------
// Função principal: lê um número do usuário e exibe o resultado
// ------------------------------------------------------------
fn main() {
    println!("Digite um número inteiro positivo:");

    // Cria uma string vazia para armazenar o que o usuário digitar
    let mut entrada = String::new();

    // Lê uma linha do teclado e armazena em `entrada`
    // &mut passa uma referência mutável — exigido pelo borrow checker
    // expect define a mensagem de erro caso a leitura falhe
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    // trim() remove espaços e a quebra de linha do final da string
    // parse() tenta converter para u32 (inteiro sem sinal)
    // match avalia o resultado: Ok = conversão ok, _ = qualquer erro
    let numero: u32 = match entrada.trim().parse() {
        Ok(n) if n > 0 => n, // Aceita apenas números positivos válidos
        _ => {
            // Captura qualquer outro caso: zero, texto, número negativo...
            println!("Por favor, digite um número inteiro positivo válido.");
            return; // Encerra o programa sem continuar
        }
    };

    // Chama a função e exibe o resultado conforme o retorno booleano
    if eh_primo(numero) {
        println!("{} é um número primo!", numero);
    } else {
        println!("{} não é um número primo.", numero);
    }
}
