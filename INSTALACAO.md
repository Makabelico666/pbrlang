# Instalação da PBRLang

Este documento explica como instalar e configurar a PBRLang em seu sistema.

## Pré-requisitos

A PBRLang requer:

1. **Rust e Cargo** - Necessários para compilar a PBRLang e os programas PBRLang para código nativo.
   - [Instale o Rust](https://www.rust-lang.org/tools/install)

## Instalação

### Método 1: Scripts de instalação automatizados

#### Windows
```powershell
# Execute no PowerShell
./install.ps1
```

#### Linux/macOS
```bash
# Execute no terminal
chmod +x ./install.sh
./install.sh
```

### Método 2: Compilar do código-fonte manualmente

1. Clone o repositório:
   ```bash
   git clone https://github.com/pbrlang/pbrlang.git
   cd pbrlang
   ```

2. Compile o projeto:
   ```bash
   cargo build --release
   ```

3. Instale o binário:
   ```bash
   # Em sistemas Unix/Linux/macOS
   cp target/release/pbrlang /usr/local/bin/pbr
   
   # Em Windows, copie o executável para um diretório em seu PATH
   # Por exemplo:
   copy target\release\pbrlang.exe C:\Windows\System32\pbr.exe
   ```

4. Configure a biblioteca padrão:
   ```bash
   # Em sistemas Unix/Linux/macOS
   mkdir -p ~/.local/share/pbrlang/lib
   cp lib/*.pbr ~/.local/share/pbrlang/lib/
   export PBRLANG_LIB=~/.local/share/pbrlang/lib
   
   # Em Windows (PowerShell)
   mkdir -Force $env:LOCALAPPDATA\PBRLang\lib
   Copy-Item .\lib\*.pbr -Destination $env:LOCALAPPDATA\PBRLang\lib
   [Environment]::SetEnvironmentVariable("PBRLANG_LIB", "$env:LOCALAPPDATA\PBRLang\lib", "User")
   ```

### Método 3: Instalação via Cargo (em breve)

```bash
cargo install pbrlang
```

Este comando instalará automaticamente o binário `pbr` em seu sistema.

## Verificando a instalação

Para verificar se a PBRLang foi instalada corretamente, execute:

```bash
pbr --version
```

Você deverá ver a versão atual da PBRLang exibida no console.

## Primeiros passos

Depois de instalar a PBRLang, você pode criar um novo projeto:

```bash
pbr novo meu_projeto
cd meu_projeto
```

E executar o programa de exemplo:

```bash
pbr rodar programa.pbr
```

## Problemas comuns

### Comando não encontrado

Se você receber o erro "comando não encontrado" após a instalação, certifique-se de que o diretório de instalação está no seu PATH.

#### Windows
Reinicie seu terminal ou adicione o diretório ao PATH manualmente:
```powershell
$env:Path += ";$env:LOCALAPPDATA\PBRLang\bin"
```

#### Linux/macOS
Execute:
```bash
export PATH=$PATH:~/.local/bin
```

### Biblioteca padrão não encontrada

Se você receber erros sobre a biblioteca padrão não encontrada, certifique-se de que a variável de ambiente PBRLANG_LIB está configurada corretamente.

#### Windows
```powershell
[Environment]::SetEnvironmentVariable("PBRLANG_LIB", "$env:LOCALAPPDATA\PBRLang\lib", "User")
```

#### Linux/macOS
```bash
export PBRLANG_LIB=~/.local/share/pbrlang/lib
```

Para configuração permanente, adicione essa linha ao seu arquivo `~/.bashrc` ou `~/.zshrc`.

## Desinstalação

### Windows
```powershell
Remove-Item -Recurse -Force $env:LOCALAPPDATA\PBRLang
```

### Linux/macOS
```bash
rm -rf ~/.local/bin/pbr ~/.local/share/pbrlang
```

Se instalou globalmente:
```bash
sudo rm -rf /usr/local/bin/pbr /usr/local/share/pbrlang
```

# Instalação da PBRLang

Este documento explica como instalar e configurar a PBRLang em seu sistema.

## Pré-requisitos

A PBRLang requer:

1. **Rust e Cargo** - Necessários para compilar a PBRLang e os programas PBRLang para código nativo.
   - [Instale o Rust](https://www.rust-lang.org/tools/install)

## Instalação

### Método 1: Compilar do código-fonte

1. Clone o repositório:
   ```bash
   git clone https://github.com/pbrlang/pbrlang.git
   cd pbrlang
   ```

2. Compile o projeto:
   ```bash
   cargo build --release
   ```

3. Instale o binário:
   ```bash
   # Em sistemas Unix/Linux/macOS
   cp target/release/pbrlang /usr/local/bin/pbr
   
   # Em Windows, copie o executável para um diretório em seu PATH
   # Por exemplo:
   copy target\release\pbrlang.exe C:\Windows\System32\pbr.exe
   ```

### Método 2: Instalação via Cargo

```bash
cargo install pbrlang
```

Este comando instalará automaticamente o binário `pbr` em seu sistema.

## Verificando a instalação

Para verificar se a PBRLang foi instalada corretamente, execute:

```bash
pbr --version
```

Você deverá ver a versão atual da PBRLang exibida no console.

## Primeiros passos

Depois de instalar a PBRLang, você pode criar um novo projeto:

```bash
pbr novo meu_projeto
cd meu_projeto
```

E executar o programa de exemplo:

```bash
pbr rodar programa.pbr
```

Para mais informações sobre como usar a PBRLang, consulte a [Documentação](./README.md).

