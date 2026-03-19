# Guia de Integração Visual Studio - TS9 VST Plugin

## Visão Geral da Integração

Este documento descreve como integrar o módulo Rust DSP com o projeto JUCE C++ no Visual Studio.

## Passo 1: Compilar a Biblioteca Rust

### Pré-requisito
- Rust instalado com MSVC toolchain: `rustup default stable-msvc`
- Visual Studio Build Tools ou Community Edition com C++ support

### Compilação

```powershell
cd C:\Users\danieo\Documents\Codes\audio_stuff\tubo-ki-grita\RustDsp
cargo build --release
```

**Resultado esperado:**
- Arquivo gerado: `RustDsp/target/release/ts9_dsp.lib`
- Tamanho: ~5-10 MB (biblioteca estática compilada)

## Passo 2: Configurar JUCE no Visual Studio

### Opção A: Usar Projucer (Recomendado)

1. Abrir `Ts9VstProject.jucer` com o Projucer (JUCE Projucer)
2. Verificar caminhos JUCE:
   - Path to JUCE modules: `C:\Path\To\JUCE\modules`
3. Configurar Visual Studio 2022:
   - Select exporter: "Visual Studio 2022"
   - Output folder: Builds/VisualStudio2022
4. **Adicionar Rust library:**
   - VS2022 > Extra Linker Flags (ou settings de linker)
   - Adicionar: `ts9_dsp.lib`
   - Library Search Paths: `$(ProjectDir)..\..\RustDsp\target\release`

5. Clicar "Save and Open in IDE"

### Opção B: Configuração Manual no Visual Studio

1. Abrir `Builds\VisualStudio2022\Ts9VST.sln`
2. Right-click no projeto > Properties
3. Ir para: Configuration Properties > VC++ Directories
   - **Include Directories:** Adicionar `$(ProjectDir)\..\..\Source`
4. Ir para: Linker > Input
   - **Additional Dependencies:** Adicionar `ts9_dsp.lib`
5. Ir para: Linker > General
   - **Additional Library Directories:** `$(ProjectDir)\..\..\RustDsp\target\release`

## Passo 3: Estrutura de Arquivos no Projeto

```
Ts9VstProject/
├── Builds/
│   └── VisualStudio2022/
│       ├── Ts9VST.sln
│       └── Ts9VST/
│           └── Ts9VST.vcxproj
├── JuceLibraryCode/
├── Source/
│   ├── PluginProcessor.h/.cpp
│   ├── PluginEditor.h/.cpp
│   ├── Ts9LookAndFeel.h
│   ├── RustBridge.h
│   └── JuceHeader.h
├── RustDsp/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   └── dsp/
│   │       ├── mod.rs
│   │       ├── oversampler.rs
│   │       ├── clipping.rs
│   │       ├── filters.rs
│   │       └── gain_stage.rs
│   └── target/release/
│       └── ts9_dsp.lib ← Gerado pelo cargo build
└── Ts9VstProject.jucer
```

## Passo 4: Compilação no Visual Studio

1. Abrir Visual Studio Solution: `Builds/VisualStudio2022/Ts9VST.sln`
2. Build Configuration: **Release** (importante para performance de áudio)
3. Menu Build > Build Solution (Ctrl+Shift+B)

**Esperado:**
- Compilação sem erros
- VST3 gerado em: `Builds/VisualStudio2022/Ts9VST_VST3/Release/Ts9VST.vst3`

## Passo 5: Colocar Plugin no DAW

### Para testes local:
1. Copiar `image_32aca5.jpg` para o diretório do VST3 gerado
2. Copiar a pasta `.vst3` para:
   ```
   C:\Program Files\Common Files\VST3\
   ```

### Para testes em DAW (ex: Reaper, FL Studio):
1. Configurar caminho VST3 no DAW:
   - Adicionar: `C:\Program Files\Common Files\VST3\`
2. Re-escanear plugins
3. Procurar por "TS9" ou "Tube Screamer"

## Troubleshooting

### Erro: "Unresolved external symbol create_ts9_dsp"
**Causa:** Rust library não linkada corretamente
**Solução:**
- Verificar se `ts9_dsp.lib` existe em `RustDsp/target/release/`
- Se não existir, rodar `cargo build --release` novamente
- Verificar paths em Linker > Input

### Erro: "image_32aca5.jpg not found"
**Causa:** Imagem não está no diretório correto
**Solução:**
- Copiar `image_32aca5.jpg` para o mesmo diretório do VST3 executável
- Ou modificar função `loadBackgroundImage()` em `PluginEditor.cpp` com path absoluto

### Plugin não aparece no DAW
**Causas possíveis:**
1. VST3 não está em `C:\Program Files\Common Files\VST3\`
2. DAW não fez rescan de plugins
3. Erro no inicialização (ver console do DAW)

**Solução:**
- Verificar paths VST3 no DAW
- Forçar re-scan de plugins
- Rodar DAW com modo debug para ver logs de erro

### Áudio distorcido ou sem som
**Teste diagnóstico:**
1. Verificar se `bypass` está OFF
2. Colocar `level` em ~0.7
3. Colocar `drive` em ~0.3
4. Testar com sinal de entrada limpo (sine wave)

**Se ainda não funcionar:**
- Revisar `PluginProcessor::processBlock()`
- Verificar valores dos parâmetros sendo passados para Rust
- Adicionar logging/debugging temporário

## Configurações Recomendadas para Release

### Build Properties
```
Platform: x64 (essencial para performance de áudio)
Configuration: Release
Optimization: /O2 (maximize speed)
```

### Compiler Flags
```
/fp:fast (faster floating point)
/Gy (enable function-level linking)
```

### Linker Flags
```
/SUBSYSTEM:WINDOWS
/OPT:REF
/OPT:ICF
```

## Próximos Passos

1. **Testes auditivos:** Comparar som com TS9 físico
2. **Otimização:** Medir CPU usage, perfil hot spots
3. **Presets:** Implementar save/load de configurações
4. **Documentação:** Adicionar comentários FFI
5. **Distribuição:** Assinatura digital do plugin, installer

## Recursos Úteis

- JUCE Framework: https://juce.com/
- Rust FFI: https://doc.rust-lang.org/nomicon/ffi.html
- VST3 Spec: https://steinbergmedia.github.io/vst3_dev_portal/
- TS9 Schematics: http://www.electrosmash.com/

---

**Versão do Guia:** 1.0  
**Última atualização:** 2024
