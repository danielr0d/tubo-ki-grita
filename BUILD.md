# Build instructions para o projeto TS9 Tube Screamer Plugin

## Pré-requisitos:
# 1. Instalar Rust: https://rustup.rs/
# 2. Configurar para MSVC: `rustup default stable-msvc`
# 3. JUCE Framework v7+ instalado
# 4. Visual Studio Community 2026+

## Passos de compilação:

### 1. Compilar módulo Rust DSP
cd RustDsp
cargo build --release
# Isso gera: RustDsp/target/release/ts9_dsp.lib

### 2. Adicionar configuração ao JUCE
# No arquivo .jucer ou CMakeLists.txt do plugin:
# - Adicionar caminho: Modules -> RustBridge
# - Link library: RustDsp/target/release/ts9_dsp.lib
# - Include paths: ./Source/

### 3. Compilar plugin C++
# Abrir Ts9VstProject.sln no Visual Studio
# Build -> Build Solution (configuração Release)

### 4. Colocar imagem de fundo
# Copiar image_32aca5.jpg para:
# - Mesmo diretório do VST3 compilado
# - Ou in C:\Program Files\Common Files\VST3\ (se instalando)

## Notas sobre Rust + FFI:
# - Arquivo lib.rs expõe funções C-compatíveis via extern "C"
# - Todos os tipos são #[repr(C)] para compatibilidade
# - Sem alocações dinâmicas no FFI (apenas ponteiros)
# - Memory safety garantido pelo Rust mesmo com FFI

## Troubleshooting:

### Cargo não encontrado:
# Adicionar ao PATH do Windows:
# C:\Users\<username>\.cargo\bin

### Link error ts9_dsp.lib not found:
# 1. Garantir cargo build --release foi executado
# 2. Verificar path correto em configuração de link
# 3. Usar absolute path se necessário

### Plugin não carrega no DAW:
# 1. Copiar arquivos dependentes para diretório VST3
# 2. Testar em host independente primeiro
# 3. Ver logs de debug do plugin
