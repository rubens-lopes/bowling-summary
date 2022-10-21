trait Numeravel {
    fn to_u16(self) -> u16;
    fn try_to_u16(self) -> Option<u16>;
}

impl Numeravel for Option<char> {
    fn to_u16(self) -> u16 {
        let carácter = self.unwrap_or_else(|| '0');

        match carácter {
            '-' => 0,
            'x' => 10,
            _ => carácter.to_string().parse().unwrap(),
        }
    }

    fn try_to_u16(self) -> Option<u16> {
        let carácter = self.unwrap_or_else(|| '-');

        match carácter {
            'x' => Some(10),
            _ => match carácter.to_string().parse::<u16>() {
                Ok(número) => Some(número),
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
enum Jogada {
    Strike,
    Spare(u16, Option<u16>),
    Comum(u16, Option<u16>),
    Bonus(u16),
}

impl PartialEq for Jogada {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Spare(l0, l1), Self::Spare(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Comum(l0, l1), Self::Comum(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Bonus(l0), Self::Bonus(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

struct Partida {
    rodadas: Vec<Jogada>,
}

impl Partida {
    #[allow(dead_code)]
    fn new(anotacoes: &str) -> Self {
        let mut rodadas = Vec::new();
        let mut anotacoes: Vec<&str> = anotacoes.split_whitespace().collect();
        let rodadas_bonus = if anotacoes.len() > 10 {
            let primeiro_bonus = anotacoes.pop().unwrap();
            let segundo_bonus = anotacoes.pop().unwrap();
            vec![segundo_bonus, primeiro_bonus]
        } else {
            vec![]
        };

        for rodada in anotacoes {
            if rodada.contains("/") {
                let mut tentativas = rodada.chars();
                let primeira_tentativa = tentativas.next().to_u16();
                tentativas.next(); // consome a /
                let tentativa_bônus = tentativas.next().try_to_u16();

                rodadas.push(Jogada::Spare(primeira_tentativa, tentativa_bônus));
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

        for rodada in rodadas_bonus {
            let mut tentativas = rodada.chars();
            rodadas.push(Jogada::Bonus(tentativas.next().to_u16()));
        }

        Partida { rodadas }
    }

    #[allow(dead_code)]
    fn calcular_pontuacao(&self) -> u16 {
        let mut pontuacao = 0u16;
        let Self { rodadas } = self;
        let mut rodada_anterior_foi_spare = false;
        let mut rodada_anterior_foi_strike = false;
        let mut duas_rodadas_pra_tras_foi_strike = false;

        for rodada in rodadas.iter() {
            match rodada {
                Jogada::Strike => {
                    pontuacao += 10;

                    if rodada_anterior_foi_spare || rodada_anterior_foi_strike {
                        pontuacao += 10;
                    }

                    if duas_rodadas_pra_tras_foi_strike {
                        pontuacao += 10;
                    }

                    rodada_anterior_foi_spare = false;
                    duas_rodadas_pra_tras_foi_strike = rodada_anterior_foi_strike;
                    rodada_anterior_foi_strike = true;
                }
                Jogada::Spare(primeira, bônus) => {
                    pontuacao += 10;
                    pontuacao += bônus.unwrap_or_default();

                    if rodada_anterior_foi_spare || duas_rodadas_pra_tras_foi_strike {
                        pontuacao += primeira;
                    }

                    if rodada_anterior_foi_strike {
                        pontuacao += 10;
                    }

                    rodada_anterior_foi_spare = true;
                    duas_rodadas_pra_tras_foi_strike = false;
                    rodada_anterior_foi_strike = false;
                }
                Jogada::Comum(primeira, segunda) => {
                    pontuacao += primeira + segunda.unwrap_or_default();

                    if rodada_anterior_foi_spare {
                        pontuacao += primeira;
                    }

                    if rodada_anterior_foi_strike {
                        pontuacao += primeira + segunda.unwrap_or_default();
                    }

                    if duas_rodadas_pra_tras_foi_strike {
                        pontuacao += primeira
                    }

                    rodada_anterior_foi_spare = false;
                    duas_rodadas_pra_tras_foi_strike = false;
                    rodada_anterior_foi_strike = false;
                }
                Jogada::Bonus(pontos) => {
                    pontuacao += pontos;

                    if duas_rodadas_pra_tras_foi_strike {
                        pontuacao += pontos
                    }

                    duas_rodadas_pra_tras_foi_strike = false;
                }
            }
        }

        pontuacao
    }
}

#[cfg(test)]
mod testes_partida_new;

#[cfg(test)]
mod testes_partida_calcular_pontuacao;
