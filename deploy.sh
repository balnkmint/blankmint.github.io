#!/bin/bash

echo "🚀 部署贪吃猫游戏到GitHub Pages"

# 检查是否有wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack 未安装，请先安装："
    echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# 编译WASM
echo "📦 编译WASM..."
wasm-pack build --target web --out-dir pkg

if [ $? -ne 0 ]; then
    echo "❌ WASM编译失败"
    exit 1
fi

echo "✅ WASM编译成功"

# 检查关键文件
if [ ! -f "pkg/snake_game.js" ]; then
    echo "❌ snake_game.js 文件不存在"
    exit 1
fi

if [ ! -f "pkg/snake_game_bg.wasm" ]; then
    echo "❌ snake_game_bg.wasm 文件不存在"
    exit 1
fi

echo "✅ 关键文件检查通过"

# 提交到Git
echo "📤 提交到Git..."
git add .
git commit -m "Deploy cat snake game with WASM files - $(date)"
git push origin main

if [ $? -eq 0 ]; then
    echo "🎉 部署成功！"
    echo "🌐 请访问: https://blankmint.top"
    echo "⏰ GitHub Pages可能需要几分钟来更新"
else
    echo "❌ Git推送失败"
    exit 1
fi
