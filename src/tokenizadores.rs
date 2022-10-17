use std::str::FromStr;

use pom::parser::*;

pub fn bloc(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        seq(b"BLOC");

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            for caractere in saida {
                resultado.push(*caractere);
            }

            return Ok((resultado.len(), String::from_str("[bloc, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn set(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        seq(b"SET");

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            for caractere in saida {
                resultado.push(*caractere);
            }

            return Ok((resultado.len(), String::from_str("[set, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn print(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        seq(b"PRINT");

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            for caractere in saida {
                resultado.push(*caractere);
            }

            return Ok((resultado.len(), String::from_str("[print, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn scan(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        seq(b"SCAN");

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            for caractere in saida {
                resultado.push(*caractere);
            }

            return Ok((resultado.len(), String::from_str("[scan, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn operador(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        seq(b"DIVR") | seq(b"NOT") | seq(b"AND") | seq(b"ADD") |
        seq(b"SUB") | seq(b"MUL") | seq(b"DIV") | seq(b"OR") |
        seq(b"AE") | seq(b"BE") | seq(b"A") | seq(b"E") |
        seq(b"B");

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            for caractere in saida {
                resultado.push(*caractere);
            }

            return Ok((resultado.len(), format!("[operador, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn abre_bloco_condicional(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b'#') +
        (seq(b"INZ") | seq(b"WNZ") | seq(b"RUI")) +
        sym(b':');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0);
            
            for caractere in saida.0.1 {
                resultado.push(*caractere);
            }

            resultado.push(saida.1);

            let tamanho = resultado.len();

            resultado.drain(0..1);
            resultado.drain((resultado.len() - 1)..(resultado.len()));

            return Ok((tamanho, format!("[abre_bloco_condicional, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn fecha_bloco_condicional(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b'#') +
        (seq(b"INZ") | seq(b"WNZ") | seq(b"RUI")) +
        sym(b';');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0);
            
            for caractere in saida.0.1 {
                resultado.push(*caractere);
            }

            resultado.push(saida.1);

            let tamanho = resultado.len();

            resultado.drain(0..1);
            resultado.drain((resultado.len() - 1)..(resultado.len()));

            return Ok((tamanho, format!("[fecha_bloco_condicional, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn abre_bloco_de_codigo(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos_de_inicio = b"QWERTYUIOPASDFGHJKLZXCVBNM";
    let simbolos = b"_1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

    let analizador =
        sym(b'#') +
        one_of(simbolos_de_inicio.as_ref()) +
        one_of(simbolos.as_ref()).repeat(0..) +
        sym(b':');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0.0);
            resultado.push(saida.0.0.1);
            
            for caractere in saida.0.1 {
                resultado.push(caractere);
            }

            resultado.push(saida.1);

            let tamanho = resultado.len();

            resultado.drain(0..1);
            resultado.drain((resultado.len() - 1)..(resultado.len()));

            return Ok((tamanho, format!("[abre_bloco_de_codigo, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn fecha_bloco_de_codigo(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos_de_inicio = b"QWERTYUIOPASDFGHJKLZXCVBNM";
    let simbolos = b"_1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

    let analizador =
        sym(b'#') +
        one_of(simbolos_de_inicio.as_ref()) +
        one_of(simbolos.as_ref()).repeat(0..) +
        sym(b';');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0.0);
            resultado.push(saida.0.0.1);
            
            for caractere in saida.0.1 {
                resultado.push(caractere);
            }

            resultado.push(saida.1);

            let tamanho = resultado.len();

            resultado.drain(0..1);
            resultado.drain((resultado.len() - 1)..(resultado.len()));

            return Ok((tamanho, format!("[fecha_bloco_de_codigo, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn tipo_de_variavel(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        seq(b"CHR") | seq(b"STR") |
        seq(b"INT8") | seq(b"INT16") | seq(b"INT32") | seq(b"INT64") |
        seq(b"UINT8") | seq(b"UINT16") | seq(b"UINT32") | seq(b"UINT64");

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            for caractere in saida {
                resultado.push(*caractere);
            }

            return Ok((resultado.len(), format!("[tipo_de_variavel, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn id_de_variavel(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos_de_inicio = b"qwertyuiopasdfghjklzxcvbnm";
    let simbolos = b"_1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

    let analizador =
        one_of(simbolos_de_inicio.as_ref()) +
        one_of(simbolos.as_ref()).repeat(0..);

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0);
            
            for caractere in saida.1 {
                resultado.push(caractere);
            }

            return Ok((resultado.len(), format!("[id_de_variavel, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn id_de_bloco(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos_de_inicio = b"QWERTYUIOPASDFGHJKLZXCVBNM";
    let simbolos = b"_1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

    let analizador =
        one_of(simbolos_de_inicio.as_ref()) +
        one_of(simbolos.as_ref()).repeat(0..);

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0);
            
            for caractere in saida.1 {
                resultado.push(caractere);
            }

            return Ok((resultado.len(), format!("[id_de_bloco, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn virgula(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b',');

    match analizador.parse(entrada) {
        Ok(_saida) => {
            return Ok((1, String::from_str("[virgula, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn ponto_e_virgula(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b';');

    match analizador.parse(entrada) {
        Ok(_saida) => {
            return Ok((1, String::from_str("[ponto_e_virgula, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn abre_parenteses(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b'(');

    match analizador.parse(entrada) {
        Ok(_saida) => {
            return Ok((1, String::from_str("[abre_parenteses, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn fecha_parenteses(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b')');

    match analizador.parse(entrada) {
        Ok(_saida) => {
            return Ok((1, String::from_str("[fecha_parenteses, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn dois_pontos(entrada: &[u8]) -> Result<(usize, String), String> {
    let analizador =
        sym(b':');

    match analizador.parse(entrada) {
        Ok(_saida) => {
            return Ok((1, String::from_str("[dois_pontos, ]").unwrap()));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn numero(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos = b"1234567890";

    let analizador =
        (sym(b'+') | sym(b'-') | one_of(simbolos.as_ref())) +
        one_of(simbolos.as_ref()).repeat(0..);

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0);

            for caractere in saida.1 {
                resultado.push(caractere);
            }

            return Ok((resultado.len(), format!("[numero, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn string(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos = b"\n\t\0 1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

    let analizador =
        sym(b'"') +
        one_of(simbolos.as_ref()).repeat(0..) +
        sym(b'"');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0);

            for caractere in saida.0.1 {
                resultado.push(caractere);
            }

            resultado.push(saida.1);

            return Ok((resultado.len(), format!("[string, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}

pub fn caractere(entrada: &[u8]) -> Result<(usize, String), String> {
    let simbolos = b"\n\t\0 1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

    let analizador =
        sym(b'\'') +
        one_of(simbolos.as_ref()) +
        sym(b'\'');

    match analizador.parse(entrada) {
        Ok(saida) => {
            let mut resultado: Vec<u8> = Vec::new();

            resultado.push(saida.0.0);
            resultado.push(saida.0.1);
            resultado.push(saida.1);

            return Ok((resultado.len(), format!("[caractere, {}]", String::from_utf8(resultado).unwrap())));
        },

        Err(erro) => {
            return Err(format!("{:?}", erro));
        },
    };
}