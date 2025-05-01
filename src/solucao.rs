use super::ObjetivoFn;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Solucao {
    pub variaveis: Vec<f64>,
    pub resultado: f64,
}

// Probabilidade de perturbar cada atributo
const P_NOISE: f64 = 0.5;

impl Solucao {
    pub fn new(f: ObjetivoFn, variaveis: Vec<f64>) -> Self {
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }

    pub fn random(f: ObjetivoFn, espaco_busca: (f64, f64)) -> Self {
        let mut variaveis = Vec::new();
        for _ in 0..f.num_vars {
            let x = rand::thread_rng().gen_range(espaco_busca.0, espaco_busca.1);
            variaveis.push(x);
        }
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }

    // TODO eliminar isso
    pub fn evaluate(f: ObjetivoFn, x: &Vec<f64>) -> Self {
        let variaveis = x.clone();
        Solucao::new(f, variaveis)
    }

    pub fn shake(&self, f: ObjetivoFn, min: f64, max: f64) -> Solucao {
        let noise = (min.abs() + max.abs()) / 100.0;

        let mut variaveis = self.variaveis.clone();
        for elem in variaveis.iter_mut() {
            if P_NOISE >= rand::thread_rng().gen_range(0.0, 1.0) {
                // Devemos perturbar esse atributo dentro dos limites
                loop {
                    let delta = rand::thread_rng().gen_range(-noise, noise);
                    let m = *elem + delta;
                    if m <= max && m >= min {
                        *elem = m;
                        break;
                    }
                }
            }
        }
        Solucao::new(f, variaveis)
    }
}
