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

↓Example in this code space

```bash
trunk build --release --public-url /todolist-rs/
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
