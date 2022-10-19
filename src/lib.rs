#[allow(dead_code)]
fn joga(texto_jogo: &str) -> u16 {
    let mut total: u16 = 0;
    let jogo = Jogo::new(texto_jogo);

    for (indice_turno, turno) in jogo.turnos.iter().enumerate() {
        for (jogada, car) in turno.iter().enumerate() {
            match *car {
                '-' => {} // zero
                '/' => {
                    // acha o número que completa a anterior
                    total += 10 - turno[jogada - 1].to_string().parse::<u16>().unwrap();
                    // soma com o próximo turno
                    total += jogo.turnos[indice_turno + 1][0]
                        .to_string()
                        .parse::<u16>()
                        .unwrap();
                }
                _ => total += car.to_string().parse::<u16>().unwrap(),
            }
        }
    }

    total
}

#[allow(dead_code)]
struct Jogo {
    turnos: Vec<Vec<char>>,
}

#[allow(dead_code)]
impl Jogo {
    fn new(texto_jogo: &str) -> Self {
        let mut turnos = Vec::new();
        for turno in texto_jogo.split_whitespace() {
            let chars: Vec<char> = turno.chars().collect();
            turnos.push(chars);
        }
        Jogo { turnos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dado_uma_string_vazia_cria_um_jogo() {
        assert_eq!(Vec::<Vec<char>>::new(), Jogo::new("").turnos);
    }

    #[test]
    fn dado_uma_string_cria_um_jogo() {
        assert_eq!(
            vec![vec!['3', '2'], vec!['2', '3']],
            Jogo::new("32 23").turnos
        );
    }

    #[test]
    fn comeco_jogo() {
        assert_eq!(0, joga(""));
    }

    #[test]
    fn uma_jogada_sem_derrubar_tudo() {
        assert_eq!(3, joga("12"));
    }

    #[test]
    fn duas_jogadas_sem_derrubar_tudo() {
        assert_eq!(10, joga("32 23"));
    }

    #[test]
    fn uma_jogada_sem_derrubar_nada() {
        assert_eq!(0, joga("--"));
    }

    #[test]
    fn duas_jogadas_sem_derrubar_nada() {
        assert_eq!(0, joga("-- --"));
    }

    #[test]
    fn duas_jogadas_primeira_sem_derrubar_nada_segunda_simples() {
        assert_eq!(5, joga("-- 23"));
    }

    #[test]
    fn um_spare_mais_uma_simples() {
        assert_eq!(17, joga("4/ 23")); // 4 + 6 + 2 + 2 + 3
    }

    // #[test]
    // fn um_spare_mais_um_spare() {
    //     assert_eq!(17, joga("4/ 2/"));
    // }
}
