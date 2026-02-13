# 电子书仓库（Ebook Storage）

一款基于 Tauri 2.0 的轻量级桌面端电子书管理工具。扫描本地目录索引电子书文件，提供实时模糊搜索，帮助快速定位所需书籍。

## 功能特性

- **多目录管理** - 添加/删除多个本地目录作为扫描源，配置自动持久化
- **递归扫描** - 递归遍历所有子目录，支持 PDF、EPUB、MOBI、AZW3、TXT 五种格式
- **实时搜索** - 输入即搜索（200ms 防抖），不区分大小写的文件名模糊匹配
- **文件操作** - 系统默认程序打开文件、在文件管理器中定位文件
- **文件重命名** - 列表中直接内联重命名
- **豆瓣搜索** - 根据书名一键跳转豆瓣搜索
- **虚拟滚动** - 基于 @tanstack/vue-virtual，轻松应对大量书籍列表
- **格式标签** - 不同格式以不同颜色标签区分（PDF 红 / EPUB 绿 / MOBI 蓝 / AZW3 紫 / TXT 灰）
- **明暗主题** - 跟随系统主题自动切换亮色/暗色模式

## 技术栈

| 层级 | 技术选型 |
| --- | --- |
| 桌面框架 | Tauri 2.0（Rust 后端 + WebView 前端） |
| 前端 | Vue 3 + TypeScript + Vite 6 |
| 包管理 | pnpm |
| 虚拟滚动 | @tanstack/vue-virtual |
| 存储 | tauri-plugin-store（KV 持久化，无数据库） |

## 项目结构

```
ebook-storage/
├── src/                          # 前端源码
│   ├── main.ts                   # Vue 应用入口
│   ├── App.vue                   # 根组件（页面切换 + 全局样式）
│   ├── types/
│   │   └── index.ts              # BookInfo 类型定义
│   ├── utils/
│   │   └── format.ts             # 文件大小格式化工具
│   └── components/
│       ├── MainView.vue          # 主界面（搜索 + 虚拟滚动列表 + 统计）
│       ├── BookItem.vue          # 书籍列表项（格式标签 + 操作按钮）
│       └── SettingsView.vue      # 设置页面（目录管理 + 重新扫描）
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml                # Rust 依赖
│   ├── tauri.conf.json           # Tauri 应用配置
│   └── src/
│       ├── main.rs               # 入口
│       └── lib.rs                # 核心业务逻辑（扫描/搜索/文件操作）
├── package.json                  # 前端依赖和脚本
├── vite.config.ts                # Vite 配置
└── PRD.md                        # 产品需求文档
```

## 前置要求

- [Node.js](https://nodejs.org/)（LTS 版本）
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)
- Tauri 2.0 系统依赖（参考 [Tauri 官方文档](https://v2.tauri.app/start/prerequisites/)）

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发模式（同时运行 Vite 开发服务器和 Tauri 桌面窗口）
pnpm tauri dev
```

开发服务器默认运行在 `http://localhost:1420`。

## 构建

```bash
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录下，包含：

- **MSI 安装包** - `msi/ebook-storage_<version>_x64_en-US.msi`
- **NSIS 安装包** - `nsis/ebook-storage_<version>_x64-setup.exe`

## 推荐 IDE 配置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 许可证

MIT
