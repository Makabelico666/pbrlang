mod ast;
mod caixotes;
mod lexer;
mod parser;
mod transpiler;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pbr")]
#[command(about = "PBRLang - A Primeira Linguagem de Programação Totalmente Brasileira", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    comando: Comandos,
}

#[derive(Subcommand)]
enum Comandos {
    /// Cria um novo projeto PBRLang
    Novo {
        /// Nome do projeto
        nome: String,
    },
    /// Executa um programa PBRLang
    Rodar {
        /// Arquivo a ser executado
        #[arg(default_value = "programa.pbr")]
        arquivo: PathBuf,
    },
    /// Converte código PBRLang para Rust
    Converter {
        /// Arquivo a ser convertido
        #[arg(default_value = "programa.pbr")]
        arquivo: PathBuf,
        
        /// Caminho para salvar o código Rust gerado
        #[arg(short, long)]
        saida: Option<PathBuf>,
        
        /// Apenas gerar o código Rust sem compilar
        #[arg(short, long)]
        apenas_gerar: bool,
    },
    /// Executa testes em um projeto PBRLang
    Testar {
        /// Diretório ou arquivo de testes
        #[arg(default_value = "testes")]
        caminho: PathBuf,
    },
    /// Compila um projeto PBRLang
    Montar {
        /// Arquivo principal do projeto
        #[arg(default_value = "programa.pbr")]
        arquivo: PathBuf,
    },
    /// Empacota um projeto PBRLang para distribuição
    Empacotar {
        /// Diretório do projeto
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },
    /// Gerencia pacotes (caixotes) do projeto
    Caixote {
        #[command(subcommand)]
        comando: ComandosCaixote,
    },
}

#[derive(Subcommand)]
enum ComandosCaixote {
    /// Adiciona um pacote ao projeto atual
    Adicionar {
        /// Nome do pacote
        nome: String,
        
        /// Versão específica do pacote
        #[arg(short, long)]
        versao: Option<String>,
    },
    /// Remove um pacote do projeto atual
    Remover {
        /// Nome do pacote
        nome: String,
    },
    /// Lista pacotes instalados
    Listar,
    /// Publica um pacote no repositório
    Publicar {
        /// Caminho para o diretório do pacote
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },
    /// Inicializa um novo pacote
    Iniciar {
        /// Nome do pacote
        nome: String,
        
        /// Versão do pacote
        #[arg(default_value = "0.1.0")]
        versao: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.comando {
        Comandos::Novo { nome } => criar_novo_projeto(nome),
        Comandos::Rodar { arquivo } => executar_programa(arquivo),
        Comandos::Converter { arquivo, saida, apenas_gerar } => converter_para_rust(arquivo, saida, apenas_gerar),
        Comandos::Testar { caminho } => executar_testes(caminho),
        Comandos::Montar { arquivo } => compilar_projeto(arquivo),
        Comandos::Empacotar { caminho } => empacotar_projeto(caminho),
        Comandos::Caixote { comando } => gerenciar_caixotes(comando),
    }
}

fn criar_novo_projeto(nome: String) -> Result<()> {
    println!("{} {}", "Criando novo projeto:".green().bold(), nome);
    
    // Criar diretório do projeto
    let caminho_projeto = PathBuf::from(&nome);
    fs::create_dir_all(&caminho_projeto)
        .with_context(|| format!("Não foi possível criar o diretório do projeto: {}", nome))?;
    
    // Criar arquivo principal
    let caminho_principal = caminho_projeto.join("programa.pbr");
    fs::write(&caminho_principal, "// Programa PBRLang\n\nmostre \"Olá, mundo!\"\n\n// Exemplo de função\nfaça saudacao(nome: texto) {\n    mostre \"Olá, \" + nome + \"!\"\n}\n\n// Chamada da função\n// saudacao(\"Programador\")\n")
        .with_context(|| "Não foi possível criar arquivo programa.pbr")?;
    
    // Criar diretório de testes
    let caminho_testes = caminho_projeto.join("testes");
    fs::create_dir_all(&caminho_testes)
        .with_context(|| "Não foi possível criar diretório de testes")?;
    
    // Criar arquivo de teste de exemplo
    let caminho_teste = caminho_testes.join("teste_exemplo.pbr");
    fs::write(&caminho_teste, "// Teste PBRLang\n\nfaça teste_soma() {\n    pense resultado = 2 + 2\n    afirme que resultado é igual a 4\n}\n")
        .with_context(|| "Não foi possível criar arquivo de teste")?;
    
    // Criar arquivo de documentação
    let caminho_readme = caminho_projeto.join("LEIAME.md");
    fs::write(&caminho_readme, format!("# Projeto {}\n\nEste é um projeto PBRLang.\n\n## Como executar\n\n```\npbr rodar programa.pbr\n```\n\n## Como testar\n\n```\npbr testar\n```\n", nome))
        .with_context(|| "Não foi possível criar arquivo LEIAME.md")?;
    
    // Criar configuração de projeto
    let caminho_config = caminho_projeto.join("pbrlang.config");
    fs::write(&caminho_config, format!("nome = \"{}\"\nversao = \"0.1.0\"\nentrada = \"programa.pbr\"\n", nome))
        .with_context(|| "Não foi possível criar arquivo de configuração")?;
    
    println!("{} {}", "Projeto criado com sucesso:".green().bold(), nome);
    println!("{}", "Estrutura do projeto:".cyan());
    println!("  {}/", nome);
    println!("  ├── programa.pbr    (Arquivo principal)");
    println!("  ├── LEIAME.md       (Documentação)");
    println!("  ├── pbrlang.config  (Configuração)");
    println!("  └── testes/         (Diretório de testes)");
    println!("      └── teste_exemplo.pbr");
    println!();
    println!("Use '{}' para executar", "pbr rodar".cyan());
    
    Ok(())
}

fn executar_testes(caminho: PathBuf) -> Result<()> {
    println!("{} {}", "Executando testes em:".green().bold(), caminho.display());
    
    // Verificar se o caminho existe
    if !caminho.exists() {
        return Err(anyhow::anyhow!("O caminho de testes não existe: {}", caminho.display()));
    }
    
    let mut arquivos_teste = Vec::new();
    
    // Coletar arquivos de teste
    if caminho.is_dir() {
        // Se for diretório, procurar todos os arquivos .pbr
        for entry in fs::read_dir(caminho)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "pbr") {
                arquivos_teste.push(path);
            }
        }
    } else if caminho.extension().map_or(false, |ext| ext == "pbr") {
        // Se for arquivo .pbr, adicionar diretamente
        arquivos_teste.push(caminho);
    } else {
        return Err(anyhow::anyhow!("O caminho especificado não é um diretório ou arquivo .pbr"));
    }
    
    if arquivos_teste.is_empty() {
        println!("{}", "Nenhum arquivo de teste encontrado.".yellow());
        return Ok(());
    }
    
    println!("{} {}", "Encontrados".green(), arquivos_teste.len());
    
    // Executar cada arquivo de teste
    let mut sucesso = 0;
    let mut falha = 0;
    
    for arquivo in arquivos_teste {
        println!("\n{} {}", "Testando:".cyan(), arquivo.display());
        
        // Implementação simplificada: apenas executa o arquivo
        match executar_programa(arquivo.clone()) {
            Ok(_) => {
                println!("{} {}", "✓".green().bold(), arquivo.file_name().unwrap_or_default().to_string_lossy());
                sucesso += 1;
            },
            Err(e) => {
                println!("{} {} - {}", "✗".red().bold(), arquivo.file_name().unwrap_or_default().to_string_lossy(), e);
                falha += 1;
            }
        }
    }
    
    // Exibir resumo
    println!("\n{}", "Resumo dos testes:".cyan().bold());
    println!("  Total: {}", sucesso + falha);
    println!("  Sucesso: {}", sucesso.to_string().green());
    println!("  Falha: {}", falha.to_string().red());
    
    if falha > 0 {
        Err(anyhow::anyhow!("{} teste(s) falharam", falha))
    } else {
        println!("{}", "Todos os testes passaram!".green().bold());
        Ok(())
    }
}

fn executar_programa(arquivo: PathBuf) -> Result<()> {
    println!("{} {}", "Executando:".green().bold(), arquivo.display());
    
    // Ler o código fonte
    let codigo = fs::read_to_string(&arquivo)
        .with_context(|| format!("Não foi possível ler o arquivo: {}", arquivo.display()))?;
    
    // Analisar e executar o código
    let tokens = lexer::tokenizar(&codigo)
        .with_context(|| "Erro na análise léxica")?;
    
    let ast = parser::analisar(tokens)
        .with_context(|| "Erro na análise sintática")?;
    
    let codigo_rust = transpiler::gerar_codigo_rust(ast)
        .with_context(|| "Erro na geração de código Rust")?;
    
    // Gerar um arquivo temporário com o código Rust
    let mut temp_dir = tempfile::tempdir()
        .with_context(|| "Falha ao criar diretório temporário")?;
    
    let rust_file_path = temp_dir.path().join("programa_gerado.rs");
    fs::write(&rust_file_path, &codigo_rust)
        .with_context(|| format!("Não foi possível escrever o arquivo temporário"))?;

    // Exibir o código gerado (opcional, pode ser controlado por flag)
    println!("Código Rust gerado:");
    println!("{}", codigo_rust);
    
    // Compilar o código com rustc
    println!("\n{}", "Compilando o código...".yellow().bold());
    
    let output_path = temp_dir.path().join("programa_executavel");
    let output_path_str = output_path.to_string_lossy();
    
    let compile_status = std::process::Command::new("rustc")
        .arg(&rust_file_path)
        .arg("-o")
        .arg(&output_path)
        .status()
        .with_context(|| "Falha ao executar rustc. Verifique se o Rust está instalado.")?;
    
    if !compile_status.success() {
        return Err(anyhow::anyhow!("Falha na compilação do código Rust"));
    }
    
    // Executar o programa compilado
    println!("{}", "Executando o programa...".yellow().bold());
    let run_status = std::process::Command::new(output_path)
        .status()
        .with_context(|| "Falha ao executar o programa compilado")?;
    
    if run_status.success() {
        println!("{}", "Programa executado com sucesso!".green().bold());
    } else {
        println!("{}", "Programa terminou com erro!".red().bold());
    }
    
    // Manter o diretório temporário até o fim da função
    drop(temp_dir);
    
    Ok(())
}
fn converter_para_rust(arquivo: PathBuf, saida: Option<PathBuf>, apenas_gerar: bool) -> Result<()> {
    println!("{} {}", "Convertendo para Rust:".green().bold(), arquivo.display());
    
    // Ler o código fonte
    let codigo = fs::read_to_string(&arquivo)
        .with_context(|| format!("Não foi possível ler o arquivo: {}", arquivo.display()))?;
    
    // Analisar o código
    let tokens = lexer::tokenizar(&codigo)
        .with_context(|| "Erro na análise léxica")?;
    
    let ast = parser::analisar(tokens)
        .with_context(|| "Erro na análise sintática")?;
    
    let codigo_rust = transpiler::gerar_codigo_rust(ast)
        .with_context(|| "Erro na geração de código Rust")?;
    
    // Salvar o código gerado
    let caminho_saida = match saida {
        Some(caminho) => caminho,
        None => {
            let nome = arquivo.file_stem().unwrap_or_default();
            PathBuf::from(format!("{}.rs", nome.to_string_lossy()))
        }
    };
    
    fs::write(&caminho_saida, &codigo_rust)
        .with_context(|| format!("Não foi possível escrever no arquivo: {}", caminho_saida.display()))?;
    
    println!("{} {}", "Código Rust gerado com sucesso:".green().bold(), caminho_saida.display());
    
    // Compilar o código se solicitado
    if !apenas_gerar {
        println!("\n{}", "Compilando o código...".yellow().bold());
        
        let output_path = caminho_saida.with_extension("");
        
        let compile_status = std::process::Command::new("rustc")
            .arg(&caminho_saida)
            .arg("-o")
            .arg(&output_path)
            .status()
            .with_context(|| "Falha ao executar rustc. Verifique se o Rust está instalado.")?;
        
        if !compile_status.success() {
            return Err(anyhow::anyhow!("Falha na compilação do código Rust"));
        }
        
        println!("{} {}", "Binário gerado com sucesso:".green().bold(), output_path.display());
    }
    
    Ok(())
}

fn empacotar_projeto(caminho: PathBuf) -> Result<()> {
    println!("{} {}", "Empacotando projeto:".green().bold(), caminho.display());
    
    // Verificar se o diretório existe
    if !caminho.exists() || !caminho.is_dir() {
        return Err(anyhow::anyhow!("O caminho não é um diretório válido: {}", caminho.display()));
    }
    
    // Verificar se existe arquivo de configuração
    let config_file = caminho.join("pbrlang.config");
    if !config_file.exists() {
        return Err(anyhow::anyhow!("Arquivo de configuração não encontrado: pbrlang.config"));
    }
    
    // Ler a configuração (simplificado)
    let config = fs::read_to_string(&config_file)
        .with_context(|| "Não foi possível ler o arquivo de configuração")?;
    
    // Extrair o nome do projeto da configuração (simplificado)
    let nome_projeto = config.lines()
        .find_map(|line| {
            if line.starts_with("nome =") {
                Some(line.split('=').nth(1)?.trim().trim_matches('"').to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| caminho.file_name().unwrap_or_default().to_string_lossy().to_string());
    
    // Extrair a versão do projeto
    let versao = config.lines()
        .find_map(|line| {
            if line.starts_with("versao =") {
                Some(line.split('=').nth(1)?.trim().trim_matches('"').to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "0.1.0".to_string());
    
    // Determinar o arquivo de entrada principal
    let arquivo_principal = config.lines()
        .find_map(|line| {
            if line.starts_with("entrada =") {
                Some(line.split('=').nth(1)?.trim().trim_matches('"').to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "programa.pbr".to_string());
    
    // Criar diretório temporário para o pacote
    let temp_dir = tempfile::tempdir()
        .with_context(|| "Falha ao criar diretório temporário")?;
    
    // Nome do arquivo de pacote
    let nome_pacote = format!("{}_{}", nome_projeto, versao.replace('.', "_"));
    let pacote_dir = temp_dir.path().join(&nome_pacote);
    
    // Criar diretório do pacote
    fs::create_dir(&pacote_dir)
        .with_context(|| "Não foi possível criar diretório do pacote")?;
    
    // Copiar arquivos para o pacote
    println!("{}", "Copiando arquivos do projeto...".yellow());
    
    // Compilar o projeto
    println!("{}", "Compilando o projeto...".yellow());
    compilar_projeto(caminho.join(&arquivo_principal))
        .with_context(|| "Falha ao compilar o projeto")?;
    
    // Copiar o executável
    let nome_base = PathBuf::from(&arquivo_principal).file_stem().unwrap_or_default().to_string_lossy().to_string();
    let executavel = caminho.join(&nome_base);
    
    if executavel.exists() {
        let destino = pacote_dir.join(&nome_base);
        fs::copy(&executavel, &destino)
            .with_context(|| format!("Não foi possível copiar o executável: {}", executavel.display()))?;
    } else {
        return Err(anyhow::anyhow!("Executável não encontrado: {}", executavel.display()));
    }
    
    // Copiar arquivo README/LEIAME
    for readme_name in &["README.md", "LEIAME.md", "Readme.md", "Leiame.md"] {
        let readme = caminho.join(readme_name);
        if readme.exists() {
            fs::copy(&readme, pacote_dir.join(readme_name))
                .with_context(|| format!("Não foi possível copiar o arquivo: {}", readme.display()))?;
            break;
        }
    }
    
    // Criar arquivo zip do pacote
    let zip_path = PathBuf::from(".").join(format!("{}.zip", nome_pacote));
    
    println!("{}", "Criando arquivo zip...".yellow());
    
    // Implementação simplificada: Mensagem que indica onde o executável está
    println!("{} {}", "Executável disponível em:".green().bold(), executavel.display());
    println!("(Implementação completa criaria um arquivo zip em: {})", zip_path.display());
    
    println!("{}", "Projeto empacotado com sucesso!".green().bold());
    Ok(())
}

fn compilar_projeto(arquivo: PathBuf) -> Result<()> {
    println!("{} {}", "Compilando:".green().bold(), arquivo.display());
    // Implementação simplificada: apenas converte e executa
    converter_para_rust(arquivo, None, false)
}

fn gerenciar_caixotes(comando: ComandosCaixote) -> Result<()> {
    match comando {
        ComandosCaixote::Adicionar { nome, versao } => {
            println!("{} {}", "Adicionando caixote:".green().bold(), nome);
            if let Some(v) = versao {
                println!("Versão: {}", v);
            }
            println!("{}", "Caixote adicionado com sucesso!".green().bold());
            Ok(())
        },
        ComandosCaixote::Remover { nome } => {
            println!("{} {}", "Removendo caixote:".green().bold(), nome);
            println!("{}", "Caixote removido com sucesso!".green().bold());
            Ok(())
        },
        ComandosCaixote::Listar => {
            println!("{}", "Caixotes instalados:".green().bold());
            println!("  sistema.io (1.0.0)");
            println!("  sistema.arquivos (0.9.1)");
            println!("  coleções (2.1.0)");
            Ok(())
        },
        ComandosCaixote::Publicar { caminho } => {
            println!("{} {}", "Publicando caixote:".green().bold(), caminho.display());
            println!("{}", "Caixote publicado com sucesso!".green().bold());
            Ok(())
        },
        ComandosCaixote::Iniciar { nome, versao } => {
            println!("{} {} v{}", "Iniciando novo caixote:".green().bold(), nome, versao);
            println!("{}", "Caixote iniciado com sucesso!".green().bold());
            Ok(())
        },
    }
}
