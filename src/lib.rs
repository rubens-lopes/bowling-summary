trait Numerável {
    fn to_u16(self) -> u16;

    fn try_to_u16(self) -> Option<u16>;
}

impl Numerável for Option<char> {
    fn to_u16(self) -> u16 {
        let carácter = self.unwrap_or_else(|| '0');

        match carácter {
            '-' => 0,
            _ => carácter.to_string().parse().unwrap(),
        }
    }

    fn try_to_u16(self) -> Option<u16> {
        let carácter = &self.unwrap_or_else(|| '-').to_string();

        match carácter.parse::<u16>() {
            Ok(número) => Some(número),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Jogada {
    Strike,
    Spare(u16, Option<u16>),
    Comum(u16, Option<u16>),
}

struct Partida {
    rodadas: Vec<Jogada>,
}

impl PartialEq for Jogada {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Spare(l0, l1), Self::Spare(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Comum(l0, l1), Self::Comum(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Partida {
    #[allow(dead_code)]
    fn new(anotações: &str) -> Self {
        let mut rodadas = Vec::new();

        for rodada in anotações.split_whitespace() {
            if rodada.contains("/") {
                let mut tentativas = rodada.chars();
                let primeira_tentativa = tentativas.next().to_u16();
                tentativas.next(); // consome a /
                let tentativa_bônus = tentativas.next().try_to_u16();

                rodadas.push(Jogada::Spare(
                    primeira_tentativa,
                    tentativa_bônus,
                ));
            } else if rodada == "x" {
                rodadas.push(Jogada::Strike)
            } else {
                let mut tentativas = rodada.chars();

                rodadas.push(Jogada::Comum(
                    tentativas.next().to_u16(),
                    tentativas.next().try_to_u16(),
                ))
            }
        }

        Partida { rodadas }
    }

    #[allow(dead_code)]
    fn calcular_pontuação(&self) -> u16 {
        let mut pontuação: u16 = 0;
        let Self { rodadas } = self;
        let mut rodada_anterior_foi_spare = false;
        let mut rodada_anterior_foi_strike = false;
        let mut duas_rodadas_atrás_foi_strike = false;

        for rodada in rodadas {
            match rodada {
                Jogada::Strike => {
                    pontuação += 10;
                    if rodada_anterior_foi_spare {
                        pontuação += 10
                    }
                    if rodada_anterior_foi_strike {
                        pontuação += 10;
                    }
                    if duas_rodadas_atrás_foi_strike {
                        pontuação += 10;
                    }

                    rodada_anterior_foi_spare = false;
                    duas_rodadas_atrás_foi_strike = rodada_anterior_foi_strike;
                    rodada_anterior_foi_strike = true;
                }
                Jogada::Spare(primeira, bônus) => {
                    pontuação += 10;
                    pontuação += bônus.unwrap_or_default();
                    if rodada_anterior_foi_spare {
                        pontuação += primeira
                    }
                    if rodada_anterior_foi_strike {
                        pontuação += 10;
                    }
                    if duas_rodadas_atrás_foi_strike {
                        pontuação += 10;
                    }
                    rodada_anterior_foi_spare = true;
                    rodada_anterior_foi_strike = false;
                }
                Jogada::Comum(primeira, segunda) => {
                    pontuação += primeira + segunda.unwrap_or_default();
                    if rodada_anterior_foi_spare {
                        pontuação += primeira
                    }
                    if rodada_anterior_foi_strike {
                        pontuação += primeira + segunda.unwrap_or_default();
                    }
                    if duas_rodadas_atrás_foi_strike {
                        pontuação += primeira + segunda.unwrap_or_default();
                    }

                    rodada_anterior_foi_spare = false;
                    rodada_anterior_foi_strike = false;
                }
            }
        }

        pontuação
    }
}

#[cfg(test)]
mod testes_partida_new {
    use super::*;

    #[test]
    fn dado_uma_anotação_vazia_cria_uma_partida() {
        assert_eq!(Vec::<Jogada>::new(), Partida::new("").rodadas);
    }

    #[test]
    fn dado_uma_anotação_cria_uma_partida() {
        assert_eq!(
            vec![Jogada::Comum(3, Some(2)), Jogada::Comum(2, Some(3))],
            Partida::new("32 23").rodadas
        );
    }

    #[test]
    fn dado_uma_anotação_com_rodada_parcial_cria_uma_partida() {
        assert_eq!(vec![Jogada::Comum(3, None)], Partida::new("3").rodadas);
    }

    #[test]
    fn dado_uma_anotação_com_um_strike_cria_uma_partida() {
        assert_eq!(vec![Jogada::Strike], Partida::new("x").rodadas);
    }

    #[test]
    fn dado_uma_anotação_com_um_spare_cria_uma_partida() {
        assert_eq!(
            vec![Jogada::Spare(2, None)],
            Partida::new("2/").rodadas
        );
    }

    #[test]
    fn dado_uma_anotação_com_um_spare_na_última_rodada_cria_uma_partida() {
        assert_eq!(
            vec![Jogada::Spare(0, Some(5))],
            Partida::new("-/5").rodadas
        );
    }
}

#[cfg(test)]
mod testes_partida_calcular_pontuação {
    use super::*;

    #[test]
    fn começo_partida() {
        assert_eq!(0, Partida::new("").calcular_pontuação());
    }

    #[test]
    fn uma_rodada_sem_derrubar_tudo() {
        assert_eq!(3, Partida::new("12").calcular_pontuação());
    }

    #[test]
    fn duas_rodadas_sem_derrubar_tudo() {
        assert_eq!(10, Partida::new("32 23").calcular_pontuação());
    }

    #[test]
    fn uma_rodada_sem_derrubar_nada() {
        assert_eq!(0, Partida::new("--").calcular_pontuação());
    }

    #[test]
    fn duas_rodadas_sem_derrubar_nada() {
        assert_eq!(0, Partida::new("-- --").calcular_pontuação());
    }

    #[test]
    fn duas_rodadas_primeira_sem_derrubar_nada_segunda_comum() {
        assert_eq!(5, Partida::new("-- 23").calcular_pontuação());
    }

    #[test]
    fn um_spare_mais_uma_comum() {
        assert_eq!(17, Partida::new("4/ 23").calcular_pontuação()); // (4 + 6 + 2) + (2 + 3)
    }

    #[test]
    fn um_spare_mais_um_spare() {
        assert_eq!(22, Partida::new("4/ 2/").calcular_pontuação()); // (4 + 6 + 2) + (2 + 8 + ??)
    }

    #[test]
    fn um_spare_mais_uma_rodada_parcial() {
        assert_eq!(14, Partida::new("4/ 2").calcular_pontuação());
    }

    #[test]
    fn uma_partida_completa() {
        assert_eq!(
            30,
            Partida::new("12 12 12 12 12 12 12 12 12 12").calcular_pontuação()
        );
    }

    #[test]
    fn uma_partida_completa_com_spare_no_final() {
        assert_eq!(
            42,
            Partida::new("12 12 12 12 12 12 12 12 12 1/5").calcular_pontuação()
        );
    }

    #[test]
    fn um_strike() {
        assert_eq!(10, Partida::new("x").calcular_pontuação());
    }

    #[test]
    fn um_strike_e_meia_rodada() {
        assert_eq!(12, Partida::new("x 1").calcular_pontuação());
    }

    #[test]
    fn um_strike_e_mais_uma_rodada() {
        assert_eq!(16, Partida::new("x 12").calcular_pontuação());
    }

    #[test]
    fn um_strike_outro_strike_e_mais_uma_rodada() {
        assert_eq!(39, Partida::new("x x 12").calcular_pontuação()); // (10 + 10 + 3) + (10 + 3 + ??) + (1 + 2)
    }

    #[test]
    fn uma_partida_completa_apenas_de_jogadas_comum() {
        assert_eq!(90, Partida::new("9- 9- 9- 9- 9- 9- 9- 9- 9- 9-").calcular_pontuação());
    }

    #[test]
    fn uma_partida_completa_apenas_de_spares() {
        assert_eq!(150, Partida::new("5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/5").calcular_pontuação());
    }

    #[test]
    #[ignore]
    fn uma_partida_completa_apenas_de_strikes() {
        assert_eq!(300, Partida::new("x x x x x x x x x x x x").calcular_pontuação());
    }
}
