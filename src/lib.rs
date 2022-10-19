fn joga(jogo: &str) -> u16 {
    let mut total: u16 = 0;
    let turnos = jogo.split_whitespace();

    for turno in turnos {
        let mut chars = turno.chars();

        while let Some(car) = chars.next() {
            total += car.to_string().parse::<u16>().unwrap();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
