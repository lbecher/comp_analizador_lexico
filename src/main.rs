use std::env;
use std::fs::File;
use std::io::prelude::*;
use debug_print::debug_println;

mod tokenizadores;
mod nao_tokenizadores;

use tokenizadores as tkz;
use nao_tokenizadores as ntkz;

fn main() {
  // obtendo argumentos do terminal
  let argumentos: Vec<String> = env::args().collect();

  // abrindo arquivo .lia
  let mut arquivo = File::open(argumentos.get(1).unwrap())
    .expect("Não foi possível abrir o arquivo de entrada!");

  let mut entrada: Vec<u8> = Vec::new();
  arquivo.read_to_end(&mut entrada)
    .expect("Não foi possível ler o arquivo de entrada!");
  
  let mut entrada_string: String = String::from_utf8(entrada)
    .expect("Não foi possível gerar uma string a partir do arquivo!");
  
  // tratando símbolos especiais
  entrada_string = entrada_string.replace("\\n", "\n");
  entrada_string = entrada_string.replace("\\t", "\t");
  entrada_string = entrada_string.replace("\\0", "\0");

  // iniciando laço do analizador
  laco(entrada_string.as_bytes().to_vec());
}

fn laco(mut entrada: Vec<u8>) {
  let mut tokens: String = String::new();
  let mut erro_lexico: bool = false;

  while !erro_lexico && entrada.len() > 1 {

    // exclusão de símbolos e cadeias inúteis
    match ntkz::irrelevantes(&entrada) {
      Ok(resultado) => {
        entrada.drain(0..resultado);
        println!("{} caractere(s) irrelevante(s) removido(s)!", resultado);
      },
      Err(_erro) => {
        debug_println!("{}", _erro);
        match ntkz::comentarios_de_linha(&entrada) {
          Ok(resultado) => {
            entrada.drain(0..resultado);
            println!("{} caracteres de comentário de linha removidos!", resultado);
          },
          Err(_erro) => {
            debug_println!("{}", _erro);
            match ntkz::comentarios_de_bloco(&entrada) {
              Ok(resultado) => {
                entrada.drain(0..resultado);
                println!("{} caracteres de comentário de bloco removidos!", resultado);
              },
              Err(_erro) => {
                debug_println!("{}", _erro);
                
                // geração de tokens
                match tkz::virgula(&entrada) {
                  Ok(resultado) => {
                    entrada.drain(0..resultado.0);
                    tokens = format!("{}\n{}", tokens, resultado.1);
                    println!("Novo token: {}", resultado.1);
                  },
                  Err(_erro) => {
                    debug_println!("{}", _erro);
                    match tkz::dois_pontos(&entrada) {
                      Ok(resultado) => {
                        entrada.drain(0..resultado.0);
                        tokens = format!("{}\n{}", tokens, resultado.1);
                        println!("Novo token: {}", resultado.1);
                      },
                      Err(_erro) => {
                        debug_println!("{}", _erro);
                        match tkz::ponto_e_virgula(&entrada) {
                          Ok(resultado) => {
                            entrada.drain(0..resultado.0);
                            tokens = format!("{}\n{}", tokens, resultado.1);
                            println!("Novo token: {}", resultado.1);
                          },
                          Err(_erro) => {
                            debug_println!("{}", _erro);
                            match tkz::abre_parenteses(&entrada) {
                              Ok(resultado) => {
                                entrada.drain(0..resultado.0);
                                tokens = format!("{}\n{}", tokens, resultado.1);
                                println!("Novo token: {}", resultado.1);
                              },
                              Err(_erro) => {
                                debug_println!("{}", _erro);
                                match tkz::fecha_parenteses(&entrada) {
                                  Ok(resultado) => {
                                    entrada.drain(0..resultado.0);
                                    tokens = format!("{}\n{}", tokens, resultado.1);
                                    println!("Novo token: {}", resultado.1);
                                  },
                                  Err(_erro) => {
                                    debug_println!("{}", _erro);
                                    match tkz::set(&entrada) {
                                      Ok(resultado) => {
                                        entrada.drain(0..resultado.0);
                                        tokens = format!("{}\n{}", tokens, resultado.1);
                                        println!("Novo token: {}", resultado.1);
                                      },
                                      Err(_erro) => {
                                        debug_println!("{}", _erro);
                                        match tkz::print(&entrada) {
                                          Ok(resultado) => {
                                            entrada.drain(0..resultado.0);
                                            tokens = format!("{}\n{}", tokens, resultado.1);
                                            println!("Novo token: {}", resultado.1);
                                          },
                                          Err(_erro) => {
                                            debug_println!("{}", _erro);
                                            match tkz::scan(&entrada) {
                                              Ok(resultado) => {
                                                entrada.drain(0..resultado.0);
                                                tokens = format!("{}\n{}", tokens, resultado.1);
                                                println!("Novo token: {}", resultado.1);
                                              },
                                              Err(_erro) => {
                                                debug_println!("{}", _erro);
                                                match tkz::bloc(&entrada) {
                                                  Ok(resultado) => {
                                                    entrada.drain(0..resultado.0);
                                                    tokens = format!("{}\n{}", tokens, resultado.1);
                                                    println!("Novo token: {}", resultado.1);
                                                  },
                                                  Err(_erro) => {
                                                    debug_println!("{}", _erro);
                                                    match tkz::operador(&entrada) {
                                                      Ok(resultado) => {
                                                        entrada.drain(0..resultado.0);
                                                        tokens = format!("{}\n{}", tokens, resultado.1);
                                                        println!("Novo token: {}", resultado.1);
                                                      },
                                                      Err(_erro) => {
                                                        debug_println!("{}", _erro);
                                                        match tkz::tipo_de_variavel(&entrada) {
                                                          Ok(resultado) => {
                                                            entrada.drain(0..resultado.0);
                                                            tokens = format!("{}\n{}", tokens, resultado.1);
                                                            println!("Novo token: {}", resultado.1);
                                                          },
                                                          Err(_erro) => {
                                                            debug_println!("{}", _erro);
                                                            match tkz::id_de_variavel(&entrada) {
                                                              Ok(resultado) => {
                                                                entrada.drain(0..resultado.0);
                                                                tokens = format!("{}\n{}", tokens, resultado.1);
                                                                println!("Novo token: {}", resultado.1);
                                                              },
                                                              Err(_erro) => {
                                                                debug_println!("{}", _erro);
                                                                match tkz::id_de_bloco(&entrada) {
                                                                  Ok(resultado) => {
                                                                    entrada.drain(0..resultado.0);
                                                                    tokens = format!("{}\n{}", tokens, resultado.1);
                                                                    println!("Novo token: {}", resultado.1);
                                                                  },
                                                                  Err(_erro) => {
                                                                    debug_println!("{}", _erro);
                                                                    match tkz::abre_bloco_condicional(&entrada) {
                                                                      Ok(resultado) => {
                                                                        entrada.drain(0..resultado.0);
                                                                        tokens = format!("{}\n{}", tokens, resultado.1);
                                                                        println!("Novo token: {}", resultado.1);
                                                                      },
                                                                      Err(_erro) => {
                                                                        debug_println!("{}", _erro);
                                                                        match tkz::fecha_bloco_condicional(&entrada) {
                                                                          Ok(resultado) => {
                                                                            entrada.drain(0..resultado.0);
                                                                            tokens = format!("{}\n{}", tokens, resultado.1);
                                                                            println!("Novo token: {}", resultado.1);
                                                                          },
                                                                          Err(_erro) => {
                                                                            debug_println!("{}", _erro);
                                                                            match tkz::abre_bloco_de_codigo(&entrada) {
                                                                              Ok(resultado) => {
                                                                                entrada.drain(0..resultado.0);
                                                                                tokens = format!("{}\n{}", tokens, resultado.1);
                                                                                println!("Novo token: {}", resultado.1);
                                                                              },
                                                                              Err(_erro) => {
                                                                                debug_println!("{}", _erro);
                                                                                match tkz::fecha_bloco_de_codigo(&entrada) {
                                                                                  Ok(resultado) => {
                                                                                    entrada.drain(0..resultado.0);
                                                                                    tokens = format!("{}\n{}", tokens, resultado.1);
                                                                                    println!("Novo token: {}", resultado.1);
                                                                                  },
                                                                                  Err(_erro) => {
                                                                                    debug_println!("{}", _erro);
                                                                                    match tkz::caractere(&entrada) {
                                                                                      Ok(resultado) => {
                                                                                        entrada.drain(0..resultado.0);
                                                                                        tokens = format!("{}\n{}", tokens, resultado.1);
                                                                                        println!("Novo token: {}", resultado.1);
                                                                                      },
                                                                                      Err(_erro) => {
                                                                                        debug_println!("{}", _erro);
                                                                                        match tkz::numero(&entrada) {
                                                                                          Ok(resultado) => {
                                                                                            entrada.drain(0..resultado.0);
                                                                                            tokens = format!("{}\n{}", tokens, resultado.1);
                                                                                            println!("Novo token: {}", resultado.1);
                                                                                          },
                                                                                          Err(_erro) => {
                                                                                            debug_println!("{}", _erro);
                                                                                            match tkz::string(&entrada) {
                                                                                              Ok(resultado) => {
                                                                                                entrada.drain(0..resultado.0);
                                                                                                tokens = format!("{}\n{}", tokens, resultado.1);
                                                                                                println!("Novo token: {}", resultado.1);
                                                                                              },
                                                                                              Err(_erro) => {
                                                                                                debug_println!("{}", _erro);
                                                                                                println!("ERRO: uma cadeia de símbolos que não pode ser reconhecida foi encontrada!");
                                                                                                erro_lexico = true;
                                                                                              },
                                                                                            };
                                                                                          },
                                                                                        };
                                                                                      },
                                                                                    };
                                                                                  },
                                                                                };
                                                                              },
                                                                            };
                                                                          },
                                                                        };
                                                                      },
                                                                    };
                                                                  },
                                                                };
                                                              },
                                                            };
                                                          },
                                                        };
                                                      },
                                                    };
                                                  },
                                                };
                                              },
                                            };
                                          },
                                        };
                                      },
                                    };
                                  },
                                };
                              },
                            };
                          },
                        };
                      },
                    };
                  },
                };
              },
            };
          },
        };
      },
    };
  }
  
  // escrevendo resultado em um arquivo de saída
  if !erro_lexico {
    let mut saida = File::create("tokens.txt")
      .expect("Não foi possível criar o arquivo de saída!");
      
    saida.write_all(tokens.as_bytes())
      .expect("Erro ao gravar no arquivo de saída!");
  }
}