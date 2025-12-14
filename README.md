# FFXIV-Best-Craft

This is a crafting simulator with solver algorithms for Final Fantasy XIV(FF14).

最终幻想14生产模拟器（附带智能求解算法）

## Online Edition 在线版

☞ <https://tnze.yyyy.games/#/>

## Desktop Edition 桌面版

推荐配置 Recommended setup:

- 操作系统 OS: Windows 11, MacOS, Linux
- 内存 Memory: 16GiB

最低配置 Required setup:

- 操作系统 OS: Windows 10
- 内存 Memory: 8GiB

下载地址：[Gitee](https://gitee.com/Tnze/ffxiv-best-craft/releases) (Windows Only)  
Download: [Github](https://github.com/Tnze/ffxiv-best-craft/releases/latest)

- Windows用户请下载 `.msi` 或者 `.nsis` 安装包
- Mac 用户请下载`dmg`或`app`文件
- Debian 及 Ubuntu 用户请下载`AppImage`或`deb`文件
- 其他 Linux 用户请下载`AppImage`文件

## Develope 开发指引

<details>
<summary>Click to expand 点击展开</summary>
<p>

```bash
pnpm install # 下载依赖 download requirements
```

### For Web Only

#### 安装Web依赖 Install Web Dependencies

为了编译Wasm模块，下载：[rustup](https://rustup.rs/)、[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)、[wasm-bindgen-cli](https://crates.io/crates/wasm-bindgen)

```bash
cargo install wasm-pack wasm-bindgen-cli
```

#### Develop Hotload Server

```bash
pnpm run dev-web
```

#### Release Build

```bash
pnpm run build-web
```

### For Tauri

#### 安装Tauri依赖 Install Tauri Dependencies

按照这篇教程安装Rust、Node和WebView2等组件：[Tauri Getting Started](https://tauri.app/zh/v1/guides/getting-started/prerequisites/)

> 以下 `cargo tauri` 开头的命令可以根据你安装 tauri-cli 方式的不同而有所变化
> 见：<https://tauri.app/v1/guides/getting-started/setup/>
> The commands start with `cargo tauri` below could be different, depending on the way you install tauri-cli.
> See: <https://tauri.app/v1/guides/getting-started/setup/>

#### Develop build

```bash
# 安装tauri-cli
cargo install tauri-cli
cargo tauri dev
# 或 or
cargo tauri dev --release
```

> ⚠️ 调试模式构建的程序运行求解器可能比发布模式慢得多。  
> ⚠️ Solving in debug mode could be much slower than release mode.

#### Release build

```bash
# 设置环境变量 set enviroments
export TAURI_PRIVATE_KEY="Path or String of your private key"
export TAURI_KEY_PASSWORD="Your private key password (optional)"
# 构建和签名 build and signing
cargo tauri build
```

</p>
</details>
