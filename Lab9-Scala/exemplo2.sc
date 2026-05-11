// Exemplo 2 – Cálculo da soma de dois vetores de tamanho 6

object SomaDeVetores{

  def somarVetores(vetor1: Array[Int], vetor2: Array[Int]): Array[Int] = {
    //require() verifica uma condição e se a condição for falsa mostra um erro
    //caso contrario, continua normalmente
    require(vetor1.length == vetor2.length && vetor1.length == 6, 
    "Os vetores devem ter o mesmo tamanho (6).")

    // a linha abaixo é o ultimo termo calculado, ou seja, é o termo
    // retornado pela função
    vetor1.zip(vetor2).map{case(x,y) => x+y}

    // vetor1 = {1,2,3,4,5,6}
    // vetor2 = {7,8,9,10,11,12}
    // vetor1.zip(vetor2) = {(1,7), (2,8), (3,9), (4,10), (5,11), (6,12)}
    // case(x=1,y=7) => 1+7 = 8
    // case(x=2,y=8) => 2+8 = 10
    // case(x=3,y=9) => 3+9 = 12
    //...
    // case(x=6,y=12) => 6+12 = 18
    // resultado = {8, 10, 12, 14, 16, 18}  
  }

  def main(args: Array[String]):Unit = {
    val vetor1 = Array(1,2,3,4,5,6)
    val vetor2 = Array(6,5,4,3,2,1)

    val resultado = somarVetores(vetor1,vetor2)

    println("Vetor1: " + vetor1.mkString(", "))
    println("Vetor2: " + vetor2.mkString(", "))

    println("Resultado da soma: " + resultado.mkString(", "))
  } 
}