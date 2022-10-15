use std::str::FromStr;
use pom::parser::*;

pub fn string(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b'"') + (
            one_of(b"qwertyuiopasdfghjklzxcvbnm") | 
            one_of(b"QWERTYUIOPASDFGHJKLZXCVBNM") | 
            one_of(b"1234567890") |
            one_of(b" \0\n")
        ).repeat(0..) + 
        sym(b'"');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0);

            for caractere in saida.0.1 {
                resultado.push(caractere.into());
            }

            resultado.push(saida.1);

            return Ok((resultado.len(), format!("[string, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(_e) => {
            println!("Erro: {:?}", _e);
            return Err(String::from_str("Há um símbolo inválido na string!").unwrap());
        },
    };
}

pub fn caractere(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b'\'') + (
            one_of(b"qwertyuiopasdfghjklzxcvbnm") | 
            one_of(b"QWERTYUIOPASDFGHJKLZXCVBNM") | 
            one_of(b"1234567890") |
            sym(b' ')
        ) + 
        sym(b'\'');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0);
            resultado.push(saida.0.1);
            resultado.push(saida.1);

            return Ok((resultado.len(), format!("[caractere, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(_e) => {
            //println!("Erro: {:?}", e);
            return Err(String::from_str("Há um símbolo inválido na string!").unwrap());
        },
    };
}

