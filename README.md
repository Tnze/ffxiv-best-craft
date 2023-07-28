# FFXIV-Best-Craft

This is a crafting simulator with solver algorithms for Final Fantasy XIV(FF14).

最终幻想14生产模拟器（附带智能求解算法）

推荐配置 Recommended setup:

- 操作系统 OS: Windows 11, MacOS, Linux
- 内存 Memory: 16GiB

最低配置 Required setup:

- 操作系统 OS: Windows 10
- 内存 Memory: 8GiB

## Download 下载

下载地址：[Gitee](https://gitee.com/Tnze/ffxiv-best-craft/releases) (Windows Only)  
Download: [Github](https://gitee.com/Tnze/ffxiv-best-craft/releases)

- Windows用户请下载 `.msi` 或者 `.nsis` 安装包
- Mac 用户请下载`dmg`或`app`文件
- Debian 及 Ubuntu 用户请下载`AppImage`或`deb`文件
- 其他 Linux 用户请下载`AppImage`文件

## Dev （开发人员帮助）

### 安装依赖项 Install Dependencies

按照这篇教程安装Rust、Node和WebView2等组件：[Tauri Getting Started](https://tauri.app/zh/v1/guides/getting-started/prerequisites/)

```bash
yarn install # 下载依赖 download requirements
```

### Develop build

```bash
yarn tauri dev
# 或 or
yarn tauri dev --release
```

> ⚠️ 调试模式构建的程序运行求解器可能极慢。  
> ⚠️ Solving could be very very slow in debug mode.

### Release build

```bash
# 设置环境变量 set enviroments
export TAURI_PRIVATE_KEY="Path or String of your private key"
export TAURI_KEY_PASSWORD="Your private key password (optional)"
# 构建和签名 build and signing
yarn tauri build
```
