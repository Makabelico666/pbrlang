# Script de instalação para PBRLang no Windows
# Este script compila e instala o CLI da PBRLang

# Função para verificar se o Rust está instalado
function Test-RustInstalled {
    try {
        $rustcVersion = rustc --version
        return $true
    }
    catch {
        return $false
    }
}

# Função para verificar se o Cargo está instalado
function Test-CargoInstalled {
    try {
        $cargoVersion = cargo --version
        return $true
    }
    catch {
        return $false
    }
}

# Verificar se o Rust e o Cargo estão instalados
Write-Host "Verificando pré-requisitos..." -ForegroundColor Cyan

if (-not (Test-RustInstalled)) {
    Write-Host "Rust não está instalado!" -ForegroundColor Red
    Write-Host "Por favor, instale o Rust visitando https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
    exit 1
}

if (-not (Test-CargoInstalled)) {
    Write-Host "Cargo não está instalado!" -ForegroundColor Red
    Write-Host "Por favor, instale o Cargo visitando https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
    exit 1
}

Write-Host "Rust e Cargo encontrados. Prosseguindo com a instalação..." -ForegroundColor Green

# Compilar o projeto em modo release
Write-Host "Compilando PBRLang..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Falha ao compilar PBRLang!" -ForegroundColor Red
    exit 1
}

# Verificar se o executável foi gerado
$exePath = ".\target\release\pbrlang.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "Executável não encontrado em $exePath!" -ForegroundColor Red
    exit 1
}

# Criar diretório para o executável e biblioteca padrão
$installDir = "$env:LOCALAPPDATA\PBRLang\bin"
$libDir = "$env:LOCALAPPDATA\PBRLang\lib"

# Criar diretórios se não existirem
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Host "Criado diretório de instalação: $installDir" -ForegroundColor Green
}

if (-not (Test-Path $libDir)) {
    New-Item -ItemType Directory -Path $libDir -Force | Out-Null
    Write-Host "Criado diretório da biblioteca padrão: $libDir" -ForegroundColor Green
}

# Copiar o executável para o diretório de instalação
Copy-Item $exePath -Destination "$installDir\pbr.exe" -Force
Write-Host "PBRLang instalado em: $installDir\pbr.exe" -ForegroundColor Green

# Copiar a biblioteca padrão
Write-Host "Instalando biblioteca padrão..." -ForegroundColor Cyan
Copy-Item ".\lib\*.pbr" -Destination $libDir -Force
Write-Host "Biblioteca padrão instalada em: $libDir" -ForegroundColor Green

# Verificar se o diretório já está no PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    # Adicionar ao PATH do usuário
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "Diretório adicionado ao PATH do usuário" -ForegroundColor Green
    Write-Host "IMPORTANTE: Você precisa reiniciar seu terminal para usar o comando 'pbr'" -ForegroundColor Yellow
} else {
    Write-Host "Diretório já está no PATH" -ForegroundColor Green
}

# Configurar variável de ambiente para a biblioteca padrão
[Environment]::SetEnvironmentVariable("PBRLANG_LIB", $libDir, "User")
Write-Host "Variável de ambiente PBRLANG_LIB configurada" -ForegroundColor Green

# Verificar a instalação
Write-Host "`nInstalação concluída!" -ForegroundColor Green
Write-Host "Para usar a PBRLang, abra um novo terminal e digite 'pbr --help'" -ForegroundColor Cyan
Write-Host "Para criar um novo projeto: pbr novo meu_projeto" -ForegroundColor Cyan
Write-Host "Para executar um programa: pbr rodar arquivo.pbr" -ForegroundColor Cyan

# Script de instalação para PBRLang no Windows
# Este script compila e instala o CLI da PBRLang

# Função para verificar se o Rust está instalado
function Test-RustInstalled {
    try {
        $rustcVersion = rustc --version
        return $true
    }
    catch {
        return $false
    }
}

# Função para verificar se o Cargo está instalado
function Test-CargoInstalled {
    try {
        $cargoVersion = cargo --version
        return $true
    }
    catch {
        return $false
    }
}

# Verificar se o Rust e o Cargo estão instalados
Write-Host "Verificando pré-requisitos..." -ForegroundColor Cyan

if (-not (Test-RustInstalled)) {
    Write-Host "Rust não está instalado!" -ForegroundColor Red
    Write-Host "Por favor, instale o Rust visitando https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
    exit 1
}

if (-not (Test-CargoInstalled)) {
    Write-Host "Cargo não está instalado!" -ForegroundColor Red
    Write-Host "Por favor, instale o Cargo visitando https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
    exit 1
}

Write-Host "Rust e Cargo encontrados. Prosseguindo com a instalação..." -ForegroundColor Green

# Compilar o projeto em modo release
Write-Host "Compilando PBRLang..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Falha ao compilar PBRLang!" -ForegroundColor Red
    exit 1
}

# Verificar se o executável foi gerado
$exePath = ".\target\release\pbrlang.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "Executável não encontrado em $exePath!" -ForegroundColor Red
    exit 1
}

# Criar diretório para o executável no PATH
$installDir = "$env:LOCALAPPDATA\PBRLang\bin"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Host "Criado diretório de instalação: $installDir" -ForegroundColor Green
}

# Copiar o executável para o diretório de instalação
Copy-Item $exePath -Destination "$installDir\pbr.exe" -Force
Write-Host "PBRLang instalado em: $installDir\pbr.exe" -ForegroundColor Green

# Verificar se o diretório já está no PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    # Adicionar ao PATH do usuário
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "Diretório adicionado ao PATH do usuário" -ForegroundColor Green
    Write-Host "IMPORTANTE: Você precisa reiniciar seu terminal para usar o comando 'pbr'" -ForegroundColor Yellow
} else {
    Write-Host "Diretório já está no PATH" -ForegroundColor Green
}

# Verificar a instalação
Write-Host "`nInstalação concluída!" -ForegroundColor Green
Write-Host "Para usar a PBRLang, abra um novo terminal e digite 'pbr --help'" -ForegroundColor Cyan
Write-Host "Para criar um novo projeto: pbr novo meu_projeto" -ForegroundColor Cyan
Write-Host "Para executar um programa: pbr rodar arquivo.pbr" -ForegroundColor Cyan

