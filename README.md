# rust
# Used the following in vimrc to use RUST
```
Plug 'rust-lang/rust.vim'
" After plugInstall do
" :CocInstall coc-rust-analyzer
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'dense-analysis/ale'

let g:rustfmt_autosave = 1
let g:rustfmt_emit_files = 1
let g:rustfmt_fail_silently = 0

inoremap <silent><expr> <TAB>
      \ pumvisible() ? "\<C-n>" :
      \ <SID>check_back_space() ? "\<TAB>" :
      \ coc#refresh()
inoremap <expr><S-TAB> pumvisible() ? "\<C-p>" : "\<C-h>"

function! s:check_back_space() abort
  let col = col('.') - 1
  return !col || getline('.')[col - 1]  =~# '\s'
endfunction

if has('nvim')
  inoremap <silent><expr> <c-space> coc#refresh()
else
  inoremap <silent><expr> <c-@> coc#refresh()
endif

nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)
set backspace=indent,eol,start
```                                            
                                 

1. cargo new <project name>
   Example:
      cargo new fibonocci
   output:
       cd fibonocci; tree
       |-- Cargo.toml
       `-- src
           `-- main.rs
      
2. cargo new empdb
        -> Creates the employee database with name, age, date of joining.

3. cargo new async_program
        -> Extends the empdb with async.
        -> Struct obj are pushed to the vector in one coroutine, and another
           corouting fetches the struct and populates hashmap

4. cell_refcell
        -> Explanations on Cell and RefCell. 
