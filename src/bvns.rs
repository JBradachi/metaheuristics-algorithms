use crate::Solucao;
use std::time::Instant;

pub fn bvns(solucao_inicial: &Solucao, vizinhanca_max: u64, mut tempo_max: u128) -> Solucao {
    let mut tempo: u128 = 0;

    // transforma segundos em ns para fazer a contagem de tempo
    tempo_max = tempo_max * 36000000;

    while tempo < tempo_max {
        let mut vizinhanca: u64 = 0;
        let start = Instant::now();

        while vizinhanca <= vizinhanca_max {
            // gera solução candidata com variação Shake(sol_otima, vizinhanca)
            // let sol_candidata:Solucao = varia_bvns(sol_otima, vizinhanca);

            // faz uma busca local nessa solução candidata
            // let sol_candidata_melhorada:Solucao = intensifica_bvns(sol_candidata);

            // faz a mudança de vizinhança
            vizinhanca = vizinhanca + 1;
        }

        let duracao = start.elapsed();
        tempo = tempo + duracao.as_nanos();
    }
    todo!()
}

fn varia_bvns(sol_otima: Solucao, vizinhanca: u64) -> Solucao {
    // TODO: fazer função de variação
    sol_otima
}

fn intensifica_bvns(sol_candidata: Solucao) -> Solucao {
    // TODO: fazer o hill climb
    sol_candidata
}

fn muda_vizinhanca(
    sol_otima: Solucao,
    sol_candidata_melhorada: Solucao,
    vizinhanca: u64,
) -> (Solucao, u64) {
    // TODO: muda vizinhanca
    (sol_otima, vizinhanca)
}
