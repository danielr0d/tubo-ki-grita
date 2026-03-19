#include "PluginEditor.h"

//==============================================================================
Ts9PluginEditor::Ts9PluginEditor(Ts9PluginProcessor& p)
    : AudioProcessorEditor(&p),
      processor(p)
{
    // Definir tamanho da janela do editor baseado na imagem de fundo
    // Assumindo imagem padrão de ~400x300px (ajuste conforme necessário)
    setSize(400, 320);

    // Carregar imagem de fundo
    loadBackgroundImage();

    // Configurar sliders (controles rotativos)
    setupSliders();

    // Iniciar repaint periódico
    setOpaque(true);
}

Ts9PluginEditor::~Ts9PluginEditor()
{
    // Remover listeners
    driveSlider.removeListener(this);
    toneSlider.removeListener(this);
    levelSlider.removeListener(this);
}

//==============================================================================
void Ts9PluginEditor::paint(juce::Graphics& g)
{
    // Desenhar imagem de fundo
    if (!backgroundImage.isNull()) {
        g.drawImageAt(backgroundImage, 0, 0);
    } else {
        // Fallback se imagem não carregar
        g.fillAll(juce::Colours::darkgrey);
        g.setColour(juce::Colours::white);
        g.setFont(14.0f);
        g.drawText("TS9 Tube Screamer", getLocalBounds(), juce::Justification::centred);
    }
}

void Ts9PluginEditor::resized()
{
    // Posicionar controles rotativos sobre a imagem de fundo
    // Os controles são invisíveis (apenas interact com mouse)

    // Drive slider - posicionado sobre o botão DRIVE da imagem
    driveSlider.setBounds(
        static_cast<int>(controlLayout.drive_x - controlLayout.control_diameter / 2),
        static_cast<int>(controlLayout.drive_y - controlLayout.control_diameter / 2),
        static_cast<int>(controlLayout.control_diameter),
        static_cast<int>(controlLayout.control_diameter)
    );

    // Tone slider - posicionado sobre o botão TONE da imagem
    toneSlider.setBounds(
        static_cast<int>(controlLayout.tone_x - controlLayout.control_diameter / 2),
        static_cast<int>(controlLayout.tone_y - controlLayout.control_diameter / 2),
        static_cast<int>(controlLayout.control_diameter),
        static_cast<int>(controlLayout.control_diameter)
    );

    // Level slider - posicionado sobre o botão LEVEL da imagem
    levelSlider.setBounds(
        static_cast<int>(controlLayout.level_x - controlLayout.control_diameter / 2),
        static_cast<int>(controlLayout.level_y - controlLayout.control_diameter / 2),
        static_cast<int>(controlLayout.control_diameter),
        static_cast<int>(controlLayout.control_diameter)
    );
}

void Ts9PluginEditor::sliderValueChanged(juce::Slider* slider)
{
    // Atualizar parâmetro do processor baseado em qual slider foi alterado
    if (slider == &driveSlider) {
        processor.driveParameter->setValueNotifyingHost(static_cast<float>(driveSlider.getValue()));
    } else if (slider == &toneSlider) {
        processor.toneParameter->setValueNotifyingHost(static_cast<float>(toneSlider.getValue()));
    } else if (slider == &levelSlider) {
        processor.levelParameter->setValueNotifyingHost(static_cast<float>(levelSlider.getValue()));
    }
}

//==============================================================================
void Ts9PluginEditor::loadBackgroundImage()
{
    // Tentar carregar image_32aca5.jpg do diretório de recursos
    // IMPORTANTE: O arquivo deve estar no mesmo diretório do plugin ou em um caminho conhecido
    
    juce::File imageFile;

    // Estratégia 1: Procurar no diretório do plugin
    imageFile = juce::File::getSpecialLocation(juce::File::currentApplicationFile)
        .getParentDirectory()
        .getChildFile("image_32aca5.jpg");

    // Estratégia 2: Procurar no diretório de trabalho
    if (!imageFile.exists()) {
        imageFile = juce::File::getCurrentWorkingDirectory()
            .getChildFile("image_32aca5.jpg");
    }

    // Estratégia 3: Procurar em um diretório fixo de recursos
    if (!imageFile.exists()) {
        imageFile = juce::File("C:/Resources/image_32aca5.jpg");
    }

    // Tentar carregar a imagem
    if (imageFile.exists()) {
        backgroundImage = juce::ImageFileFormat::loadFrom(imageFile);
        
        // Se conseguiu carregar, redimensionar a janela para caber a imagem
        if (!backgroundImage.isNull()) {
            setSize(backgroundImage.getWidth(), backgroundImage.getHeight());
        }
    }

    // Se não conseguiu carregar, será desenhado um fallback em paint()
}

//==============================================================================
void Ts9PluginEditor::setupSliders()
{
    // Configurar Drive Slider
    driveSlider.setSliderStyle(juce::Slider::RotaryVerticalDrag);
    driveSlider.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
    driveSlider.setRange(0.0, 1.0, 0.01);
    driveSlider.setValue(processor.driveParameter->get());
    driveSlider.setLookAndFeel(&ts9LookAndFeel); // Usar LookAndFeel customizado
    driveSlider.addListener(this);
    addAndMakeVisible(driveSlider);

    // Configurar Tone Slider
    toneSlider.setSliderStyle(juce::Slider::RotaryVerticalDrag);
    toneSlider.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
    toneSlider.setRange(0.0, 1.0, 0.01);
    toneSlider.setValue(processor.toneParameter->get());
    toneSlider.setLookAndFeel(&ts9LookAndFeel);
    toneSlider.addListener(this);
    addAndMakeVisible(toneSlider);

    // Configurar Level Slider
    levelSlider.setSliderStyle(juce::Slider::RotaryVerticalDrag);
    levelSlider.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
    levelSlider.setRange(0.0, 1.0, 0.01);
    levelSlider.setValue(processor.levelParameter->get());
    levelSlider.setLookAndFeel(&ts9LookAndFeel);
    levelSlider.addListener(this);
    addAndMakeVisible(levelSlider);

    // Fazer sliders invisíveis para interação apenas (sem desenho próprio)
    // Isso permite que apenas a imagem de fundo seja visível
    driveSlider.setOpaque(false);
    toneSlider.setOpaque(false);
    levelSlider.setOpaque(false);

    // Inicial layout dos componentes
    resized();
}
