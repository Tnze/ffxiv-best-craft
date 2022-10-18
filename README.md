# FFXIV-Best-Craft

最终幻想14生产模拟器

优点：

- 模拟效果准确（应该蛮准
- 求解器能自动写宏（主打功能好吗！！，完全不会玩生产的人也可以用！！
- 可下载随时使用，无需在线加载网页
- 程序极小（这年头还能去哪找十几M的小程序
- 跨平台（以防万一有人用Linux和Mac玩FF14

缺点：

- ~~【数据删除】~~

> （求解器真的很好用

## Dev （开发人员帮助）

### 安装依赖项

按照这篇教程安装Rust、Node和WebView2等组件：[Tauri Getting Started](https://tauri.app/zh/v1/guides/getting-started/prerequisites/)

```bash
yarn # download requirements
```

### Develop build

```bash
yarn tauri dev
```

> Solving is really slow in debug mode.

### Release build

```bash
# set enviroments
export TAURI_PRIVATE_KEY="Path or String of your private key"
export TAURI_KEY_PASSWORD="Your private key password (optional)"
# build and signing
yarn tauri build
```
