object CalculadoraFatorial{
  // Wrapper
  def calcularFatorial(numero:Int):BigInt = {
    if(numero == 0) 1
    // (1 to 5) = {1, 2, 3, 4, 5}
    else(1 to numero).map(BigInt.apply).product
  }

  // static void main(String[] args)
  def main(args: Array[String]):Unit = {

      val numero = 5;

      if(numero < 0){
        println("Não é possível calcular o fatorial de números negativos.")
      }else{
        val resultado = calcularFatorial(numero)
        println(s"O fatorial de $numero é: $resultado")
      }
  }
}