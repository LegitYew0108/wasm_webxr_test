動かし方
`wasm-pack`をインストール
```
cargo install wasm-pack
```

wasm-pack経由でビルド
```
wasm-pack build --target web
```

ローカルサーバーを立ち上げる
(miniserveの場合)
```
miniserve ./ --index index.html
```
