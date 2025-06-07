use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Representa um arquivo de manifesto de pacote (caixote.pbr)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifesto {
    /// Nome do pacote
    pub nome: String,
    
    /// Versão do pacote
    pub versao: String,
    
    /// Autor(es) do pacote
    #[serde(default)]
    pub autores: Vec<String>,
    
    /// Descrição do pacote
    #[serde(default)]
    pub descricao: String,
    
    /// Arquivo principal
    #[serde(default = "default_arquivo_principal")]
    pub principal: String,
    
    /// Dependências do pacote
    #[serde(default)]
    pub dependencias: HashMap<String, String>,
    
    /// Palavras-chave para busca
    #[serde(default)]
    pub palavras_chave: Vec<String>,
    
    /// Licença do pacote
    #[serde(default)]
    pub licenca: String,
    
    /// Repositório do pacote
    #[serde(default)]
    pub repositorio: String,
}

fn default_arquivo_principal() -> String {
    "src/principal.pbr".to_string()
}

impl Manifesto {
    /// Cria um novo manifesto com valores padrão
    pub fn novo(nome: &str, versao: &str) -> Self {
        Manifesto {
            nome: nome.to_string(),
            versao: versao.to_string(),
            autores: Vec::new(),
            descricao: String::new(),
            principal: default_arquivo_principal(),
            dependencias: HashMap::new(),
            palavras_chave: Vec::new(),
            licenca: "MIT".to_string(),
            repositorio: String::new(),
        }
    }
    
    /// Carrega um manifesto a partir de um arquivo
    pub fn carregar<P: AsRef<Path>>(caminho: P) -> Result<Self> {
        let conteudo = fs::read_to_string(caminho)
            .context("Não foi possível ler o arquivo de manifesto")?;
        
        Self::parse(&conteudo)
    }
    
    /// Salva o manifesto em um arquivo
    pub fn salvar<P: AsRef<Path>>(&self, caminho: P) -> Result<()> {
        let conteudo = self.to_string()?;
        fs::write(caminho, conteudo)
            .context("Não foi possível salvar o arquivo de manifesto")?;
        
        Ok(())
    }
    
    /// Converte o manifesto para uma string formatada
    pub fn to_string(&self) -> Result<String> {
        let mut linhas = Vec::new();
        
        linhas.push(format!("nome = \"{}\"", self.nome));
        linhas.push(format!("versao = \"{}\"", self.versao));
        
        if !self.autores.is_empty() {
            let autores = self.autores.iter()
                .map(|a| format!("\"{}\"", a))
                .collect::<Vec<_>>()
                .join(", ");
            linhas.push(format!("autores = [{}]", autores));
        }
        
        if !self.descricao.is_empty() {
            linhas.push(format!("descricao = \"{}\"", self.descricao));
        }
        
        linhas.push(format!("principal = \"{}\"", self.principal));
        
        if !self.dependencias.is_empty() {
            linhas.push("dependencias = {".to_string());
            for (nome, versao) in &self.dependencias {
                linhas.push(format!("    \"{}\" = \"{}\"", nome, versao));
            }
            linhas.push("}".to_string());
        }
        
        if !self.palavras_chave.is_empty() {
            let palavras = self.palavras_chave.iter()
                .map(|p| format!("\"{}\"", p))
                .collect::<Vec<_>>()
                .join(", ");
            linhas.push(format!("palavras_chave = [{}]", palavras));
        }
        
        if !self.licenca.is_empty() {
            linhas.push(format!("licenca = \"{}\"", self.licenca));
        }
        
        if !self.repositorio.is_empty() {
            linhas.push(format!("repositorio = \"{}\"", self.repositorio));
        }
        
        Ok(linhas.join("\n"))
    }
    
    /// Analisa uma string para criar um manifesto
    pub fn parse(conteudo: &str) -> Result<Self> {
        // Implementação simplificada para analisar um formato chave-valor
        let mut manifesto = Manifesto::novo("temp", "0.1.0");
        let mut estado_atual = Estado::Normal;
        let mut chave_temp = String::new();
        
        for linha in conteudo.lines() {
            let linha = linha.trim();
            
            if linha.is_empty() || linha.starts_with('#') {
                continue;
            }
            
            match estado_atual {
                Estado::Normal => {
                    if let Some(pos) = linha.find('=') {
                        let chave = linha[..pos].trim().to_string();
                        let valor = linha[pos+1..].trim();
                        
                        if valor == "{" {
                            // Início de uma seção de dependências
                            estado_atual = Estado::Dependencias;
                            chave_temp = chave;
                        } else {
                            // Valor único
                            Self::definir_valor(&mut manifesto, &chave, valor)?;
                        }
                    }
                },
                Estado::Dependencias => {
                    if linha == "}" {
                        estado_atual = Estado::Normal;
                    } else if let Some(pos) = linha.find('=') {
                        let chave = linha[..pos].trim();
                        let valor = linha[pos+1..].trim();
                        
                        // Remover aspas
                        let chave = chave.trim_matches('"').to_string();
                        let valor = valor.trim_matches('"').to_string();
                        
                        manifesto.dependencias.insert(chave, valor);
                    }
                }
            }
        }
        
        Ok(manifesto)
    }
    
    /// Define um valor no manifesto com base na chave
    fn definir_valor(&mut self, chave: &str, valor: &str) -> Result<()> {
        match chave {
            "nome" => self.nome = valor.trim_matches('"').to_string(),
            "versao" => self.versao = valor.trim_matches('"').to_string(),
            "descricao" => self.descricao = valor.trim_matches('"').to_string(),
            "principal" => self.principal = valor.trim_matches('"').to_string(),
            "licenca" => self.licenca = valor.trim_matches('"').to_string(),
            "repositorio" => self.repositorio = valor.trim_matches('"').to_string(),
            "autores" => {
                if valor.starts_with('[') && valor.ends_with(']') {
                    let autores = valor[1..valor.len()-1]
                        .split(',')
                        .map(|a| a.trim().trim_matches('"').to_string())
                        .collect();
                    self.autores = autores;
                }
            },
            "palavras_chave" => {
                if valor.starts_with('[') && valor.ends_with(']') {
                    let palavras = valor[1..valor.len()-1]
                        .split(',')
                        .map(|p| p.trim().trim_matches('"').to_string())
                        .collect();
                    self.palavras_chave = palavras;
                }
            },
            _ => {
                // Ignorar chaves desconhecidas
            }
        }
        
        Ok(())
    }
}

/// Estado do parser
enum Estado {
    Normal,
    Dependencias,
}

/// Gerenciador de pacotes
pub struct GerenciadorPacotes {
    /// Diretório de cache para os pacotes
    cache_dir: PathBuf,
    
    /// Repositório de pacotes (URL)
    repositorio: String,
}

impl GerenciadorPacotes {
    /// Cria um novo gerenciador de pacotes
    pub fn novo() -> Result<Self> {
        // Diretório de cache padrão
        let home = dirs::home_dir().context("Não foi possível determinar o diretório home")?;
        let cache_dir = home.join(".pbrlang").join("caixotes");
        
        // Criar diretório se não existir
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .context("Não foi possível criar diretório de cache")?;
        }
        
        Ok(Self {
            cache_dir,
            repositorio: "https://caixotes.pbrlang.org".to_string(),
        })
    }
    
    /// Define o URL do repositório
    pub fn definir_repositorio(&mut self, url: &str) {
        self.repositorio = url.to_string();
    }
    
    /// Adiciona um pacote ao projeto atual
    pub fn adicionar_pacote(&self, nome: &str, versao: Option<&str>) -> Result<()> {
        println!("Adicionando pacote: {}", nome);
        
        // Verificar se já existe
        let versao = versao.unwrap_or("latest");
        
        // Baixar pacote (simulado)
        println!("Baixando pacote {} versão {}", nome, versao);
        
        // Instalar pacote (simulado)
        println!("Instalando pacote {} versão {}", nome, versao);
        
        // Atualizar manifesto
        // Isso seria implementado para atualizar o manifesto do projeto atual
        
        println!("Pacote {} adicionado com sucesso", nome);
        
        Ok(())
    }
    
    /// Remove um pacote do projeto atual
    pub fn remover_pacote(&self, nome: &str) -> Result<()> {
        println!("Removendo pacote: {}", nome);
        
        // Remover pacote (simulado)
        println!("Removendo pacote {}", nome);
        
        // Atualizar manifesto
        // Isso seria implementado para atualizar o manifesto do projeto atual
        
        println!("Pacote {} removido com sucesso", nome);
        
        Ok(())
    }
    
    /// Lista pacotes instalados
    pub fn listar_pacotes(&self) -> Result<Vec<String>> {
        // Isso seria implementado para ler o manifesto do projeto atual
        // e retornar a lista de pacotes instalados
        
        Ok(vec!["exemplo".to_string(), "teste".to_string()])
    }
    
    /// Publica um pacote no repositório
    pub fn publicar_pacote(&self, caminho: &Path) -> Result<()> {
        println!("Publicando pacote: {}", caminho.display());
        
        // Verificar se o manifesto existe
        let manifesto_caminho = caminho.join("caixote.pbr");
        if !manifesto_caminho.exists() {
            return Err(anyhow::anyhow!("Manifesto não encontrado: {}", manifesto_caminho.display()));
        }
        
        // Ler manifesto
        let manifesto = Manifesto::carregar(&manifesto_caminho)?;
        
        // Validar manifesto
        // Verificar campos obrigatórios
        
        // Empacotar (simulado)
        println!("Empacotando {}", manifesto.nome);
        
        // Enviar para repositório (simulado)
        println!("Enviando para repositório: {}", self.repositorio);
        
        println!("Pacote {} publicado com sucesso", manifesto.nome);
        
        Ok(())
    }
}

