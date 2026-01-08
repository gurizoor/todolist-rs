# index.html
```html
    <head>
        <base data-trunk-public-url/>
    </head>
```

# bash(powershell)

```bash
trunk build --release --public-url /[project name]/
```

↓例

```bash
trunk build --release --public-url /todolist-rs/
```


```bash
# 生成されたファイルをdocsフォルダーに移動
trunk build --release --public-url /todolist-rs/ && rm -rf docs/* && mv dist/* docs/
```

# github pages

ブランチはmaster, フォルダーはdocsに設定しておく。
rootにtrunk.tomlを作成して
```toml
    [build]
    target = "index.html"
    dist = "docs"
```
と記述しておく。
(または、trunk build ~で生成されたファイルたち(distフォルダーに生成されたもの)をdocsフォルダーに移動させておく。)

あとは普通にデプロイすれば完了。
