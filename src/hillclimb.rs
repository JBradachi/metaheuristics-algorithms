use std::io::repeat;
use crate::ObjetivoFn;
use crate::Solucao;
use rand::Rng;

pub fn hillclimb(sol_candidata: Solucao, espaco_busca: (f64, f64), fo: ObjetivoFn) -> Solucao {

    // probabilidade de aceitar o ruído
    let p_noise = 0.5;
    let mut sol_melhorada = sol_candidata;

    for elemen in sol_melhorada.variaveis.iter(){
        if p_noise >= rand::thread_rng().gen_range(0.0, 1.0){
            loop {
                let noise = elemen / 100.0;
                let variacao = rand::thread_rng().gen_range(-noise, noise);
                
                if espaco_busca.0 <= variacao+elemen && variacao+elemen <= espaco_busca.1{
                    break;
                }

            }
            
        }
        continue;
    } 
    sol_melhorada
}

fn ruído (sol_candidata: Solucao) -> Solucao {
    
    sol_candidata
}