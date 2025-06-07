/// Árvore Sintática Abstrata (AST) para PBRLang
/// Representa a estrutura do código fonte após análise sintática

#[derive(Debug, Clone, PartialEq)]
pub enum Tipo {
    Texto,
    Numero,
    Logico,
    Void,
    Personalizado(String),
    // Para representar tipos opcionais (equivalente a Option<T> em Rust)
    Opcional(Box<Tipo>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expressao {
    // Literais
    TextoLiteral(String),
    NumeroLiteral(f64),
    LogicoLiteral(bool),
    Nada,
    ListaLiteral(Vec<Expressao>),
    DicionarioLiteral(Vec<(Expressao, Expressao)>),
    
    // Variáveis e operações
    Identificador(String),
    Chamada {
        nome: String,
        argumentos: Vec<Expressao>,
    },
    Operacao {
        operador: Operador,
        esquerda: Box<Expressao>,
        direita: Box<Expressao>,
    },
    Atribuicao {
        nome: String,
        valor: Box<Expressao>,
    },
    
    // Acesso a membros (como em objetos/structs)
    AcessoMembro {
        objeto: Box<Expressao>,
        membro: String,
    },
    
    // Operador unário (negação)
    Negacao {
        expressao: Box<Expressao>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operador {
    Soma,
    Subtracao,
    Multiplicacao,
    Divisao,
    Resto,
    Igual,
    Diferente,
    Maior,
    Menor,
    MaiorIgual,
    MenorIgual,
    Contem,   // Para o operador "em"
    E,        // Para operador lógico "e"
    Ou,       // Para operador lógico "ou"
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaracao {
    // Declaração de variável
    Variavel {
        nome: String,
        tipo: Option<Tipo>,
        valor: Option<Expressao>,
        publico: bool,
    },
    
    // Expressão como declaração (ex: chamada de função)
    Expressao(Expressao),
    
    // Bloco de código
    Bloco(Vec<Declaracao>),
    
    // Estruturas de controle
    Se {
        condicao: Expressao,
        bloco_se: Box<Declaracao>,
        bloco_senao: Option<Box<Declaracao>>,
    },
    
    ParaCada {
        variavel: String,
        inicio: Expressao,
        fim: Expressao,
        corpo: Box<Declaracao>,
    },
    
    Enquanto {
        condicao: Expressao,
        corpo: Box<Declaracao>,
    },
    
    Repita {
        corpo: Box<Declaracao>,
        condicao: Expressao,
    },
    
    Pare,
    
    Continue,
    
    // Tratamento de erros
    QuandoDerErro {
        bloco_try: Box<Declaracao>,
        variavel_erro: String,
        bloco_catch: Box<Declaracao>,
    },
    
    // Funções
    Funcao {
        nome: String,
        parametros: Vec<(String, Tipo)>,
        tipo_retorno: Option<Tipo>,
        corpo: Box<Declaracao>,
        publico: bool,
    },
    
    Retorno(Option<Expressao>),
    
    // Mostrar no console
    Mostrar(Expressao),
    
    // Declaração de modelo (struct)
    Modelo {
        nome: String,
        campos: Vec<(String, Tipo, bool)>,  // (nome, tipo, publico)
        publico: bool,
    },
    
    Modulo {
        nome: String,
        declaracoes: Vec<Declaracao>,
    },
    
    Importar {
        caminho: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Programa {
    pub declaracoes: Vec<Declaracao>,
}

impl Programa {
    pub fn new() -> Self {
        Programa {
            declaracoes: Vec::new(),
        }
    }
    
    pub fn adicionar_declaracao(&mut self, declaracao: Declaracao) {
        self.declaracoes.push(declaracao);
    }
}

