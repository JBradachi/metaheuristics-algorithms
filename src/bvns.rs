use crate::Solucao;
use std::time::Instant;
use rand::Rng;
use crate::ObjetivoFn;
use std::f64::consts::PI;

pub fn bvns(solucao_inicial: &Solucao, vizinhanca_max: u64, mut tempo_max: u128) -> Solucao {
    let mut tempo: u128 = 0;
    // TODO: Conferir onde é o melhor lugar para se deixar essa variavel
    let tamanho_passo = 1.0; // é o raio que será variado entre cada vizinhanca

    // transforma segundos em ns para fazer a contagem de tempo
    tempo_max = tempo_max * 36000000;

    // Cria estrutura de vizinhancas
    let mut v: Vizinhanca = Vizinhanca::new(vizinhanca_max, tamanho_passo);
    Vizinhanca::print_raio(&v);
    print!("Vizinhanca: {:?}\n", v);

    let new_sol = v.gera_vizinho_aleatorio(solucao_inicial);
    print!("New Sol: {:?}\n", new_sol);

    let mut solucao_atual = solucao_inicial.clone();

    while tempo < tempo_max {
        // Vizinhanca::reset_vizinhanca(&mut v); // seta vizinhanca para maior raio
        v.reset_estrutura_vizinhanca(); // seta vizinhanca para maior raio

        let start = Instant::now();
        print!("==============================================================");

        while !v.estruturas_totalmente_exploradas() {
            print!("\n");
            print!("--------------\n");
            print!("Vizinhanca: {:?}\n", v);

            // gera solução candidata com variação Shake(sol_otima, vizinhanca)
            let sol_candidata: Solucao = varia_solucao(&solucao_atual, &v);

            // faz uma busca local nessa solução candidata
            // let sol_candidata_melhorada:Solucao = intensifica_bvns(sol_candidata);

            // faz a mudança de vizinhança
            // A funcao já se responsabiliza por alterar os valores das solucoes candidatas e atualizar a vizinnhanca
            muda_vizinhanca(&mut solucao_atual, &sol_candidata, &mut v);
        }

        let duracao = start.elapsed();
        tempo = tempo + duracao.as_nanos();
    }
    solucao_atual
}

/// Shake
fn varia_solucao(sol_otima:&Solucao, vizinhanca: &Vizinhanca) -> Solucao {
    // TODO: fazer função de variação
    let solucao_variada = vizinhanca.gera_vizinho_aleatorio(sol_otima);
    solucao_variada
}

/// Local Search
fn intensifica_bvns(sol_candidata: Solucao) -> Solucao {
    // TODO: fazer o hill climb
    sol_candidata
}

fn muda_vizinhanca( sol_atual: &mut Solucao, sol_candidata: &Solucao, vizinhanca: &mut Vizinhanca,) {
    // TODO: muda vizinhanca
    let resultado_x_atual = Solucao::evaluate(ObjetivoFn::f1(), &sol_atual.variaveis);
    let resultado_x_canditato = Solucao::evaluate(ObjetivoFn::f1(), &sol_candidata.variaveis);
    print!("---Sol atual {:?} \n", sol_atual);
    print!("---Sol candidata {:?} \n", sol_candidata);
    print!("---Sol atual (r) {:?} \n", resultado_x_atual.resultado);
    print!("---Sol candidata (r) {:?} \n", resultado_x_canditato.resultado);

    if resultado_x_canditato.resultado < resultado_x_atual.resultado {
        print!("- Nova solucao otima\n");
        vizinhanca.reset_estrutura_vizinhanca();
        *sol_atual = sol_candidata.clone();
    }
    else {
        print!("- Atualiza a vizinhanca\n");
        vizinhanca.atualiza_estrutura_vizinhanca();
    }
}

#[derive(Clone, Debug)]
pub struct Vizinhanca {
    pub tot_vizinhancas: u64, // diz o total de vizinhancas a serem visitadas
    pub passo_vizinhanca: f64, // diz a distancia do raio entre cada vizinhanca
    pub vizinhanca_atual: f64, // raio da vizinhanca atual
}

impl Vizinhanca {
    // DESCOMENTAR CASO VA USAR (pode ser que não use, ai só apaga)
    // pub fn set_tot_vizinhanca(&mut self, n: u64) {
    //     self.tot_vizinhancas = n;
    // }
    pub fn reset_estrutura_vizinhanca(&mut self) {
        self.vizinhanca_atual = self.tot_vizinhancas as f64 * self.passo_vizinhanca;
    }
    pub fn estruturas_totalmente_exploradas(&self) -> bool {
        self.vizinhanca_atual == 0.0
    }
    pub fn atualiza_estrutura_vizinhanca(&mut self) {
        self.vizinhanca_atual = self.vizinhanca_atual - self.passo_vizinhanca;
        if self.vizinhanca_atual < 0.0 {
            self.vizinhanca_atual = 0.0
        }
    }
    pub fn new(tot_vizinhancas: u64, passo_vizinhanca: f64 ) -> Self {
        let vizinhanca_atual = tot_vizinhancas as f64 * passo_vizinhanca;
        Vizinhanca { tot_vizinhancas, passo_vizinhanca, vizinhanca_atual}
    }
    pub fn print_raio(&self) {
        print!("Raio da vizinhanca atual: {}\n", self.vizinhanca_atual);
    }
    pub fn gera_vizinho_aleatorio(&self, solucao:&Solucao) -> Solucao {
        let mut variaveis : Vec<f64>= Vec::new();
        variaveis.resize(2, 0.0);
        let x = solucao.variaveis[0];
        let y = solucao.variaveis[1];

        // As funcoes trigonometricas de rust usam radianos
        let rand: f64 = rand::thread_rng().gen_range(0.0, 2.0*PI);

        print!("Valor do angulo (viz): {}\n", rand);
        variaveis[0] = x + self.vizinhanca_atual * rand.cos();
        variaveis[1] = y + self.vizinhanca_atual * rand.sin();
        
        let new_resultado= Solucao::evaluate(ObjetivoFn::f1(),&variaveis);
        new_resultado 
    }
    // pub fn calcula_vizinhanca(solucao: Solucao) -> &mut Self {}
}

// Teoria dos calculos
// A ideia para geração das vizinhanças é criar circulos cilindricos concentricos em torno da
// coordenada atual do mapa. O calculo para gerar uma nova casca dado um raio r é:
// - x = r cos(teta) , y = r sen(teta)
//
// A ideia é:
// - Cada vizinhança é um circulo. Logo, o valor da vizinhança é o raio do circulo
// - Um valor da vizinhança é um ponto sobre o círculo
