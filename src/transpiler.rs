use anyhow::{anyhow, Result};
use crate::ast::{Declaracao, Expressao, Operador, Programa, Tipo};

pub fn gerar_codigo_rust(programa: Programa) -> Result<String> {
    let mut saida = String::new();
    
    // Adiciona o preâmbulo padrão
    saida.push_str("use std::io::{self, Write};\n");
    saida.push_str("use std::collections::HashMap;\n\n");
    
    // Gera código para cada declaração
    for declaracao in programa.declaracoes {
        gerar_declaracao(&declaracao, &mut saida)?;
        saida.push('\n');
    }
    
    Ok(saida)
}
fn gerar_declaracao(decl: &Declaracao, saida: &mut String) -> Result<()> {
    match decl {
        Declaracao::Variavel { nome, tipo, valor, publico } => {
            if *publico {
                write!(saida, "pub ")?;
            }
            write!(saida, "let mut {} = ", nome)?;
            if let Some(v) = valor {
                gerar_expressao(v, saida)?;
            } else {
                match tipo {
                    Some(Tipo::Texto) => write!(saida, "String::new()")?,
                    Some(Tipo::Numero) => write!(saida, "0.0")?,
                    Some(Tipo::Logico) => write!(saida, "false")?,
                    Some(Tipo::Personalizado(t)) => write!(saida, "{}::default()", t)?,
                    None => write!(saida, "Default::default()")?,
                }
            }
            write!(saida, ";\n")?;
        },
        
        Declaracao::Funcao { nome, parametros, tipo_retorno, corpo, publico } => {
            if *publico {
                write!(saida, "pub ")?;
            }
            write!(saida, "fn {}(", nome)?;
            
            // Parâmetros
            for (i, (nome_param, tipo_param)) in parametros.iter().enumerate() {
                if i > 0 {
                    write!(saida, ", ")?;
                }
                write!(saida, "{}: ", nome_param)?;
                gerar_tipo(tipo_param, saida)?;
            }
            write!(saida, ")")?;
            
            // Tipo de retorno
            if let Some(tipo) = tipo_retorno {
                write!(saida, " -> ")?;
                gerar_tipo(tipo, saida)?;
            }
            
            write!(saida, " {{\n")?;
            gerar_declaracao(corpo, saida)?;
            write!(saida, "}}\n")?;
        },
        
        Declaracao::Modelo { nome, campos, publico } => {
            if *publico {
                write!(saida, "#[derive(Default)]\npub struct {} {{\n", nome)?;
            } else {
                write!(saida, "#[derive(Default)]\nstruct {} {{\n", nome)?;
            }
            
            for (nome_campo, tipo_campo, campo_publico) in campos {
                if *campo_publico {
                    write!(saida, "    pub ")?;
                } else {
                    write!(saida, "    ")?;
                }
                write!(saida, "{}: ", nome_campo)?;
                gerar_tipo(tipo_campo, saida)?;
                write!(saida, ",\n")?;
            }
            
            write!(saida, "}}\n")?;
        },
        
        Declaracao::Modulo { nome, declaracoes } => {
            write!(saida, "mod {} {{\n", nome)?;
            
            for decl in declaracoes {
                gerar_declaracao(decl, saida)?;
            }
            
            write!(saida, "}}\n")?;
        },
        
        Declaracao::Importar { caminho } => {
            write!(saida, "use ")?;
            for (i, parte) in caminho.iter().enumerate() {
                if i > 0 {
                    write!(saida, "::")?;
                }
                write!(saida, "{}", parte)?;
            }
            write!(saida, ";\n")?;
        },
        
        Declaracao::Enquanto { condicao, corpo } => {
            write!(saida, "while ")?;
            gerar_expressao(condicao, saida)?;
            write!(saida, " {{\n")?;
            gerar_declaracao(corpo, saida)?;
            write!(saida, "}}\n")?;
        },
        
        Declaracao::Repita { corpo, condicao } => {
            write!(saida, "loop {{\n")?;
            gerar_declaracao(corpo, saida)?;
            write!(saida, "    if ")?;
            gerar_expressao(condicao, saida)?;
            write!(saida, " {{ break; }}\n}}\n")?;
        },
        
        Declaracao::Pare => {
            write!(saida, "break;\n")?;
        },
        
        Declaracao::Continue => {
            write!(saida, "continue;\n")?;
        },
        
        Declaracao::Se { condicao, bloco_se, bloco_senao } => {
            write!(saida, "if ")?;
            gerar_expressao(condicao, saida)?;
            write!(saida, " {{\n")?;
            gerar_declaracao(bloco_se, saida)?;
            
            if let Some(senao) = bloco_senao {
                write!(saida, "}} else {{\n")?;
                gerar_declaracao(senao, saida)?;
            }
            
            write!(saida, "}}\n")?;
        },
        
        Declaracao::ParaCada { variavel, inicio, fim, corpo } => {
            write!(saida, "for {} in ", variavel)?;
            gerar_expressao(inicio, saida)?;
            write!(saida, "..=")?;
            gerar_expressao(fim, saida)?;
            write!(saida, " {{\n")?;
            gerar_declaracao(corpo, saida)?;
            write!(saida, "}}\n")?;
        },
        
        Declaracao::QuandoDerErro { bloco_try, variavel_erro, bloco_catch } => {
            write!(saida, "match (|| -> Result<(), Box<dyn std::error::Error>> {{\n")?;
            gerar_declaracao(bloco_try, saida)?;
            write!(saida, "    Ok(())\n}}() {{\n")?;
            write!(saida, "    Ok(_) => {{}},\n")?;
            write!(saida, "    Err({}) => {{\n", variavel_erro)?;
            gerar_declaracao(bloco_catch, saida)?;
            write!(saida, "    }}\n}}\n")?;
        },
        
        Declaracao::Retorno(expr) => {
            write!(saida, "return")?;
            
            if let Some(e) = expr {
                write!(saida, " ")?;
                gerar_expressao(e, saida)?;
            }
            
            write!(saida, ";\n")?;
        },
        
        Declaracao::Mostrar(expr) => {
            write!(saida, "println!(\"{:?}\", ")?;
            gerar_expressao(expr, saida)?;
            write!(saida, ");\n")?;
        },
        
        Declaracao::Bloco(declaracoes) => {
            for decl in declaracoes {
                gerar_declaracao(decl, saida)?;
            }
        },
        
        Declaracao::Expressao(expr) => {
            gerar_expressao(expr, saida)?;
            write!(saida, ";\n")?;
        },
        _ => return Err(anyhow!("Tipo de declaração não suportado: {:?}", decl)),
    }
    
    Ok(())
}
fn gerar_expressao(expr: &Expressao, saida: &mut String) -> Result<()> {
    match expr {
        Expressao::TextoLiteral(texto) => write!(saida, "\"{}\"", escapar_string_para_rust(texto))?,
        Expressao::NumeroLiteral(num) => write!(saida, "{}", num)?,
        Expressao::LogicoLiteral(bool) => write!(saida, "{}", bool)?,
        Expressao::Nada => write!(saida, "None")?,
        Expressao::ListaLiteral(elementos) => {
            write!(saida, "vec![")?;
            for (i, elem) in elementos.iter().enumerate() {
                if i > 0 { write!(saida, ", ")? }
                gerar_expressao(elem, saida)?;
            }
            write!(saida, "]")?
        },
        Expressao::DicionarioLiteral(pares) => {
            write!(saida, "HashMap::from([")?;
            for (i, (chave, valor)) in pares.iter().enumerate() {
                if i > 0 { write!(saida, ", ")? }
                write!(saida, "(")?;
                gerar_expressao(chave, saida)?;
                write!(saida, ", ")?;
                gerar_expressao(valor, saida)?;
                write!(saida, ")")?;
            }
            write!(saida, "])")?
        },
        Expressao::Identificador(nome) => write!(saida, "{}", nome)?,
        Expressao::Chamada { nome, argumentos } => {
            write!(saida, "{}(", nome)?;
            for (i, arg) in argumentos.iter().enumerate() {
                if i > 0 { write!(saida, ", ")? }
                gerar_expressao(arg, saida)?;
            }
            write!(saida, ")")?;
        },
        Expressao::Negacao { expressao } => {
            write!(saida, "!")?;
            gerar_expressao(expressao, saida)?;
        },
        Expressao::Operacao { operador, esquerda, direita } => {
            write!(saida, "(")?;
            gerar_expressao(esquerda, saida)?;
            
            match operador {
                Operador::Soma => write!(saida, " + ")?,
                Operador::Subtracao => write!(saida, " - ")?,
                Operador::Multiplicacao => write!(saida, " * ")?,
                Operador::Divisao => write!(saida, " / ")?,
                Operador::Resto => write!(saida, " % ")?,
                Operador::Igual => write!(saida, " == ")?,
                Operador::Diferente => write!(saida, " != ")?,
                Operador::Maior => write!(saida, " > ")?,
                Operador::Menor => write!(saida, " < ")?,
                Operador::MaiorIgual => write!(saida, " >= ")?,
                Operador::MenorIgual => write!(saida, " <= ")?,
                Operador::Contem => write!(saida, ".contains(&")?,
                Operador::E => write!(saida, " && ")?,
                Operador::Ou => write!(saida, " || ")?,
            }
            
            gerar_expressao(direita, saida)?;
            if *operador == Operador::Contem {
                write!(saida, ")")?;
            }
            write!(saida, ")")?;
        },
        Expressao::Atribuicao { nome, valor } => {
            write!(saida, "{} = ", nome)?;
            gerar_expressao(valor, saida)?;
        },
        Expressao::AcessoMembro { objeto, membro } => {
            gerar_expressao(objeto, saida)?;
            write!(saida, ".{}", membro)?;
        },
        _ => return Err(anyhow!("Tipo de expressão não suportado: {:?}", expr)),
    }
    
    Ok(())
}

fn gerar_tipo(tipo: &Tipo, saida: &mut String) -> Result<()> {
    match tipo {
        Tipo::Texto => write!(saida, "String")?,
        Tipo::Numero => write!(saida, "f64")?,
        Tipo::Logico => write!(saida, "bool")?,
        Tipo::Personalizado(nome) => write!(saida, "{}", nome)?,
    }
    Ok(())
}

fn escapar_string_para_rust(texto: &str) -> String {
    texto
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
}
