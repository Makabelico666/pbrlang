# Referência da Linguagem PBRLang

Este documento fornece uma referência completa da sintaxe e recursos da linguagem PBRLang.

## Sumário

- [Tipos de dados](#tipos-de-dados)
- [Variáveis](#variáveis)
- [Operadores](#operadores)
- [Estruturas de controle](#estruturas-de-controle)
- [Funções](#funções)
- [Entrada e Saída](#entrada-e-saída)
- [Tratamento de erros](#tratamento-de-erros)
- [Modelos (Structs)](#modelos-structs)
- [Comentários](#comentários)

## Tipos de dados

PBRLang suporta os seguintes tipos de dados básicos:

| Tipo | Descrição | Exemplo |
|------|-----------|---------|
| `texto` | Sequência de caracteres (string) | `"Olá, mundo!"` |
| `número` | Valor numérico (inteiro ou decimal) | `42`, `3.14` |
| `lógico` | Valor booleano (verdadeiro ou falso) | `verdadeiro`, `falso` |
| `nada` | Representa ausência de valor | `nada` |

## Variáveis

Variáveis são declaradas usando a palavra-chave `pense`:

```pbr
pense nome = "Maria"
pense idade = 25
pense é_estudante = verdadeiro
```

Você pode especificar o tipo explicitamente:

```pbr
pense nome: texto = "João"
pense pi: número = 3.14159
```

## Operadores

### Operadores aritméticos

- `+` - Adição ou concatenação de strings
- `-` - Subtração
- `*` - Multiplicação
- `/` - Divisão

### Operadores de comparação

- `>` - Maior que
- `<` - Menor que
- `>=` - Maior ou igual a
- `<=` - Menor ou igual a
- `é igual a` ou `==` - Igual a
- `!=` - Diferente de

### Operadores lógicos

- `e` - AND lógico
- `ou` - OR lógico
- `não` - NOT lógico

## Estruturas de controle

### Condicionais

```pbr
se idade >= 18 {
    mostre "Maior de idade"
} senão {
    mostre "Menor de idade"
}
```

### Loops

```pbr
para cada número de 1 até 10 {
    mostre "Número: " + número
}
```

## Funções

Declare funções com a palavra-chave `faça`:

```pbr
faça saudação(nome: texto) {
    mostre "Olá, " + nome + "!"
}

faça calcular_área(largura: número, altura: número) -> número {
    volte largura * altura
}
```

Chamando funções:

```pbr
saudação("Maria")
pense área = calcular_área(5, 3)
mostre "A área é: " + área
```

## Entrada e Saída

### Saída para o console

```pbr
mostre "Olá, mundo!"
mostre "A resposta é: " + 42
```

### Entrada do usuário

```pbr
pense nome = leia_texto("Digite seu nome: ")
mostre "Olá, " + nome + "!"
```

## Tratamento de erros

Use `quando der erro` e `se falhar com` para tratar erros:

```pbr
quando der erro {
    pense conteúdo = ler_arquivo("dados.txt")
    mostre conteúdo
} se falhar com erro {
    mostre "Erro ao ler arquivo: " + erro.mensagem
}
```

## Modelos (Structs)

Defina estruturas de dados personalizadas com `modelo`:

```pbr
modelo Pessoa {
    nome: texto
    idade: número
    endereço: texto
}

pense joão = Pessoa {
    nome: "João Silva",
    idade: 30,
    endereço: "Av. Brasil, 123"
}

mostre "Nome: " + joão.nome
```

## Comentários

Comentários de linha começam com `//`:

```pbr
// Este é um comentário de linha única
pense x = 10  // Comentário no final da linha
```

---

Para mais exemplos, consulte a pasta `examples/` no repositório do projeto.

