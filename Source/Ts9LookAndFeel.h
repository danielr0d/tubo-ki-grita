#pragma once

#include <juce_gui_basics/juce_gui_basics.h>

//==============================================================================
/// LookAndFeel customizado para sliders rotativos do TS9
/// Desenha knobs invisíveis com feedback visual sutil
//==============================================================================
class Ts9LookAndFeel : public juce::LookAndFeel_V4
{
public:
    Ts9LookAndFeel()
    {
        // Cores customizadas
        setColour(juce::Slider::rotarySliderFillColourId, juce::Colour(0xff00ff00));    // Verde luminoso
        setColour(juce::Slider::rotarySliderOutlineColourId, juce::Colour(0xff000000)); // Preto
        setColour(juce::Slider::thumbColourId, juce::Colour(0xffcccccc));              // Cinza claro
    }

    //==============================================================================
    void drawRotarySlider(juce::Graphics& g, int x, int y, int width, int height,
                         float sliderPos, const float rotaryStartAngle,
                         const float rotaryEndAngle, juce::Slider& slider) override
    {
        auto radius = (float)juce::jmin(width / 2, height / 2) - 2.0f;
        auto centreX = (float)x + (float)width * 0.5f;
        auto centreY = (float)y + (float)height * 0.5f;
        auto rx = centreX - radius;
        auto ry = centreY - radius;
        auto rw = radius * 2.0f;
        auto angle = rotaryStartAngle + sliderPos * (rotaryEndAngle - rotaryStartAngle);

        // Desenhar fundo do círculo (muito sutil)
        g.setColour(juce::Colour(0x33ffffff)); // Branco semi-transparente
        g.fillEllipse(rx, ry, rw, rw);

        // Desenhar arco que indica posição
        juce::Path arcPath;
        arcPath.addCentredArc(centreX, centreY, radius, radius, 0.0f,
                             rotaryStartAngle, angle, true);

        g.setColour(juce::Colour(0xff00dd00)); // Verde brilho
        g.strokePath(arcPath, juce::PathStrokeType(3.0f, juce::PathStrokeType::curved,
                                                   juce::PathStrokeType::rounded));

        // Desenhar indicador de posição (linha pequena)
        juce::Path indicatorPath;
        auto indicatorLength = radius * 0.7f;
        auto indicatorX = centreX + indicatorLength * std::cos(angle - juce::MathConstants<float>::halfPi);
        auto indicatorY = centreY + indicatorLength * std::sin(angle - juce::MathConstants<float>::halfPi);

        indicatorPath.startNewSubPath(centreX, centreY);
        indicatorPath.lineTo(indicatorX, indicatorY);

        g.setColour(juce::Colours::white);
        g.strokePath(indicatorPath, juce::PathStrokeType(2.0f,
                                                         juce::PathStrokeType::curved,
                                                         juce::PathStrokeType::rounded));

        // Desenhar borda
        g.setColour(slider.findColour(juce::Slider::rotarySliderOutlineColourId));
        g.drawEllipse(rx, ry, rw, rw, 1.5f);
    }

private:
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(Ts9LookAndFeel)
};
