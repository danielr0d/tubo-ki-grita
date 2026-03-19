#include "PluginProcessor.h"
#include "PluginEditor.h"

//==============================================================================
Ts9PluginProcessor::Ts9PluginProcessor()
    : dspContext(nullptr),
      currentSampleRate(44100.0)
{
    // Criar parâmetros do plugin
    addParameter(driveParameter = new juce::AudioParameterFloat(
        "drive",
        "Drive",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.5f
    ));

    addParameter(toneParameter = new juce::AudioParameterFloat(
        "tone",
        "Tone",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.5f
    ));

    addParameter(levelParameter = new juce::AudioParameterFloat(
        "level",
        "Level",
        juce::NormalisableRange<float>(0.0f, 1.0f, 0.01f),
        0.5f
    ));

    addParameter(bypassParameter = new juce::AudioParameterBool(
        "bypass",
        "Bypass",
        false
    ));

    // Inicializar contexto DSP
    dspContext = create_ts9_dsp(static_cast<float>(currentSampleRate));
}

Ts9PluginProcessor::~Ts9PluginProcessor()
{
    // Destruir contexto DSP ao desalocar plugin
    if (dspContext != nullptr) {
        destroy_ts9_dsp(dspContext);
        dspContext = nullptr;
    }
}

//==============================================================================
void Ts9PluginProcessor::prepareToPlay(double sampleRate, int samplesPerBlock)
{
    currentSampleRate = sampleRate;
    
    // Atualizar sample rate no contexto DSP
    set_sample_rate(dspContext, static_cast<float>(sampleRate));
    
    // Alocar buffer de processamento
    processBuffer.resize(samplesPerBlock);
}

void Ts9PluginProcessor::releaseResources()
{
    // Limpar buffers se necessário
    processBuffer.clear();
}

bool Ts9PluginProcessor::isBusesLayoutSupported(const BusesLayout& layouts) const
{
    // Suportar mono e estéreo
    if (layouts.getMainOutputChannelSet() != juce::AudioChannelSet::mono()
        && layouts.getMainOutputChannelSet() != juce::AudioChannelSet::stereo())
        return false;

    if (layouts.getMainOutputChannelSet() != layouts.getMainInputChannelSet())
        return false;

    return true;
}

void Ts9PluginProcessor::processBlock(juce::AudioBuffer<float>& buffer, juce::MidiBuffer& midiMessages)
{
    juce::ScopedNoDenormals noDenormals;
    auto totalNumInputChannels = getTotalNumInputChannels();
    auto totalNumOutputChannels = getTotalNumOutputChannels();
    auto numSamples = buffer.getNumSamples();

    // Limpar canais de saída desnecessários
    for (auto i = totalNumInputChannels; i < totalNumOutputChannels; ++i)
        buffer.clear(i, 0, numSamples);

    // Obter valores dos parâmetros
    float drive = driveParameter->get();
    float tone = toneParameter->get();
    float level = levelParameter->get();
    bool isBypassed = bypassParameter->get();

    // Garantir que temos buffers de entrada e saída válidos
    if (dspContext == nullptr || totalNumInputChannels == 0)
        return;

    // Processar cada canal
    for (int channel = 0; channel < totalNumInputChannels; ++channel)
    {
        auto* channelData = buffer.getWritePointer(channel);

        // Preparar ponteiros para entrada e saída
        const float* inputs[] = { buffer.getReadPointer(channel) };
        float* outputs[] = { channelData };

        // Chamar função DSP do Rust
        process_ts9_block(
            dspContext,
            inputs,
            outputs,
            1,  // num_channels = 1 (processamos um por vez)
            numSamples,
            drive,
            tone,
            level,
            isBypassed
        );
    }
}

//==============================================================================
bool Ts9PluginProcessor::hasEditor() const
{
    return true;
}

juce::AudioProcessorEditor* Ts9PluginProcessor::createEditor()
{
    return new Ts9PluginEditor(*this);
}

//==============================================================================
const juce::String Ts9PluginProcessor::getName() const
{
    return JucePlugin_Name;
}

bool Ts9PluginProcessor::acceptsMidi() const
{
    return false;
}

bool Ts9PluginProcessor::producesMidi() const
{
    return false;
}

bool Ts9PluginProcessor::isMidiEffect() const
{
    return false;
}

double Ts9PluginProcessor::getTailLengthSeconds() const
{
    return 0.0;
}

//==============================================================================
int Ts9PluginProcessor::getNumPrograms()
{
    return 1;
}

int Ts9PluginProcessor::getCurrentProgram()
{
    return 0;
}

void Ts9PluginProcessor::setCurrentProgram(int index)
{
    juce::ignoreUnused(index);
}

const juce::String Ts9PluginProcessor::getProgramName(int index)
{
    juce::ignoreUnused(index);
    return {};
}

void Ts9PluginProcessor::changeProgramName(int index, const juce::String& newName)
{
    juce::ignoreUnused(index, newName);
}

//==============================================================================
void Ts9PluginProcessor::getStateInformation(juce::MemoryBlock& destData)
{
    // Você pode implementar save/load de preset aqui
    auto state = parameters.copyState();
    std::unique_ptr<juce::XmlElement> xml(state.createXml());
    copyXmlToBinary(*xml, destData);
}

void Ts9PluginProcessor::setStateInformation(const void* data, int sizeInBytes)
{
    std::unique_ptr<juce::XmlElement> xmlState(getXmlFromBinary(data, sizeInBytes));
    if (xmlState.get() != nullptr)
        if (xmlState->hasTagName(parameters.state.getType()))
            parameters.replaceState(juce::ValueTree::fromXml(*xmlState));
}

//==============================================================================
juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new Ts9PluginProcessor();
}
