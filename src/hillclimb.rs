use crate::ObjetivoFn;
use crate::Solucao;
use rand::Rng;

pub fn hillclimb(sol_candidata_sm: Solucao, espaco_busca: (f64, f64), fo: ObjetivoFn) -> Solucao {

    // probabilidade de aceitar o ruído
    let mut sol_candidata = sol_candidata_sm.clone();
    let mut sol_melhorada = sol_candidata_sm.clone();
    // critério de parada: não ter alteração em 100 iterações seguidas
    let mut sem_alteracao = 0;
    
    loop{

        sol_candidata = ajuste(sol_candidata, espaco_busca, fo);

        if sol_candidata.resultado < sol_melhorada.resultado{
            sol_melhorada = sol_candidata.clone();
            sem_alteracao = 0;

        }else {
            sem_alteracao = sem_alteracao + 1;

            if sem_alteracao > 100 {
                break;
            }
        }
    }
    sol_melhorada
}

fn ajuste (sol_candidata: Solucao, espaco_busca: (f64, f64), fo: ObjetivoFn) -> Solucao {
    
    let p_noise = 0.5; // probabilidade de mudar um parâmetro
    let noise = (espaco_busca.0.abs() + espaco_busca.1.abs())/100.0;
    let mut sol_variada;
    let mut sol_melhor = sol_candidata.clone();
    let mut n_steps = 0;

    loop{

        sol_variada = sol_candidata.clone();
        for elemen in sol_variada.variaveis.iter_mut(){
            if p_noise >= rand::thread_rng().gen_range(0.0, 1.0){
                loop {
                    
                    let variacao = rand::thread_rng().gen_range(-noise, noise);
                    
                    if espaco_busca.0 <= variacao + *elemen && variacao + *elemen <= espaco_busca.1 {
                        *elemen = *elemen + variacao;
                        break;
                    }
                }
            }
            continue;
        }
        sol_variada = Solucao::evaluate(fo, &sol_variada.variaveis);
        if sol_variada.resultado < sol_melhor.resultado{
            sol_melhor = sol_variada.clone();
        }

        n_steps = n_steps + 1;

        // verifica 10 vezes na mesma solução e pega o melhor  
        if n_steps == 50 {
            break;
        }
    }
     
    Solucao::evaluate(fo, &sol_melhor.variaveis)
}