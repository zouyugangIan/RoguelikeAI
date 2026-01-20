# 2026 AI 全栈工程师 (Rust + LLM) 实战演练路线图 v2.0

> **文档定位**：这是一份精确到“章节”和“代码行”的作战地图。
> **目标用户**：拥有编程基础，希望通过 3-6 个月高强度训练，掌握构建高性能端侧 AI 系统能力的工程师。

---

## 阶段一：Rust 系统级重塑 (The System Architect)
**周期**：4-6 周
**目标**：不仅会写 Rust，还要能写出“零拷贝”的高性能服务。

### 1. 语言核心：《The Rust Programming Language》
*   **深度阅读指南**：
    *   **Chapter 4 (Ownership)**: 必须手画出内存堆栈图，理解 `Move` vs `Copy`。
    *   **Chapter 10 (Generics & Traits)**: 学会如何定义“行为”而非“数据”。这是写 AI 推理引擎泛型的基础。
    *   **Chapter 13 (Iterators & Closures)**: 函数式编程思维，对处理大规模数据集至关重要。
    *   **Chapter 15 (Smart Pointers)**: `Box<T>`, `Rc<T>`, `RefCell<T>`。不懂这个就看不懂神经网络的图结构实现。
*   **验收项目**：**命令行 grep 工具** (复刻书中的 minigrep，但要求支持正则表达式)。

### 2. 工程化落地：《Zero To Production In Rust》
*   **核心技能点**：
    *   **TDD (测试驱动开发)**: 如何在 Rust 中写集成测试。
    *   **Error Handling**: 抛弃 `unwrap()`，学会使用 `thiserror` 和 `anyhow` 设计错误树。
    *   **Async Runtime**: 深入理解 `tokio` 的任务调度，这对高并发推理服务至关重要。
*   **验收项目**：**邮件订阅微服务**。要求：部署在 Docker 中，包含完整的结构化日志 (Tracing) 和健康检查接口。

### 3. 高性能进阶：《Rust for Rustaceans》
*   **技术深潜**：
    *   **GATs (Generic Associated Types)**: 现在的 Rust 异步库都在用这个。
    *   **Macros (声明式与过程宏)**: 学会写 `derive` 宏，这是通过一两行代码自动为 Struct 生成 JSON 序列化或数据库 schema 的魔法。
*   **验收项目**：**自定义过程宏**。写一个 `#[derive(MyJson)]`，自动为结构体生成 `to_json()` 方法。

---

## 阶段二：神经网络直觉构建 (The AI Intuition)
**周期**：4 周
**目标**：脱离框架（PyTorch），用纯数学和代码理解 AI 是如何“思考”的。

### 1. 启蒙必修：Andrej Karpathy's "Zero to Hero"
*   **课程拆解**：
    *   **Part 1: MicroGrad**: 也就是“反向传播 (Backpropagation)”的本质。
        *   *关键任务*: 用 Python 从零实现 `Val` 类，支持 `+`, `*`, `pow`，并自动计算梯度。
        *   *检验标准*: 能训练一个简单的神经元拟合 `y=x^2`。
    *   **Part 2: Makemore**: 语言模型入门。
        *   *关键任务*: 理解 Bigram, MLP, RNN, LSTM, GRU 的演进。手写 Batch Normalization。
    *   **Part 5: GPT**: Transformer 的诞生。
        *   *关键任务*: 手写 Multi-Head Attention 机制。这是 2026 年所有大模型的心脏。
*   **验收项目**：**MiniGPT**。基于莎士比亚文集训练一个能以此风格说话的小模型（约 1000 行代码以内）。

### 2. 框架实战：《Deep Learning for Coders》 (FastAI)
*   **核心技能点**：
    *   **Data Block API**: 如何高效加载图片/文本数据。
    *   **Learning Rate Finder**: 如何科学地设置学习率，而不是瞎猜。
*   **验收项目**：**猫狗分类器 Web 版**。训练一个 ResNet 模型，导出为 ONNX，并写一个简单的 Python 网页供上传识别。

---

## 阶段三：LLM 工程化与架构 (The AI Engineer)
**周期**：4-6 周
**目标**：掌握大模型的全生命周期管理——训练、微调、评估、部署。

### 1. 模型微调：QLoRA 论文精读与实战
*   **资源**："QLoRA: Efficient Finetuning of Quantized LLMs"
*   **技术深潜**：
    *   **Quantization**: 理解 FP16, INT8, NF4 数据类型的区别。
    *   **LoRA (Low-Rank Adaptation)**: 为什么只训练 1% 的参数就能改变模型行为？
*   **验收项目**：**个人口癖克隆**。收集 500 条你的微信/聊天记录，微调 Llama-3-8B，让它学会你的说话语气。

### 2. 向量数据库与 RAG实战：从原型验证理解核心概念
*   **RAG 核心原理解析**：不要只看书。RAG (Retrieval-Augmented Generation) 本质就是“开卷考试”。
    1.  **Chunking (切片)**：把你的 Markdown 笔记切成 500 字的小块。
    2.  **Embedding (向量化)**：用一个小模型 (如 `all-MiniLM-L6-v2`) 把文字变成 384 维的数字数组。
    3.  **Indexing (索引)**：把这些数组存进向量库 (Vector DB)，像 Google 搜索一样建立索引。
    4.  **Retrieval (检索)**：用户问问题 -> 变向量 -> 找最相似的 3 个切片。
    5.  **Synthesis (综合)**：把这 3 个切片喂给 Llama 3，让它总结答案。
*   **快速验证路径**：
    *   **工具选型**：使用 **LanceDB** (Rust 原生，无需服务器，嵌入式运行，性能极强) + **Bert-tiny**。
    *   **实战动作**：
        1.  先用 Python 写一个 50 行的脚本跑通流程 (使用 `lancedb` python 库 + `sentence-transformers`)。
        2.  **理解难点**：观察当你改变 Chunk 大小时，搜索结果会怎么变？（这就是 RAG 的调优核心）。
        3.  **Rust 落地**：再用 Rust 的 `lancedb` crate 重写，接入我们的系统。
*   **验收项目**：**"第二大脑"原型**。拖入文件夹，自动建立索引，然后你可以问：“我上周关于 WASM 的笔记通过什么解决了内存泄漏问题？”。

---

## 阶段四：终极之战 - 端侧 AI 与 Rust (The Edge Master)
**周期**：持续精进
**目标**：构建像 Windows 记事本一样轻快，但拥有 GPT-4 智商的应用。

### 1. 推理引擎源码分析：HuggingFace Candle
*   **源码路径**：`candle-core/src/tensor.rs`, `candle-nn/src/linear.rs`
*   **学习重点**：
    *   **Strided Tensor**: 也是 Rust 数组操作的核心。如何不复制内存就实现 Transpose 操作？
    *   **CUDA Kernels**: 如何通过 Rust 调用 `.cu` 文件加速计算？
*   **验收项目**：**手写 MatMul**。用 Rust SIMD 指令集尝试优化 1024x1024 的矩阵乘法，对比纯循环的性能提升。

### 2. WebAssembly 极限优化：《High Performance WebAssembly》
*   **技术深潜**：
    *   **Memory Model**: 线形内存与 Rust 堆栈的交互。
    *   **SIMD for WASM**: 在浏览器里调用 CPU 向量指令。
*   **验收项目**：**WASM MNIST**。将手写数字识别模型编译为 WASM，纯前端运行，识别率 >98%，延迟 <10ms。

---

## 总结：你的毕业设计 (Capstone Project)
**项目代号**：Project Triumvirate (本仓库)
**任务**：将上述所有技能点融合。
1.  用 **Rust** 写后端推理 (Phase 1 & 4)
2.  用 **Web原理** 理解 Attention (Phase 2)
3.  用 **WASM** 写前端 UI (Phase 4)
4.  用 **QLoRA** 做自主进化 (Phase 3)

开始你的旅程吧。
