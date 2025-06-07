# Biblioteca Padrão da PBRLang

Este documento descreve os módulos e funções disponíveis na biblioteca padrão da PBRLang.

## Módulos Disponíveis

A biblioteca padrão da PBRLang é organizada em vários módulos, cada um com funções específicas:

- [texto](#módulo-texto): Manipulação de strings
- [numero](#módulo-numero): Funções matemáticas
- [arquivo](#módulo-arquivo): Operações de arquivo
- [sistema](#módulo-sistema): Interação com o sistema operacional
- [colecoes](#módulo-colecoes): Listas e mapas
- [console](#módulo-console): Interação com o console
- [web](#módulo-web): Requisições HTTP e manipulação de JSON

## Como Utilizar

Para usar um módulo da biblioteca padrão, use a palavra-chave `use`:

```pbr
use texto

faça principal() {
    pense nome = "mundo"
    mostre "Olá, " + texto.maiusculas(nome) + "!"
}
```

Você também pode usar um alias para o módulo:

```pbr
use texto como t

faça principal() {
    pense nome = "mundo"
    mostre "Olá, " + t.maiusculas(nome) + "!"
}
```

## Módulo texto

O módulo `texto` fornece funções para manipulação de strings.

### Funções

- **minusculas(texto: texto) -> texto**: Converte um texto para minúsculas
- **maiusculas(texto: texto) -> texto**: Converte um texto para maiúsculas
- **comeca_com(texto: texto, prefixo: texto) -> lógico**: Verifica se um texto começa com um prefixo
- **termina_com(texto: texto, sufixo: texto) -> lógico**: Verifica se um texto termina com um sufixo
- **substituir(texto: texto, busca: texto, substituto: texto) -> texto**: Substitui partes de um texto
- **dividir(texto: texto, separador: texto) -> lista de texto**: Divide um texto em partes
- **unir(lista: lista de texto, separador: texto) -> texto**: Une uma lista de textos em um único texto
- **tamanho(texto: texto) -> número**: Obtém o tamanho (número de caracteres) de um texto
- **esta_vazio(texto: texto) -> lógico**: Verifica se um texto está vazio
- **parte(texto: texto, inicio: número, fim: número) -> texto**: Obtém uma parte de um texto (substring)
- **aparar(texto: texto) -> texto**: Remove espaços em branco do início e fim do texto

### Exemplos

```pbr
use texto

pense nome = "João Silva"
mostre texto.maiusculas(nome)  // "JOÃO SILVA"
mostre texto.tamanho(nome)     // 10
```

## Módulo numero

O módulo `numero` fornece funções matemáticas e utilitários para números.

### Funções

- **absoluto(n: número) -> número**: Retorna o valor absoluto de um número
- **arredondar(n: número) -> número**: Arredonda um número para o inteiro mais próximo
- **arredondar_para_baixo(n: número) -> número**: Arredonda um número para baixo
- **arredondar_para_cima(n: número) -> número**: Arredonda um número para cima
- **potencia(base: número, expoente: número) -> número**: Calcula a potência de um número
- **raiz_quadrada(n: número) -> número**: Calcula a raiz quadrada de um número
- **maior(a: número, b: número) -> número**: Retorna o maior valor entre dois números
- **menor(a: número, b: número) -> número**: Retorna o menor valor entre dois números
- **é_par(n: número) -> lógico**: Verifica se um número é par
- **é_impar(n: número) -> lógico**: Verifica se um número é ímpar
- **texto_para_numero(texto: texto) -> número**: Converte um texto para número
- **numero_para_texto(n: número) -> texto**: Converte um número para texto

### Exemplos

```pbr
use numero

mostre numero.absoluto(-5)           // 5
mostre numero.potencia(2, 3)         // 8
mostre numero.é_par(10)              // verdadeiro
```

## Módulo arquivo

O módulo `arquivo` fornece funções para manipulação de arquivos.

### Funções

- **ler_texto(caminho: texto) -> texto**: Lê o conteúdo de um arquivo como texto
- **escrever_texto(caminho: texto, conteudo: texto) -> lógico**: Escreve texto em um arquivo
- **arquivo_existe(caminho: texto) -> lógico**: Verifica se um arquivo existe
- **criar_diretorio(caminho: texto) -> lógico**: Cria um diretório
- **listar_arquivos(diretorio: texto) -> lista de texto**: Lista arquivos em um diretório
- **remover_arquivo(caminho: texto) -> lógico**: Remove um arquivo
- **remover_diretorio(caminho: texto) -> lógico**: Remove um diretório
- **copiar_arquivo(origem: texto, destino: texto) -> lógico**: Copia um arquivo
- **mover_arquivo(origem: texto, destino: texto) -> lógico**: Move um arquivo
- **tamanho_arquivo(caminho: texto) -> número**: Obtém o tamanho de um arquivo em bytes
- **é_diretorio(caminho: texto) -> lógico**: Verifica se um caminho é um diretório
- **é_arquivo(caminho: texto) -> lógico**: Verifica se um caminho é um arquivo

### Exemplos

```pbr
use arquivo

se arquivo.arquivo_existe("dados.txt") {
    pense conteudo = arquivo.ler_texto("dados.txt")
    mostre conteudo
} senão {
    arquivo.escrever_texto("dados.txt", "Olá, mundo!")
}
```

## Módulo sistema

O módulo `sistema` fornece funções para interagir com o sistema operacional.

### Funções

- **executar_comando(comando: texto) -> texto**: Executa um comando do sistema operacional
- **obter_variavel_ambiente(nome: texto) -> texto**: Obtém variáveis de ambiente
- **definir_variavel_ambiente(nome: texto, valor: texto) -> lógico**: Define variáveis de ambiente
- **obter_diretorio_atual() -> texto**: Obtém o diretório atual
- **mudar_diretorio(caminho: texto) -> lógico**: Muda o diretório atual
- **obter_argumentos() -> lista de texto**: Obtém os argumentos da linha de comando
- **sair(codigo: número)**: Sai do programa com um código de status
- **dormir(milissegundos: número)**: Dorme por um número de milissegundos
- **obter_sistema_operacional() -> texto**: Obtém o nome do sistema operacional
- **obter_arquitetura() -> texto**: Obtém a arquitetura do sistema
- **obter_diretorio_home() -> texto**: Obtém o diretório home do usuário
- **obter_nome_usuario() -> texto**: Obtém o nome do usuário

### Exemplos

```pbr
use sistema

pense so = sistema.obter_sistema_operacional()
pense usuario = sistema.obter_nome_usuario()
mostre "Olá, " + usuario + "! Você está usando " + so
```

## Módulo colecoes

O módulo `colecoes` fornece funções para manipulação de listas e mapas.

### Funções para Listas

- **nova_lista() -> lista de coisa**: Cria uma nova lista vazia
- **adicionar(lista: lista de coisa, item: coisa)**: Adiciona um item ao final de uma lista
- **remover_indice(lista: lista de coisa, indice: número) -> coisa**: Remove um item de uma lista pelo índice
- **tamanho_lista(lista: lista de coisa) -> número**: Obtém o tamanho de uma lista
- **lista_vazia(lista: lista de coisa) -> lógico**: Verifica se uma lista está vazia
- **limpar_lista(lista: lista de coisa)**: Limpa uma lista (remove todos os itens)
- **inverter_lista(lista: lista de coisa)**: Inverte a ordem dos itens em uma lista
- **ordenar_lista(lista: lista de coisa)**: Ordena os itens de uma lista

### Funções para Mapas

- **novo_mapa() -> mapa de coisa**: Cria um novo mapa vazio
- **inserir(mapa: mapa de coisa, chave: texto, valor: coisa)**: Insere um par chave-valor em um mapa
- **remover_chave(mapa: mapa de coisa, chave: texto) -> coisa**: Remove um par chave-valor de um mapa
- **contem_chave(mapa: mapa de coisa, chave: texto) -> lógico**: Verifica se um mapa contém uma chave
- **chaves_mapa(mapa: mapa de coisa) -> lista de texto**: Obtém as chaves de um mapa
- **valores_mapa(mapa: mapa de coisa) -> lista de coisa**: Obtém os valores de um mapa
- **tamanho_mapa(mapa: mapa de coisa) -> número**: Obtém o tamanho de um mapa
- **mapa_vazio(mapa: mapa de coisa) -> lógico**: Verifica se um mapa está vazio
- **limpar_mapa(mapa: mapa de coisa)**: Limpa um mapa (remove todos os pares chave-valor)

### Exemplos

```pbr
use colecoes

pense numeros = [1, 2, 3, 4, 5]
colecoes.adicionar(numeros, 6)
colecoes.inverter_lista(numeros)
mostre numeros  // [6, 5, 4, 3, 2, 1]

pense pessoa = {
    "nome": "Maria",
    "idade": 30,
    "cidade": "São Paulo"
}
colecoes.inserir(pessoa, "profissao", "Desenvolvedora")
mostre colecoes.contem_chave(pessoa, "idade")  // verdadeiro
```

## Módulo console

O módulo `console` fornece funções para interação com o console.

### Funções

- **ler_linha() -> texto**: Lê uma linha de texto do console
- **ler_numero() -> número**: Lê um número do console
- **exibir(texto: texto)**: Exibe texto no console
- **exibir_linha(texto: texto)**: Exibe texto no console com uma nova linha no final
- **limpar_tela()**: Limpa a tela do console
- **exibir_colorido(texto: texto, cor: texto)**: Exibe texto colorido no console
- **exibir_menu(titulo: texto, opcoes: lista de texto) -> número**: Exibe um menu e obtém a escolha do usuário
- **exibir_progresso(atual: número, total: número, largura: número)**: Exibe uma barra de progresso

### Exemplos

```pbr
use console

console.exibir_linha("Qual é o seu nome?")
pense nome = console.ler_linha()
console.exibir_colorido("Olá, " + nome + "!", "verde")

pense opcoes = ["Novo jogo", "Carregar jogo", "Opções", "Sair"]
pense escolha = console.exibir_menu("Menu Principal", opcoes)
```

## Módulo web

O módulo `web` fornece funções para interação com a web.

### Funções

- **obter(url: texto) -> texto**: Realiza uma requisição HTTP GET
- **postar(url: texto, corpo: texto) -> texto**: Realiza uma requisição HTTP POST
- **colocar(url: texto, corpo: texto) -> texto**: Realiza uma requisição HTTP PUT
- **deletar(url: texto) -> texto**: Realiza uma requisição HTTP DELETE
- **codificar_url(texto: texto) -> texto**: Codifica um texto para URL (url encoding)
- **decodificar_url(texto: texto) -> texto**: Decodifica um texto de URL (url decoding)
- **para_json(objeto: coisa) -> texto**: Codifica um objeto para JSON
- **de_json(texto: texto) -> coisa**: Decodifica um texto JSON para um objeto
- **url_valida(url: texto) -> lógico**: Verifica se uma URL é válida
- **obter_status(url: texto) -> número**: Obtém o código de status de uma URL
- **baixar_arquivo(url: texto, caminho_destino: texto) -> lógico**: Faz download de um arquivo da web

### Exemplos

```pbr
use web

pense resposta = web.obter("https://api.exemplo.com/dados")
pense dados = web.de_json(resposta)
mostre dados["titulo"]

pense usuario = {
    "nome": "João",
    "email": "joao@exemplo.com"
}
pense json = web.para_json(usuario)
web.postar("https://api.exemplo.com/usuarios", json)
```

