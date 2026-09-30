Job 1 = Runtime Controller：让 EdgeLens 能可靠地启动、检测、关闭真实 Supabase Edge Runtime。
先别碰 WebSocket、CPU limit、benchmark、compare。你的第一周本来就应该先把真实 Runtime 跑通，并完成 runtime up/status/down。   粘贴的文本 (1)
CLI 第一版只做这 3 个命令
edge-lens runtime up
edge-lens runtime status
edge-lens runtime down

它们分别对应：
edge-lens runtime up
        │
        ▼
启动 edge-runtime 子进程
        │
        ├── 保存 PID / 状态
        ├── 捕获 stdout
        ├── 捕获 stderr
        └── 等待 Runtime Ready
                 │
                 ▼
        http://localhost:9998

edge-lens runtime status
        │
        ├── 进程还活着吗？
        └── HTTP endpoint 能访问吗？

edge-lens runtime down
        │
        ├── graceful shutdown
        ├── 等待进程退出
        └── 超时后强制 kill

这其实已经不是一个“玩具 CLI”了。它会逼你实际使用文件里要求体现的 Tokio 子进程管理、stdout/stderr 异步读取、cancellation、graceful shutdown、稳定退出码。   粘贴的文本 (1)
第一步具体写什么
目录先不要一上来拆 5 个 crate。你现在学习和推进速度比“架构漂亮”重要。
直接：
edge-lens/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── cli.rs
    └── runtime.rs

依赖第一版只需要：
clap
tokio
reqwest
serde
serde_json
anyhow

连 tracing、thiserror、sysinfo 都可以第二步再加。
CLI 数据结构大概应该是：
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Runtime(RuntimeArgs),
}

#[derive(Args)]
struct RuntimeArgs {
    #[command(subcommand)]
    command: RuntimeCommand,
}

#[derive(Subcommand)]
enum RuntimeCommand {
    Up,
    Status,
    Down,
}

然后：
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Runtime(runtime) => {
            match runtime.command {
                RuntimeCommand::Up => runtime::up().await?,
                RuntimeCommand::Status => runtime::status().await?,
                RuntimeCommand::Down => runtime::down().await?,
            }
        }
    }

    Ok(())
}

这个时候 CLI 层到此为止。
不要往 main.rs 塞运行逻辑。
真正的 Job 1：runtime::up()
第一版目标很简单：
pub async fn up() -> Result<()> {
    // 1. 找到 edge-runtime 目录

    // 2. 执行 ./scripts/run.sh

    // 3. stdin/stdout/stderr 配置

    // 4. spawn，而不是 output().await

    // 5. 异步读取 stdout/stderr

    // 6. 等待 localhost:9998 可用

    // 7. 输出：
    // Runtime started
    // PID: xxxx
    // Endpoint: http://localhost:9998

    Ok(())
}

这里有一个非常重要的学习点：
不要这样：
Command::new("./scripts/run.sh")
    .output()
    .await?;

因为 output() 是“启动 → 等它结束 → 收集结果”。
而 Edge Runtime 是长期运行进程。
你应该开始理解：
let mut child = Command::new("./scripts/run.sh")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

这就是这个项目第一次真正接触 process supervision。
也是 Supabase 这个岗位真正相关的东西。
status 也不要只看 PID
第一版就设计成两层判断：
Process:
  alive

Endpoint:
  http://localhost:9998
  reachable

Status:
  healthy

逻辑类似：
pub async fn status() -> Result<()> {
    let process_alive = ...;
    let endpoint_alive =
        reqwest::get("http://localhost:9998/...").await.is_ok();

    ...
}

为什么这样设计？
因为：
process alive
      ≠
runtime healthy

这是一个非常重要的后端 / infrastructure 思维。
程序没有死，不代表它能服务请求。
down 是这个 Job 里最值得学的部分
不要第一版就：
kill -9

而是形成：
request shutdown
      ↓
等待例如 3 秒
      ↓
still alive?
   /       \
 no         yes
 │           │
done       force kill

以后这里自然就能演化到：
tokio::select! {
    status = child.wait() => { ... }

    _ = tokio::time::sleep(timeout) => {
        child.kill().await?;
    }
}

后面你学 CancellationToken、graceful shutdown、task lifecycle 的时候，这一块会继续升级。
这和计划第二个月要求掌握的 JoinSet / CancellationToken / select! / cancellation safety / graceful shutdown 是一条连续路线。   粘贴的文本 (1)
你今天真正应该完成的 Definition of Done
今天不要给自己列 20 个 TODO。
就做到：
$ edge-lens runtime up

✓ Starting Supabase Edge Runtime...
✓ Runtime ready

PID: 21871
Endpoint: http://localhost:9998

然后：
$ edge-lens runtime status

Runtime: healthy
Process: alive
Endpoint: reachable
PID: 21871

最后：
$ edge-lens runtime down

✓ Shutting down runtime...
✓ Runtime stopped

然后再执行：
edge-lens runtime up
edge-lens runtime down
edge-lens runtime up
edge-lens runtime down

连续几次都不残留进程，才算 Job 1 完成。
文件里的阶段目标也是把 runtime up/status/down 放在最早交付物里，而不是一开始就做 scenario/bench。   粘贴的文本 (1)
Job 1 完成以后，Job 2 才是第一个真正的诊断场景
顺序我建议直接固定下来：
Job 1
Runtime Controller
up / status / down
        ↓
Job 2
hello HTTP scenario
scenario run hello
        ↓
Job 3
统一 ScenarioResult
        ↓
Job 4
structured log parser
        ↓
Job 5
background-task + waitUntil
        ↓
Job 6
WebSocket EarlyDrop
        ↓
Job 7
SSE
        ↓
Job 8
CPU / wall-clock
        ↓
Job 9
run-all
        ↓
Job 10
compare

所以你现在连 edge-lens scenario run websocket-idle 都不要写。
你当前要学的 Rust 主题也非常明确：
clap → tokio::process::Command → Child → stdout/stderr → PID → process lifetime → graceful shutdown。
这一个小 Job 做完，你已经第一次真正从“学 Rust 语法”进入了 Rust 系统工程；而最终 EdgeLens 的目标本身就是围绕真实 Edge Runtime 做生命周期和可靠性诊断，而不是普通压测 CLI。
