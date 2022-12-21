# FFXIV-Best-Craft

最终幻想14生产模拟器

优点：

- 模拟效果准确，与游戏完全一致
- 内置基于DP算法的求解器，求解效果一级棒
- 自动排序，将更好的宏排在靠前位置，无需手动记录最优解
- 离线使用，秒开秒用，体验不受网络环境影响
- 程序极小，电脑硬盘空间不足的玩家也可以放心安装
- 跨平台，（支持Linux与MacOS系统）Windows用户请在[发行版页面](https://gitee.com/Tnze/ffxiv-best-craft/releases)下载`.msi`安装包

缺点：

- 开启“掌握”支持时求解器占用内存较大
- 用的人太少了（？）

## Dev （开发人员帮助）

### 安装依赖项

按照这篇教程安装Rust、Node和WebView2等组件：[Tauri Getting Started](https://tauri.app/zh/v1/guides/getting-started/prerequisites/)

```bash
yarn install # download requirements
```

### Develop build

```bash
yarn tauri dev
# or
yarn tauri dev --release
```

> Solving could be very very slow in debug mode.

### Release build

```bash
# set enviroments
export TAURI_PRIVATE_KEY="Path or String of your private key"
export TAURI_KEY_PASSWORD="Your private key password (optional)"
# build and signing
yarn tauri build
```
