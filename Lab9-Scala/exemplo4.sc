// Exemplo 4 – Calcular a posição da sub string “fofo” na string
// "Gatossaoosanimaismaisfofosdaterra".

object BuscaSubstring {

  def encontrarSubstring(stringPrincipal: String, subString: String): Option[Int] = {

    
    val index = stringPrincipal.indexOf(subString)

    // Se a substring existe na string principal
    if(index != -1) Some(index)
    else None 
  }

  def main(args: Array[String]): Unit = {
    val stringPrincipal = "Gatossaoosanimaismaisfofosdaterra"
    val subString = "fofo"

    encontrarSubstring(stringPrincipal, subString) match {
      case Some(posicao) => println(s"A substring '$subString' foi encontrada na posição $posicao")
      case None => println(s"A substring '$subString' não foi encontrada na string principal.")
    }
  }
}

