paddr = physical address
vaddr = virtual

pt_paddr = page table address

基本的に、アドレスはusizeとして扱い、
オフセットやレジスタに代入する際はisizeにキャストすれば良い。

ポインタの扱いが難しい。Rustの制約上、生ポインタをグローバル変数として保持できないので、
u32の値をグルーバル変数として保持して、必要に応じてキャストする必要がある。


### 子モジュールのクレートタイプについて
クレートタイプは，子モジュールとかは以下の設定ではよくない．
```
[lib]
crate-type = ["staticlib"]
```
これは静的リンクライブラリ．完全に自己完結したバイナリを生成する．

以下のようにするとよい．
[lib]
crate-type = ["rlib"]

(コンパイラによって対応クレートが異なる．)

- s
```
[lib]
crate-type = ["staticlib"]
```
これでアウトプットファイルが.aになる