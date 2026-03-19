#!/usr/bin/env python3
"""
Script de teste para validação do plugin TS9 VST
Verifica integridade dos arquivos gerados e estrutura do projeto
"""

import os
import sys
from pathlib import Path

def check_file_exists(filepath, description):
    """Verifica se arquivo existe e exibe status"""
    exists = Path(filepath).exists()
    status = "✅" if exists else "❌"
    print(f"  {status} {description}")
    return exists

def check_rust_module(base_path):
    """Valida estrutura do módulo Rust"""
    print("\n📦 Verificando Módulo Rust DSP...")
    
    files = [
        ("Cargo.toml", "Configuração Cargo"),
        ("src/lib.rs", "FFI Principal"),
        ("src/dsp/mod.rs", "Módulo DSP (exports)"),
        ("src/dsp/oversampler.rs", "Oversampler 2x"),
        ("src/dsp/clipping.rs", "Soft Clipping"),
        ("src/dsp/filters.rs", "Filtros (mid-hump, tone control)"),
        ("src/dsp/gain_stage.rs", "Estágio de ganho"),
    ]
    
    all_ok = True
    for file, desc in files:
        path = os.path.join(base_path, "RustDsp", file)
        if not check_file_exists(path, desc):
            all_ok = False
    
    return all_ok

def check_cpp_module(base_path):
    """Valida estrutura do módulo C++"""
    print("\n🎛️  Verificando Módulo C++ (JUCE)...")
    
    files = [
        ("RustBridge.h", "FFI Bridge Header"),
        ("PluginProcessor.h", "Processor Header"),
        ("PluginProcessor.cpp", "Processor Implementation"),
        ("PluginEditor.h", "Editor Header"),
        ("PluginEditor.cpp", "Editor Implementation"),
        ("Ts9LookAndFeel.h", "Custom Look and Feel"),
    ]
    
    all_ok = True
    for file, desc in files:
        path = os.path.join(base_path, "Source", file)
        if not check_file_exists(path, desc):
            all_ok = False
    
    return all_ok

def check_documentation(base_path):
    """Valida documentação"""
    print("\n📚 Verificando Documentação...")
    
    docs = [
        ("README.md", "README Principal"),
        ("BUILD.md", "Instruções de Build"),
        ("INTEGRATION_GUIDE.md", "Guia de Integração"),
        ("BUILD_CHECKLIST.md", "Checklist de Build"),
        ("PROJECT_STATUS.md", "Status do Projeto"),
        ("QUICKSTART.md", "Quick Start Guide"),
    ]
    
    all_ok = True
    for file, desc in docs:
        path = os.path.join(base_path, file)
        if not check_file_exists(path, desc):
            all_ok = False
    
    return all_ok

def check_config_files(base_path):
    """Valida arquivos de configuração"""
    print("\n⚙️  Verificando Configuração...")
    
    configs = [
        ("Ts9VstProject.jucer", "Configuração JUCE"),
        ("RustDsp/Cargo.toml", "Cargo.toml"),
    ]
    
    all_ok = True
    for file, desc in configs:
        path = os.path.join(base_path, file)
        if not check_file_exists(path, desc):
            all_ok = False
    
    return all_ok

def validate_file_content(filepath, keywords, description):
    """Valida se arquivo contém keywords esperadas"""
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            missing = []
            for keyword in keywords:
                if keyword not in content:
                    missing.append(keyword)
            
            if missing:
                print(f"    ⚠️  {description}: faltam {len(missing)} keywords")
                return False
            else:
                print(f"    ✅ {description}")
                return True
    except Exception as e:
        print(f"    ❌ Erro lendo {filepath}: {e}")
        return False

def validate_source_code(base_path):
    """Valida conteúdo dos arquivos de código"""
    print("\n🔍 Validando Conteúdo de Código...")
    
    tests = [
        (
            "RustDsp/src/lib.rs",
            ["extern \"C\"", "Ts9DspContext", "process_ts9_block", "create_ts9_dsp"],
            "FFI declarations"
        ),
        (
            "RustDsp/src/dsp/clipping.rs",
            ["SoftClipping", "soft_clip_cubic", "process"],
            "Soft clipping implementation"
        ),
        (
            "RustDsp/src/dsp/filters.rs",
            ["BiquadFilter", "MidHumpFilter", "ToneControlFilter"],
            "Filtros Biquad"
        ),
        (
            "Source/PluginProcessor.cpp",
            ["processBlock", "process_ts9_block", "driveParameter"],
            "Processor implementation"
        ),
        (
            "Source/PluginEditor.cpp",
            ["loadBackgroundImage", "setupSliders", "paint"],
            "Editor implementation"
        ),
    ]
    
    all_ok = True
    for filepath, keywords, desc in tests:
        full_path = os.path.join(base_path, filepath)
        if not validate_file_content(full_path, keywords, desc):
            all_ok = False
    
    return all_ok

def check_rust_compilation_status(base_path):
    """Verifica status de compilação Rust"""
    print("\n🔨 Status de Compilação Rust...")
    
    lib_path = os.path.join(base_path, "RustDsp/target/release/ts9_dsp.lib")
    
    if Path(lib_path).exists():
        size = os.path.getsize(lib_path) / (1024 * 1024)
        print(f"  ✅ ts9_dsp.lib compilado ({size:.1f} MB)")
        return True
    else:
        print(f"  ⏳ ts9_dsp.lib não compilado ainda")
        print(f"     Execute: cd RustDsp && cargo build --release")
        return False

def main():
    """Executa validação completa"""
    print("\n" + "=" * 60)
    print("🔧 VALIDADOR DE ESTRUTURA - TS9 VST Plugin")
    print("=" * 60)
    
    # Determinar base path
    base_path = os.path.dirname(os.path.abspath(__file__))
    if not os.path.exists(os.path.join(base_path, "RustDsp")):
        base_path = os.getcwd()
    
    print(f"\n📍 Base Path: {base_path}")
    
    # Executar validações
    results = {
        "Módulo Rust": check_rust_module(base_path),
        "Módulo C++": check_cpp_module(base_path),
        "Documentação": check_documentation(base_path),
        "Configuração": check_config_files(base_path),
        "Conteúdo de Código": validate_source_code(base_path),
        "Compilação Rust": check_rust_compilation_status(base_path),
    }
    
    # Sumário
    print("\n" + "=" * 60)
    print("📊 SUMÁRIO DE VALIDAÇÃO")
    print("=" * 60)
    
    passed = sum(1 for v in results.values() if v)
    total = len(results)
    
    for section, result in results.items():
        status = "✅ PASS" if result else "⚠️  PENDING/FAIL"
        print(f"  {status:15} {section}")
    
    print(f"\n  Total: {passed}/{total} validações passaram")
    
    # Recomendações
    print("\n" + "=" * 60)
    print("📝 PRÓXIMOS PASSOS")
    print("=" * 60)
    
    if results["Compilação Rust"]:
        print("  1. ✅ Rust compilado")
        print("  2. ⏳ Abrir Ts9VstProject.jucer com Projucer")
        print("  3. ⏳ Gerar Visual Studio solution")
        print("  4. ⏳ Build em Visual Studio")
    else:
        print("  1. ⏳ Compilar Rust: cd RustDsp && cargo build --release")
        print("  2. ⏳ Abrir Ts9VstProject.jucer com Projucer")
        print("  3. ⏳ Gerar Visual Studio solution")
        print("  4. ⏳ Build em Visual Studio")
    
    print("\n" + "=" * 60)
    
    if passed == total and results["Compilação Rust"]:
        print("🎉 TUDO PRONTO PARA COMPILAÇÃO COMPLETA!")
    elif passed == total:
        print("⚠️  Estrutura OK, falta compilar Rust DSP")
    else:
        print(f"❌ Faltam {total - passed} validações")
    
    print("=" * 60 + "\n")
    
    return 0 if passed == total else 1

if __name__ == "__main__":
    sys.exit(main())
