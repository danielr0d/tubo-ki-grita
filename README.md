# TS9 Tube Screamer VST3 Plugin
## Emulação de Alta Fidelidade do Pedal Ibanez TS9

### Estrutura do Projeto

```
tubo-ki-grita/
├── RustDsp/                    # Módulo DSP em Rust
│   ├── Cargo.toml             # Configuração Rust
│   └── src/
│       ├── lib.rs            # Interface FFI principal
│       └── dsp/
│           ├── mod.rs        # Exportação de módulos
│           ├── oversampler.rs # Oversampler 2x
│           ├── clipping.rs   # Soft clipping
│           ├── filters.rs    # Filtros (mid-hump, tone control)
│           └── gain_stage.rs # Controle de ganho (drive, level)
├── Source/                     # Código C++ (JUCE)
│   ├── RustBridge.h          # FFI declarations
│   ├── PluginProcessor.h      # Processador de áudio
│   ├── PluginProcessor.cpp
│   ├── PluginEditor.h         # GUI do plugin
│   └── PluginEditor.cpp
├── image_32aca5.jpg            # Imagem de fundo da interface
└── notas.md                    # Especificação original
```

### Pré-requisitos

1. **Visual Studio Community 2026+** (com suporte a C++)
2. **Rust Toolchain** - instalado e configurado para MSVC
   ```powershell
   rustup default stable-msvc
   ```
3. **JUCE Framework v7+** - para compilação do plugin
4. **CMake** (opcional, dependendo da configuração de build)

### Compilação do Módulo Rust

```powershell
cd RustDsp
cargo build --release
```

Isso gera `RustDsp/target/release/ts9_dsp.lib` que será linkado ao projeto C++.

### Compilação do Plugin C++ (JUCE)

1. Abrir `Ts9VstProject.jucer` no Projucer (JUCE)
2. Configurar as paths:
   - Lincar a biblioteca Rust: `RustDsp/target/release/ts9_dsp.lib`
   - Include Rust FFI: `Source/RustBridge.h`
3. Gerar projeto Visual Studio
4. Abrir e compilar em Visual Studio

### Colocação da Imagem

- Copiar `image_32aca5.jpg` para:
  - Mesmo diretório do executável do plugin
  - Ou especificar path em `PluginEditor.cpp` (função `loadBackgroundImage()`)

### Arquitetura DSP Rust

#### Pipeline de Processamento (ordem):
1. **Ganho de Entrada (Drive)** - Amplifica o sinal baseado no parâmetro Drive
2. **Filtro Mid-Hump** - Cria pico em ~720 Hz (caractere do TS9)
3. **Soft Clipping com Oversampling 2x** - Distorção suave tipo diodo
4. **Filtro Tone Control** - Controle de tons ajustável (1.5k - 7k Hz)
5. **Ganho de Saída (Level)** - Controla volume final

#### Principais Características:
- **Oversampling 2x**: Reduz aliasing na distorção não-linear
- **Soft Clipping Polinomial**: Emula comportamento de diodo silício
- **Filtros Biquad**: Resposta em frequência precisa
- **Simétrico**: Clipping funciona igualmente em sinais positivos e negativos

### Parâmetros do Plugin

| Parâmetro | Range | Descrição |
|-----------|-------|-----------|
| **Drive** | 0.0 - 1.0 | Ganho de entrada (0dB a +34dB) |
| **Tone** | 0.0 - 1.0 | Controle de agudos (1.5k - 7k Hz) |
| **Level** | 0.0 - 1.0 | Ganho de saída (-20dB a 0dB) |
| **Bypass** | On/Off | Desativa o efeito |

### Próximos Passos (Melhorias Futuras)

- [ ] LookAndFeel customizado para sliders com visual de knobs
- [ ] Presets/Programas salvos
- [ ] Extensão para VST2 (se necessário)
- [ ] Análise de resposta em frequência e benchmarks
- [ ] Validação de aliasing com análise FFT
- [ ] Suporte a AAX (Pro Tools)
- [ ] Testes auditivos contra TS9 físico

### Troubleshooting

**Erro: "DLL não encontrada" na inicialização**
- Garantir que `ts9_dsp.lib` foi compilado em modo release
- Verificar que Rust foi compilado com target MSVC

**Sem som**
- Verificar que `bypass` está desativado
- Testar com Level em 0.5 e acima

**Distorção digitais**
- Aumentar oversampling de 2x para 4x em `oversampler.rs`
- Reduzir ganho de Drive

### Contato & Desenvolvimento

Este é um projeto de pesquisa em emulação de áudio. Para mais informações sobre TS9, ver:
- Ibanez TS9 Service Notes
- Schematic disponível em http://www.electrosmash.com

---

**Versão**: 0.1.0  
**Data**: 2024  
**Linguagens**: Rust + C++ (JUCE)
