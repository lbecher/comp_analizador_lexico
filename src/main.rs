mod tokenizadores;
mod nao_tokenizadores;

use tokenizadores as tkz;
use nao_tokenizadores as ntkz;

fn main() {
    let entrada: Vec<u8> =
b"#DATA:
    UINT32: x, y, zS;
#DATA;

#MAIN:
    SET: x, AND(x, y);
    BLOC: OPERACAO;
#MAIN;

#OPERACAO:
    #WNZ: NOT(B(zS, 100)):
        SET: zS, MUL(x, y);
    #WNZ;
#OPERACAO;
".to_vec();

    laco(entrada);
}

fn laco(mut entrada: Vec<u8>) {
    let mut continuar_laco: bool = true;
    let mut proxima_analize: bool;

    while continuar_laco && entrada.len() > 1 {
        proxima_analize = true;

        // ******************************************************
        // exclusão de símbolos e cadeias inúteis
        if proxima_analize {
            match ntkz::irrelevantes(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado);
                    println!("{} caracteres irrelevantes removidos!", resultado);
                    proxima_analize = false;
                },
        
                Err(_erro) => {
                    //println!("{}", _erro);
                },
            };
        }

        if proxima_analize {
            match ntkz::comentarios_de_linha(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado);
                    println!("{} caracteres de comentário de linha removidos!", resultado);
                    proxima_analize = false;
                },
        
                Err(_erro) => {
                    //println!("{}", _erro);
                },
            };
        }

        if proxima_analize {
            match ntkz::comentarios_de_bloco(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado);
                    println!("{} caracteres de comentário de bloco removidos!", resultado);
                    proxima_analize = false;
                },
        
                Err(_erro) => {

                },
            };
        }

        // ******************************************************
        // geração de tokens
        if proxima_analize {
            match tkz::bloc(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::set(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::operador(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::abre_bloco_condicional(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::fecha_bloco_condicional(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::abre_bloco_de_codigo(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::fecha_bloco_de_codigo(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::tipo_de_variavel(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::id_de_variavel(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::id_de_bloco(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::virgula(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::ponto_e_virgula(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::abre_parenteses(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::fecha_parenteses(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::dois_pontos(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::numero(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::string(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        if proxima_analize {
            match tkz::caractere(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }

        // se proxima_analize for verdadeiro nesse ponto
        // significa que ocorreu um erro léxico
        continuar_laco = !proxima_analize;
    }

    if entrada.len() > 1 {
        println!("Erro!");
    }
}