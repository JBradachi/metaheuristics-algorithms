mod solucao {
    extern crate rand;

    use rand::Rng;

    #[derive(Clone, Default)]
    struct Solucao{
        variaveis: Vec<f64>,
        resultado: f64,
    }
    
    impl Solucao {
        fn mostra_solucao(&self){
            print!("Variaveis: ");
            for x in self.variaveis.iter(){
                print!("{} ", x);
            }
            println!("\nResultado: {}", &self.resultado);
        }
    
        // uma solução para abordar os dois tipos de solução é fazer sobrecarga
        // rust nn aceita sobrecarga obosta
        fn gera_solucao_inicial(&mut self, f: fn(f64, f64) -> f64, xmin: f64, xmax: f64) {
            let x1 = rand::thread_rng().gen_range(xmin, xmax);
            let x2 = rand::thread_rng().gen_range(xmin, xmax);
    
            self.variaveis = vec![x1, x2];
            self.resultado = f(x1, x2);
        }
    }

}

