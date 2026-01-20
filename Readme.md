# Roguelike AI：Project "Triumvirate" (Generative AI Forge)

> **定位**：一个开源的 AI 全栈开发与学习环境 (The AI Workshop)。
> **目标**：主要用于**辅助开发者构建更聪明的 AI**，同时作为一个“实战道场”，教授 2026 年最重要的 AI 工程化技能 (Edge AI, Quantization, Evaluation)。

## 1. 核心理念
这个 AI 不仅仅是给你答案，它教你**如何制造答案**。
它提供了一个完整的**本地闭环 (Local Loop)**，将通常分散在 Python 脚本、HuggingFace 和云端 API 中的流程，整合进一个纯 Rust 构建的高性能环境中。

## 2. 三大支柱 (The Trinity)
*   **导师 (The Mentor)**: 辅助你写代码、重构架构。利用 RAG 技术，它比通用 AI 更懂 Rust 和现代 AI 栈。
*   **工坊 (The Forge)**: 你的本地训练中心。数据清洗、LoRA 微调、模型量化，全流程可视化管理。利用 32GB 内存压榨出硬件的最大潜力。
*   **竞技场 (The Arena)**: 自动化测试与 CI/CD。通过“LLM 打分”和“WASM 性能基准测试”，客观评估你的 AI 是否变强了。

## 3. 技术栈 (Pure Rust + WASM)
我们选择这条路，是为了展示**极致性能**与**端侧未来**。
*   **后端**: Rust (Tauri v2 + Candle) - 告别 Python 依赖地狱，享受类型安全与极速推理。
*   **前端**: Dioxus (WASM) - 同样的 Rust 代码，既是桌面应用，也是 Web 应用。
*   **部署**: 一键打包。你的高配开发机生成的模型，经过量化后，可以轻松跑在用户的低配笔记本上。

## 4. 你将学到的 2026 核心技能
1.  **Rust for AI**: 不要只做 Python 调包侠。深入底层，用 Candle/Burn 手写推理引擎。
2.  **WebAssembly Edge AI**: 如何将大模型塞进浏览器，利用 WebGPU 加速。
3.  **AI CI/CD**: 像测试代码一样测试“智能”。建立自动化评估流水线。
4.  **Local Fine-tuning**: 掌握数据为王的真理。如何用少量高质量数据让小模型超越大模型。
