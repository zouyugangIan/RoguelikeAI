# Roguelike AI：Triumvirate (纯 Rust 版) - 技术规格说明书 (SPEC) v2.1

## 1. 架构终极裁决：为何回归 Dioxus？
在“极致性能 (Leptos)”与“工程落地 (Dioxus)”之间，我们经过深度评估，**最终选择 Dioxus**。

### 1.1 深度对比分析 (Decision Record)
*   **开发效率 (DX)**：
    *   **Dioxus**: 采用 `rsx!` 宏，极似 React。对于本项目“AI 辅助教学”的目标，Dioxus 的代码结构更易于被 LLM 理解和生成，也更易于用户（通常懂一点 React）上手。
    *   **Leptos**: 采用 Signals。虽然性能强，但心智负担重（闭包克隆、生命周期管理复杂）。
*   **真实性能瓶颈**：
    *   AI 应用的瓶颈在于 **GPU 推理速度 (Tokens/s)**，通常为 20-50 TPS。Dioxus 的 V-DOM 处理这个频率的更新**绰绰有余**（它可以轻松跑 60FPS 游戏）。Leptos 的微秒级优势在此场景下感知不强，属于“过早优化”。
*   **生态稳定性**：
    *   Dioxus 拥有官方的桌面端支持 (`dioxus-desktop`) 和更成熟的组件生态。

**结论**：为了保证项目的**长期可维护性**和**开发速度**，Dioxus 是综合评分最高的选择。

---

## 2. 最终技术栈 (The Frozen Stack)

| 层级 | 技术组件 | 角色与选型理由 |
| :--- | :--- | :--- |
| **App Shell** | **Tauri v2** | 跨平台外壳。提供系统级能力（文件读写、系统托盘）。 |
| **前端 (UI)** | **Dioxus (Web)** | 编译为 WASM。提供组件化 UI 开发体验，生态最成熟。 |
| **样式** | **TailwindCSS** | 现代原子化 CSS，保证界面“精美”。 |
| **后端 (Core)** | **Rust Native** | 负责模型加载与推理。通过 Tauri Command 与前端通信。 |
| **AI 引擎** | **Candle** | 去 Python 化的关键。Rust 原生推理。 |

---

## 3. 硬件适配策略

### 3.1 开发环境 (32GB RAM + 3080)
*   运行完整版 Llama-3-8B-Instruct (Q8)。
*   启用完整的“本地训练循环” (Local Training Loop)。

### 3.2 用户环境 (最低 8GB RAM)
*   运行量化版 Llama-3-8B (Q4_K_M)。
*   仅保留推理功能，禁用训练模块以节省资源。

---

## 4. 项目结构 (Project Structure)

```
c:\BW\develop\RoguelikeAI\
├── src-tauri/              # [后端] Rust 主进程
│   ├── src/
│   │   ├── main.rs         # 入口
│   │   ├── ai_engine.rs    # Candle 推理逻辑
│   │   └── commands.rs     # Tauri 指令
│   ├── Cargo.toml
│   └── capabilities/
├── src-ui/                 # [前端] Dioxus WASM
│   ├── src/
│   │   ├── main.rs         # UI 入口
│   │   ├── components/     # 卡牌、聊天框
│   │   └── routes.rs       # 路由配置
│   ├── Cargo.toml
│   └── Dioxus.toml         # 打包配置
├── models/                 # 模型文件 (.gguf)
├── SPEC.md                 # 本文档
└── Cargo.toml              # Workspace 根配置
```
