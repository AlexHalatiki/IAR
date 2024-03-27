use std::{cell::RefCell, rc::Rc};

use rand::Rng;

pub struct Formiga {
    raio_de_visao: i32,
    pub posicao_x: i32,
    pub posicao_y: i32,
    pub estado: bool,
}

impl Formiga {
    pub fn new(raio_de_visao: i32, posicao_x: i32, posicao_y: i32) -> Self {
        Self {
            raio_de_visao: raio_de_visao,
            posicao_x: posicao_x,
            posicao_y: posicao_y,
            estado: false,
        }
    }

    pub fn iteracao(
        &mut self,
        ambiente: &mut Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
        width: i32,
        height: i32,
    ) {
        let (corpo, _) = ambiente[self.posicao_x as usize][self.posicao_y as usize];

        if !self.estado && corpo {
            self.pegar(ambiente, width, height);
        }

        if self.estado && !corpo {
            self.soltar(ambiente, width, height);
        }

        self.andar(ambiente, width, height);
    }

    fn porcentagem_soltar(
        &self,
        ambiente: &mut Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
        width: i32,
        height: i32,
    ) -> f64 {
        let mut itens_redor: i32 = 0;

        let raio_32 = self.raio_de_visao as i32;

        let celulas_redor = (raio_32 * 2 + 1).pow(2) - 1;

        for i in self.posicao_x - raio_32..=self.posicao_x + raio_32 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = width + i;
            }

            if i >= width {
                indice_x = i - width;
            }

            for j in self.posicao_y - raio_32..=self.posicao_y + raio_32 {
                let mut indice_y = j;

                if j < 0 {
                    indice_y = height + j;
                }

                if j >= width {
                    indice_y = j - height;
                }

                let (corpo, _) = ambiente[indice_x as usize][indice_y as usize];

                if corpo {
                    itens_redor += 1;
                }
            }
        }

        itens_redor as f64 / celulas_redor as f64
    }

    fn pegar(
        &mut self,
        ambiente: &mut Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
        width: i32,
        height: i32,
    ) {
        let porcentagem_pegar = 1.0 - self.porcentagem_soltar(ambiente, width, height);

        if porcentagem_pegar == 0.0 {
            return;
        }

        let mut rng = rand::thread_rng();

        let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

        if numero_aleatorio > porcentagem_pegar {
            return;
        }

        self.estado = true;

        ambiente[self.posicao_x as usize][self.posicao_y as usize].0 = false;
    }

    fn soltar(
        &mut self,
        ambiente: &mut Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
        width: i32,
        height: i32,
    ) {
        let porcentagem_soltar = self.porcentagem_soltar(ambiente, width, height);

        if porcentagem_soltar == 0.0 {
            return;
        }

        let mut rng = rand::thread_rng();

        let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

        if numero_aleatorio > porcentagem_soltar {
            return;
        }

        self.estado = false;

        ambiente[self.posicao_x as usize][self.posicao_y as usize].0 = true;
    }

    fn andar(
        &mut self,
        ambiente: &mut Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
        width: i32,
        height: i32,
    ) {
        let mut sem_formiga: Vec<(i32, i32)> = Vec::new();

        for i in self.posicao_x - 1..=self.posicao_x + 1 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = width + i;
            }

            if i >= width {
                indice_x = i - width;
            }

            for j in self.posicao_y - 1..=self.posicao_y + 1 {
                let mut indice_y = j;

                if j < 0 {
                    indice_y = height + j;
                }

                if j >= width {
                    indice_y = j - height;
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

        let (corpo, _) = ambiente[x as usize][y as usize];

        let (corpo_atual, formiga_atual) =
            ambiente[self.posicao_x as usize][self.posicao_y as usize].clone();

        ambiente[x as usize][y as usize] = (corpo, formiga_atual);

        ambiente[self.posicao_x as usize][self.posicao_y as usize] = (corpo_atual, None);

        self.posicao_x = x;
        self.posicao_y = y;
    }
}
