# Checklist de Configuração e Build - TS9 VST Plugin

## ✅ Pré-Requisitos & Setup Inicial

### Sistema & Ferramentas
- [ ] Windows 10/11 64-bit
- [ ] Visual Studio Community 2022+ instalado
- [ ] Visual Studio C++ workload ativado
- [ ] Rust instalado (`rustup`)
- [ ] MSVC Rust toolchain configurado (`rustup default stable-msvc`)

### Validação
```powershell
# Verificar Rust
rustc --version
rustup show

# Verificar Visual Studio
"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" x64
```

---

## ✅ Fase 1: Preparação Rust DSP

### 1.1 Estrutura de Diretórios
- [ ] `RustDsp/` existe com `Cargo.toml`
- [ ] `RustDsp/src/lib.rs` criado (FFI principal)
- [ ] `RustDsp/src/dsp/` contém:
  - [ ] `mod.rs`
  - [ ] `oversampler.rs`
  - [ ] `clipping.rs`
  - [ ] `filters.rs`
  - [ ] `gain_stage.rs`

### 1.2 Compilação Rust
```powershell
cd RustDsp

# Testar build
cargo build --release

# Verificar artefatos
ls target/release/ts9_dsp.* | Select-Object Name, Length
```

**Resultado esperado:**
- `ts9_dsp.lib` (~5-10 MB)
- Sem erros de compilação
- Warnings são OK (FFI pode gerar alguns)

### 1.3 Validação de Testes (Opcional)
```powershell
cargo test --release
```

Expected output:
```
running 10 tests
test oversampler::tests::test_oversampler_buffer_size ... ok
test clipping::tests::test_soft_clip_symmetry ... ok
[...]
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured
```

---

## ✅ Fase 2: Configuração C++ / JUCE

### 2.1 Arquivos C++ Criados
- [ ] `Source/RustBridge.h`
- [ ] `Source/PluginProcessor.h`
- [ ] `Source/PluginProcessor.cpp`
- [ ] `Source/PluginEditor.h`
- [ ] `Source/PluginEditor.cpp`
- [ ] `Source/Ts9LookAndFeel.h`

### 2.2 Configuração JUCE
- [ ] `Ts9VstProject.jucer` criado
- [ ] JUCE Framework v7+ instalado
- [ ] Projucer disponível

### 2.3 Gerar Visual Studio Solution (via Projucer)

**Passos:**
1. [ ] Abrir `Ts9VstProject.jucer` com Projucer
2. [ ] Verificar JUCE path:
   - `Edit` → `Global Paths` → `JUCE modules path`
   - Path: `C:\Path\To\JUCE\modules`
3. [ ] Selecionar exporter:
   - [ ] "Visual Studio 2022"
   - [ ] Marcar "Include source"
4. [ ] Linker Configuration:
   - [ ] Adicionar Include Path: `$(ProjectDir)\..\..\Source`
   - [ ] Adicionar Lib Path: `$(ProjectDir)\..\..\RustDsp\target\release`
   - [ ] Adicionar Library: `ts9_dsp.lib`
5. [ ] Clicar "Save and Open in IDE"

**Resultado:** `Builds/VisualStudio2022/Ts9VST.sln` criado

---

## ✅ Fase 3: Compilação Visual Studio

### 3.1 Abrir Solution
```powershell
Start-Process "Builds/VisualStudio2022/Ts9VST.sln"
```

### 3.2 Linker Configuration (se não automático)
- [ ] Right-click projeto `Ts9VST`
- [ ] Properties → Configuration Properties
  - [ ] VC++ Directories:
    - Include: Add `..\..\Source`
    - Library: Add `..\..\RustDsp\target\release`
  - [ ] Linker → Input:
    - Additional Dependencies: Add `ts9_dsp.lib`

### 3.3 Build
- [ ] Select Configuration: **Release**
- [ ] Select Platform: **x64**
- [ ] Build → Build Solution (Ctrl+Shift+B)

**Expected:**
```
========== Build: 1 succeeded, 0 failed, 0 up-to-date, 0 skipped ==========
```

**Output files:**
- [ ] VST3 plugin: `Builds/VisualStudio2022/Ts9VST_VST3/Release/Ts9VST.vst3`
- [ ] Tamanho: ~2-3 MB

---

## ✅ Fase 4: Recursos & Runtime

### 4.1 Imagem de Fundo
- [ ] `image_32aca5.jpg` existe no repo

**Colocação (escolher um):**
1. [ ] Método A: Copiar para pasta do plugin
   ```powershell
   cp image_32aca5.jpg `
     "Builds/VisualStudio2022/Ts9VST_VST3/Release/"
   ```
2. [ ] Método B: Colocar em diretório fixo
   - Path: `C:\Resources\image_32aca5.jpg`
   - Editar `PluginEditor.cpp` função `loadBackgroundImage()`

### 4.2 Dependências Runtime
- [ ] Microsoft Visual C++ Redistributables 2022 instalado
  - Download: https://support.microsoft.com/en-us/help/2977003

---

## ✅ Fase 5: Instalação para DAW

### 5.1 Copiar Plugin para VST3 Directory
```powershell
$vst3Dir = "C:\Program Files\Common Files\VST3"

# Criar diretório se não existir
if (-not (Test-Path $vst3Dir)) {
    New-Item -ItemType Directory -Path $vst3Dir -Force
}

# Copiar plugin
Copy-Item `
  "Builds/VisualStudio2022/Ts9VST_VST3/Release/Ts9VST.vst3" `
  -Destination $vst3Dir
```

### 5.2 Configurar DAW
**Exemplo: Reaper**
- [ ] Options → Preferences → Plug-ins → VST3
- [ ] Add path: `C:\Program Files\Common Files\VST3`
- [ ] Actions → Refresh Plug-in List

**Exemplo: FL Studio**
- [ ] Options → File Settings → VST3 Plugins
- [ ] Add Folder: `C:\Program Files\Common Files\VST3`
- [ ] Rescan

---

## ✅ Fase 6: Validação & Testes

### 6.1 Plugin Carrega?
- [ ] Abrir DAW
- [ ] Procurar por "TS9" ou "Tube Screamer"
- [ ] Plugin aparece na lista?

**Se não:**
- [ ] Verificar console do DAW (Debug Mode)
- [ ] Conferir caminho VST3
- [ ] Tentar rescan de plugins

### 6.2 Teste de Áudio Básico
- [ ] Criar faixa de áudio nova
- [ ] Adicionar TS9 VST como plugin
- [ ] Gerar tom de teste (sine wave ~1 kHz)
- [ ] Play com plugin ativado

**Comportamento esperado:**
- [ ] Sem som com `bypass` = ON
- [ ] Som processado com `bypass` = OFF
- [ ] Som se torna mais "quente" com drive > 0.5
- [ ] Tons agudos reduzem com tone = 0.0

### 6.3 Teste de Parâmetros
```
Drive:  0.0 → 1.0
  [ ] Aumenta distorção
  [ ] Mantém amplitude controlada (level compensation)

Tone:   0.0 → 1.0
  [ ] 0.0 = som mais escuro (sem agudos)
  [ ] 1.0 = som mais brilhante (agudos presentes)
  [ ] Transição suave

Level:  0.0 → 1.0
  [ ] Controla ganho de saída
  [ ] 0.0 = bem silencioso
  [ ] 1.0 = saída máxima (~0dB)
```

### 6.4 Teste de Performance
- [ ] CPU Usage < 5% (típico para um plugin)
- [ ] Sem clicks ou glitches de áudio
- [ ] Resposta em tempo real (latência < 1ms)

---

## ❌ Troubleshooting

### Erro: "ts9_dsp.lib not found" durante link
**Solução:**
1. [ ] Verificar se `RustDsp/target/release/ts9_dsp.lib` existe
2. [ ] Se não: `cd RustDsp && cargo build --release`
3. [ ] Verificar paths em Linker configuration
4. [ ] Usar absolute path se necessário:
   ```
   C:\Users\danieo\Documents\Codes\audio_stuff\tubo-ki-grita\RustDsp\target\release\ts9_dsp.lib
   ```

### Erro: "Cannot find image_32aca5.jpg"
**Solução:**
1. [ ] Copiar imagem para mesmo diretório do VST3
2. [ ] Ou editar `PluginEditor.cpp` com path absoluto:
   ```cpp
   imageFile = juce::File("C:/full/path/to/image_32aca5.jpg");
   ```

### Erro: "Unresolved external symbol create_ts9_dsp"
**Solução:**
1. [ ] Verificar FFI declarations em `RustBridge.h`
2. [ ] Confirmar `extern "C"` correto
3. [ ] Rerecompilar biblioteca Rust em release
4. [ ] Limpar build cache C++:
   ```powershell
   rm -r Builds/VisualStudio2022/Ts9VST/Release/
   ```

### Plugin não aparece no DAW
**Checklist:**
1. [ ] Arquivo `.vst3` existe?
   ```powershell
   ls "C:\Program Files\Common Files\VST3\*.vst3"
   ```
2. [ ] DAW configurada para VST3?
   - [ ] Preferences → Plugin Path inclui VST3 dir
3. [ ] DAW fez rescan?
   - [ ] Menu → Rescan Plugins
4. [ ] Plugin é 64-bit?
   - [ ] Visual Studio: Platform = x64

### Áudio distorcido ou ausente
**Teste diagnóstico:**
1. [ ] Bypass = OFF?
2. [ ] Level > 0.3?
3. [ ] Drive não extremo (0.3-0.7)?
4. [ ] Sinal de entrada válido?

**Se ainda com problema:**
- [ ] Adicionar logging em `PluginProcessor::processBlock()`
- [ ] Verificar valores dos parâmetros
- [ ] Testar com sinal conhecido (sine, pink noise)

---

## ✅ Sign-Off Checklist

### Development
- [ ] Código Rust compila sem erros
- [ ] Código C++ compila sem erros
- [ ] VST3 plugin gerado

### Integration
- [ ] Plugin carrega em DAW
- [ ] Interface GUI visível (imagem de fundo)
- [ ] Sliders rotativos funcionam

### Audio
- [ ] Audio passa através (bypass funciona)
- [ ] Parâmetros afetam som
- [ ] Distorção audível com drive > 0.5
- [ ] Tone control altera timbre

### Performance
- [ ] CPU usage aceitável (< 5%)
- [ ] Sem artefatos de áudio
- [ ] Tempo real estável

---

## 📞 Próximos Passos

1. **Se tudo passou:**
   - [ ] Documentar configuração final
   - [ ] Criar presets de teste
   - [ ] Fazer comparação A/B com TS9 físico
   - [ ] Otimizar performance se necessário

2. **Melhorias Futuras:**
   - [ ] Implementar presets/programas
   - [ ] Adicionar visualizador de forma de onda
   - [ ] Análise FFT de resposta em frequência
   - [ ] Suporte para VST2 (se necessário)
   - [ ] Assinatura digital do plugin

---

**Última atualização:** 2024  
**Status:** ✅ Ready to Build  
