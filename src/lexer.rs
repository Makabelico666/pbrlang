use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palavras-chave
    Faca,           // faça
    Mostre,         // mostre
    Volte,          // volte
    Se,             // se
    Senao,          // senão
    ParaCada,       // para cada
    De,             // de
    Ate,            // até
    Modelo,         // modelo
    QuandoDerErro,  // quando der erro
    FalharCom,      // falhar com
    Pense,          // pense
    Verdadeiro,     // verdadeiro
    Falso,          // falso
    Nada,           // nada
    
    // Tipos
    TipoTexto,      // texto
    TipoNumero,     // número
    TipoLogico,     // lógico
    
    // Identificadores e literais
    Identificador(String),
    NumeroLiteral(f64),
    TextoLiteral(String),
    
    // Operadores
    Mais,           // +
    Menos,          // -
    Vezes,          // *
    Dividido,       // /
    Igual,          // =
    IgualA,         // é igual a
    Diferente,      // !=
    Maior,          // >
    Menor,          // <
    MaiorIgual,     // >=
    MenorIgual,     // <=
    
    // Símbolos
    AbreChave,      // {
    FechaChave,     // }
    AbreParentese,  // (
    FechaParentese, // )
    Virgula,        // ,
    DoisPontos,     // :
    PontoVirgula,   // ;
    
    // Fim de arquivo
    EOF,
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Caractere inesperado: {0}")]
    CaractereInesperado(char),
    
    #[error("String não terminada")]
    StringNaoTerminada,
    
    #[error("Erro no formato do número: {0}")]
    ErroNumero(String),
}

pub struct Lexer {
    entrada: Vec<char>,
    posicao: usize,
    linha: usize,
    coluna: usize,
}

impl Lexer {
    pub fn new(entrada: &str) -> Self {
        Lexer {
            entrada: entrada.chars().collect(),
            posicao: 0,
            linha: 1,
            coluna: 1,
        }
    }
    
    fn avancar(&mut self) {
        if self.posicao < self.entrada.len() {
            if self.entrada[self.posicao] == '\n' {
                self.linha += 1;
                self.coluna = 1;
            } else {
                self.coluna += 1;
            }
            self.posicao += 1;
        }
    }
    
    fn caractere_atual(&self) -> Option<char> {
        if self.posicao < self.entrada.len() {
            Some(self.entrada[self.posicao])
        } else {
            None
        }
    }
    
    fn proximo_caractere(&self) -> Option<char> {
        if self.posicao + 1 < self.entrada.len() {
            Some(self.entrada[self.posicao + 1])
        } else {
            None
        }
    }
    
    fn pular_espacos(&mut self) {
        while let Some(c) = self.caractere_atual() {
            if c.is_whitespace() {
                self.avancar();
            } else {
                break;
            }
        }
    }
    
    fn ler_identificador(&mut self) -> String {
        let mut id = String::new();
        
        while let Some(c) = self.caractere_atual() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.avancar();
            } else {
                break;
            }
        }
        
        id
    }
    
    fn ler_numero(&mut self) -> Result<f64, LexerError> {
        let mut num = String::new();
        let mut tem_ponto = false;
        
        while let Some(c) = self.caractere_atual() {
            if c.is_digit(10) {
                num.push(c);
                self.avancar();
            } else if c == '.' && !tem_ponto {
                num.push(c);
                tem_ponto = true;
                self.avancar();
            } else {
                break;
            }
        }
        
        num.parse::<f64>().map_err(|_| LexerError::ErroNumero(num))
    }
    
    fn ler_texto(&mut self) -> Result<String, LexerError> {
        let mut texto = String::new();
        self.avancar(); // Pular a aspas inicial
        
        while let Some(c) = self.caractere_atual() {
            if c == '"' {
                self.avancar(); // Pular a aspas final
                return Ok(texto);
            } else {
                texto.push(c);
                self.avancar();
            }
        }
        
        Err(LexerError::StringNaoTerminada)
    }
    
    pub fn proximo_token(&mut self) -> Result<Token, LexerError> {
        self.pular_espacos();
        
        match self.caractere_atual() {
            None => Ok(Token::EOF),
            
            Some(c) => {
                match c {
                    // Operadores e símbolos
                    '{' => { self.avancar(); Ok(Token::AbreChave) },
                    '}' => { self.avancar(); Ok(Token::FechaChave) },
                    '(' => { self.avancar(); Ok(Token::AbreParentese) },
                    ')' => { self.avancar(); Ok(Token::FechaParentese) },
                    ',' => { self.avancar(); Ok(Token::Virgula) },
                    ':' => { self.avancar(); Ok(Token::DoisPontos) },
                    ';' => { self.avancar(); Ok(Token::PontoVirgula) },
                    '+' => { self.avancar(); Ok(Token::Mais) },
                    '-' => { self.avancar(); Ok(Token::Menos) },
                    '*' => { self.avancar(); Ok(Token::Vezes) },
                    '/' => { 
                        self.avancar();
                        // Verificar se é comentário
                        if let Some('/') = self.caractere_atual() {
                            // Pular até o final da linha
                            while let Some(c) = self.caractere_atual() {
                                if c == '\n' {
                                    self.avancar();
                                    break;
                                }
                                self.avancar();
                            }
                            self.proximo_token() // Retornar o próximo token após o comentário
                        } else {
                            Ok(Token::Dividido)
                        }
                    },
                    '=' => { self.avancar(); Ok(Token::Igual) },
                    '>' => {
                        self.avancar();
                        if let Some('=') = self.caractere_atual() {
                            self.avancar();
                            Ok(Token::MaiorIgual)
                        } else {
                            Ok(Token::Maior)
                        }
                    },
                    '<' => {
                        self.avancar();
                        if let Some('=') = self.caractere_atual() {
                            self.avancar();
                            Ok(Token::MenorIgual)
                        } else {
                            Ok(Token::Menor)
                        }
                    },
                    '!' => {
                        self.avancar();
                        if let Some('=') = self.caractere_atual() {
                            self.avancar();
                            Ok(Token::Diferente)
                        } else {
                            Err(LexerError::CaractereInesperado('!'))
                        }
                    },
                    
                    // Literais de texto
                    '"' => self.ler_texto().map(Token::TextoLiteral),
                    
                    // Números
                    '0'..='9' => self.ler_numero().map(Token::NumeroLiteral),
                    
                    // Identificadores e palavras-chave
                    'a'..='z' | 'A'..='Z' | '_' | 'á'..='ú' | 'Á'..='Ú' => {
                        let id = self.ler_identificador();
                        
                        match id.as_str() {
                            "faça" => Ok(Token::Faca),
                            "mostre" => Ok(Token::Mostre),
                            "volte" => Ok(Token::Volte),
                            "se" => Ok(Token::Se),
                            "senão" => Ok(Token::Senao),
                            "para" => {
                                // Verifica se é "para cada"
                                self.pular_espacos();
                                if let Some(c) = self.caractere_atual() {
                                    if c == 'c' {
                                        let resto = self.ler_identificador();
                                        if resto == "cada" {
                                            return Ok(Token::ParaCada);
                                        }
                                    }
                                }
                                Ok(Token::Identificador(format!("para_{}", self.ler_identificador())))
                            },
                            "cada" => Ok(Token::Identificador("cada".to_string())),
                            "de" => Ok(Token::De),
                            "até" | "ate" => Ok(Token::Ate),
                            "modelo" => Ok(Token::Modelo),
                            "quando" => {
                                // Verifica se é "quando der erro"
                                self.pular_espacos();
                                if let Some(c) = self.caractere_atual() {
                                    if c == 'd' {
                                        let der = self.ler_identificador();
                                        if der == "der" {
                                            self.pular_espacos();
                                            if let Some(c) = self.caractere_atual() {
                                                if c == 'e' {
                                                    let erro = self.ler_identificador();
                                                    if erro == "erro" {
                                                        return Ok(Token::QuandoDerErro);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Ok(Token::Identificador("quando".to_string()))
                            },
                            "falhar" => {
                                // Verifica se é "falhar com"
                                self.pular_espacos();
                                if let Some(c) = self.caractere_atual() {
                                    if c == 'c' {
                                        let com = self.ler_identificador();
                                        if com == "com" {
                                            return Ok(Token::FalharCom);
                                        }
                                    }
                                }
                                Ok(Token::Identificador("falhar".to_string()))
                            },
                            "pense" => Ok(Token::Pense),
                            "verdadeiro" => Ok(Token::Verdadeiro),
                            "falso" => Ok(Token::Falso),
                            "nada" => Ok(Token::Nada),
                            
                            // Tipos
                            "texto" => Ok(Token::TipoTexto),
                            "número" | "numero" => Ok(Token::TipoNumero),
                            "lógico" | "logico" => Ok(Token::TipoLogico),
                            
                            // Comparações especiais
                            "é" | "e" => {
                                // Verifica se é "é igual a"
                                self.pular_espacos();
                                if let Some(c) = self.caractere_atual() {
                                    if c == 'i' {
                                        let igual = self.ler_identificador();
                                        if igual == "igual" {
                                            self.pular_espacos();
                                            if let Some(c) = self.caractere_atual() {
                                                if c == 'a' {
                                                    self.avancar();
                                                    return Ok(Token::IgualA);
                                                }
                                            }
                                        }
                                    }
                                }
                                Ok(Token::Identificador("é".to_string()))
                            },
                            
                            // Identificador normal
                            _ => Ok(Token::Identificador(id)),
                        }
                    },
                    
                    // Qualquer outro caractere é um erro
                    _ => Err(LexerError::CaractereInesperado(c)),
                }
            }
        }
    }
}

pub fn tokenizar(codigo: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(codigo);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.proximo_token() {
            Ok(Token::EOF) => {
                tokens.push(Token::EOF);
                break;
            },
            Ok(token) => tokens.push(token),
            Err(e) => return Err(anyhow::Error::new(e)),
        }
    }
    
    Ok(tokens)
}

