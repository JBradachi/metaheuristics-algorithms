use crate::objetivo::ObjetivoFn;
use crate::solucao::{EspacoBusca, Solucao};

pub fn hillclimb_puro(obj: ObjetivoFn, esp: EspacoBusca) -> Solucao {
    let sol = Solucao::random(obj, esp);
    hillclimb(sol, esp, obj)
}

pub fn hillclimb(sol_candidata_sm: Solucao, espaco_busca: EspacoBusca, fo: ObjetivoFn) -> Solucao {
    // probabilidade de aceitar o ruído
    let mut sol_candidata = sol_candidata_sm.clone();
    let mut sol_melhorada = sol_candidata_sm.clone();
    // critério de parada: não ter alteração em 100 iterações seguidas
    let mut sem_alteracao = 0;

    loop {
        sol_candidata = steepest_ascent(sol_candidata, espaco_busca, fo);

        if sol_candidata.resultado < sol_melhorada.resultado {
            sol_melhorada = sol_candidata.clone();
            sem_alteracao = 0;
        } else {
            sem_alteracao = sem_alteracao + 1;

            if sem_alteracao > 100 {
                break;
            }
        }
    }
    sol_melhorada
}

fn steepest_ascent(sol_candidata: Solucao, espaco_busca: EspacoBusca, fo: ObjetivoFn) -> Solucao {
    let mut sol_melhor = sol_candidata.clone();
    let mut n_steps = 0;

    loop {
        let sol_variada = sol_candidata.shake(fo, espaco_busca);
        if sol_variada.resultado < sol_melhor.resultado {
            sol_melhor = sol_variada.clone();
        }
        n_steps += 1;

        // verifica 10 vezes na mesma solução e pega o melhor
        if n_steps == 50 {
            break;
        }
    }
    sol_melhor
}
