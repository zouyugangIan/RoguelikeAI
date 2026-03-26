# 2026 Q2 AI Developer & Training Mastery Roadmap / 2026 年第二季度 AI 开发与训练掌握路线图

> **Verification date / 核验日期**: 2026-03-25  
> **Target window / 适用窗口**: 2026-04 and the following quarter / 面向 2026 年 4 月及其后一季度  
> **Who this is for / 适用人群**: developers who want a structured path from beginner to advanced AI engineering, model training, evaluation, and deployment / 想从初学者逐步进入 AI 工程、模型训练、评估与部署全流程的开发者  
> **How this guide ranks resources / 本文如何评价资源**: when a platform rating exists, it is prioritized; otherwise this guide uses strong community signals such as GitHub stars, university adoption, official-course status, and evidence of active maintenance / 优先采用平台评分；没有公开评分时，采用 GitHub 星标、大学采用规模、官方课程身份和持续维护情况等强口碑信号

---

## 1. Executive Summary / 执行摘要

**Best-practice conclusion for 2026 Q2 / 2026 年第二季度最佳实践结论**

1. Learn AI in loops, not in silos: concept -> notebook -> small project -> evaluation -> writeup.  
   学 AI 最有效的方式不是囤课，而是“概念 -> Notebook -> 小项目 -> 评估 -> 复盘”的闭环。
2. Start with Python, PyTorch, data handling, and evaluation before chasing agents or giant-model hype.  
   先稳住 Python、PyTorch、数据处理和评估，再碰 agent 编排和超大模型。
3. For most learners, fine-tuning and post-training are far more valuable than pretraining from scratch.  
   对绝大多数学习者来说，微调和后训练的价值远高于一开始就从零预训练。
4. Learn small-model craftsmanship first: 100M to 8B scale teaches more per dollar than blindly running 70B+ systems.  
   先学会小模型的工艺和调优，100M 到 8B 规模的学习性价比远高于盲目上 70B+。
5. Evaluation, dataset quality, and reproducibility now matter more than framework fashion.  
   现在比起框架追新，评估、数据质量和可复现性更重要。
6. Agents should come after you can already build solid prompting, RAG, tool-use, and eval pipelines.  
   Agent 学习应排在你已经能稳定做 Prompt、RAG、工具调用和评估流水线之后。

---

## 2. How To Use This Document / 如何使用这份文档

**One sentence rule / 一句话规则**  
For each stage, pick one theory-heavy resource, one implementation-heavy resource, and one concrete project. Do not try to finish every recommended item before moving on.  
每个阶段只需要抓住“一门偏理论 + 一门偏实现 + 一个项目”即可，不要试图把所有推荐资源全部学完再前进。

**Suggested pacing / 建议节奏**

| Stage / 阶段 | Focus / 重点 | Typical pace / 常见节奏 |
| --- | --- | --- |
| 0 | Orientation, tooling, math refresh / 建立认知、环境和数学底子 | 1-3 weeks / 1-3 周 |
| 1 | Classical ML and data intuition / 传统机器学习与数据直觉 | 3-6 weeks / 3-6 周 |
| 2 | Deep learning fundamentals / 深度学习核心 | 4-8 weeks / 4-8 周 |
| 3 | LLM and AI application engineering / LLM 与 AI 应用工程 | 4-8 weeks / 4-8 周 |
| 4 | Fine-tuning, post-training, LLM internals / 微调、后训练、LLM 内部原理 | 6-10 weeks / 6-10 周 |
| 5 | Systems, evals, serving, MLOps / 系统、评估、服务化与 MLOps | 6-10 weeks / 6-10 周 |
| 6 | Specialization tracks / 专项深挖方向 | ongoing / 持续精进 |

**If you only want one path / 如果你只想走一条主线**

1. Stage 0 -> Stage 1 -> Stage 2 -> Stage 3 -> Stage 4 -> Stage 5  
2. Then choose one specialization track in Stage 6  
3. Finish with a capstone project that includes data, training, evaluation, and deployment

---

## 3. 2026 Q2 Study Principles / 2026 年第二季度学习原则

### Principle A / 原则 A: Build before you over-read
Read enough to implement. If a course or book does not lead to code, data, and measurable output, it should not dominate your schedule.  
阅读只服务于实现。如果一个资源不能导向代码、数据和可度量结果，它就不应该占据你的主学习时间。

### Principle B / 原则 B: Prefer official and maintained resources
AI tooling changes fast. Prioritize official docs, official courses, and resources that are still maintained in 2025-2026.  
AI 工具链变化很快。优先选择仍在 2025-2026 持续维护的官方文档、官方课程和主流项目。

### Principle C / 原则 C: Separate the tracks
Do not confuse AI application engineering, model training, research reading, and systems optimization. They overlap, but they are not the same job.  
不要把 AI 应用工程、模型训练、研究阅读和系统优化混成一件事。它们有交集，但不是同一岗位能力。

### Principle D / 原则 D: Evaluation is mandatory
Every serious AI learner in 2026 needs basic evaluation literacy: train/validation/test splits, offline eval, human review, regression checks, and prompt or model versioning.  
到 2026 年，任何认真学 AI 的人都必须具备基本评估素养：训练集/验证集/测试集划分、离线评估、人工复核、回归检查，以及 Prompt 或模型版本管理。

### Principle E / 原则 E: Small models first
You learn more from fine-tuning, profiling, and evaluating a small model you understand than from renting a giant model you cannot debug.  
调通、剖析并评估一个你真正理解的小模型，学习收益通常大于租一个你根本调不明白的大模型。

---

## 4. Core Tool Stack / 核心工具栈

**Default stack for most learners / 大多数学习者的默认工具栈**

- `Python 3.11+` or `3.12`, `uv` or `conda`, `Jupyter`, `VS Code`
- `PyTorch`, `transformers`, `datasets`, `tokenizers`
- `scikit-learn`, `pandas`, `numpy`, `matplotlib`
- `Weights & Biases` or `MLflow` for experiment tracking
- `vLLM` or `llama.cpp` for inference experiments
- `LitGPT`, `TRL`, or equivalent training recipes when you reach fine-tuning
- `Kaggle`, `Colab`, or local single-GPU machines for early labs

**Do not optimize too early / 不要过早优化**

- Do not start with distributed training.
- Do not start with custom kernels.
- Do not start with agent frameworks before you understand prompting, retrieval, and evals.
- Do not pretrain a transformer from scratch until you can already fine-tune and benchmark smaller models correctly.

---

## 5. The Roadmap / 分阶段路线图

### Stage 0. Orientation, Math, and Setup / 阶段 0：认知建立、数学补底与环境搭建

**Goal / 目标**  
Understand what modern AI workflows actually are, set up a working Python/PyTorch environment, and refresh the math needed for later stages.  
理解现代 AI 工作流到底是什么，搭好 Python/PyTorch 环境，并补齐后续学习必需的数学基础。

| Priority / 优先级 | Resource / 资源 | Signal / 口碑信号 | Why it belongs here / 推荐理由 |
| --- | --- | --- | --- |
| Must-do / 必做 | [Generative AI for Everyone](https://www.coursera.org/learn/generative-ai-for-everyone) | Coursera `4.8/5`, `4,903` reviews, `96%` liked, verified 2026-03-25 | Best non-technical overview of what GenAI can and cannot do. Good for building the mental map before touching code. / 最适合先建立生成式 AI 的全局认知，先知道它能做什么、不能做什么，再进入代码层。 |
| Must-do / 必做 | [Machine Learning Crash Course](https://developers.google.com/machine-learning/crash-course) | Google official; page states millions have relied on it since 2018, refreshed with recent AI content | Fast, practical, and updated with LLM, production, and fairness modules. / 节奏快、够实用，而且新版已经覆盖 LLM、生产系统与公平性模块。 |
| Should-do / 建议完成 | [Mathematics for Machine Learning and Data Science](https://www.coursera.org/specializations/mathematics-for-machine-learning-and-data-science) | Coursera `4.6/5`, `3,158` reviews | Efficient math bridge for calculus, linear algebra, statistics, and probability without forcing a pure-math detour. / 很适合作为微积分、线代、统计和概率的实用桥接课，不会把你带偏成纯数学路线。 |
| Should-do / 建议完成 | [PyTorch Learn the Basics](https://docs.pytorch.org/tutorials/beginner/basics/intro.html) | Official PyTorch tutorials, actively maintained in `v2.10.0+cu128` docs | The cleanest entry to tensors, autograd, datasets, and training loops. / 学张量、自动求导、数据集与训练循环，官方教程是最稳入口。 |

**Exit criteria / 出关标准**

- You can explain the difference between ML, deep learning, LLMs, RAG, fine-tuning, and inference.
- You can build and run a PyTorch notebook locally or in the cloud.
- You can read tensor shapes without panic.

**Suggested project / 建议项目**  
Train a simple classifier on MNIST or Fashion-MNIST and write a one-page bilingual note explaining tensors, gradients, loss, and validation.  
训练一个 MNIST 或 Fashion-MNIST 分类器，并写一页中英双语笔记，说明张量、梯度、损失和验证集。

---

### Stage 1. Classical ML and Data Intuition / 阶段 1：传统机器学习与数据直觉

**Goal / 目标**  
Build strong intuition for supervised learning, data leakage, overfitting, evaluation metrics, and feature-based modeling.  
建立对监督学习、数据泄漏、过拟合、评估指标和特征工程建模的扎实直觉。

| Priority / 优先级 | Resource / 资源 | Signal / 口碑信号 | Why it belongs here / 推荐理由 |
| --- | --- | --- | --- |
| Must-do / 必做 | [Machine Learning Specialization](https://www.coursera.org/specializations/machine-learning-introduction) | Coursera `4.9/5`, `38,281` reviews, verified 2026-03-25 | Still the strongest structured beginner-to-intermediate ML sequence. It balances intuition, code, and practical advice. / 仍然是最稳定的 ML 入门到进阶主线，兼顾直觉、代码和实战建议。 |
| Must-do / 必做 | [Dive into Deep Learning](https://d2l.ai/) | `27.7k` GitHub stars; adopted at `500` universities in `70` countries | Open, rigorous, and hands-on. Excellent bridge from equations to notebooks. / 开放、系统、可运行，是从公式过渡到 Notebook 实战的优质桥梁。 |
| Should-do / 建议完成 | [MLCC core modules](https://developers.google.com/machine-learning/crash-course) | Google official; updated modules on data, overfitting, fairness, and production | Use it as a compact reinforcement layer after Andrew Ng's path. / 很适合在 Andrew Ng 路线之后做一轮高密度巩固。 |

**Exit criteria / 出关标准**

- You can choose metrics for classification, ranking, and regression with a reason.
- You can explain underfitting, overfitting, leakage, and distribution shift.
- You can compare a linear model, tree-based model, and shallow neural baseline on the same dataset.

**Suggested project / 建议项目**  
Build one tabular ML pipeline with `scikit-learn`: cleaning, train/validation/test split, baseline comparison, error analysis, and a short model card.  
用 `scikit-learn` 做一个完整表格数据 pipeline：清洗、数据划分、基线对比、误差分析，再附一页简短 model card。

---

### Stage 2. Deep Learning Core / 阶段 2：深度学习核心能力

**Goal / 目标**  
Move from feature engineering to representation learning, and understand optimization, backpropagation, architectures, and training dynamics.  
从特征工程进入表征学习，真正理解优化、反向传播、网络结构和训练动力学。

| Priority / 优先级 | Resource / 资源 | Signal / 口碑信号 | Why it belongs here / 推荐理由 |
| --- | --- | --- | --- |
| Must-do / 必做 | [Deep Learning Specialization](https://www.coursera.org/specializations/deep-learning) | Coursera `4.8/5`, `147,021` reviews, verified 2026-03-25 | Massive proof of quality. Still one of the best structured paths for optimization, CNNs, sequence models, and transformers. / 用户口碑极强，结构完整，覆盖优化、CNN、序列模型和 Transformer 基础。 |
| Must-do / 必做 | [Practical Deep Learning for Coders](https://course.fast.ai/) | Long-standing practitioner favorite; free; strongly project-driven | Best for turning deep learning into working applications quickly. / 最适合把深度学习快速转化为可跑的应用项目。 |
| Must-do / 必做 | [karpathy/nn-zero-to-hero](https://github.com/karpathy/nn-zero-to-hero) | `19.7k` GitHub stars | Few resources build neural network intuition as well as this one. It is especially strong for autograd, MLPs, normalization, and early language modeling intuition. / 在神经网络直觉建立方面几乎是现象级资源，尤其适合理解自动求导、MLP、归一化和早期语言模型。 |
| Good reference / 参考必备 | [PyTorch beginner tutorials](https://docs.pytorch.org/tutorials/beginner/basics/intro.html) | Official PyTorch docs | Keep this open while implementing. / 实现阶段建议一直开着当参考手册。 |

**Exit criteria / 出关标准**

- You can implement a training loop from scratch in PyTorch.
- You can diagnose unstable training with learning rate, normalization, initialization, and data issues.
- You can explain attention, residual connections, embeddings, and batching at an intuitive level.

**Suggested project / 建议项目**  
Implement `micrograd`-style autograd or a minimal character-level language model, then train a CNN or tiny transformer on a real dataset.  
手写一个 `micrograd` 风格自动求导或最小字符级语言模型，再训练一个 CNN 或 tiny transformer 到真实数据上。

---

### Stage 3. LLM and AI Application Engineering / 阶段 3：LLM 与 AI 应用工程

**Goal / 目标**  
Learn how modern language-model applications are built: tokenization, prompting, retrieval, embeddings, tool use, evaluation, and product framing.  
掌握现代语言模型应用的构建方式：分词、Prompt、检索、向量、工具调用、评估和产品化思维。

| Priority / 优先级 | Resource / 资源 | Signal / 口碑信号 | Why it belongs here / 推荐理由 |
| --- | --- | --- | --- |
| Must-do / 必做 | [Hugging Face Learn](https://huggingface.co/learn) | Official hub covering `LLM Course`, `Agents Course`, `MCP Course`, `Deep RL Course`, `Open-Source AI Cookbook`, and more | In 2026, this is one of the best maintained, broad, and practical learning hubs for open-model development. / 到 2026 年，这已经是最值得长期跟进的开源 AI 学习入口之一。 |
| Must-do / 必做 | [Generative AI with Large Language Models](https://www.coursera.org/learn/generative-ai-with-llms) | Coursera `4.8/5`, `3,426` reviews, `95%` liked | Efficient overview of prompting, fine-tuning, PEFT, and evaluation from a production-minded angle. / 以非常高效的方式串起 Prompt、微调、PEFT 和评估，非常适合进入 LLM 工程阶段。 |
| Must-do / 必做 | [Stanford CS224N](https://web.stanford.edu/class/cs224n/) | Stanford official; Winter 2026 course with public 2024 video playlist referenced on course page | Best bridge between NLP foundations and modern LLM thinking. / 这是从经典 NLP 过渡到现代 LLM 体系的最佳桥梁之一。 |
| Should-do / 建议完成 | [Open-Source AI Cookbook](https://huggingface.co/learn) | Official Hugging Face learning collection | Good for moving from theory to replicable notebook patterns. / 很适合把抽象概念变成可复制的 notebook 工作流。 |

**Best-practice note / 实践提示**  
Do not start with agents. First learn prompts, structured outputs, retrieval, tools, and evals. Most weak AI applications fail because these basics are not solid.  
不要一上来学 agents。先把 Prompt、结构化输出、检索、工具调用和评估学扎实。很多 AI 应用失败并不是因为 agent 不够高级，而是基础能力没打牢。

**Exit criteria / 出关标准**

- You can explain RAG versus fine-tuning and when to use each.
- You can build a retrieval pipeline with chunking, embeddings, reranking, and grounded response generation.
- You can run offline evals and simple regression checks on prompt or application changes.

**Suggested project / 建议项目**  
Build a domain RAG assistant with citation support, an evaluation sheet, and at least one controlled A/B comparison for chunking or retrieval strategy.  
做一个带引用能力的垂直领域 RAG 助手，并建立评估表，至少完成一次针对切块策略或检索策略的 A/B 对比。

---

### Stage 4. Fine-tuning, Post-training, and LLM Internals / 阶段 4：微调、后训练与 LLM 内部原理

**Goal / 目标**  
Move from using models to adapting and understanding them: tokenizer internals, transformer implementation, SFT, PEFT, alignment, data curation, and small-scale training.  
从“使用模型”进入“改造模型”和“理解模型本体”：分词器、Transformer 实现、监督微调、参数高效微调、对齐、数据构建和小规模训练。

| Priority / 优先级 | Resource / 资源 | Signal / 口碑信号 | Why it belongs here / 推荐理由 |
| --- | --- | --- | --- |
| Must-do / 必做 | [Build a Large Language Model (From Scratch) / LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch) | `82.2k` GitHub stars | The clearest modern path for implementing, pretraining, and finetuning a GPT-like model from the ground up. / 这是当代最清晰的从零实现、预训练和微调 GPT 类模型的资源之一。 |
| Must-do / 必做 | [Stanford CS336: Language Modeling from Scratch](https://cs336.stanford.edu/) | Stanford official Spring `2026` course; covers basics, systems, scaling, data, and alignment/reasoning RL | Among the highest-value advanced resources if you want to deeply understand modern language-model training. / 如果你想真正理解现代语言模型训练，这门课的价值极高。 |
| Must-do / 必做 | [LitGPT](https://github.com/Lightning-AI/litgpt) | `13.1k` GitHub stars | Strong practical bridge from education to real recipes: pretrain, finetune, evaluate, and deploy with readable code. / 它是从“会原理”走向“会跑真实 recipe”的优秀桥梁，代码可读性也很好。 |
| Good bridge / 过渡补充 | [Hugging Face Learn: a smol course](https://huggingface.co/learn) | Official Hugging Face short course listed under Learn | Good lightweight bridge into post-training concepts without drowning in theory. / 适合快速建立后训练概念，不会一下子陷入过重理论。 |

**Best-practice note / 实践提示**

- Start with SFT and LoRA or QLoRA before attempting serious pretraining.
- Use small, curated, versioned datasets.
- Track every experiment, prompt, and data revision.
- Evaluate before and after tuning on held-out tasks, not only with demos.

先从 SFT 和 LoRA/QLoRA 做起，再考虑更重的预训练。  
数据集要小而精、可版本化。  
每次实验、Prompt、数据修改都要记录。  
评估必须看保留集与真实任务，不要只看 Demo 手感。

**Exit criteria / 出关标准**

- You can explain tokenization, attention, loss masking, SFT, LoRA, QLoRA, and alignment basics.
- You can fine-tune a small open model and compare it against a base model with reproducible evaluation.
- You can describe why data quality often beats more training steps.

**Suggested project / 建议项目**  
Take a `1B-8B` open model, build a task-specific SFT dataset, fine-tune with PEFT, evaluate on a held-out set, and publish a short training report.  
选择一个 `1B-8B` 开源模型，构建特定任务的 SFT 数据集，用 PEFT 微调，在保留集上评估，并产出一份简洁训练报告。

---

### Stage 5. Systems, Evals, Serving, and MLOps / 阶段 5：系统、评估、服务化与 MLOps

**Goal / 目标**  
Learn how to turn notebooks into reliable AI systems: experiment tracking, testing, deployment, monitoring, serving, and iteration in production.  
学会把 Notebook 变成可靠 AI 系统：实验跟踪、测试、部署、监控、服务化以及生产环境迭代。

| Priority / 优先级 | Resource / 资源 | Signal / 口碑信号 | Why it belongs here / 推荐理由 |
| --- | --- | --- | --- |
| Must-do / 必做 | [Full Stack Deep Learning](https://fullstackdeeplearning.com/) | Free course; site highlights best practices across the lifecycle of AI-powered products; used by learners from Berkeley, UW, and beyond | Still one of the best resources for the gap between “I trained a model” and “I shipped a real system.” / 它非常擅长填补“我会训练模型”和“我能上线系统”之间的巨大鸿沟。 |
| Must-do / 必做 | [Made With ML](https://madewithml.com/) | Community of `40K+` developers; strong practitioner endorsements; production-focused curriculum | Probably one of the most practical MLOps learning resources on the web. / 这几乎是网上最务实的 MLOps 学习资源之一。 |
| Must-do / 必做 | [vLLM](https://github.com/vllm-project/vllm) | `65.6k` GitHub stars | The serving engine you should understand if you care about real-world open-model inference in 2026. / 如果你关心 2026 年真实世界中的开源模型服务化，vLLM 是绕不开的核心项目。 |
| Good reinforcement / 巩固补充 | [MLCC: Production ML Systems](https://developers.google.com/machine-learning/crash-course) | Google official | Good compact reinforcement of serving and production concepts. / 适合做生产系统概念的紧凑巩固。 |

**Best-practice note / 实践提示**

- Treat prompts, datasets, and models as versioned artifacts.
- Add regression tests for data, prompts, and outputs.
- Measure latency, cost, quality, and failure modes together.
- Build dashboards only after you define what “good” means.

把 Prompt、数据集和模型都当作版本化制品。  
为数据、Prompt 和输出都建立回归测试。  
延迟、成本、质量和失败模式必须一起看。  
先定义“什么是好”，再做监控看板。

**Exit criteria / 出关标准**

- You can serve a model behind an API and benchmark throughput and latency.
- You can design offline evals plus basic online checks.
- You can explain model, data, and application regressions separately.

**Suggested project / 建议项目**  
Deploy one small model or LLM workflow as a service with logging, evaluation scripts, basic monitoring, and a rollback plan.  
把一个小模型或 LLM 工作流部署成服务，补齐日志、评估脚本、基础监控和回滚方案。

---

### Stage 6. Specialization Tracks / 阶段 6：专项深挖方向

Choose only one specialization at first. Depth compounds faster than scattered breadth.  
第一次深挖时只选一个方向。专精带来的复利通常比泛泛铺开更大。

#### 6A. NLP and LLM Research Track / NLP 与 LLM 研究方向

| Resource / 资源 | Signal / 口碑信号 | Why it is top-tier / 顶级理由 |
| --- | --- | --- |
| [Stanford CS224N](https://web.stanford.edu/class/cs224n/) | Stanford official | Best formal bridge between NLP foundations and modern LLM work. / 经典 NLP 与现代 LLM 之间的最佳正式桥梁之一。 |
| [Stanford CS336](https://cs336.stanford.edu/) | Stanford official Spring 2026 | Best advanced implementation-heavy path for language modeling from scratch. / 目前最强的一类“从零做语言模型”的实现型高级课程。 |
| [LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch) | `82.2k` stars | Practical internalization through code, not just slides. / 通过代码真正内化，而不是只停留在课件层。 |

#### 6B. Computer Vision and Multimodal Track / 计算机视觉与多模态方向

| Resource / 资源 | Signal / 口碑信号 | Why it is top-tier / 顶级理由 |
| --- | --- | --- |
| [Stanford CS231n](https://cs231n.stanford.edu/) | Stanford official | Still the gold-standard deep learning course for computer vision foundations. / 仍然是视觉深度学习基础课里的金标准。 |
| [Practical Deep Learning for Coders](https://course.fast.ai/) | Strong practitioner reputation | Excellent for getting vision models working quickly in practice. / 很适合快速做出能跑的视觉项目。 |
| [Hugging Face Learn](https://huggingface.co/learn) | Official hub with Community Computer Vision Course and multimodal resources | Good for modern tooling and open-model workflows. / 很适合接入现代开源模型和多模态工作流。 |

#### 6C. RL, Agents, and Interactive Systems Track / 强化学习、Agent 与交互系统方向

| Resource / 资源 | Signal / 口碑信号 | Why it is top-tier / 顶级理由 |
| --- | --- | --- |
| [Hugging Face Deep RL Course](https://huggingface.co/learn) | Official HF learning track | Best accessible entry into practical deep RL in the modern open-source ecosystem. / 是目前最易进入、又足够实践化的现代开源 RL 学习路径之一。 |
| [Hugging Face Agents Course](https://huggingface.co/learn) | Official HF learning track | Good when you already understand tools, structured outputs, and evals. / 当你已经懂工具调用、结构化输出和评估后，它非常适合继续向 agent 深入。 |
| [Hugging Face MCP Course](https://huggingface.co/learn) | Official HF learning track | Useful for the growing standardized tool and context ecosystem around AI applications. / 对理解 AI 应用中越来越重要的标准化工具与上下文协议很有帮助。 |

#### 6D. Production AI Engineer Track / 生产级 AI 工程方向

| Resource / 资源 | Signal / 口碑信号 | Why it is top-tier / 顶级理由 |
| --- | --- | --- |
| [Full Stack Deep Learning](https://fullstackdeeplearning.com/) | Free, widely used | Best “full lifecycle” product-building course. / “全生命周期 AI 产品构建”最强课程之一。 |
| [Made With ML](https://madewithml.com/) | `40K+` developers, strong endorsements | Strongest practical MLOps systems resource in this list. / 这份名单里最强的 MLOps 实战资源之一。 |
| [vLLM](https://github.com/vllm-project/vllm) | `65.6k` stars | Critical serving engine knowledge for open-model systems. / 开源模型系统中的关键服务引擎知识。 |

---

## 6. What To De-Prioritize / 什么不该过度投入

1. Do not spend months on prompt tricks before learning evaluation and retrieval.  
   不要在学会评估和检索之前，把几个月时间都砸在 Prompt 小技巧上。
2. Do not make “training a frontier model from scratch” your first serious goal.  
   不要把“从零训练前沿大模型”设为第一个严肃目标。
3. Do not learn five orchestration frameworks before you can write one stable baseline.  
   在你写不出一个稳定基线之前，不要先学五种编排框架。
4. Do not trust demo quality without held-out evaluation.  
   没有保留集评估支撑的 Demo 质量，不值得信。
5. Do not confuse GitHub hype with durable learning value.  
   不要把 GitHub 热度误当成长期学习价值。

---

## 7. Recommended Capstone / 推荐毕业项目

**Project idea / 项目建议**  
Build a vertical AI system in one domain you care about: coding, finance, education, legal analysis, gaming, or research support.  
在你真正关心的一个垂直领域中，做一个完整 AI 系统，比如编程、金融、教育、法务分析、游戏或科研辅助。

**Minimum capstone requirements / 最低毕业要求**

1. A real dataset or curated domain corpus / 真实数据集或精心整理的领域语料  
2. A baseline model or baseline prompt pipeline / 一个可复现的基线模型或基线 Prompt 流程  
3. An evaluation suite / 一套评估方案  
4. At least one improvement loop with measurable gain / 至少完成一轮有可量化收益的优化迭代  
5. A deployable interface or API / 一个可部署的界面或 API  
6. A short bilingual report covering data, model choice, evaluation, failures, and next steps / 一份中英双语简报，说明数据、模型选择、评估、失败案例和下一步

**Gold-standard version / 高阶版本**

- Retrieval + fine-tuning + evaluation + serving + monitoring in one system  
- 一个系统里同时具备检索、微调、评估、服务化和监控

---

## 8. Fast Path by Role / 按目标角色给出的快速路径

| Role / 目标角色 | Fastest high-value sequence / 最高价值快捷路径 |
| --- | --- |
| AI application engineer / AI 应用工程师 | Stage 0 -> Stage 1 -> Stage 2 -> Stage 3 -> Stage 5 |
| LLM engineer / LLM 工程师 | Stage 0 -> Stage 1 -> Stage 2 -> Stage 3 -> Stage 4 -> Stage 5 |
| Model trainer / 模型训练工程师 | Stage 0 -> Stage 1 -> Stage 2 -> Stage 4 -> Stage 5 |
| NLP researcher / NLP 研究向 | Stage 0 -> Stage 1 -> Stage 2 -> Stage 3 -> Stage 4 -> Stage 6A |
| CV / multimodal engineer / 视觉与多模态工程师 | Stage 0 -> Stage 1 -> Stage 2 -> Stage 6B -> Stage 5 |
| RL or agent builder / RL 或 Agent 开发者 | Stage 0 -> Stage 1 -> Stage 2 -> Stage 3 -> Stage 6C -> Stage 5 |

---

## 9. Final Advice / 最后建议

If you want to be strong by 2026 standards, do not ask only “which model is best?” Ask these five questions instead:  
如果你想达到 2026 年标准，不要只问“哪个模型最好”，而是反复问下面五个问题：

1. What problem am I solving? / 我到底在解决什么问题？
2. What dataset or corpus defines success? / 哪个数据集或语料决定了成功标准？
3. How will I evaluate quality, cost, and latency? / 我怎么评估质量、成本和延迟？
4. What is the smallest system that proves value? / 最小可验证价值的系统是什么？
5. What will break when this goes to real users? / 这套东西一到真实用户那里，最可能先坏在哪里？

Keep the loop tight. Build small. Measure honestly. Then scale.  
保持学习与实现闭环，小步快跑，真实测量，然后再扩张。
