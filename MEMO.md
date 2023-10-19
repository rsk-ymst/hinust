paddr = physical address
vaddr = virtual

pt_paddr = page table address

基本的に、アドレスはusizeとして扱い、
オフセットやレジスタに代入する際はisizeにキャストすれば良い。

ポインタの扱いが難しい。Rustの制約上、生ポインタをグローバル変数として保持できないので、
u32の値をグルーバル変数として保持して、必要に応じてキャストする必要がある。
