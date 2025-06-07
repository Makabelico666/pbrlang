# Guia de Contribuição para PBRLang

Obrigado pelo interesse em contribuir com a PBRLang! Este documento fornece orientações para contribuir com o projeto.

## Como Contribuir

### Relatando Bugs

Se você encontrar um bug, por favor, crie uma issue no GitHub com as seguintes informações:

- Título claro e descritivo
- Passos detalhados para reproduzir o bug
- Comportamento esperado vs. comportamento observado
- Capturas de tela, se aplicável
- Ambiente (sistema operacional, versão da PBRLang, etc.)

### Sugerindo Melhorias

Para sugerir melhorias:

1. Verifique se sua ideia já foi sugerida
2. Crie uma nova issue com detalhes sobre sua sugestão
3. Explique por que isso seria útil para o projeto
4. Forneça exemplos de uso, se possível

### Pull Requests

1. Fork o repositório
2. Crie um branch para sua feature ou correção
3. Implemente suas alterações
4. Adicione testes para sua nova funcionalidade
5. Garanta que todos os testes estão passando
6. Envie um Pull Request

## Estrutura do Projeto

- `/src` - Código-fonte principal
- `/examples` - Exemplos de programas PBRLang
- `/docs` - Documentação
- `/tests` - Testes automatizados
- `/lib` - Biblioteca padrão

## Padrões de Código

- Use formatação consistente (execute `cargo fmt` antes de enviar)
- Adicione comentários claros e significativos
- Mantenha as funções pequenas e focadas
- Escreva testes para novas funcionalidades

## Adicionando Funcionalidades à Linguagem

Ao adicionar novas funcionalidades à linguagem:

1. Atualize o lexer para reconhecer novos tokens
2. Modifique o parser para suportar a nova sintaxe
3. Atualize o transpiler para gerar o código Rust correspondente
4. Adicione testes para a nova funcionalidade
5. Atualize a documentação (incluindo `REFERENCIA.md`)
6. Adicione exemplos demonstrando a nova funcionalidade

## Processo de Desenvolvimento

1. Escolha uma issue para trabalhar
2. Comente na issue que você está trabalhando nela
3. Desenvolva e teste localmente
4. Envie um Pull Request
5. Responda aos comentários de revisão

## Licença

Ao contribuir com o projeto, você concorda que suas contribuições serão licenciadas sob a mesma licença do projeto (MIT).

## Contato

Se você tiver dúvidas, entre em contato conosco através das issues do GitHub ou pelo email [contato@pbrlang.org](mailto:contato@pbrlang.org).

Obrigado por contribuir para tornar a PBRLang melhor!

