// ============================================================
// Exercício 6 — Verificador de Palíndromo
// Uma palavra ou frase é palíndromo se pode ser lida da mesma
// forma da esquerda para a direita e da direita para a esquerda,
// ignorando espaços, acentuação e maiúsculas/minúsculas.
//
// Exemplos palíndromos:
//   "arara"    → a-r-a-r-a invertido = a-r-a-r-a ✓
//   "Ame a ema" → (limpo) ameaema invertido = ameaema ✓
//   "Socorram-me subi no ônibus em Marrocos" ✓
//
// Exemplos não palíndromos:
//   "OpenAI"   → openai invertido = ianepo ✗
// ============================================================

// Importa o módulo de entrada/saída para ler do teclado.
use std::io;

// ------------------------------------------------------------
// Função: limpar
// Recebe uma fatia de string (&str — referência, não possui os dados)
// e retorna uma String nova contendo apenas letras e números,
// todos convertidos para minúsculo.
//
// Isso garante que "Ame a ema" e "ameaema" sejam tratados igual,
// ignorando espaços, hífens, acentos e diferenças de capitalização.
// ------------------------------------------------------------
fn limpar(s: &str) -> String {
    s.chars()                        // Cria um iterador sobre cada caractere Unicode
     .filter(|c| c.is_alphanumeric()) // Mantém apenas letras e dígitos (descarta espaços, vírgulas, hífens...)
     .map(|c| c.to_ascii_lowercase()) // Converte cada caractere para minúsculo
     .collect()                       // Consome o iterador e monta uma String com o resultado
}

// ------------------------------------------------------------
// Função: eh_palindromo
// Recebe uma fatia de string, limpa o texto e compara
// a string resultante com ela mesma invertida.
// Retorna true se for palíndromo, false caso contrário.
// ------------------------------------------------------------
fn eh_palindromo(s: &str) -> bool {
    // Limpa o texto removendo caracteres irrelevantes
    let limpo = limpar(s);

    // chars() cria iterador de caracteres
    // rev() inverte a ordem do iterador
    // collect::<String>() especifica explicitamente o tipo de destino
    // — necessário porque collect() pode montar vários tipos diferentes
    let invertido = limpo.chars().rev().collect::<String>();

    // Compara a string limpa com a versão invertida
    // Se forem iguais, é palíndromo
    limpo == invertido
}

// ------------------------------------------------------------
// Função principal: lê o texto do usuário e exibe o resultado
// ------------------------------------------------------------
fn main() {
    println!("Digite uma palavra ou frase:");

    // Cria uma string vazia para armazenar o que o usuário digitar
    let mut entrada = String::new();

    // Lê uma linha do teclado incluindo espaços (diferente de scanf em C)
    // &mut passa referência mutável — exigido pelo borrow checker
    // expect define mensagem de erro caso a leitura falhe
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    // trim() remove espaços e a quebra de linha do início e fim da string
    // Não usamos parse() aqui pois o texto já é uma String — não precisa converter
    let texto = entrada.trim();

    // Chama a função e exibe o resultado
    // {:?} é o formato de debug: exibe a string entre aspas,
    // deixando claro ao usuário exatamente qual texto foi avaliado
    if eh_palindromo(texto) {
        println!("{:?} e um palindromo!", texto);
    } else {
        println!("{:?} nao e um palindromo.", texto);
    }
}
