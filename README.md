# FFXIV-Best-Craft

最终幻想14生产模拟器

推荐配置：

- 操作系统：Windows 11、MacOS或Linux
- 内存：16GiB以上

最低配置：

- 操作系统：Windows 10
- 内存：8Gib

## Download 下载

Windows用户请在[发行版页面](https://gitee.com/Tnze/ffxiv-best-craft/releases)下载包含`.msi`字样的安装包

Linux 与 MacOS 用户请移步[Github Releases](https://github.com/Tnze/ffxiv-best-craft/releases)下载对应的安装包

- Mac 用户请下载`dmg`或`app`文件
- Debian 及 Ubuntu 用户请下载`AppImage`或`deb`文件
- 其他 Linux 用户请下载`AppImage`文件

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
