use crate::objetivo::ObjetivoFn;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct EspacoBusca {
    pub min: f64,
    pub max: f64,
}

impl EspacoBusca {
    pub fn new(x: f64) -> Self {
        EspacoBusca { min: -x, max: x }
    }
}

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

    pub fn random(f: ObjetivoFn, esp: EspacoBusca) -> Self {
        let mut variaveis = Vec::new();
        for _ in 0..f.num_vars {
            let x = rand::thread_rng().gen_range(esp.min, esp.max);
            variaveis.push(x);
        }
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }

    pub fn shake(&self, f: ObjetivoFn, esp: EspacoBusca) -> Solucao {
        let noise = (esp.min.abs() + esp.max.abs()) / 100.0;

        let mut variaveis = self.variaveis.clone();
        for elem in variaveis.iter_mut() {
            if P_NOISE >= rand::thread_rng().gen_range(0.0, 1.0) {
                // Devemos perturbar esse atributo dentro dos limites
                loop {
                    let delta = rand::thread_rng().gen_range(-noise, noise);
                    let m = *elem + delta;
                    if m <= esp.max && m >= esp.min {
                        *elem = m;
                        break;
                    }
                }
            }
        }
        Solucao::new(f, variaveis)
    }
}
