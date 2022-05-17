let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Programming/ACTIVE/clinvoice
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
set shortmess=aoO
badd +37 ./crates/adapters/clinvoice_adapter_postgres/src/schema/timesheet/timesheet_adapter.rs
badd +1 ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs
badd +40 ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/expense_adapter.rs
badd +16 crates/adapters/clinvoice_adapter/src/schema/contact_info_adapter.rs
badd +10 crates/clinvoice_schema/src/expense.rs
badd +108 crates/adapters/clinvoice_adapter_postgres/src/schema/contact_info/contact_info_adapter.rs
badd +1 /tmp/nvimjGGjz9/LocalPostgreSQL-query-2022-05-16\ 18∶50∶25
badd +0 ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter_postgres/src/schema.rs
badd +1 /tmp/nvimjGGjz9/LocalPostgreSQL-query-2022-05-16\ 18∶52∶44
badd +44 crates/clinvoice_schema/src/timesheet.rs
badd +23 crates/adapters/clinvoice_adapter/src/schema/mod.rs
argglobal
%argdel
$argadd ./crates/adapters/clinvoice_adapter_postgres/src/schema/timesheet/timesheet_adapter.rs
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit crates/clinvoice_schema/src/expense.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 34 + 78) / 157)
exe '2resize ' . ((&lines * 15 + 19) / 38)
exe 'vert 2resize ' . ((&columns * 122 + 78) / 157)
exe '3resize ' . ((&lines * 19 + 19) / 38)
exe 'vert 3resize ' . ((&columns * 122 + 78) / 157)
argglobal
balt /tmp/nvimjGGjz9/LocalPostgreSQL-query-2022-05-16\ 18∶50∶25
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 10 - ((8 * winheight(0) + 17) / 35)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 10
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs", ":p")) | buffer ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs | else | edit ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs
endif
balt ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/expense_adapter.rs
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 7) / 15)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs", ":p")) | buffer ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs | else | edit ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/employee_adapter.rs
endif
balt ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter/src/schema/expense_adapter.rs
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 9) / 19)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
wincmd w
3wincmd w
exe 'vert 1resize ' . ((&columns * 34 + 78) / 157)
exe '2resize ' . ((&lines * 15 + 19) / 38)
exe 'vert 2resize ' . ((&columns * 122 + 78) / 157)
exe '3resize ' . ((&lines * 19 + 19) / 38)
exe 'vert 3resize ' . ((&columns * 122 + 78) / 157)
tabnext
edit ~/Programming/ACTIVE/clinvoice/crates/adapters/clinvoice_adapter_postgres/src/schema.rs
argglobal
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 247 - ((23 * winheight(0) + 19) / 38)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 247
normal! 0
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
