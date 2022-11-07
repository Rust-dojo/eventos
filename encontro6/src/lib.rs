/*

Leia um valor inteiro. A seguir, calcule o menor número de notas 
possíveis (cédulas) no qual o valor pode ser decomposto. As notas 
consideradas são de 100, 50, 20, 10, 5, 2 e 1. A seguir mostre o 
valor lido e a relação de notas necessárias.

Entrada
O arquivo de entrada contém um valor inteiro N (0 < N < 1000000).

Saída
Imprima o valor lido e, em seguida, a quantidade mínima de notas 
de cada tipo necessárias, conforme o exemplo fornecido. Não esqueça 
de imprimir o fim de linha após cada linha, caso contrário seu 
programa apresentará a mensagem: “Presentation Error”.

Exemplo de Entrada
576

Exemplo de Saída
576
5 nota(s) de R$ 100,00
1 nota(s) de R$ 50,00
1 nota(s) de R$ 20,00
0 nota(s) de R$ 10,00
1 nota(s) de R$ 5,00
0 nota(s) de R$ 2,00
1 nota(s) de R$ 1,00

Exemplo de Entrada
11257

Exemplo de Saída
11257
112 nota(s) de R$ 100,00
1 nota(s) de R$ 50,00
0 nota(s) de R$ 20,00
0 nota(s) de R$ 10,00
1 nota(s) de R$ 5,00
1 nota(s) de R$ 2,00
0 nota(s) de R$ 1,00

*/

pub fn value_as_notes(v: u32) -> String {
    let count_100 = v / 100;
    let mut value = v % 100;

    let count_50 = value / 50;
    value %= 50;

    let count_20 = value / 20;
    value %= 20;

    let count_10 = value / 10;
    value %= 10;
    
    let count_5 = value / 5;
    value %= 5;

    let count_2 = value / 2;
    value %= 2;

    let count_1 = value / 1;
    
    format!("{count_100} nota(s) de R$ 100,00\n\
    {count_50} nota(s) de R$ 50,00\n\
    {count_20} nota(s) de R$ 20,00\n\
    {count_10} nota(s) de R$ 10,00\n\
    {count_5} nota(s) de R$ 5,00\n\
    {count_2} nota(s) de R$ 2,00\n\
    {count_1} nota(s) de R$ 1,00")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_500() {
        let result = value_as_notes(500);
        assert_eq!(result, "5 nota(s) de R$ 100,00\n\
        0 nota(s) de R$ 50,00\n\
        0 nota(s) de R$ 20,00\n\
        0 nota(s) de R$ 10,00\n\
        0 nota(s) de R$ 5,00\n\
        0 nota(s) de R$ 2,00\n\
        0 nota(s) de R$ 1,00");
   }
   #[test]
   fn testa_501() {
       let result = value_as_notes(501);
       assert_eq!(result, "5 nota(s) de R$ 100,00\n\
       0 nota(s) de R$ 50,00\n\
       0 nota(s) de R$ 20,00\n\
       0 nota(s) de R$ 10,00\n\
       0 nota(s) de R$ 5,00\n\
       0 nota(s) de R$ 2,00\n\
       1 nota(s) de R$ 1,00");
  }
  #[test]
   fn testa_576() {
       let result = value_as_notes(576);
       assert_eq!(result, "5 nota(s) de R$ 100,00\n\
       1 nota(s) de R$ 50,00\n\
       1 nota(s) de R$ 20,00\n\
       0 nota(s) de R$ 10,00\n\
       1 nota(s) de R$ 5,00\n\
       0 nota(s) de R$ 2,00\n\
       1 nota(s) de R$ 1,00");
  }
  #[test]
   fn testa_11257() {
       let result = value_as_notes(11257);
       assert_eq!(result, "112 nota(s) de R$ 100,00\n\
       1 nota(s) de R$ 50,00\n\
       0 nota(s) de R$ 20,00\n\
       0 nota(s) de R$ 10,00\n\
       1 nota(s) de R$ 5,00\n\
       1 nota(s) de R$ 2,00\n\
       0 nota(s) de R$ 1,00");
  }
}