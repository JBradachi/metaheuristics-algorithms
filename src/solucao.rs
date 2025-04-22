extern crate rand;

use rand::Rng;

#[derive(Clone, Default)]
pub struct Solucao{
    pub variaveis: Vec<f64>,
    pub resultado: f64,
}

impl Solucao {
    pub fn mostra_solucao(&self){
        print!("Variaveis: ");
        for x in self.variaveis.iter(){
            print!("{} ", x);
        }
        println!("\nResultado: {}", &self.resultado);
    }

    // uma solução para abordar os dois tipos de solução é fazer sobrecarga
    // rust nn aceita sobrecarga obosta
    pub fn gera_solucao_inicial(f: fn(f64, f64) -> f64, xmin: f64, xmax: f64) -> Solucao {
        
        let x1 = rand::thread_rng().gen_range(xmin, xmax);
        let x2 = rand::thread_rng().gen_range(xmin, xmax);

        let x: Solucao = Solucao { 
            variaveis: vec![x1, x2], 
            resultado: f(x1, x2), 
        };
        x
    }
}


