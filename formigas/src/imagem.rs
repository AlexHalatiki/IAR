use image::{Rgb, RgbImage};
use std::{cell::RefCell, rc::Rc};

use crate::{dado::Dado, formiga::Formiga, HEIGHT, WIDTH};

pub struct Imagem;

impl Imagem {
    fn grupo_rgb(grupo: i32) -> Rgb<u8> {
        match grupo {
            1 => Rgb([255, 0, 0]),      // Vermelho
            2 => Rgb([0, 255, 0]),      // Verde
            3 => Rgb([0, 0, 255]),      // Azul
            4 => Rgb([255, 255, 0]),    // Amarelo
            5 => Rgb([255, 0, 255]),    // Magenta
            6 => Rgb([0, 255, 255]),    // Ciano
            7 => Rgb([255, 128, 0]),    // Laranja
            8 => Rgb([128, 0, 255]),    // Roxo
            9 => Rgb([0, 0, 0]),        // Preto
            10 => Rgb([0, 128, 0]),     // Verde Escuro
            11 => Rgb([0, 0, 128]),     // Azul Escuro
            12 => Rgb([128, 0, 0]),     // Vermelho Escuro
            13 => Rgb([255, 192, 203]), // Rosa
            14 => Rgb([128, 128, 0]),   // Oliva
            15 => Rgb([0, 128, 128]),   // Turquesa
            _ => Rgb([255, 255, 255]),        // Preto para qualquer outro valor
        }
    }

    pub fn salvar_matriz(matriz: &Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>>, path: &str) {
        let mut imagem = RgbImage::new(WIDTH as u32, HEIGHT as u32);

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let (dado, formiga) = &matriz[i as usize][j as usize];

                if formiga.is_some() {
                    let rgb: Rgb<u8>;
                    if dado.is_some() {
                        rgb = Rgb([64, 64, 64]);
                    } else {
                        rgb = Rgb([192, 192, 192]);
                    }
                    imagem.put_pixel(i as u32, j as u32, rgb);
                    continue;
                }

                match dado {
                    Some(valor) => {
                        imagem.put_pixel(i as u32, j as u32, Self::grupo_rgb(valor.grupo))
                    }
                    None => imagem.put_pixel(i as u32, j as u32, Rgb([255, 255, 255])),
                }
            }
        }

        imagem.save(path).expect(&format!("Erro ao salvar imagem {}", path));
    }
}
