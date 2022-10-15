mod tokenizadores;
mod nao_tokenizadores;

use tokenizadores as tkz;
use nao_tokenizadores as ntkz;

fn main() {
    let entrada: Vec<u8> = b"// comentario de linha

    // c

    ---
    Comentario de bloco
    ---

    'd'

    
    \0".to_vec();


    laco(entrada);
}

fn laco(mut entrada: Vec<u8>) {
    let mut continuar_laco: bool = true;
    let mut proxima_analize: bool = true;

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
                    //println!("Prosseguindo para próxima análize...\n");
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
                    //println!("Prosseguindo para próxima análize...\n");
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
                    //println!("Prosseguindo para próxima análize...\n");
                },
            };
        }

        // ******************************************************
        // geração de tokens
        /*if proxima_analize {
            match tkz::string(&entrada) {
                Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    println!("Novo token: {}", resultado.1);
                    proxima_analize = false;
                }

                Err(_erro) => {

                }
            }
        }*/

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

        continuar_laco = !proxima_analize;
    }

    if entrada.len() > 1 {
        println!("Erro!");
    }
}