# Documentação da CLI PBRLang

Este documento descreve os comandos disponíveis na interface de linha de comando (CLI) da PBRLang.

## Visão geral

A ferramenta de linha de comando `pbr` permite criar, executar, testar e gerenciar projetos PBRLang.

## Comandos

### `pbr novo`

Cria um novo projeto PBRLang.

```bash
pbr novo nome_do_projeto
```

Isso criará uma nova pasta com a estrutura básica de um projeto PBRLang:
- `programa.pbr` - Arquivo principal do programa
- `testes/` - Diretório para testes
- `LEIAME.md` - Documentação básica
- `pbrlang.config` - Arquivo de configuração do projeto

### `pbr rodar`

Executa um programa PBRLang.

```bash
pbr rodar [arquivo.pbr]
```

Se nenhum arquivo for especificado, tentará executar `programa.pbr` no diretório atual.

### `pbr converter`

Converte um programa PBRLang para código Rust.

```bash
pbr converter [arquivo.pbr] [--saida arquivo.rs] [--apenas-gerar]
```

Opções:
- `--saida`, `-s`: Especifica o arquivo de saída para o código Rust
- `--apenas-gerar`, `-a`: Apenas gera o código Rust sem compilar

### `pbr testar`

Executa testes em um projeto PBRLang.

```bash
pbr testar [caminho]
```

Se nenhum caminho for especificado, tentará executar os testes na pasta `testes/` do diretório atual.

### `pbr montar`

Compila um programa PBRLang para um executável nativo.

```bash
pbr montar [arquivo.pbr]
```

Se nenhum arquivo for especificado, tentará compilar `programa.pbr` no diretório atual.

### `pbr empacotar`

Empacota um projeto PBRLang para distribuição.

```bash
pbr empacotar [caminho]
```

Se nenhum caminho for especificado, empacota o projeto no diretório atual.

## Exemplos

### Criar e executar um novo projeto

```bash
# Criar novo projeto
pbr novo meu_app

# Entrar no diretório do projeto
cd meu_app

# Executar o programa
pbr rodar
```

### Compilar um programa para executável nativo

```bash
# Compilar o programa
pbr montar programa.pbr

# O executável será criado no mesmo diretório
```

### Converter para Rust sem compilar

```bash
pbr converter programa.pbr --apenas-gerar
```

Isso gerará um arquivo `programa.rs` contendo o código Rust equivalente.

## Arquivo de configuração

O arquivo `pbrlang.config` no diretório do projeto define configurações como:

```
nome = "MeuProjeto"
versao = "0.1.0"
entrada = "programa.pbr"
```

Essas configurações são usadas pelos comandos `pbr montar` e `pbr empacotar`.

