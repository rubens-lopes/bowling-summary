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
    assert_eq!(vec![Jogada::Spare(2, None)], Partida::new("2/").rodadas);
}

#[test]
fn dado_uma_anotação_com_um_spare_na_última_rodada_cria_uma_partida() {
    assert_eq!(vec![Jogada::Spare(0, Some(5))], Partida::new("-/5").rodadas);
}

#[test]
fn dado_uma_anotação_com_12_rodadas_cria_uma_partida() {
    assert_eq!(
        vec![
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Strike,
            Jogada::Bonus(10),
            Jogada::Bonus(5),
        ],
        Partida::new("x x x x x x x x x x x 5").rodadas
    );
}
