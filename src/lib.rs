fn joga(jogo: &str) -> u16 {
    let mut chars = jogo.chars();
    let mut total: u16 = 0;
    if let Some(car) = chars.next() {}
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
}
