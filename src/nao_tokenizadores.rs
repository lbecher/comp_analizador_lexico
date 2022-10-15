use std::{str::FromStr};
use pom::parser::*;

pub fn irrelevantes(entrada: &[u8]) -> Result<usize, String> {
    let analizador =
        one_of(b" \n\t").repeat(1..);

    match analizador.parse(entrada) {
        Ok(saida) => {
            return Ok(saida.len());
        },

        Err(_e) => {
            //println!("Erro: {:?}", e);
            return Err(String::from_str("Erro!").unwrap());
        },
    };
}

pub fn comentarios_de_linha(entrada: &[u8]) -> Result<usize, String> {
    let analizador =
        sym(b'/') +
        sym(b'/') + (
            one_of(b"qwertyuiopasdfghjklzxcvbnm") | 
            one_of(b"QWERTYUIOPASDFGHJKLZXCVBNM") | 
            one_of(b"1234567890") |
            one_of(b" \t\"\\'!@#$%&*()_-=+|,.:;?/<>")
        ).repeat(0..) + 
        sym(b'\n');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0.0);
            resultado.push(saida.0.0.1);

            for caractere in saida.0.1 {
                resultado.push(caractere.into());
            }

            resultado.push(saida.1);
            
            return Ok(resultado.len());
        },

        Err(_e) => {
            //println!("Erro: {:?}", e);
            return Err(String::from_str("Erro!").unwrap());
        },
    };
}

pub fn comentarios_de_bloco(entrada: &[u8]) -> Result<usize, String> {
    let analizador =
        sym(b'-') +
        sym(b'-') +
        sym(b'-') + (
            one_of(b"qwertyuiopasdfghjklzxcvbnm") |
            one_of(b"QWERTYUIOPASDFGHJKLZXCVBNM") | 
            one_of(b"1234567890") |
            one_of(b" \n\t\"\\'!@#$%&*()_=+|,.:;?/<>")
        ).repeat(0..) + 
        sym(b'-') +
        sym(b'-') +
        sym(b'-');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0.0.0.0.0);
            resultado.push(saida.0.0.0.0.0.1);
            resultado.push(saida.0.0.0.0.1);

            for caractere in saida.0.0.0.1 {
                resultado.push(caractere.into());
            }

            resultado.push(saida.0.0.1);
            resultado.push(saida.0.1);
            resultado.push(saida.1);
            return Ok(resultado.len());
        },

        Err(_e) => {
            return Err(String::from_str("Erro!").unwrap());
        },
    };
}