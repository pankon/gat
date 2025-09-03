Current best
---
```bash
whatever:~/code/gat/fake_repo$ RUST_BACKTRACE=1 ~/code/gat/gat/target/debug/gat ~/code/gat/fake_repo/main.rs rust
type opt: {34: 1, 84: 1, 30: 6, 97: 1, 17: 1, 63: 3, 33: 19, 52: 1, 51: 1, 31: 12, 36: 25, 88: 1, 39: 6, 16: 34, 76: 1, 68: 1, 42: 1, 61: 1, 13: 1, 48: 1, 41: 1, 87: 1, 14: 1, 35: 13, 2: 1, 24: 1, 29: 13, 11: 16, 64: 4, 3: 5, 99: 1, 47: 6, 44: 1, 10: 6, 40: 9, 7: 25, 46: 1, 19: 1, 4: 13, 66: 1, 38: 1, 53: 1, 22: 1, 67: 1, 104: 1, 54: 1, 50: 1, 43: 1, 49: 1, 5: 89, 27: 6, 12: 6, 6: 14, 23: 1, 37: 20, 20: 5, 0: 1, 26: 35, 86: 1, 15: 1, 18: 2, 28: 1, 32: 1, 1: 1, 45: 2}
whatever:~/code/gat/fake_repo$ ls -lt
total 48
-rw-rw-r-- 1 nathan nathan   999 Sep  3 09:46 data.gat
-rw-rw-r-- 1 nathan nathan   380 Sep  3 09:46 types.gat
-rw-rw-r-- 1 nathan nathan   451 Sep  3 09:46 strings.gat
-rw-rw-r-- 1 nathan nathan  1854 Sep  3 09:46 node_file.gat
-rw-rw-r-- 1 nathan nathan 28367 Sep  3 09:46 ast.gat
-rw-rw-r-- 1 nathan nathan  1543 Sep  1 22:10 main.rs
```

uncompressed
---
```bash
whatever:~/code/gat/fake_repo$ ls -l
total 48
-rw-rw-r-- 1 nathan nathan 27826 Sep  3 09:59 ast.gat
-rw-rw-r-- 1 nathan nathan  1186 Sep  3 09:59 data.gat
-rw-rw-r-- 1 nathan nathan  1543 Sep  1 22:10 main.rs
-rw-rw-r-- 1 nathan nathan  2408 Sep  3 09:59 node_file.gat
-rw-rw-r-- 1 nathan nathan   571 Sep  3 09:59 strings.gat
-rw-rw-r-- 1 nathan nathan   610 Sep  3 09:59 types.gat
```

Exploration
----
```bash
whatever:~/code/gat/fake_repo/.git/objects/68$ zlib-flate -uncompress <b03772f7195bb622d52b9ecf60ba67130a2713
commit 472tree e1047538fabf9f2cac6fcd6d64552bf270afebfd
parent 2b990f8d9e9da501f927cd15088b6bbb0eba98d0
author pankon <[redacted]> 1756753813 +0300
committer pankon <[redacted]> 1756753813 +0300
gpgsig -----BEGIN PGP SIGNATURE-----

[redacted]
-----END PGP SIGNATURE-----

another line

whatever:~/code/gat/fake_repo/.git/objects/e1$ zlib-flate -uncompress <047538fabf9f2cac6fcd6d64552bf270afebfd | xxd
00000000: 7472 6565 2033 3500 3130 3036 3434 206d  tree 35.100644 m
00000010: 6169 6e2e 7273 000e 6203 d282 fc2b 55e7  ain.rs..b....+U.
00000020: 3e74 2fc0 01a4 56cf 0d54 f4              >t/...V..T.

// 0e 6203 d282 fc2b 55e7 is our file

whatever:~/code/gat/fake_repo/.git/objects/0e$ ls -l
total 4
-r--r--r-- 1 nathan nathan 757 Sep  1 22:10 6203d282fc2b55e73e742fc001a456cf0d54f4

whatever:~/code/gat/fake_repo/.git/objects/0e$ zlib-flate -uncompress <6203d282fc2b55e73e742fc001a456cf0d54f4 | xxd
00000000: 626c 6f62 2031 3534 3300 7573 6520 7374  blob 1543.use st
00000010: 643a 3a61 6e79 3a3a 416e 793b 0a75 7365  d::any::Any;.use
00000020: 2073 7464 3a3a 6673 3a3a 4669 6c65 3b0a   std::fs::File;.
00000030: 7573 6520 7374 643a 3a69 6f3a 3a52 6561  use std::io::Rea
00000040: 643b 0a75 7365 2073 7464 3a3a 6f73 3a3a  d;.use std::os::
00000050: 6664 3a3a 4173 5261 7746 643b 0a75 7365  fd::AsRawFd;.use
00000060: 2074 7265 655f 7369 7474 6572 3a3a 7b50   tree_sitter::{P
00000070: 6172 7365 722c 2054 7265 652c 204e 6f64  arser, Tree, Nod
00000080: 652c 2051 7565 7279 7d3b 0a66 6e20 7072  e, Query};.fn pr
00000090: 696e 745f 7472 6565 286e 6f64 653a 2026  int_tree(node: &
000000a0: 4e6f 6465 2c20 6465 7074 683a 2069 3332  Node, depth: i32
000000b0: 2c20 736f 7572 6365 3a20 265b 7538 5d29  , source: &[u8])
000000c0: 207b 0a20 2020 206c 6574 206d 7574 2070   {.    let mut p
000000d0: 6164 6469 6e67 203d 2022 222e 6173 5f73  adding = "".as_s
000000e0: 7472 2829 3b0a 2020 2020 6c65 7420 6d75  tr();.    let mu
000000f0: 7420 6964 7820 3d20 303b 0a20 2020 2077  t idx = 0;.    w
00000100: 6869 6c65 2069 6478 203c 2064 6570 7468  hile idx < depth
00000110: 207b 0a20 2020 2020 2020 2070 6164 6469   {.        paddi
00000120: 6e67 202b 3d20 2220 2020 2022 3b0a 2020  ng += "    ";.  
00000130: 2020 2020 2020 6920 2b3d 2031 3b0a 2020        i += 1;.  
00000140: 2020 7d0a 0a20 2020 206c 6574 2074 6578    }..    let tex
00000150: 7420 3d20 6e6f 6465 2e75 7466 385f 7465  t = node.utf8_te
00000160: 7874 2873 6f75 7263 6529 3b0a 2020 2020  xt(source);.    
00000170: 6966 206e 6f64 652e 6368 696c 645f 636f  if node.child_co
00000180: 756e 7428 2920 3d3d 2030 207b 0a20 2020  unt() == 0 {.   
00000190: 2020 2020 2070 7269 6e74 6c6e 2128 227b       println!("{
000001a0: 7d7b 7d3a 7b7d 2020 207b 7d22 2c20 7061  }{}:{}   {}", pa
000001b0: 6464 696e 672c 206e 6f64 652e 6964 2829  dding, node.id()
000001c0: 2c20 6e6f 6465 2e6b 696e 6428 292c 2074  , node.kind(), t
000001d0: 6578 742e 756e 7772 6170 2829 290a 2020  ext.unwrap()).  

# ...etc.
```

Conclusion
- AST based compression does not appear to be a viable format even with the language separated out and the strings taken.
- This is a surprising conclusion as it would appear there is a lot of wasted space in files
- Now, it is possible that using the strings *alone* and encoding only their numbers might introduce some benefit