# PBRLang — A Primeira Linguagem de Programação Totalmente Brasileira

> Fale com o computador no seu idioma.

> **PBRLang** é uma linguagem de programação de **alta produtividade, performance e acessibilidade**, 100% escrita em **português brasileiro**.

> Ideal para **educação, automação, desenvolvimento web, CLI, ciência de dados e aplicações reais**.

---

## 💡 Visão

**Tornar a PBRLang a linguagem mais acessível, segura e produtiva para quem fala português.**

Combinando a legibilidade do Python com a performance e segurança do Rust, PBRLang quer ser uma linguagem de propósito geral para todos os níveis de programadores.

---

## 🎯 Missão

- 📘 Que qualquer pessoa alfabetizada em português possa aprender.

- 🔐 Que aplicações sejam **seguras por padrão** (memória, concorrência, entrada de dados, etc.).

- ⚡ Que rode com **velocidade nativa**, sem intérprete lento.

- 🛠️ Que tenha **produtividade de desenvolvimento muito alta** (como Python).

- 🌍 Que possa ser usada em **educação, web, automação, IA, scripts, microserviços** e além.

- 🧩 Que tenha **extensibilidade real** com pacotes, bindings e interoperabilidade.

---

## 🚀 Diferenciais

| Característica               | Detalhes                                                                 |
|-----------------------------|--------------------------------------------------------------------------|
| **100% em português**        | Sintaxe simples, direta, natural, sem anglicismos                        |
| **Transpila para Rust**      | Código convertido para Rust puro e compilado com `cargo`                 |
| **CLI poderosa**             | Ferramenta `pbr` para criar, rodar, testar, empacotar e publicar         |
| **Segura por padrão**        | Sem ponteiros brutos, nulls controlados, concorrência segura             |
| **Tipagem simples e opcional**| Tipos automáticos, mas suportando declaração estática se desejado       |
| **Educacional e Profissional** | Ideal para escolas, universidades, startups e produção real             |
| **Biblioteca padrão em português** | De fácil leitura e uso prático — como Python                           |

---

## 📐 Exemplos de Código

### Olá, mundo

```pbr
mostre "Olá, mundo!"
```

### Função com retorno

```pbr
faça dobrar(valor: número) {
    volte valor * 2
}
```

### Condicional

```pbr
se idade >= 18 {
    mostre "Maior de idade"
} senão {
    mostre "Menor de idade"
}
```

### Repetição

```pbr
para cada número de 1 até 10 {
    mostre "Número: " + número
}
```

### Estrutura de dados

```pbr
modelo Pessoa {
    nome: texto
    idade: número
}
```

### Tratamento de erro

```pbr
quando der erro {
    pense resultado = abrir_arquivo("dados.txt")
    mostre resultado
} se falhar com erro {
    mostre "Falhou: " + erro.mensagem
}
```

## 📦 Instalação

### Pré-requisitos

- [Rust e Cargo](https://www.rust-lang.org/tools/install)

### Instalação rápida

#### Windows (PowerShell)
```powershell
./install.ps1
```

#### Linux/macOS
```bash
chmod +x ./install.sh
./install.sh
```

Para mais detalhes, veja [INSTALACAO.md](docs/INSTALACAO.md).

## 📚 Documentação

- [Instalação](docs/INSTALACAO.md) - Como instalar a PBRLang
- [Referência da Linguagem](docs/REFERENCIA.md) - Guia completo da sintaxe
- [CLI](docs/CLI.md) - Comandos da ferramenta de linha de comando
- [Biblioteca Padrão](docs/BIBLIOTECA_PADRAO.md) - Funções e módulos disponíveis
- [Sistema de Pacotes (Caixotes)](docs/CAIXOTES.md) - Como usar e criar pacotes

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor, leia [CONTRIBUINDO.md](CONTRIBUINDO.md) para detalhes sobre nosso código de conduta e o processo de envio de pull requests.

## 📄 Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.
