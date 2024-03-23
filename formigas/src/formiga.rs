use crate::Matriz;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Formiga<'a> {
    ambiente: Rc<RefCell<Matriz<Formiga<'a>>>>,
    raio_de_visao: u16,
    pub posicao_x: i32,
    pub posicao_y: i32,
    pub estado: bool,
}

impl<'a> Formiga<'a> {
    pub fn new(
        ambiente: Rc<RefCell<Matriz<Formiga<'a>>>>,
        raio_de_visao: u16,
        posicao_x: i32,
        posicao_y: i32,
    ) -> Formiga<'a> {
        Formiga {
            ambiente: ambiente,
            raio_de_visao: raio_de_visao,
            posicao_x: posicao_x,
            posicao_y: posicao_y,
            estado: false,
        }
    }

    pub fn iteracao(&mut self) {
        let ambiente = self.ambiente.borrow();

        let (corpo, _) = ambiente.data[self.posicao_x as usize][self.posicao_y as usize];

        drop(ambiente);

        if !self.estado && corpo {
            self.pegar();
        }

        if self.estado && !corpo {
            self.soltar();
        }

        self.andar();
    }

    fn porcentagem_soltar(&self) -> f64 {
        let ambiente = self.ambiente.borrow();

        let mut itens_redor: i32 = 0;

        let raio_32 = self.raio_de_visao as i32;

        let celulas_redor = (raio_32 * 2 + 1).pow(2) - 1;

        for i in self.posicao_x - raio_32..=self.posicao_x + raio_32 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = ambiente.width + i;
            }

            if i >= ambiente.width {
                indice_x = i - ambiente.width;
            }

            for j in self.posicao_y - raio_32..=self.posicao_y + raio_32 {
                let mut indice_y = j;

                if j < 0 {
                    indice_y = ambiente.height + j;
                }

                if j >= ambiente.width {
                    indice_y = j - ambiente.height;
                }

                let (corpo, _) = ambiente.data[indice_x as usize][indice_y as usize];

                if corpo {
                    itens_redor += 1;
                }
            }
        }

        itens_redor as f64 / celulas_redor as f64
    }

    fn andar(&mut self) {
        let mut sem_formiga: Vec<(i32, i32)> = Vec::new();

        let mut ambiente = self.ambiente.borrow_mut();

        for i in self.posicao_x - 1..=self.posicao_x + 1 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = ambiente.width + i;
            }

            if i >= ambiente.width {
                indice_x = i - ambiente.width;
            }

            for j in self.posicao_y - 1..=self.posicao_y + 1 {
                let mut indice_y = j;

                if j < 0 {
                    indice_y = ambiente.height + j;
                }

                if j >= ambiente.width {
                    indice_y = j - ambiente.height;
                }

                let (_, formiga) = &ambiente.data[indice_x as usize][indice_y as usize];

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

        let (corpo, _) = ambiente.data[x as usize][y as usize];

        let (corpo_atual, formiga_atual) =
            ambiente.data[self.posicao_x as usize][self.posicao_y as usize].clone();

        ambiente.data[x as usize][y as usize] = (corpo, formiga_atual);

        ambiente.data[self.posicao_x as usize][self.posicao_y as usize] = (corpo_atual, None);

        self.posicao_x = x;
        self.posicao_y = y;
    }

    fn pegar(&mut self) {
        let porcentagem_pegar = 1.0 - self.porcentagem_soltar();

        if porcentagem_pegar == 0.0 {
            return;
        }

        let mut rng = rand::thread_rng();

        let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

        if numero_aleatorio > porcentagem_pegar {
            return;
        }

        self.estado = true;

        let mut ambiente = self.ambiente.borrow_mut();
        
        ambiente.data[self.posicao_x as usize][self.posicao_y as usize].0 = false;
    }

    fn soltar(&mut self) {
        let porcentagem_soltar = self.porcentagem_soltar();

        if porcentagem_soltar == 0.0 {
            return;
        }

        let mut rng = rand::thread_rng();

        let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

        if numero_aleatorio > porcentagem_soltar {
            return;
        }
        
        self.estado = false;

        let mut ambiente = self.ambiente.borrow_mut();
        
        ambiente.data[self.posicao_x as usize][self.posicao_y as usize].0 = true;
    }
}
