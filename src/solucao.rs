use super::ObjetivoFn;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Solucao {
    pub variaveis: Vec<f64>,
    pub resultado: f64,
}

// Porcentagem de variação em cada atributo em uma perturbação
const SHAKE_PERCENT: f64 = 0.05;

impl Solucao {
    pub fn new(f: ObjetivoFn, variaveis: Vec<f64>) -> Self {
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }

    pub fn random(f: ObjetivoFn, min: f64, max: f64) -> Self {
        let mut variaveis = Vec::new();
        for _ in 0..f.num_vars {
            let x = rand::thread_rng().gen_range(min, max);
            variaveis.push(x);
        }
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }

    pub fn shake(&self, f: ObjetivoFn, min: f64, max: f64) -> Solucao {
        let mut variaveis = self.variaveis.clone();
        for x in variaveis.iter_mut() {
            let p = rand::thread_rng().gen_range(-SHAKE_PERCENT, SHAKE_PERCENT);
            *x *= 1.0 + p;
            if *x > max {
                *x = max;
            } else if *x < min {
                *x = min;
            }
        }
        Solucao::new(f, variaveis)
    }
}
