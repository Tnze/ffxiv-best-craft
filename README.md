# FFXIV-Best-Craft

最终幻想14生产模拟器

> 我知道我不写README你们就没有尝试一下的欲望。但是我还是懒得写.self

> （但是真的很好用

## Dev

### 安装依赖项

按照这篇教程安装Rust、Node和WebView2等组件：[Tauri](https://tauri.studio/docs/getting-started/prerequisites)

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
