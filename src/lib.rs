pub trait Numeravel<T> {
    fn numero(t: &Self) -> u16;
}

impl Numeravel<&char> for char {
    fn numero(c: &Self) -> u16 {
        c.to_string().parse::<u16>().unwrap()
    }
}

#[allow(dead_code)]
fn joga(texto_jogo: &str) -> u16 {
    let mut total: u16 = 0;
    let jogo = Jogo::new(texto_jogo);

    for (indice_turno, turno) in jogo.turnos.iter().enumerate() {
        for (jogada, car) in turno.iter().enumerate() {
            match *car {
                '-' => {} // zero
                'x' => {
                    // strike
                    total += 10;
                    // soma com o próximo turno
                    if let Some(jogada) = jogo.turnos.get(indice_turno + 1) {
                        total += jogada[0].to_string().parse::<u16>().unwrap();
                        if let Some(segunda_jogada) = jogada.get(1) {
                            total += segunda_jogada.to_string().parse::<u16>().unwrap();
                        }
                    }
                }
                '/' => {
                    // acha o número que completa a anterior
                    total += 10 - turno[jogada - 1].to_string().parse::<u16>().unwrap();
                    // soma com o próximo turno
                    if let Some(jogada) = jogo.turnos.get(indice_turno + 1) {
                        total += jogada[0].to_string().parse::<u16>().unwrap();
                    }
                }
                _ => total += car.numero(),
            }
        }
    }

    total
}

fn parse_jogada(c: &char) -> u16 {
    c.to_string().parse::<u16>().unwrap()
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

    #[test]
    fn um_spare_mais_um_spare() {
        assert_eq!(22, joga("4/ 2/")); // 4 + 6 + 2 + 2 + 8 + ??
    }

    #[test]
    fn uma_partida_completa() {
        assert_eq!(30, joga("12 12 12 12 12 12 12 12 12 12"));
    }

    #[test]
    fn uma_partida_completa_com_spare_no_final() {
        assert_eq!(42, joga("12 12 12 12 12 12 12 12 12 1/5"));
    }

    #[test]
    fn um_strike() {
        assert_eq!(10, joga("x"));
    }

    #[test]
    fn um_strike_e_meia_jogada() {
        assert_eq!(12, joga("x 1"));
    }

    #[test]
    fn um_strike_e_mais_uma_jogada() {
        assert_eq!(16, joga("x 12"));
    }
}
