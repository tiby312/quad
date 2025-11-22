wasm-pack build --target no-modules --out-dir target/dist/website
cp index.html target/dist/website

cd target/dist/website
python3 -m http.server 8000



