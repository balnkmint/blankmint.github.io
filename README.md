# 🐱 blankmint的贪吃猫游戏 - Rust + WebAssembly

一个使用 Rust 编写并编译为 WebAssembly 的特色贪吃猫游戏，可爱白猫吃粑粑主题，支持键盘和触屏控制。

## 🎮 游戏特性

- **多平台控制**:
  - 🖥️ 桌面端：方向键或WASD控制
  - 📱 移动端：触屏虚拟按键控制
- **可爱主题**: 白猫猫头蛇身，金色大眼睛，吃粑粑获得分数
- **暂停功能**: 支持空格键或按钮暂停/继续游戏
- **实时渲染**: 使用 Canvas API 进行流畅的游戏渲染
- **分数系统**: 吃到粑粑获得分数并增长身体
- **游戏重启**: 支持游戏结束后重新开始
- **无限时间**: 没有时间限制，尽情享受游戏
- **响应式设计**: 完美适配桌面和移动设备

## 🛠️ 技术栈

- **Rust**: 游戏核心逻辑
- **WebAssembly (WASM)**: 高性能 Web 运行时
- **wasm-bindgen**: Rust 与 JavaScript 互操作
- **Canvas API**: 游戏渲染
- **HTML5 + CSS3**: 用户界面

## 🚀 本地开发

### 前置要求

- Rust (https://rustup.rs/)
- wasm-pack (https://rustwasm.github.io/wasm-pack/)

### 构建步骤

1. 克隆项目
```bash
git clone <your-repo-url>
cd blankmint.github.io
```

2. 编译 WASM
```bash
wasm-pack build --target web --out-dir pkg
```

3. 启动本地服务器
```bash
python3 -m http.server 8000
```

4. 在浏览器中访问 `http://localhost:8000`

## 🎯 游戏玩法

1. **使用方向键或WASD** 控制猫咪的移动方向
2. **吃粑粑** 获得分数并增长身体
3. **避免撞到自己** 的身体
4. **按空格键** 暂停/继续游戏
5. **点击画布** 或点击重新开始按钮重新游戏

## ⌨️ 控制方式

### 桌面端
- **方向键** 或 **WASD**: 控制猫咪移动方向
- **空格键**: 暂停/继续游戏
- **鼠标点击**: 游戏结束时重新开始

### 移动端
- **虚拟方向键**: 点击屏幕左侧的方向按钮控制猫咪
- **暂停按钮**: 点击暂停/继续按钮
- **触屏点击**: 游戏结束时重新开始

## 📁 项目结构

```
blankmint.github.io/
├── src/
│   └── lib.rs          # Rust 游戏逻辑
├── pkg/                # 编译生成的 WASM 文件
├── Cargo.toml          # Rust 项目配置
├── index.html          # 游戏主页面
└── README.md           # 项目说明
```

## 🌐 在线体验

访问 GitHub Pages: [https://blankmint.github.io](https://blankmint.github.io)

## 📝 开发说明

- 游戏使用 Rust 实现核心逻辑，确保高性能和内存安全
- 通过 WebAssembly 在浏览器中运行，获得接近原生的性能
- 使用 wasm-bindgen 实现 Rust 与 JavaScript 的无缝集成
- 支持现代浏览器的 ES6 模块系统

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License
