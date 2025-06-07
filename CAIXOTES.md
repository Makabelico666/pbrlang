# Sistema de Pacotes (Caixotes) da PBRLang

Este documento explica como funciona o sistema de pacotes da PBRLang, chamado "Caixotes".

## O que são Caixotes?

Caixotes são pacotes de código reutilizável que podem ser compartilhados e utilizados em projetos PBRLang. Eles são semelhantes aos pacotes do npm (JavaScript), crates do Cargo (Rust) ou gems do Ruby.

## Arquivo de Manifesto

Cada caixote é definido por um arquivo de manifesto chamado `caixote.pbr`, que contém metadados sobre o pacote e suas dependências.

Exemplo de arquivo `caixote.pbr`:

```
nome = "meu_caixote"
versao = "0.1.0"
autores = ["Seu Nome <seu.email@exemplo.com>"]
descricao = "Uma descrição do seu caixote"
principal = "src/principal.pbr"

dependencias = {
    "texto" = "1.0.0",
    "web" = "0.2.1"
}

palavras_chave = ["utilidades", "texto", "web"]
licenca = "MIT"
repositorio = "https://github.com/seu-usuario/meu_caixote"
```

## Comandos do CLI

### Iniciar um Novo Caixote

```bash
pbr caixote iniciar nome_do_caixote
```

Este comando cria um novo diretório com a estrutura básica de um caixote, incluindo o arquivo `caixote.pbr`.

### Adicionar um Caixote como Dependência

```bash
pbr caixote adicionar nome_do_caixote
```

Opcionalmente, você pode especificar uma versão:

```bash
pbr caixote adicionar nome_do_caixote --versao 1.2.3
```

### Listar Caixotes Instalados

```bash
pbr caixote listar
```

### Publicar um Caixote

```bash
pbr caixote publicar
```

### Remover um Caixote

```bash
pbr caixote remover nome_do_caixote
```

## Estrutura de Diretórios

A estrutura recomendada para um caixote é:

```
meu_caixote/
├── caixote.pbr
├── src/
│   └── principal.pbr
├── exemplos/
│   └── exemplo.pbr
└── testes/
    └── teste.pbr
```

## Importando Caixotes

Para usar um caixote em seu código, utilize a palavra-chave `use`:

```pbr
use texto como t

faça principal() {
    pense texto = "Olá, mundo!"
    mostre t.maiusculas(texto)
}
```

## Versionamento

PBRLang segue o versionamento semântico (SemVer):

- **Versão Maior (X.0.0)**: Mudanças incompatíveis com versões anteriores
- **Versão Menor (0.X.0)**: Adições compatíveis com versões anteriores
- **Correção (0.0.X)**: Correções de bugs compatíveis com versões anteriores

## Biblioteca Padrão

PBRLang vem com uma biblioteca padrão que inclui vários caixotes:

- **texto**: Manipulação de strings
- **numero**: Funções matemáticas
- **arquivo**: Operações de arquivo
- **sistema**: Interação com o sistema operacional
- **colecoes**: Listas e mapas
- **console**: Interação com o console
- **web**: Requisições HTTP e manipulação de JSON

## Segurança

Todos os caixotes passam por uma verificação de segurança antes de serem publicados no repositório oficial. Isso inclui análise estática de código e verificação de vulnerabilidades conhecidas.

## Repositório de Caixotes

O repositório oficial de caixotes está disponível em:

https://caixotes.pbrlang.org

Você pode navegar por categorias, buscar por palavras-chave, e ver avaliações e estatísticas de uso.

