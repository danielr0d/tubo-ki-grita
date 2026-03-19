#pragma once

#include <juce_gui_basics/juce_gui_basics.h>
#include "PluginProcessor.h"
#include "Ts9LookAndFeel.h"

//==============================================================================
/// Editor visual do plugin TS9 Tube Screamer
/// Carrega a imagem image_32aca5.jpg como fundo e posiciona controles rotativos
//==============================================================================
class Ts9PluginEditor : public juce::AudioProcessorEditor,
                       private juce::Slider::Listener
{
public:
    explicit Ts9PluginEditor(Ts9PluginProcessor&);
    ~Ts9PluginEditor() override;

    //==============================================================================
    void paint(juce::Graphics&) override;
    void resized() override;

    void sliderValueChanged(juce::Slider* slider) override;

private:
    Ts9PluginProcessor& processor;

    // LookAndFeel customizado para sliders
    Ts9LookAndFeel ts9LookAndFeel;

    // Imagem de fundo
    juce::Image backgroundImage;

    // Controles rotativos invisíveis posicionados sobre a imagem
    juce::Slider driveSlider;
    juce::Slider toneSlider;
    juce::Slider levelSlider;

    // Posições e tamanhos dos controles em pixels (relativos à imagem)
    struct ControlLayout {
        float drive_x = 80.0f;      // Posição X do botão DRIVE na imagem
        float drive_y = 60.0f;      // Posição Y do botão DRIVE na imagem
        float control_diameter = 50.0f; // Diâmetro do controle rotativo

        float tone_x = 180.0f;      // Posição X do botão TONE
        float tone_y = 60.0f;       // Posição Y do botão TONE

        float level_x = 280.0f;     // Posição X do botão LEVEL
        float level_y = 60.0f;      // Posição Y do botão LEVEL
    } controlLayout;

    /// Carrega a imagem de fundo (image_32aca5.jpg)
    void loadBackgroundImage();

    /// Configura os sliders (posição, range, visual)
    void setupSliders();

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(Ts9PluginEditor)
};
