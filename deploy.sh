#!/bin/bash

# 0. Install wasm-pack if missing (for Vercel/Netlify builds)
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
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
