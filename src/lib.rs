#[allow(dead_code)]
fn joga(jogo: &str) -> u16 {
    let mut total: u16 = 0;
    let turnos = jogo.split_whitespace();
    
    for turno in turnos {
        let chars: Vec<char> = turno.chars().collect();
        for (i, car) in chars.enumerate() {
            if car == '-' {
                continue;
            }
            if car == '/' {
                total += chars[i - 1];
                continue;
            }
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
}
