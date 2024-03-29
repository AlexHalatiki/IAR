use rand::Rng;
use std::{cell::RefCell, rc::Rc};

use crate::{dado::Dado, ALFA, HEIGHT, K1, K2, RAIO_DE_VISAO, WIDTH};

pub struct Formiga {
    pub posicao_x: i32,
    pub posicao_y: i32,
    pub estado: Option<Dado>,
}

impl Formiga {
    pub fn new(posicao_x: i32, posicao_y: i32) -> Self {
        Self {
            posicao_x: posicao_x,
            posicao_y: posicao_y,
            estado: None,
        }
    }

    pub fn iteracao(
        &mut self,
        ambiente: &mut Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>>,
    ) {
        let dado = ambiente[self.posicao_x as usize][self.posicao_y as usize]
            .0
            .clone();

        if self.estado.is_none() && dado.is_some() {
            let similaridade = self.similaridade_dados(ambiente, dado.as_ref().unwrap());
            self.pegar(ambiente, similaridade);
        }

        if self.estado.is_some() && dado.is_none() {
            let similaridade = self.similaridade_dados(ambiente, self.estado.as_ref().unwrap());
            self.soltar(ambiente, similaridade);
        }

        self.andar(ambiente);
    }

    fn similaridade_dados(
        &self,
        ambiente: &mut Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>>,
        dado: &Dado,
    ) -> f64 {
        let celulas_redor: i32 = (RAIO_DE_VISAO * 2 + 1).pow(2) - 1;
        let mut somatorio_distancia: f64 = 0.0;

        let raio_32 = RAIO_DE_VISAO as i32;

        for i in self.posicao_x - raio_32..=self.posicao_x + raio_32 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = WIDTH + i;
            }

            if i >= WIDTH {
                indice_x = i - WIDTH;
            }

            for j in self.posicao_y - raio_32..=self.posicao_y + raio_32 {
                if i == self.posicao_x && j == self.posicao_y {
                    continue;
                }

                let mut indice_y = j;

                if j < 0 {
                    indice_y = HEIGHT + j;
                }

                if j >= HEIGHT {
                    indice_y = j - HEIGHT;
                }

                let dado_atual = &ambiente[indice_x as usize][indice_y as usize].0;

                if dado_atual.is_some() {
                    let dado_atual = dado_atual.as_ref().unwrap();
                    let mut distancia: f64 = 0.0;

                    for i in 0..dado.props.len() {
                        distancia += (dado.props[i] - dado_atual.props[i]).powf(2.0);
                    }

                    distancia = distancia.sqrt();
                    somatorio_distancia += 1.0 - distancia / ALFA;
                }
            }
        }

        if somatorio_distancia <= 0.0 {
            return 0.0;
        }

        somatorio_distancia / celulas_redor as f64
    }

    fn pegar(
        &mut self,
        ambiente: &mut Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>>,
        similaridade: f64,
    ) {
        let mut rng = rand::thread_rng();

        let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

        let porcentagem_pegar = (K1 / (K1 + similaridade)).powf(2.0);

        if numero_aleatorio > porcentagem_pegar {
            return;
        }

        self.estado = ambiente[self.posicao_x as usize][self.posicao_y as usize].0.clone();

        ambiente[self.posicao_x as usize][self.posicao_y as usize].0 = None;
    }

    fn soltar(
        &mut self,
        ambiente: &mut Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>>,
        similaridade: f64,
    ) {
        let mut rng = rand::thread_rng();

        let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

        let porcentagem_soltar = (similaridade / (K2 + similaridade)).powf(2.0);

        if numero_aleatorio > porcentagem_soltar {
            return;
        }

        ambiente[self.posicao_x as usize][self.posicao_y as usize].0 = self.estado.clone();

        self.estado = None;
    }

    fn andar(&mut self, ambiente: &mut Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>>) {
        let mut sem_formiga: Vec<(i32, i32)> = Vec::new();

        for i in self.posicao_x - 1..=self.posicao_x + 1 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = WIDTH + i;
            }

            if i >= WIDTH {
                indice_x = i - WIDTH;
            }

            for j in self.posicao_y - 1..=self.posicao_y + 1 {
                let mut indice_y = j;

                if j < 0 {
                    indice_y = HEIGHT + j;
                }

                if j >= HEIGHT {
                    indice_y = j - HEIGHT;
                }

                let formiga = &ambiente[indice_x as usize][indice_y as usize].1;

                if formiga.is_none() {
                    sem_formiga.push((indice_x, indice_y));
                }
            }
        }

        if sem_formiga.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();

        let (x, y) = sem_formiga[rng.gen_range(0..sem_formiga.len())];

        let corpo = ambiente[x as usize][y as usize].0.clone();

        let (corpo_atual, formiga_atual) =
            ambiente[self.posicao_x as usize][self.posicao_y as usize].clone();

        ambiente[x as usize][y as usize] = (corpo, formiga_atual);

        ambiente[self.posicao_x as usize][self.posicao_y as usize] = (corpo_atual, None);

        self.posicao_x = x;
        self.posicao_y = y;
    }
}
