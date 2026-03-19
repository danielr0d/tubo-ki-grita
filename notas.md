# Projeto VST: Ibanez TS9 "Tube Screamer" Emulação

Este arquivo Markdown fornece as instruções e a estrutura do projeto para o GitHub Copilot criar um plugin VST3 de alta fidelidade que emula o som e a aparência do pedal Ibanez TS9 Tube Screamer, utilizando a imagem fornecida (`image_32aca5.jpg`) como interface.

## Visão Geral do Projeto

O objetivo é criar um plugin VST para Windows (Visual Studio / C++) que reproduza fielmente o timbre clássico do TS9.

**Recursos Principais:**
* **Emulação DSP Avançada:** Foco no "mid-hump" característico, recorte de diodo simétrico/suave e controle de tom do circuito analógico original.
* **Arquitetura Híbrida (C++ / Rust):** O núcleo de processamento de áudio será um módulo Rust compilado como uma biblioteca estática, integrado ao C++ via FFI (Foreign Function Interface). O objetivo é unir a segurança de memória e performance do Rust com a estrutura de plugins do C++.
* **GUI Dinâmica Baseada em Imagem:** A imagem `image_32aca5.jpg` será usada como o background completo do VST. Controles rotativos de interface de usuário (Knobs) e um footswitch serão sobrepostos de forma invisível ou estilizada em posições precisas sobre a imagem original para controlar os parâmetros (Drive, Tone, Level e Bypass).

## Pré-requisitos e Ferramentas

Para construir este projeto no Visual Studio, precisaremos de:

1.  **IDE:** Microsoft Visual Studio (com suporte a C++).
2.  **Framework VST:** JUCE (v7+) (Recomendado para facilitar a criação da GUI e o VST wrapper).
3.  **Linguagens:** C++ (para JUCE, GUI e host communication) e Rust (para DSP).
4.  **Rust Toolchain:** Instalado e configurado para compilar bibliotecas estáticas compatíveis com C (target `*-pc-windows-msvc`).

## Estrutura do Projeto

* `/Ts9VstProject` (Raiz do projeto)
    * `Source/` (C++ JUCE)
        * `PluginProcessor.cpp/.h` (Gerencia o áudio e os parâmetros do host)
        * `PluginEditor.cpp/.h` (Gerencia a GUI, carrega `image_32aca5.jpg`, desenha e posiciona os knobs virtuais)
        * `BinaryData.cpp/.h` (Gerado pelo JUCE para embutir a imagem `image_32aca5.jpg`)
        * `RustBridge.h` (Declarações C-compatíveis para acessar as funções exportadas pelo Rust)
    * `RustDsp/` (Módulo DSP em Rust)
        * `Cargo.toml` (Configuração para compilar como `staticlib`)
        * `src/lib.rs` (Expõe a API FFI `extern "C"` e gerencia o estado global do DSP)
        * `src/dsp/`
            * `oversampler.rs` (Oversampling para evitar aliasing na distorção)
            * `clipping.rs` (Modelagem matemática do recorte suave de diodo)
            * `filters.rs` (Filtros: Mid-hump pré-distorção e o Tone Control pós-distorção)
            * `gain_stage.rs` (Gerenciamento de ganho do Drive e Level)

## Conceitos Chave de DSP (Implementação em Rust)

Para emular o TS9 de forma realista, a implementação em Rust deve focar nestes aspectos:

1.  **Oversampling:**
    * Implementar sobreamostragem polifásica (2x ou 4x). O processamento de distorção não-linear (clipping) deve ocorrer nesta taxa mais alta para reduzir o aliasing harmônico.
2.  **O "Mid-Hump" (O Segredo do TS9):**
    * O TS9 possui um filtro passa-alta e passa-baixa antes do estágio de clipagem que cria um pico característico (em torno de 720 Hz).
    * Implementar os filtros RC equivalentes ou um filtro de pico para moldar o sinal antes de distorcer.
3.  **Symmetrical Soft Diode Clipping:**
    * O TS9 usa dois diodos de silício na malha de feedback do op-amp.
    * Utilizar uma função de transferência suave. Em vez de um simples `tanh`, tentar uma aproximação polinomial ou resolução de equação não linear (como Newton-Raphson para a equação de Shockley do diodo) para um timbre mais autêntico.
4.  **Tone Control Circuit:**
    * O controle de tom é um filtro passa-baixas ativo que interage com os médios e agudos.
    * Implementar um filtro biquad com resposta ajustável baseada no valor do knob "TONE".

## Arquitetura da Ponte Rust <-> C++ (FFI)

O módulo Rust deve expor funções que o C++ possa chamar durante o loop de áudio.

**Exemplo de API esperada em `RustBridge.h`:**
```cpp
extern "C" {
    typedef struct Ts9DspContext Ts9DspContext;
    
    Ts9DspContext* create_ts9_dsp(float sample_rate);
    void destroy_ts9_dsp(Ts9DspContext* dsp);
    void process_ts9_block(
        Ts9DspContext* dsp,
        const float** inputs,
        float** outputs,
        int num_channels,
        int num_samples,
        float drive,
        float tone,
        float level,
        bool is_bypassed
    );
}