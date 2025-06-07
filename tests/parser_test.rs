use anyhow::Result;
use pbrlang::{
    ast::{Declaracao, Expressao, Tipo},
    parser::analisar_codigo,
};
use std::fs;
use std::path::Path;

#[test]
fn test_parser_example_file() -> Result<()> {
    let example_path = Path::new("examples/completo.pbr");
    assert!(example_path.exists(), "O arquivo de exemplo não existe");
    
    let codigo = fs::read_to_string(example_path)?;
    let programa = analisar_codigo(&codigo)?;
    
    // Verificar estrutura básica do programa
    assert!(!programa.declaracoes.is_empty(), "O programa não deve estar vazio");
    
    // Encontrar o módulo principal
    let modulo_principal = programa.declaracoes.iter().find(|d| {
        matches!(d, Declaracao::Modulo { nome, .. } if nome == "principal")
    }).expect("Módulo principal não encontrado");
    
    if let Declaracao::Modulo { declaracoes, .. } = modulo_principal {
        // Verificar modelo Pessoa
        let modelo_pessoa = declaracoes.iter().find(|d| {
            matches!(d, Declaracao::Modelo { nome, .. } if nome == "Pessoa")
        }).expect("Modelo Pessoa não encontrado");
        
        if let Declaracao::Modelo { campos, publico, .. } = modelo_pessoa {
            assert!(*publico, "Modelo Pessoa deve ser público");
            assert_eq!(campos.len(), 2, "Modelo Pessoa deve ter 2 campos");
        }
        
        // Verificar função principal
        let funcao_principal = declaracoes.iter().find(|d| {
            matches!(d, Declaracao::Funcao { nome, .. } if nome == "principal")
        }).expect("Função principal não encontrada");
        
        if let Declaracao::Funcao { publico, .. } = funcao_principal {
            assert!(*publico, "Função principal deve ser pública");
        }
        
        // Verificar função calcular_soma
        let funcao_soma = declaracoes.iter().find(|d| {
            matches!(d, Declaracao::Funcao { nome, .. } if nome == "calcular_soma")
        }).expect("Função calcular_soma não encontrada");
        
        if let Declaracao::Funcao { parametros, tipo_retorno, publico, .. } = funcao_soma {
            assert!(*publico, "Função calcular_soma deve ser pública");
            assert_eq!(parametros.len(), 2, "calcular_soma deve ter 2 parâmetros");
            assert!(matches!(tipo_retorno, Some(Tipo::Numero)), 
                   "calcular_soma deve retornar número");
        }
    }
    
    Ok(())
}

#[test]
fn test_parser_string_interpolation() -> Result<()> {
    let codigo = r#"pense msg = "Olá, ${nome}!";"#;
    let programa = analisar_codigo(codigo)?;
    assert!(!programa.declaracoes.is_empty());
    
    // Verificar se é uma declaração de variável com interpolação
    if let Some(Declaracao::Variavel { nome, valor, .. }) = programa.declaracoes.first() {
        assert_eq!(nome, "msg");
        assert!(matches!(valor, Some(Expressao::TextoLiteral(_))));
    } else {
        panic!("Esperava uma declaração de variável");
    }
    
    Ok(())
}

#[test]
fn test_parser_estruturas_controle() -> Result<()> {
    let codigo = r#"
        se x > 0 {
            mostre "Positivo";
        } senão {
            mostre "Não positivo";
        }
        
        para cada i de 1 até 10 {
            se i == 5 { continue; }
            se i == 8 { pare; }
        }
        
        enquanto x < 10 {
            x = x + 1;
        }
        
        repita {
            x = x - 1;
        } até x <= 0;
    "#;
    let programa = analisar_codigo(codigo)?;
    assert!(!programa.declaracoes.is_empty());
    
    // Verificar se temos todas as estruturas de controle
    let tem_se = programa.declaracoes.iter().any(|d| matches!(d, Declaracao::Se { .. }));
    let tem_para = programa.declaracoes.iter().any(|d| matches!(d, Declaracao::ParaCada { .. }));
    let tem_enquanto = programa.declaracoes.iter().any(|d| matches!(d, Declaracao::Enquanto { .. }));
    let tem_repita = programa.declaracoes.iter().any(|d| matches!(d, Declaracao::Repita { .. }));
    
    assert!(tem_se, "Deve conter estrutura 'se'");
    assert!(tem_para, "Deve conter estrutura 'para cada'");
    assert!(tem_enquanto, "Deve conter estrutura 'enquanto'");
    assert!(tem_repita, "Deve conter estrutura 'repita'");
    
    Ok(())
}

#[test]
fn test_parser_operadores() -> Result<()> {
    let codigo = r#"
        pense a = 5 + 3 * 2;
        pense b = 10 resto 3;
        pense c = não verdadeiro;
        pense d = x em lista;
        pense e = a > b e b < c;
    "#;
    let programa = analisar_codigo(codigo)?;
    assert!(!programa.declaracoes.is_empty());
    
    // Verificar se temos declarações com diferentes operadores
    let tem_aritmetico = programa.declaracoes.iter().any(|d| {
        if let Declaracao::Variavel { valor: Some(Expressao::Operacao { operador, .. }), .. } = d {
            matches!(operador, Operador::Soma | Operador::Multiplicacao)
        } else {
            false
        }
    });
    
    let tem_resto = programa.declaracoes.iter().any(|d| {
        if let Declaracao::Variavel { valor: Some(Expressao::Operacao { operador, .. }), .. } = d {
            matches!(operador, Operador::Resto)
        } else {
            false
        }
    });
    
    assert!(tem_aritmetico, "Deve conter operadores aritméticos");
    assert!(tem_resto, "Deve conter operador 'resto'");
    
    Ok(())
}
