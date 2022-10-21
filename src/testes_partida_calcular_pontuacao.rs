use super::*;

#[test]
fn começo_partida() {
    assert_eq!(0, Partida::new("").calcular_pontuacao());
}

#[test]
fn uma_rodada_sem_derrubar_tudo() {
    assert_eq!(3, Partida::new("12").calcular_pontuacao());
}

#[test]
fn duas_rodadas_sem_derrubar_tudo() {
    assert_eq!(10, Partida::new("32 23").calcular_pontuacao());
}

#[test]
fn uma_rodada_sem_derrubar_nada() {
    assert_eq!(0, Partida::new("--").calcular_pontuacao());
}

#[test]
fn duas_rodadas_sem_derrubar_nada() {
    assert_eq!(0, Partida::new("-- --").calcular_pontuacao());
}

#[test]
fn duas_rodadas_primeira_sem_derrubar_nada_segunda_comum() {
    assert_eq!(5, Partida::new("-- 23").calcular_pontuacao());
}

#[test]
fn um_spare_mais_uma_comum() {
    assert_eq!(17, Partida::new("4/ 23").calcular_pontuacao());
    // (4 + 6 + 2) + (2 + 3)
}

#[test]
fn um_spare_mais_um_spare() {
    assert_eq!(22, Partida::new("4/ 2/").calcular_pontuacao());
    // (4 + 6 + 2) + (2 + 8 + ??)
}

#[test]
fn um_spare_mais_uma_rodada_parcial() {
    assert_eq!(14, Partida::new("4/ 2").calcular_pontuacao());
}

#[test]
fn uma_partida_completa() {
    assert_eq!(
        30,
        Partida::new("12 12 12 12 12 12 12 12 12 12").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_completa_com_spare_no_final() {
    assert_eq!(
        42,
        Partida::new("12 12 12 12 12 12 12 12 12 1/5").calcular_pontuacao()
    );
}

#[test]
fn um_strike() {
    assert_eq!(10, Partida::new("x").calcular_pontuacao());
}

#[test]
fn um_strike_e_meia_rodada() {
    assert_eq!(12, Partida::new("x 1").calcular_pontuacao());
}

#[test]
fn um_strike_e_mais_uma_rodada() {
    assert_eq!(16, Partida::new("x 12").calcular_pontuacao());
    // (10 + 1 + 2) + (1 + 2)
}

#[test]
fn um_strike_outro_strike_e_mais_uma_rodada() {
    assert_eq!(37, Partida::new("x x 12").calcular_pontuacao());
    // (10 + 10 + 1) + (10 + 1 + 2) + (1 + 2)
}

#[test]
fn uma_partida_completa_apenas_de_jogadas_comum() {
    assert_eq!(
        90,
        Partida::new("9- 9- 9- 9- 9- 9- 9- 9- 9- 9-").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_completa_apenas_de_spares() {
    assert_eq!(
        150,
        Partida::new("5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/5").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_completa_apenas_de_spares_com_bônus_de_10_pinos() {
    assert_eq!(
        155,
        Partida::new("5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/x").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_completa_apenas_de_strikes() {
    assert_eq!(
        300,
        Partida::new("x x x x x x x x x x x x").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_completa_apenas_de_strikes_terminada_em_jogada_comum() {
    assert_eq!(
        295,
        Partida::new("x x x x x x x x x x x 5").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_completa_de_strikes_terminada_em_jogadas_comum() {
    assert_eq!(
        289,
        Partida::new("x x x x x x x x x x 7 5").calcular_pontuacao()
    );
}

#[test]
fn uma_partida_terminada_em_strike_e_jogadas_comum() {
    // 30 + 27 + 19 + 9 + 10 + 9 + 20 + 19 + 9 + 28
    assert_eq!(
        180,
        Partida::new("x x x 72 8/ -9 x 7/ 9- x x 8").calcular_pontuacao()
    );
}
