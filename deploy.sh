#!/bin/bash

# 0. Install wasm-pack if missing (for Vercel/Netlify builds)
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 0.5 Ensure wasm32-unknown-unknown target is installed
if command -v rustup &> /dev/null; then
    echo "Adding wasm32-unknown-unknown target via rustup..."
    rustup target add wasm32-unknown-unknown
else
    echo "Rustup not found. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    rustup target add wasm32-unknown-unknown
fi

# 1. build the wasm
cd resume-wasm
wasm-pack build --target web
cd ..

# 2. Prepare dist folder
rm -rf dist
mkdir dist

# 3. Copy files
cp index.html dist/
cp -r resume-wasm/pkg dist/pkg

# 4. Patch index.html to look for pkg in the right place
# In dist/index.html, we need to change './resume-wasm/pkg/resume_wasm.js' to './pkg/resume_wasm.js'
sed -i '' 's|\./resume-wasm/pkg/resume_wasm.js|\./pkg/resume_wasm.js|g' dist/index.html 2>/dev/null || sed -i 's|\./resume-wasm/pkg/resume_wasm.js|\./pkg/resume_wasm.js|g' dist/index.html

echo "✅ Build complete in ./dist folder"
