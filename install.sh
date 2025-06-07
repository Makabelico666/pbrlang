#!/bin/bash
# Script de instalação para PBRLang em sistemas Unix/Linux/macOS

# Cores para formatação
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Função para verificar se um comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar se o Rust e Cargo estão instalados
echo -e "${CYAN}Verificando pré-requisitos...${NC}"

if ! command_exists rustc; then
    echo -e "${RED}Rust não está instalado!${NC}"
    echo -e "${YELLOW}Por favor, instale o Rust visitando https://www.rust-lang.org/tools/install${NC}"
    exit 1
fi

if ! command_exists cargo; then
    echo -e "${RED}Cargo não está instalado!${NC}"
    echo -e "${YELLOW}Por favor, instale o Cargo visitando https://www.rust-lang.org/tools/install${NC}"
    exit 1
fi

echo -e "${GREEN}Rust e Cargo encontrados. Prosseguindo com a instalação...${NC}"

# Compilar o projeto em modo release
echo -e "${CYAN}Compilando PBRLang...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Falha ao compilar PBRLang!${NC}"
    exit 1
fi

# Verificar se o executável foi gerado
EXE_PATH="./target/release/pbrlang"
if [ ! -f "$EXE_PATH" ]; then
    echo -e "${RED}Executável não encontrado em $EXE_PATH!${NC}"
    exit 1
fi

# Determinar os diretórios de instalação
if [ "$(uname)" == "Darwin" ]; then
    # macOS
    INSTALL_DIR="$HOME/.local/bin"
    LIB_DIR="$HOME/.local/share/pbrlang/lib"
else
    # Linux
    INSTALL_DIR="$HOME/.local/bin"
    LIB_DIR="$HOME/.local/share/pbrlang/lib"
    
    # Se tiver permissão de sudo, pode instalar globalmente
    if command_exists sudo && [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
        read -p "Instalar PBRLang globalmente? (S/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Ss]$ ]] || [ -z "$REPLY" ]; then
            INSTALL_DIR="/usr/local/bin"
            LIB_DIR="/usr/local/share/pbrlang/lib"
        fi
    fi
fi

# Criar diretórios se não existirem
mkdir -p "$INSTALL_DIR"
mkdir -p "$LIB_DIR"

echo -e "${GREEN}Criados diretórios de instalação:${NC}"
echo -e "  Binários: $INSTALL_DIR"
echo -e "  Biblioteca: $LIB_DIR"

# Copiar o executável para o diretório de instalação
cp "$EXE_PATH" "$INSTALL_DIR/pbr"
chmod +x "$INSTALL_DIR/pbr"
echo -e "${GREEN}PBRLang instalado em: $INSTALL_DIR/pbr${NC}"

# Copiar a biblioteca padrão
echo -e "${CYAN}Instalando biblioteca padrão...${NC}"
cp ./lib/*.pbr "$LIB_DIR/"
echo -e "${GREEN}Biblioteca padrão instalada em: $LIB_DIR${NC}"

# Verificar se o diretório está no PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}ATENÇÃO: $INSTALL_DIR não está no seu PATH.${NC}"
    echo -e "${YELLOW}Adicione a seguinte linha ao seu ~/.bashrc ou ~/.zshrc:${NC}"
    echo -e "${CYAN}export PATH=\$PATH:$INSTALL_DIR${NC}"
    
    # Perguntar se deve adicionar ao .bashrc automaticamente
    if [ -f "$HOME/.bashrc" ]; then
        read -p "Adicionar ao PATH automaticamente no .bashrc? (S/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Ss]$ ]] || [ -z "$REPLY" ]; then
            echo "export PATH=\$PATH:$INSTALL_DIR" >> "$HOME/.bashrc"
            echo -e "${GREEN}Adicionado ao PATH no .bashrc${NC}"
            echo -e "${YELLOW}Execute 'source ~/.bashrc' para atualizar o PATH na sessão atual${NC}"
        fi
    fi
else
    echo -e "${GREEN}Diretório já está no PATH${NC}"
fi

# Configurar variável de ambiente para a biblioteca padrão
if [ -f "$HOME/.bashrc" ]; then
    grep -q "PBRLANG_LIB" "$HOME/.bashrc" || echo "export PBRLANG_LIB=$LIB_DIR" >> "$HOME/.bashrc"
    echo -e "${GREEN}Variável de ambiente PBRLANG_LIB configurada no .bashrc${NC}"
fi

if [ -f "$HOME/.zshrc" ]; then
    grep -q "PBRLANG_LIB" "$HOME/.zshrc" || echo "export PBRLANG_LIB=$LIB_DIR" >> "$HOME/.zshrc"
    echo -e "${GREEN}Variável de ambiente PBRLANG_LIB configurada no .zshrc${NC}"
fi

# Verificar a instalação
echo -e "\n${GREEN}Instalação concluída!${NC}"
echo -e "${CYAN}Para usar a PBRLang, você pode precisar reiniciar seu terminal ou executar:${NC}"
echo -e "  source ~/.bashrc (se usar Bash)"
echo -e "  source ~/.zshrc (se usar Zsh)"
echo -e "${CYAN}Para criar um novo projeto: pbr novo meu_projeto${NC}"
echo -e "${CYAN}Para executar um programa: pbr rodar arquivo.pbr${NC}"

#!/bin/bash
# Script de instalação para PBRLang em sistemas Unix/Linux/macOS

# Cores para formatação
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Função para verificar se um comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar se o Rust e Cargo estão instalados
echo -e "${CYAN}Verificando pré-requisitos...${NC}"

if ! command_exists rustc; then
    echo -e "${RED}Rust não está instalado!${NC}"
    echo -e "${YELLOW}Por favor, instale o Rust visitando https://www.rust-lang.org/tools/install${NC}"
    exit 1
fi

if ! command_exists cargo; then
    echo -e "${RED}Cargo não está instalado!${NC}"
    echo -e "${YELLOW}Por favor, instale o Cargo visitando https://www.rust-lang.org/tools/install${NC}"
    exit 1
fi

echo -e "${GREEN}Rust e Cargo encontrados. Prosseguindo com a instalação...${NC}"

# Compilar o projeto em modo release
echo -e "${CYAN}Compilando PBRLang...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Falha ao compilar PBRLang!${NC}"
    exit 1
fi

# Verificar se o executável foi gerado
EXE_PATH="./target/release/pbrlang"
if [ ! -f "$EXE_PATH" ]; then
    echo -e "${RED}Executável não encontrado em $EXE_PATH!${NC}"
    exit 1
fi

# Determinar o diretório de instalação
INSTALL_DIR="$HOME/.local/bin"
if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
fi

# Criar diretório se não existir
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
    echo -e "${GREEN}Criado diretório de instalação: $INSTALL_DIR${NC}"
fi

# Copiar o executável para o diretório de instalação
cp "$EXE_PATH" "$INSTALL_DIR/pbr"
chmod +x "$INSTALL_DIR/pbr"
echo -e "${GREEN}PBRLang instalado em: $INSTALL_DIR/pbr${NC}"

# Verificar se o diretório está no PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}ATENÇÃO: $INSTALL_DIR não está no seu PATH.${NC}"
    echo -e "${YELLOW}Adicione a seguinte linha ao seu ~/.bashrc ou ~/.zshrc:${NC}"
    echo -e "${CYAN}export PATH=\$PATH:$INSTALL_DIR${NC}"
else
    echo -e "${GREEN}Diretório já está no PATH${NC}"
fi

# Verificar a instalação
echo -e "\n${GREEN}Instalação concluída!${NC}"
echo -e "${CYAN}Para usar a PBRLang, digite 'pbr --help'${NC}"
echo -e "${CYAN}Para criar um novo projeto: pbr novo meu_projeto${NC}"
echo -e "${CYAN}Para executar um programa: pbr rodar arquivo.pbr${NC}"

