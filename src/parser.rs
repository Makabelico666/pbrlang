use crate::ast::{Declaracao, Expressao, Operador, Programa, Tipo};
use crate::lexer::Token;
use anyhow::{anyhow, Context, Result};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PBRParser;

/// Converte uma lista de tokens em uma AST
pub fn analisar(tokens: Vec<Token>) -> Result<Programa> {
    // Por enquanto, esta é apenas uma implementação básica que constrói
    // uma AST simples diretamente a partir dos tokens
    let mut programa = Programa::new();
    
    let mut i = 0;
    while i < tokens.len() && tokens[i] != Token::EOF {
        match &tokens[i] {
            Token::Mostre => {
                i += 1; // Avançar depois de 'mostre'
                
                if i < tokens.len() {
                    // Pode ser uma string literal simples ou uma expressão de concatenação
                    let expr = analisar_expressao(&mut i, &tokens)?;
                    programa.adicionar_declaracao(Declaracao::Mostrar(expr));
                } else {
                    return Err(anyhow::anyhow!("Expressão incompleta após 'mostre'"));
                }
            },
            
            Token::Faca => {
                i += 1; // Avançar depois de 'faça'
                
                // Extrai o nome da função
                let nome = if i < tokens.len() {
                    if let Token::Identificador(nome) = &tokens[i] {
                        nome.clone()
                    } else {
                        return Err(anyhow::anyhow!("Esperava nome de função após 'faça'"));
                    }
                } else {
                    return Err(anyhow::anyhow!("Expressão incompleta após 'faça'"));
                };
                
                i += 1; // Avançar depois do nome
                
                // Parâmetros
                let mut parametros = Vec::new();
                if i < tokens.len() && tokens[i] == Token::AbreParentese {
                    i += 1; // Avançar depois de '('
                    
                    while i < tokens.len() && tokens[i] != Token::FechaParentese {
                        if let Token::Identificador(nome_param) = &tokens[i] {
                            i += 1; // Avançar depois do nome do parâmetro
                            
                            // Verifica se há um tipo especificado
                            if i < tokens.len() && tokens[i] == Token::DoisPontos {
                                i += 1; // Avançar depois de ':'
                                
                                let tipo = match &tokens[i] {
                                    Token::TipoTexto => Tipo::Texto,
                                    Token::TipoNumero => Tipo::Numero,
                                    Token::TipoLogico => Tipo::Logico,
                                    Token::Identificador(nome_tipo) => Tipo::Personalizado(nome_tipo.clone()),
                                    _ => return Err(anyhow::anyhow!("Tipo de parâmetro inválido")),
                                };
                                
                                parametros.push((nome_param.clone(), tipo));
                                i += 1; // Avançar depois do tipo
                                
                                // Verifica se há uma vírgula
                                if i < tokens.len() && tokens[i] == Token::Virgula {
                                    i += 1; // Avançar depois da vírgula
                                }
                            } else {
                                return Err(anyhow::anyhow!("Esperava ':' após nome do parâmetro"));
                            }
                        } else {
                            return Err(anyhow::anyhow!("Parâmetro de função inválido"));
                        }
                    }
                    
                    i += 1; // Avançar depois de ')'
                } else {
                    return Err(anyhow::anyhow!("Esperava '(' após nome da função"));
                }
                
                // Corpo da função
                if i < tokens.len() && tokens[i] == Token::AbreChave {
                    i += 1; // Avançar depois de '{'
                    
                    // Parse do corpo da função (simplificado)
                    let mut declaracoes_corpo = Vec::new();
                    
                    while i < tokens.len() && tokens[i] != Token::FechaChave {
                        match &tokens[i] {
                            Token::Volte => {
                                i += 1; // Avançar depois de 'volte'
                                
                                if i < tokens.len() {
                                    if let Token::NumeroLiteral(valor) = tokens[i] {
                                        let expr = Expressao::NumeroLiteral(valor);
                                        declaracoes_corpo.push(Declaracao::Retorno(Some(expr)));
                                        i += 1;
                                    } else if let Token::TextoLiteral(texto) = &tokens[i] {
                                        let expr = Expressao::TextoLiteral(texto.clone());
                                        declaracoes_corpo.push(Declaracao::Retorno(Some(expr)));
                                        i += 1;
                                    } else if let Token::Identificador(id) = &tokens[i] {
                                        let expr = Expressao::Identificador(id.clone());
                                        declaracoes_corpo.push(Declaracao::Retorno(Some(expr)));
                                        i += 1;
                                    } else {
                                        return Err(anyhow::anyhow!("Expressão de retorno inválida"));
                                    }
                                } else {
                                    return Err(anyhow::anyhow!("Expressão incompleta após 'volte'"));
                                }
                            },
                            
                            Token::Mostre => {
                                i += 1; // Avançar depois de 'mostre'
                                
                                if i < tokens.len() {
                                    // Pode ser uma string literal simples ou uma expressão de concatenação
                                    let expr = analisar_expressao(&mut i, &tokens)?;
                                    declaracoes_corpo.push(Declaracao::Mostrar(expr));
                                } else {
                                    return Err(anyhow::anyhow!("Expressão incompleta após 'mostre'"));
                                }
                            },
                            
                            // Outros tipos de declarações no corpo...
                            _ => {
                                // Simplificado: pular tokens desconhecidos
                                i += 1;
                            }
                        }
                    }
                    
                    i += 1; // Avançar depois de '}'
                    
                    let corpo = Declaracao::Bloco(declaracoes_corpo);
                    programa.adicionar_declaracao(Declaracao::Funcao {
                        nome,
                        parametros,
                        tipo_retorno: None, // Simplificado
                        corpo: Box::new(corpo),
                        publico: false,
                    });
                } else {
                    return Err(anyhow::anyhow!("Esperava '{{' para o corpo da função"));
                }
            },
            
            // Chamada de função
            Token::Identificador(nome) => {
                let nome_func = nome.clone();
                i += 1; // Avançar depois do nome da função
                
                if i < tokens.len() && tokens[i] == Token::AbreParentese {
                    i += 1; // Avançar depois de '('
                    
                    let mut argumentos = Vec::new();
                    
                    // Analisar argumentos
                    while i < tokens.len() && tokens[i] != Token::FechaParentese {
                        // Analisar o argumento (que pode ser uma expressão)
                        let arg = analisar_expressao(&mut i, &tokens)?;
                        argumentos.push(arg);
                        
                        // Verificar se há uma vírgula
                        if i < tokens.len() && tokens[i] == Token::Virgula {
                            i += 1; // Avançar depois da vírgula
                        }
                    }
                    
                    if i < tokens.len() && tokens[i] == Token::FechaParentese {
                        i += 1; // Avançar depois de ')'
                        
                        let expr = Expressao::Chamada {
                            nome: nome_func,
                            argumentos,
                        };
                        
                        programa.adicionar_declaracao(Declaracao::Expressao(expr));
                    } else {
                        return Err(anyhow::anyhow!("Esperava ')' após argumentos da função"));
                    }
                } else {
                    return Err(anyhow::anyhow!("Esperava '(' após nome da função"));
                }
            },
            
            // Outros tipos de declarações...
            _ => {
                // Simplificado: pular tokens desconhecidos
                i += 1;
            }
        }
    }
    
    Ok(programa)
}

// Implementação completa usando pest
pub fn analisar_com_pest(codigo: &str) -> Result<Programa> {
    let resultado = PBRParser::parse(Rule::programa, codigo)
        .with_context(|| "Erro de análise de gramática PBRLang")?;
    
    // O programa é o primeiro par da análise
    if let Some(programa_par) = resultado.peek() {
        return PBRParser::parse_programa(programa_par);
    }
    
    // Se não houver programa, retornar um programa vazio
    Ok(Programa::new())
}

impl PBRParser {
    fn parse_programa(pair: Pair<Rule>) -> Result<Programa> {
        let mut programa = Programa::new();
        
        // Iterar sobre as declarações dentro do programa
        for declaracao in pair.into_inner() {
            match declaracao.as_rule() {
                Rule::declaracao => {
                    let decl = Self::parse_declaracao(declaracao)?;
                    programa.adicionar_declaracao(decl);
                },
                Rule::EOI => {}, // Fim de arquivo, ignorar
                _ => {
                    return Err(anyhow!("Regra inesperada na análise do programa: {:?}", 
                                      declaracao.as_rule()));
                }
            }
        }
        
        Ok(programa)
    }
    
    fn parse_declaracao(pair: Pair<Rule>) -> Result<Declaracao> {
        let inner = pair.into_inner().next()
            .ok_or_else(|| anyhow!("Declaração vazia"))?;
        
        match inner.as_rule() {
            Rule::declaracao_variavel => Self::parse_declaracao_variavel(inner),
            Rule::declaracao_funcao => Self::parse_declaracao_funcao(inner),
            Rule::declaracao_retorno => Self::parse_declaracao_retorno(inner),
            Rule::declaracao_condicional => Self::parse_declaracao_condicional(inner),
            Rule::declaracao_para_cada => Self::parse_declaracao_para_cada(inner),
            Rule::declaracao_enquanto => Self::parse_declaracao_enquanto(inner),
            Rule::declaracao_repita => Self::parse_declaracao_repita(inner),
            Rule::declaracao_pare => Ok(Declaracao::Pare),
            Rule::declaracao_continue => Ok(Declaracao::Continue),
            Rule::declaracao_quando_der_erro => Self::parse_declaracao_quando_der_erro(inner),
            Rule::declaracao_modelo => Self::parse_declaracao_modelo(inner),
            Rule::declaracao_modulo => Self::parse_declaracao_modulo(inner),
            Rule::declaracao_importar => Self::parse_declaracao_importar(inner),
            Rule::declaracao_mostrar => Self::parse_declaracao_mostrar(inner),
            Rule::expressao => {
                let expr = Self::parse_expressao(inner)?;
                Ok(Declaracao::Expressao(expr))
            },
            _ => Err(anyhow!("Tipo de declaração desconhecido: {:?}", inner.as_rule())),
        }
    }
    
    fn parse_declaracao_variavel(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        let mut publico = false;
        
        // Verifica se há modificador de visibilidade
        let primeiro = inner.peek().ok_or_else(|| anyhow!("Declaração de variável vazia"))?;
        if primeiro.as_rule() == Rule::modificador_visibilidade {
            publico = true;
            inner.next(); // Consome o modificador
        }
        
        // Pega o "pense" (já consumido na verificação da regra)
        let identificador = inner.next()
            .ok_or_else(|| anyhow!("Identificador não encontrado na declaração de variável"))?;
        
        let nome = identificador.as_str().to_string();
        let mut tipo = None;
        let mut valor = None;
        
        // Verifica se há tipo e valor
        while let Some(next) = inner.next() {
            match next.as_rule() {
                Rule::tipo => {
                    tipo = Some(Self::parse_tipo(next)?);
                },
                Rule::expressao => {
                    valor = Some(Self::parse_expressao(next)?);
                },
                _ => {
                    return Err(anyhow!("Elemento inesperado na declaração de variável: {:?}", 
                                      next.as_rule()));
                }
            }
        }
        
        Ok(Declaracao::Variavel { nome, tipo, valor, publico })
    }
    
    fn parse_declaracao_funcao(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        let mut publico = false;
        
        // Verifica se há modificador de visibilidade
        let primeiro = inner.peek().ok_or_else(|| anyhow!("Declaração de função vazia"))?;
        if primeiro.as_rule() == Rule::modificador_visibilidade {
            publico = true;
            inner.next(); // Consome o modificador
        }
        
        // Pega o nome da função
        let nome_par = inner.next()
            .ok_or_else(|| anyhow!("Nome da função não encontrado"))?;
        let nome = nome_par.as_str().to_string();
        
        // Pega os parâmetros
        let params_par = inner.next()
            .ok_or_else(|| anyhow!("Parâmetros da função não encontrados"))?;
        let parametros = Self::parse_parametros(params_par)?;
        
        // Verifica se há tipo de retorno
        let mut tipo_retorno = None;
        let proximo = inner.peek();
        
        if let Some(next) = proximo {
            if next.as_rule() == Rule::tipo {
                inner.next(); // Consome o tipo
                tipo_retorno = Some(Self::parse_tipo(next)?);
            }
        }
        
        // Pega o corpo da função
        let bloco_par = inner.next()
            .ok_or_else(|| anyhow!("Corpo da função não encontrado"))?;
        let corpo = Self::parse_bloco(bloco_par)?;
        
        Ok(Declaracao::Funcao {
            nome,
            parametros,
            tipo_retorno,
            corpo: Box::new(corpo),
            publico,
        })
    }
    
    fn parse_parametros(pair: Pair<Rule>) -> Result<Vec<(String, Tipo)>> {
        let mut parametros = Vec::new();
        
        for param in pair.into_inner() {
            if param.as_rule() == Rule::parametro {
                let mut inner = param.into_inner();
                
                let nome_par = inner.next()
                    .ok_or_else(|| anyhow!("Nome do parâmetro não encontrado"))?;
                let nome = nome_par.as_str().to_string();
                
                let tipo_par = inner.next()
                    .ok_or_else(|| anyhow!("Tipo do parâmetro não encontrado"))?;
                let tipo = Self::parse_tipo(tipo_par)?;
                
                parametros.push((nome, tipo));
            }
        }
        
        Ok(parametros)
    }
    
    fn parse_tipo(pair: Pair<Rule>) -> Result<Tipo> {
        let tipo_str = pair.as_str();
        
        match tipo_str {
            "texto" => Ok(Tipo::Texto),
            "número" | "numero" => Ok(Tipo::Numero),
            "lógico" | "logico" => Ok(Tipo::Logico),
            _ => Ok(Tipo::Personalizado(tipo_str.to_string())),
        }
    }
    
    fn parse_bloco(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut declaracoes = Vec::new();
        
        for declaracao in pair.into_inner() {
            if declaracao.as_rule() == Rule::declaracao {
                let decl = Self::parse_declaracao(declaracao)?;
                declaracoes.push(decl);
            }
        }
        
        Ok(Declaracao::Bloco(declaracoes))
    }
    
    fn parse_declaracao_retorno(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut expressao = None;
        
        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::expressao {
                expressao = Some(Self::parse_expressao(inner)?);
                break;
            }
        }
        
        Ok(Declaracao::Retorno(expressao))
    }
    
    fn parse_declaracao_condicional(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        
        let condicao_par = inner.next()
            .ok_or_else(|| anyhow!("Condição não encontrada na declaração condicional"))?;
        let condicao = Self::parse_expressao(condicao_par)?;
        
        let bloco_se_par = inner.next()
            .ok_or_else(|| anyhow!("Bloco 'se' não encontrado na declaração condicional"))?;
        let bloco_se = Self::parse_bloco(bloco_se_par)?;
        
        let mut bloco_senao = None;
        if let Some(senao_par) = inner.next() {
            let senao = Self::parse_bloco(senao_par)?;
            bloco_senao = Some(Box::new(senao));
        }
        
        Ok(Declaracao::Se {
            condicao,
            bloco_se: Box::new(bloco_se),
            bloco_senao,
        })
    }
    
    fn parse_declaracao_para_cada(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        
        let var_par = inner.next()
            .ok_or_else(|| anyhow!("Variável não encontrada no loop para cada"))?;
        let variavel = var_par.as_str().to_string();
        
        let inicio_par = inner.next()
            .ok_or_else(|| anyhow!("Expressão de início não encontrada no loop para cada"))?;
        let inicio = Self::parse_expressao(inicio_par)?;
        
        let fim_par = inner.next()
            .ok_or_else(|| anyhow!("Expressão de fim não encontrada no loop para cada"))?;
        let fim = Self::parse_expressao(fim_par)?;
        
        let corpo_par = inner.next()
            .ok_or_else(|| anyhow!("Corpo não encontrado no loop para cada"))?;
        let corpo = Self::parse_bloco(corpo_par)?;
        
        Ok(Declaracao::ParaCada {
            variavel,
            inicio,
            fim,
            corpo: Box::new(corpo),
        })
    }
    
    fn parse_declaracao_enquanto(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        
        let condicao_par = inner.next()
            .ok_or_else(|| anyhow!("Condição não encontrada no loop enquanto"))?;
        let condicao = Self::parse_expressao(condicao_par)?;
        
        let corpo_par = inner.next()
            .ok_or_else(|| anyhow!("Corpo não encontrado no loop enquanto"))?;
        let corpo = Self::parse_bloco(corpo_par)?;
        
        Ok(Declaracao::Enquanto {
            condicao,
            corpo: Box::new(corpo),
        })
    }
    
    fn parse_declaracao_repita(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        
        let corpo_par = inner.next()
            .ok_or_else(|| anyhow!("Corpo não encontrado no loop repita"))?;
        let corpo = Self::parse_bloco(corpo_par)?;
        
        let condicao_par = inner.next()
            .ok_or_else(|| anyhow!("Condição não encontrada no loop repita"))?;
        let condicao = Self::parse_expressao(condicao_par)?;
        
        Ok(Declaracao::Repita {
            corpo: Box::new(corpo),
            condicao,
        })
    }
    
    fn parse_declaracao_quando_der_erro(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        
        let bloco_try_par = inner.next()
            .ok_or_else(|| anyhow!("Bloco try não encontrado no tratamento de erro"))?;
        let bloco_try = Self::parse_bloco(bloco_try_par)?;
        
        let var_erro_par = inner.next()
            .ok_or_else(|| anyhow!("Variável de erro não encontrada no tratamento de erro"))?;
        let variavel_erro = var_erro_par.as_str().to_string();
        
        let bloco_catch_par = inner.next()
            .ok_or_else(|| anyhow!("Bloco catch não encontrado no tratamento de erro"))?;
        let bloco_catch = Self::parse_bloco(bloco_catch_par)?;
        
        Ok(Declaracao::QuandoDerErro {
            bloco_try: Box::new(bloco_try),
            variavel_erro,
            bloco_catch: Box::new(bloco_catch),
        })
    }
    
    fn parse_declaracao_modelo(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        let mut publico = false;
        
        // Verifica se há modificador de visibilidade
        let primeiro = inner.peek().ok_or_else(|| anyhow!("Declaração de modelo vazia"))?;
        if primeiro.as_rule() == Rule::modificador_visibilidade {
            publico = true;
            inner.next(); // Consome o modificador
        }
        
        let nome_par = inner.next()
            .ok_or_else(|| anyhow!("Nome do modelo não encontrado"))?;
        let nome = nome_par.as_str().to_string();
        
        let mut campos = Vec::new();
        
        // Processa todos os campos
        for campo in inner {
            if campo.as_rule() == Rule::campo {
                let mut campo_inner = campo.into_inner();
                let mut campo_publico = false;
                
                // Verifica se há modificador de visibilidade no campo
                let primeiro_campo = campo_inner.peek()
                    .ok_or_else(|| anyhow!("Campo vazio no modelo"))?;
                if primeiro_campo.as_rule() == Rule::modificador_visibilidade {
                    campo_publico = true;
                    campo_inner.next(); // Consome o modificador
                }
                
                let nome_campo_par = campo_inner.next()
                    .ok_or_else(|| anyhow!("Nome do campo não encontrado"))?;
                let nome_campo = nome_campo_par.as_str().to_string();
                
                let tipo_campo_par = campo_inner.next()
                    .ok_or_else(|| anyhow!("Tipo do campo não encontrado"))?;
                let tipo_campo = Self::parse_tipo(tipo_campo_par)?;
                
                campos.push((nome_campo, tipo_campo, campo_publico));
            }
        }
        
        Ok(Declaracao::Modelo { nome, campos, publico })
    }
    
    fn parse_declaracao_modulo(pair: Pair<Rule>) -> Result<Declaracao> {
        let mut inner = pair.into_inner();
        
        let nome_par = inner.next()
            .ok_or_else(|| anyhow!("Nome do módulo não encontrado"))?;
        let nome = nome_par.as_str().to_string();
        
        let bloco_par = inner.next()
            .ok_or_else(|| anyhow!("Bloco do módulo não encontrado"))?;
        
        let mut declaracoes = Vec::new();
        
        for declaracao in bloco_par.into_inner() {
            if declaracao.as_rule() == Rule::declaracao {
                let decl = Self::parse_declaracao(declaracao)?;
                declaracoes.push(decl);
            }
        }
        
        Ok(Declaracao::Modulo { nome, declaracoes })
    }
    
    fn parse_declaracao_importar(pair: Pair<Rule>) -> Result<Declaracao> {
        let caminho_par = pair.into_inner().next()
            .ok_or_else(|| anyhow!("Caminho do módulo não encontrado"))?;
        
        let mut caminho = Vec::new();
        
        for parte in caminho_par.into_inner() {
            caminho.push(parte.as_str().to_string());
        }
        
        Ok(Declaracao::Importar { caminho })
    }
    
    fn parse_declaracao_mostrar(pair: Pair<Rule>) -> Result<Declaracao> {
        let expressao_par = pair.into_inner().next()
            .ok_or_else(|| anyhow!("Expressão não encontrada na declaração mostrar"))?;
        
        let expressao = Self::parse_expressao(expressao_par)?;
        
        Ok(Declaracao::Mostrar(expressao))
    }
    
    fn parse_expressao(pair: Pair<Rule>) -> Result<Expressao> {
        match pair.as_rule() {
            Rule::expressao => {
                // Passar para o próximo nível (atribuição)
                let inner = pair.into_inner().next()
                    .ok_or_else(|| anyhow!("Expressão vazia"))?;
                Self::parse_expressao(inner)
            },
            Rule::atribuicao => Self::parse_atribuicao(pair),
            Rule::or_expr => Self::parse_or_expr(pair),
            Rule::and_expr => Self::parse_and_expr(pair),
            Rule::comparacao => Self::parse_comparacao(pair),
            Rule::soma => Self::parse_soma(pair),
            Rule::termo => Self::parse_termo(pair),
            Rule::fator => Self::parse_fator(pair),
            _ => Err(anyhow!("Tipo de expressão desconhecido: {:?}", pair.as_rule())),
        }
    }
    
    fn parse_atribuicao(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let esquerda_par = inner.next()
            .ok_or_else(|| anyhow!("Lado esquerdo da atribuição não encontrado"))?;
        let esquerda = Self::parse_expressao(esquerda_par)?;
        
        // Se houver mais partes, é uma atribuição
        if let Some(valor_par) = inner.next() {
            let valor = Self::parse_expressao(valor_par)?;
            
            // Se o lado esquerdo for um identificador, é uma atribuição direta
            if let Expressao::Identificador(nome) = esquerda {
                return Ok(Expressao::Atribuicao {
                    nome,
                    valor: Box::new(valor),
                });
            } else {
                return Err(anyhow!("Lado esquerdo de atribuição deve ser um identificador"));
            }
        }
        
        // Se não houver mais partes, é apenas a expressão
        Ok(esquerda)
    }
    
    fn parse_or_expr(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let primeiro_par = inner.next()
            .ok_or_else(|| anyhow!("Primeira parte da expressão 'ou' não encontrada"))?;
        let mut expr = Self::parse_expressao(primeiro_par)?;
        
        // Se houver mais partes, são operações 'ou'
        while let Some(proximo_par) = inner.next() {
            let direita = Self::parse_expressao(proximo_par)?;
            
            expr = Expressao::Operacao {
                operador: Operador::Ou,
                esquerda: Box::new(expr),
                direita: Box::new(direita),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_and_expr(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let primeiro_par = inner.next()
            .ok_or_else(|| anyhow!("Primeira parte da expressão 'e' não encontrada"))?;
        let mut expr = Self::parse_expressao(primeiro_par)?;
        
        // Se houver mais partes, são operações 'e'
        while let Some(proximo_par) = inner.next() {
            let direita = Self::parse_expressao(proximo_par)?;
            
            expr = Expressao::Operacao {
                operador: Operador::E,
                esquerda: Box::new(expr),
                direita: Box::new(direita),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparacao(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let esquerda_par = inner.next()
            .ok_or_else(|| anyhow!("Lado esquerdo da comparação não encontrado"))?;
        let esquerda = Self::parse_expressao(esquerda_par)?;
        
        // Se houver mais partes, é uma comparação
        if let Some(op_par) = inner.next() {
            let op = match op_par.as_str() {
                ">" => Operador::Maior,
                ">=" => Operador::MaiorIgual,
                "<" => Operador::Menor,
                "<=" => Operador::MenorIgual,
                "==" => Operador::Igual,
                "!=" => Operador::Diferente,
                "é igual a" => Operador::Igual,
                "em" => Operador::Contem,
                _ => return Err(anyhow!("Operador de comparação desconhecido: {}", op_par.as_str())),
            };
            
            let direita_par = inner.next()
                .ok_or_else(|| anyhow!("Lado direito da comparação não encontrado"))?;
            let direita = Self::parse_expressao(direita_par)?;
            
            return Ok(Expressao::Operacao {
                operador: op,
                esquerda: Box::new(esquerda),
                direita: Box::new(direita),
            });
        }
        
        // Se não houver mais partes, é apenas a expressão
        Ok(esquerda)
    }
    
    fn parse_soma(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let primeiro_par = inner.next()
            .ok_or_else(|| anyhow!("Primeiro termo da soma não encontrado"))?;
        let mut expr = Self::parse_expressao(primeiro_par)?;
        
        // Processar operações de soma/subtração em sequência
        while let Some(op_par) = inner.next() {
            let op = match op_par.as_str() {
                "+" => Operador::Soma,
                "-" => Operador::Subtracao,
                _ => return Err(anyhow!("Operador de soma/subtração desconhecido: {}", op_par.as_str())),
            };
            
            let termo_par = inner.next()
                .ok_or_else(|| anyhow!("Termo após operador não encontrado"))?;
            let termo = Self::parse_expressao(termo_par)?;
            
            expr = Expressao::Operacao {
                operador: op,
                esquerda: Box::new(expr),
                direita: Box::new(termo),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_termo(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let primeiro_par = inner.next()
            .ok_or_else(|| anyhow!("Primeiro fator do termo não encontrado"))?;
        let mut expr = Self::parse_expressao(primeiro_par)?;
        
        // Processar operações de multiplicação/divisão em sequência
        while let Some(op_par) = inner.next() {
            let op = match op_par.as_str() {
                "*" => Operador::Multiplicacao,
                "/" => Operador::Divisao,
                "resto" | "%" => Operador::Resto,
                _ => return Err(anyhow!("Operador de multiplicação/divisão desconhecido: {}", op_par.as_str())),
            };
            
            let fator_par = inner.next()
                .ok_or_else(|| anyhow!("Fator após operador não encontrado"))?;
            let fator = Self::parse_expressao(fator_par)?;
            
            expr = Expressao::Operacao {
                operador: op,
                esquerda: Box::new(expr),
                direita: Box::new(fator),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_fator(pair: Pair<Rule>) -> Result<Expressao> {
        let inner = pair.clone().into_inner().next()
            .ok_or_else(|| anyhow!("Fator vazio"))?;
        
        match inner.as_rule() {
            Rule::texto_literal => Self::parse_texto_literal(inner),
            Rule::numero_literal => {
                let valor = inner.as_str().parse::<f64>()
                    .map_err(|_| anyhow!("Não foi possível converter para número: {}", inner.as_str()))?;
                Ok(Expressao::NumeroLiteral(valor))
            },
            Rule::logico_literal => {
                let valor = match inner.as_str() {
                    "verdadeiro" => true,
                    "falso" => false,
                    _ => return Err(anyhow!("Valor lógico inválido: {}", inner.as_str())),
                };
                Ok(Expressao::LogicoLiteral(valor))
            },
            Rule::nada_literal => Ok(Expressao::Nada),
            Rule::lista_literal => Self::parse_lista_literal(inner),
            Rule::dicionario_literal => Self::parse_dicionario_literal(inner),
            Rule::chamada => Self::parse_chamada(inner),
            Rule::acesso_membro => Self::parse_acesso_membro(inner),
            Rule::identificador => Ok(Expressao::Identificador(inner.as_str().to_string())),
            Rule::expressao => Self::parse_expressao(inner),
            _ => {
                // Verificar se é uma negação
                if inner.as_str() == "!" || inner.as_str() == "não" {
                    let next_pair = pair.into_inner().next()
                        .ok_or_else(|| anyhow!("Expressão após operador de negação não encontrada"))?;
                    
                    if let Some(expr_pair) = next_pair.into_inner().next() {
                        let expressao = Self::parse_expressao(expr_pair)?;
                        
                        return Ok(Expressao::Negacao {
                            expressao: Box::new(expressao),
                        });
                    } else {
                        return Err(anyhow!("Expressão após operador de negação não encontrada"));
                    }
                }
                
                Err(anyhow!("Tipo de fator desconhecido: {:?}", inner.as_rule()))
            }
        }
    }
    
    fn parse_texto_literal(pair: Pair<Rule>) -> Result<Expressao> {
        let inner = pair.into_inner().next()
            .ok_or_else(|| anyhow!("Literal de texto vazio"))?;
        
        match inner.as_rule() {
            Rule::texto_simples => {
                // Remover as aspas do início e fim
                let texto = inner.as_str();
                let sem_aspas = &texto[1..texto.len()-1];
                Ok(Expressao::TextoLiteral(sem_aspas.to_string()))
            },
            Rule::texto_multilinha => {
                // Remover as três aspas do início e fim
                let texto = inner.as_str();
                let sem_aspas = &texto[3..texto.len()-3];
                Ok(Expressao::TextoLiteral(sem_aspas.to_string()))
            },
            Rule::texto_interpolado => {
                // Interpolação é tratada como concatenação de strings
                // Implementação simplificada: apenas retorna o texto literal
                let texto = inner.as_str();
                let sem_aspas = &texto[1..texto.len()-1];
                Ok(Expressao::TextoLiteral(sem_aspas.to_string()))
            },
            _ => Err(anyhow!("Tipo de texto literal desconhecido: {:?}", inner.as_rule())),
        }
    }
    
    fn parse_lista_literal(pair: Pair<Rule>) -> Result<Expressao> {
        let mut elementos = Vec::new();
        
        for elemento in pair.into_inner() {
            if elemento.as_rule() == Rule::expressao {
                let expr = Self::parse_expressao(elemento)?;
                elementos.push(expr);
            }
        }
        
        Ok(Expressao::ListaLiteral(elementos))
    }
    
    fn parse_dicionario_literal(pair: Pair<Rule>) -> Result<Expressao> {
        let mut pares = Vec::new();
        
        for par in pair.into_inner() {
            if par.as_rule() == Rule::par_chave_valor {
                let mut inner = par.into_inner();
                
                let chave_par = inner.next()
                    .ok_or_else(|| anyhow!("Chave não encontrada no par chave-valor"))?;
                let chave = Self::parse_expressao(chave_par)?;
                
                let valor_par = inner.next()
                    .ok_or_else(|| anyhow!("Valor não encontrado no par chave-valor"))?;
                let valor = Self::parse_expressao(valor_par)?;
                
                pares.push((chave, valor));
            }
        }
        
        Ok(Expressao::DicionarioLiteral(pares))
    }
    
    fn parse_chamada(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let nome_par = inner.next()
            .ok_or_else(|| anyhow!("Nome da função não encontrado na chamada"))?;
        let nome = nome_par.as_str().to_string();
        
        let mut argumentos = Vec::new();
        
        // Processa todos os argumentos
        for arg in inner {
            if arg.as_rule() == Rule::expressao {
                let expr = Self::parse_expressao(arg)?;
                argumentos.push(expr);
            }
        }
        
        Ok(Expressao::Chamada { nome, argumentos })
    }
    
    fn parse_acesso_membro(pair: Pair<Rule>) -> Result<Expressao> {
        let mut inner = pair.into_inner();
        
        let objeto_par = inner.next()
            .ok_or_else(|| anyhow!("Objeto não encontrado no acesso a membro"))?;
        let objeto = Expressao::Identificador(objeto_par.as_str().to_string());
        
        let membro_par = inner.next()
            .ok_or_else(|| anyhow!("Membro não encontrado no acesso a membro"))?;
        let membro = membro_par.as_str().to_string();
        
        Ok(Expressao::AcessoMembro {
            objeto: Box::new(objeto),
            membro,
        })
    }
}

// Esta função seria chamada pelo main.rs, escolhendo qual implementação usar
pub fn analisar_codigo(codigo: &str) -> Result<Programa> {
    // Utilizamos a implementação baseada em Pest
    analisar_com_pest(codigo)
    
    // Alternativa baseada em tokens (legado)
    // let tokens = crate::lexer::tokenizar(codigo)?;
    // analisar(tokens)
}

// Analisa uma expressão que pode incluir strings, identificadores, números e operações
fn analisar_expressao(i: &mut usize, tokens: &[Token]) -> Result<Expressao> {
    // Primeiro termo (lado esquerdo)
    let mut expr = analisar_termo(*i, tokens)?;
    *i += 1; // Avançar após o primeiro termo
    
    // Verifica se há operações (como +)
    while *i < tokens.len() {
        if tokens[*i] == Token::Mais {
            *i += 1; // Avançar após o operador
            
            if *i < tokens.len() {
                let termo_direito = analisar_termo(*i, tokens)?;
                *i += 1; // Avançar após o termo
                
                expr = Expressao::Operacao {
                    operador: Operador::Soma,
                    esquerda: Box::new(expr),
                    direita: Box::new(termo_direito),
                };
            } else {
                return Err(anyhow::anyhow!("Expressão incompleta após '+'"));
            }
        } else {
            // Não é uma operação, sair do loop
            break;
        }
    }
    
    Ok(expr)
}

// Analisa um termo simples (literal, identificador, etc.)
fn analisar_termo(i: usize, tokens: &[Token]) -> Result<Expressao> {
    if i >= tokens.len() {
        return Err(anyhow::anyhow!("Esperava um termo"));
    }
    
    match &tokens[i] {
        Token::TextoLiteral(texto) => {
            Ok(Expressao::TextoLiteral(texto.clone()))
        },
        Token::NumeroLiteral(valor) => {
            Ok(Expressao::NumeroLiteral(*valor))
        },
        Token::Identificador(nome) => {
            Ok(Expressao::Identificador(nome.clone()))
        },
        Token::Verdadeiro => {
            Ok(Expressao::LogicoLiteral(true))
        },
        Token::Falso => {
            Ok(Expressao::LogicoLiteral(false))
        },
        _ => {
            Err(anyhow::anyhow!("Termo inválido na expressão"))
        }
    }
}

