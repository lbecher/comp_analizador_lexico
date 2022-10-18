# Analizador Léxico

Analizador Léxico para disciplina de Compiladores do Curso de Ciência da Computação, por Luiz Fernando Becher de Araujo e Marcos Augusto Campagnaro. Código disponível no [GitHub](https://github.com/lbecher/comp_analizador_lexico/). Instruções para instalação do compilador Rust está disponível [aqui](https://www.rust-lang.org/pt-BR/tools/install).

## Execução

Após acessar a raiz do projeto, execute o comando:

```
cargo run --release -- /caminho/para/o/arquivo/de/entrada.lia
```

Ou, para executar em modo debug, execute o comando:

```
cargo run -- /caminho/para/o/arquivo/de/entrada.lia
```

## Sobre os testes disponibilizados

Na raiz do projeto há quatro arquivos ```.lia```. ```teste.lia``` e ```teste2.lia``` são exemplos que não terminam em erro, enquanto ```teste3.lia``` e ```teste4.lia``` possuem caracteres não previstos pela linguagem e geram erro.
