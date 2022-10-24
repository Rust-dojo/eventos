/*

Tempo de Jogo com Minutos

Leia a hora inicial, minuto inicial, hora final e minuto final de um jogo. A seguir calcule a duração do jogo.

Obs: O jogo tem duração mínima de um (1) minuto e duração máxima de 24 horas.

Entrada
Quatro números inteiros representando a hora de início e fim do jogo.

Saída
Mostre a seguinte mensagem: “O JOGO DUROU XXX HORA(S) E YYY MINUTO(S)” .

Exemplo de Entrada e Exemplo de Saída

7 8 9 10
O JOGO DUROU 2 HORA(S) E 2 MINUTO(S)

7 7 7 7
O JOGO DUROU 24 HORA(S) E 0 MINUTO(S)

7 10 8 9
O JOGO DUROU 0 HORA(S) E 59 MINUTO(S)

*/

pub fn game_duration(hora_inicio: i32, min_inicio: i32, hora_final: i32, min_final: i32) -> String {
    let mut tempo_horas = hora_final - hora_inicio;
    let mut tempo_minutos = min_final - min_inicio;
    if hora_final == hora_inicio{
        tempo_horas = 24;
    }
    // tempo_minutos.to_string()

    if tempo_minutos < 0{
        let tempo_minutos2 = 60 + tempo_minutos;
        tempo_minutos = tempo_minutos2;
        let tempo_horas2 = tempo_horas - 1;
        tempo_horas = tempo_horas2;
    }
    return format!("O JOGO DUROU {tempo_horas} HORA(S) E {tempo_minutos} MINUTO(S)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_game1() {
        let result = game_duration(7, 8, 9, 10);
        assert_eq!(result, "O JOGO DUROU 2 HORA(S) E 2 MINUTO(S)");
    }
    #[test]
    fn quick_game2() {
        let result = game_duration(7, 7, 7, 7);
        assert_eq!(result, "O JOGO DUROU 24 HORA(S) E 0 MINUTO(S)");
    }
    #[test]
    fn quick_game3() {
        let result = game_duration(7, 10, 8, 9);
        assert_eq!(result, "O JOGO DUROU 0 HORA(S) E 59 MINUTO(S)");
    }
}
