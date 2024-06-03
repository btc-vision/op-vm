(module
 (type $0 (func (param i32 i32)))
 (type $1 (func (param i32) (result i32)))
 (type $2 (func (param i32 i32) (result i32)))
 (type $3 (func (param i32 i32 i32)))
 (type $4 (func (param i32)))
 (type $5 (func (result i32)))
 (type $6 (func (param i32 i32 i32) (result i32)))
 (type $7 (func (param i32 i32 i32 i32)))
 (type $8 (func))
 (type $9 (func (param i32 i32 i32 i32) (result i32)))
 (type $10 (func (param i32 i32 i64)))
 (type $11 (func (param i32 i32 i32 i32 i32) (result i32)))
 (type $12 (func (param i64) (result i64)))
 (type $13 (func (param i64 i64) (result i64)))
 (type $14 (func (param i32 i32 i32 i32 i32 i32)))
 (type $15 (func (param i64 i64 i64 i64 i64 i64 i64 i64) (result i32)))
 (type $16 (func (param i64 i64 i64 i64) (result i32)))
 (type $17 (func (param i32) (result i64)))
 (type $18 (func (param i32 i64)))
 (type $19 (func (param i32 i32) (result i64)))
 (type $20 (func (param i32 i32 i32 i32 i32)))
 (import "env" "abort" (func $~lib/builtins/abort (param i32 i32 i32 i32)))
 (global $~lib/rt/itcms/total (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/threshold (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/state (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/visitCount (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/pinSpace (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/iter (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/toSpace (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/white (mut i32) (i32.const 0))
 (global $~lib/rt/itcms/fromSpace (mut i32) (i32.const 0))
 (global $~lib/rt/tlsf/ROOT (mut i32) (i32.const 0))
 (global $src/btc/universal/ABIRegistry/ABIRegistry (mut i32) (i32.const 0))
 (global $src/btc/env/index/Blockchain (mut i32) (i32.const 0))
 (global $src/btc/types/SafeMath/SafeMath.ZERO (mut i32) (i32.const 0))
 (global $src/btc/math/rnd/random_state0_64 (mut i64) (i64.const 0))
 (global $src/btc/math/rnd/random_state1_64 (mut i64) (i64.const 0))
 (global $src/btc/math/rnd/random_state0_32 (mut i32) (i32.const 0))
 (global $src/btc/math/rnd/random_seeded (mut i32) (i32.const 0))
 (global $~argumentsLength (mut i32) (i32.const 0))
 (global $~lib/as-bignum/assembly/globals/__res128_hi (mut i64) (i64.const 0))
 (global $~lib/as-bignum/assembly/globals/__carry (mut i64) (i64.const 0))
 (global $~lib/rt/__rtti_base i32 (i32.const 8624))
 (global $~lib/memory/__stack_pointer (mut i32) (i32.const 41580))
 (memory $0 1)
 (data $0 (i32.const 1036) "\1c\01")
 (data $0.1 (i32.const 1048) "\01\00\00\00\00\01\00\00\98/\8aB\91D7q\cf\fb\c0\b5\a5\db\b5\e9[\c2V9\f1\11\f1Y\a4\82?\92\d5^\1c\ab\98\aa\07\d8\01[\83\12\be\851$\c3}\0cUt]\ber\fe\b1\de\80\a7\06\dc\9bt\f1\9b\c1\c1i\9b\e4\86G\be\ef\c6\9d\c1\0f\cc\a1\0c$o,\e9-\aa\84tJ\dc\a9\b0\\\da\88\f9vRQ>\98m\c61\a8\c8\'\03\b0\c7\7fY\bf\f3\0b\e0\c6G\91\a7\d5Qc\ca\06g))\14\85\n\b7\'8!\1b.\fcm,M\13\r8STs\ne\bb\njv.\c9\c2\81\85,r\92\a1\e8\bf\a2Kf\1a\a8p\8bK\c2\a3Ql\c7\19\e8\92\d1$\06\99\d6\855\0e\f4p\a0j\10\16\c1\a4\19\08l7\1eLwH\'\b5\bc\b04\b3\0c\1c9J\aa\d8NO\ca\9c[\f3o.h\ee\82\8ftoc\a5x\14x\c8\84\08\02\c7\8c\fa\ff\be\90\eblP\a4\f7\a3\f9\be\f2xq\c6")
 (data $1 (i32.const 1324) ",")
 (data $1.1 (i32.const 1336) "\04\00\00\00\10\00\00\00 \04\00\00 \04\00\00\00\01\00\00@")
 (data $2 (i32.const 1372) "<")
 (data $2.1 (i32.const 1384) "\01\00\00\00 \00\00\00j\t\e6g\bbg\ae\85<n\f3r\a5O\f5:Q\0eR\7f\9b\05h\8c\1f\83\d9\ab[\e0\cd\19")
 (data $3 (i32.const 1436) ",")
 (data $3.1 (i32.const 1448) "\05\00\00\00\10\00\00\00p\05\00\00p\05\00\00 \00\00\00 ")
 (data $4 (i32.const 1484) "<")
 (data $4.1 (i32.const 1496) "\02\00\00\00(\00\00\00A\00l\00l\00o\00c\00a\00t\00i\00o\00n\00 \00t\00o\00o\00 \00l\00a\00r\00g\00e")
 (data $5 (i32.const 1548) "<")
 (data $5.1 (i32.const 1560) "\02\00\00\00 \00\00\00~\00l\00i\00b\00/\00r\00t\00/\00i\00t\00c\00m\00s\00.\00t\00s")
 (data $8 (i32.const 1676) "<")
 (data $8.1 (i32.const 1688) "\02\00\00\00$\00\00\00I\00n\00d\00e\00x\00 \00o\00u\00t\00 \00o\00f\00 \00r\00a\00n\00g\00e")
 (data $9 (i32.const 1740) ",")
 (data $9.1 (i32.const 1752) "\02\00\00\00\14\00\00\00~\00l\00i\00b\00/\00r\00t\00.\00t\00s")
 (data $11 (i32.const 1820) "<")
 (data $11.1 (i32.const 1832) "\02\00\00\00\1e\00\00\00~\00l\00i\00b\00/\00r\00t\00/\00t\00l\00s\00f\00.\00t\00s")
 (data $12 (i32.const 1884) ",")
 (data $12.1 (i32.const 1896) "\02\00\00\00\1c\00\00\00I\00n\00v\00a\00l\00i\00d\00 \00l\00e\00n\00g\00t\00h")
 (data $13 (i32.const 1932) "<")
 (data $13.1 (i32.const 1944) "\02\00\00\00&\00\00\00~\00l\00i\00b\00/\00a\00r\00r\00a\00y\00b\00u\00f\00f\00e\00r\00.\00t\00s")
 (data $14 (i32.const 1996) "<")
 (data $14.1 (i32.const 2008) "\02\00\00\00 \00\00\00R\00u\00n\00t\00i\00m\00e\00E\00x\00c\00e\00p\00t\00i\00o\00n")
 (data $15 (i32.const 2060) "\1c")
 (data $15.1 (i32.const 2072) "\10\00\00\00\08\00\00\00\01")
 (data $16 (i32.const 2092) "\1c")
 (data $16.1 (i32.const 2104) "\01")
 (data $17 (i32.const 2124) "L")
 (data $17.1 (i32.const 2136) "\02\00\00\00:\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00e\00n\00v\00/\00B\00T\00C\00E\00n\00v\00i\00r\00o\00n\00m\00e\00n\00t\00.\00t\00s")
 (data $18 (i32.const 2204) "l")
 (data $18.1 (i32.const 2216) "\02\00\00\00R\00\00\00C\00O\00N\00T\00R\00A\00C\00T\00 \00D\00O\00E\00S\00N\00T\00 \00N\00O\00T\00 \00K\00N\00O\00W\00 \00I\00T\00S\00 \00O\00W\00N\00 \00A\00D\00D\00R\00E\00S\00S\00.")
 (data $19 (i32.const 2316) "L")
 (data $19.1 (i32.const 2328) "\02\00\00\006\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00c\00o\00n\00t\00r\00a\00c\00t\00s\00/\00O\00P\00_\00N\00E\00T\00.\00t\00s")
 (data $20 (i32.const 2396) "L")
 (data $20.1 (i32.const 2408) "\02\00\00\00<\00\00\00N\00O\00 \00C\00O\00N\00T\00R\00A\00C\00T\00 \00I\00N\00I\00T\00I\00A\00L\00I\00Z\00E\00R\00 \00F\00O\00U\00N\00D\00.")
 (data $21 (i32.const 2476) "L")
 (data $21.1 (i32.const 2488) "\02\00\00\000\00\00\00C\00O\00N\00T\00R\00A\00C\00T\00 \00A\00L\00R\00E\00A\00D\00Y\00 \00E\00X\00I\00S\00T\00S\00.")
 (data $22 (i32.const 2556) "<")
 (data $22.1 (i32.const 2568) "\02\00\00\00\"\00\00\00O\00w\00n\00e\00r\00 \00i\00s\00 \00r\00e\00q\00u\00i\00r\00e\00d")
 (data $23 (i32.const 2620) "<")
 (data $23.1 (i32.const 2632) "\02\00\00\00 \00\00\00S\00e\00l\00f\00 \00i\00s\00 \00r\00e\00q\00u\00i\00r\00e\00d")
 (data $24 (i32.const 2684) ",")
 (data $24.1 (i32.const 2696) "\02\00\00\00\0e\00\00\00a\00d\00d\00r\00e\00s\00s")
 (data $25 (i32.const 2732) "<")
 (data $25.1 (i32.const 2744) "\02\00\00\00$\00\00\00U\00n\00p\00a\00i\00r\00e\00d\00 \00s\00u\00r\00r\00o\00g\00a\00t\00e")
 (data $26 (i32.const 2796) ",")
 (data $26.1 (i32.const 2808) "\02\00\00\00\1c\00\00\00~\00l\00i\00b\00/\00s\00t\00r\00i\00n\00g\00.\00t\00s")
 (data $27 (i32.const 2844) "<")
 (data $27.1 (i32.const 2856) "\02\00\00\00$\00\00\00~\00l\00i\00b\00/\00t\00y\00p\00e\00d\00a\00r\00r\00a\00y\00.\00t\00s")
 (data $28 (i32.const 2908) ",")
 (data $28.1 (i32.const 2920) "\02\00\00\00\1a\00\00\00~\00l\00i\00b\00/\00a\00r\00r\00a\00y\00.\00t\00s")
 (data $29 (i32.const 2956) "<")
 (data $29.1 (i32.const 2968) "\02\00\00\00&\00\00\00~\00l\00i\00b\00/\00s\00t\00a\00t\00i\00c\00a\00r\00r\00a\00y\00.\00t\00s")
 (data $30 (i32.const 3020) "<")
 (data $30.1 (i32.const 3032) "\02\00\00\00$\00\00\00K\00e\00y\00 \00d\00o\00e\00s\00 \00n\00o\00t\00 \00e\00x\00i\00s\00t")
 (data $31 (i32.const 3084) ",")
 (data $31.1 (i32.const 3096) "\02\00\00\00\16\00\00\00~\00l\00i\00b\00/\00m\00a\00p\00.\00t\00s")
 (data $32 (i32.const 3132) "<")
 (data $32.1 (i32.const 3144) "\02\00\00\00 \00\00\00~\00l\00i\00b\00/\00d\00a\00t\00a\00v\00i\00e\00w\00.\00t\00s")
 (data $33 (i32.const 3196) "\1c")
 (data $33.1 (i32.const 3208) "\01")
 (data $34 (i32.const 3228) "\1c")
 (data $34.1 (i32.const 3240) "\01")
 (data $35 (i32.const 3260) "\1c")
 (data $35.1 (i32.const 3272) "\02\00\00\00\n\00\00\00o\00w\00n\00e\00r")
 (data $36 (i32.const 3292) ",")
 (data $36.1 (i32.const 3304) "\02\00\00\00\1c\00\00\00i\00s\00A\00d\00d\00r\00e\00s\00s\00O\00w\00n\00e\00r")
 (data $37 (i32.const 3340) ",")
 (data $37.1 (i32.const 3352) "\02\00\00\00\0e\00\00\00M\00e\00t\00h\00o\00d\00 ")
 (data $38 (i32.const 3388) "<")
 (data $38.1 (i32.const 3400) "\02\00\00\00 \00\00\00 \00a\00l\00r\00e\00a\00d\00y\00 \00e\00x\00i\00s\00t\00s\00.")
 (data $39 (i32.const 3452) "\1c\00\00\00\03\00\00\00\00\00\00\00$\00\00\00\0c\00\00\00 \r\00\00\00\00\00\00P\r")
 (data $40 (i32.const 3484) "\1c")
 (data $40.1 (i32.const 3496) "\02")
 (data $41 (i32.const 3516) "\\")
 (data $41.1 (i32.const 3528) "\02\00\00\00@\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00u\00n\00i\00v\00e\00r\00s\00a\00l\00/\00A\00B\00I\00R\00e\00g\00i\00s\00t\00r\00y\00.\00t\00s")
 (data $42 (i32.const 3612) "|")
 (data $42.1 (i32.const 3624) "\02\00\00\00^\00\00\00E\00l\00e\00m\00e\00n\00t\00 \00t\00y\00p\00e\00 \00m\00u\00s\00t\00 \00b\00e\00 \00n\00u\00l\00l\00a\00b\00l\00e\00 \00i\00f\00 \00a\00r\00r\00a\00y\00 \00i\00s\00 \00h\00o\00l\00e\00y")
 (data $43 (i32.const 3740) "\1c")
 (data $43.1 (i32.const 3752) "\02\00\00\00\n\00\00\00O\00P\00_\002\000")
 (data $44 (i32.const 3772) "\1c")
 (data $44.1 (i32.const 3784) "\02\00\00\00\04\00\00\00O\00P")
 (data $45 (i32.const 3804) ",")
 (data $45.1 (i32.const 3816) "\02\00\00\00\10\00\00\00M\00o\00t\00o\00s\00w\00a\00p")
 (data $46 (i32.const 3852) "\1c")
 (data $46.1 (i32.const 3864) "\02\00\00\00\08\00\00\00M\00O\00T\00O")
 (data $47 (i32.const 3884) "<")
 (data $47.1 (i32.const 3896) "\02\00\00\00,\00\00\00S\00e\00l\00f\00 \00a\00d\00d\00r\00e\00s\00s\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d")
 (data $48 (i32.const 3948) "\\")
 (data $48.1 (i32.const 3960) "\02\00\00\00@\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00l\00a\00n\00g\00/\00C\00o\00n\00t\00r\00a\00c\00t\00D\00e\00f\00a\00u\00l\00t\00s\00.\00t\00s")
 (data $49 (i32.const 4044) "L")
 (data $49.1 (i32.const 4056) "\02\00\00\00.\00\00\00O\00w\00n\00e\00r\00 \00a\00d\00d\00r\00e\00s\00s\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d")
 (data $50 (i32.const 4124) ",")
 (data $50.1 (i32.const 4136) "\02\00\00\00\12\00\00\00S\00e\00l\00e\00c\00t\00o\00r\00 ")
 (data $51 (i32.const 4172) ",")
 (data $51.1 (i32.const 4184) "\02\00\00\00\16\00\00\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d\00.")
 (data $52 (i32.const 4220) "\1c\00\00\00\03\00\00\00\00\00\00\00$\00\00\00\0c\00\00\000\10\00\00\00\00\00\00`\10")
 (data $53 (i32.const 4252) "|")
 (data $53.1 (i32.const 4264) "\02\00\00\00d\00\00\00t\00o\00S\00t\00r\00i\00n\00g\00(\00)\00 \00r\00a\00d\00i\00x\00 \00a\00r\00g\00u\00m\00e\00n\00t\00 \00m\00u\00s\00t\00 \00b\00e\00 \00b\00e\00t\00w\00e\00e\00n\00 \002\00 \00a\00n\00d\00 \003\006")
 (data $54 (i32.const 4380) "<")
 (data $54.1 (i32.const 4392) "\02\00\00\00&\00\00\00~\00l\00i\00b\00/\00u\00t\00i\00l\00/\00n\00u\00m\00b\00e\00r\00.\00t\00s")
 (data $55 (i32.const 4444) "\1c")
 (data $55.1 (i32.const 4456) "\02\00\00\00\02\00\00\000")
 (data $56 (i32.const 4476) "\\")
 (data $56.1 (i32.const 4488) "\02\00\00\00H\00\00\000\001\002\003\004\005\006\007\008\009\00a\00b\00c\00d\00e\00f\00g\00h\00i\00j\00k\00l\00m\00n\00o\00p\00q\00r\00s\00t\00u\00v\00w\00x\00y\00z")
 (data $57 (i32.const 4572) "<")
 (data $57.1 (i32.const 4584) "\02\00\00\00&\00\00\00C\00o\00n\00t\00r\00a\00c\00t\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d\00.")
 (data $58 (i32.const 4636) "L")
 (data $58.1 (i32.const 4648) "\02\00\00\00<\00\00\00M\00e\00t\00h\00o\00d\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d\00 \00f\00o\00r\00 \00s\00e\00l\00e\00c\00t\00o\00r\00 ")
 (data $59 (i32.const 4716) "L")
 (data $59.1 (i32.const 4728) "\02\00\00\000\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00e\00x\00p\00o\00r\00t\00s\00/\00i\00n\00d\00e\00x\00.\00t\00s")
 (data $60 (i32.const 4796) "<")
 (data $60.1 (i32.const 4808) "\02\00\00\00\"\00\00\00E\00x\00p\00e\00c\00t\00e\00d\00 \00t\00o\00 \00r\00e\00a\00d\00 ")
 (data $61 (i32.const 4860) "<")
 (data $61.1 (i32.const 4872) "\02\00\00\00 \00\00\00 \00b\00y\00t\00e\00s\00 \00b\00u\00t\00 \00r\00e\00a\00d\00 ")
 (data $62 (i32.const 4924) "\1c")
 (data $62.1 (i32.const 4936) "\02\00\00\00\0c\00\00\00 \00b\00y\00t\00e\00s")
 (data $63 (i32.const 4956) ",\00\00\00\03\00\00\00\00\00\00\00$\00\00\00\14\00\00\00\d0\12\00\00\00\00\00\00\10\13\00\00\00\00\00\00P\13")
 (data $64 (i32.const 5004) "L")
 (data $64.1 (i32.const 5016) "\02\00\00\00:\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00b\00u\00f\00f\00e\00r\00/\00B\00y\00t\00e\00s\00R\00e\00a\00d\00e\00r\00.\00t\00s")
 (data $65 (i32.const 5084) "<")
 (data $65.1 (i32.const 5096) "\02\00\00\00 \00\00\00M\00e\00t\00h\00o\00d\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d")
 (data $66 (i32.const 5148) "L")
 (data $66.1 (i32.const 5160) "\02\00\00\00.\00\00\00D\00e\00f\00a\00u\00l\00t\00s\00 \00a\00l\00r\00e\00a\00d\00y\00 \00l\00o\00a\00d\00e\00d")
 (data $67 (i32.const 5228) "L")
 (data $67.1 (i32.const 5240) "\02\00\00\000\00\00\00N\00O\00 \00C\00O\00N\00T\00R\00A\00C\00T\00 \00O\00W\00N\00E\00R\00 \00F\00O\00U\00N\00D\00.")
 (data $68 (i32.const 5308) "\1c\00\00\00\03\00\00\00\00\00\00\00$\00\00\00\0c\00\00\000\10\00\00\00\00\00\00`\10")
 (data $69 (i32.const 5340) "L")
 (data $69.1 (i32.const 5352) "\02\00\00\008\00\00\00V\00i\00e\00w\00 \00n\00o\00t\00 \00f\00o\00u\00n\00d\00 \00f\00o\00r\00 \00s\00e\00l\00e\00c\00t\00o\00r\00 ")
 (data $70 (i32.const 5420) "<")
 (data $70.1 (i32.const 5432) "\02\00\00\00&\00\00\00A\00d\00d\00r\00e\00s\00s\00 \00i\00s\00 \00t\00o\00o\00 \00l\00o\00n\00g")
 (data $71 (i32.const 5484) "L")
 (data $71.1 (i32.const 5496) "\02\00\00\00:\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00b\00u\00f\00f\00e\00r\00/\00B\00y\00t\00e\00s\00W\00r\00i\00t\00e\00r\00.\00t\00s")
 (data $72 (i32.const 5564) "l")
 (data $72.1 (i32.const 5576) "\02\00\00\00R\00\00\00E\00v\00e\00n\00t\00 \00d\00a\00t\00a\00 \00l\00e\00n\00g\00t\00h\00 \00e\00x\00c\00e\00e\00d\00s\00 \00m\00a\00x\00i\00m\00u\00m\00 \00l\00e\00n\00g\00t\00h\00.")
 (data $73 (i32.const 5676) "L")
 (data $73.1 (i32.const 5688) "\02\00\00\004\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00e\00v\00e\00n\00t\00s\00/\00N\00e\00t\00E\00v\00e\00n\00t\00.\00t\00s")
 (data $74 (i32.const 5756) "\1c")
 (data $74.1 (i32.const 5768) "\01")
 (data $75 (i32.const 5788) "\1c")
 (data $75.1 (i32.const 5800) "\01")
 (data $76 (i32.const 5820) "\1c")
 (data $76.1 (i32.const 5832) "\01")
 (data $77 (i32.const 5852) "<")
 (data $77.1 (i32.const 5864) "\02\00\00\00*\00\00\00O\00b\00j\00e\00c\00t\00 \00a\00l\00r\00e\00a\00d\00y\00 \00p\00i\00n\00n\00e\00d")
 (data $78 (i32.const 5916) "<")
 (data $78.1 (i32.const 5928) "\02\00\00\00(\00\00\00O\00b\00j\00e\00c\00t\00 \00i\00s\00 \00n\00o\00t\00 \00p\00i\00n\00n\00e\00d")
 (data $79 (i32.const 5980) ",")
 (data $79.1 (i32.const 5992) "\02\00\00\00\12\00\00\00a\00l\00l\00o\00w\00a\00n\00c\00e")
 (data $80 (i32.const 6028) ",")
 (data $80.1 (i32.const 6040) "\02\00\00\00\0e\00\00\00a\00p\00p\00r\00o\00v\00e")
 (data $81 (i32.const 6076) ",")
 (data $81.1 (i32.const 6088) "\02\00\00\00\12\00\00\00b\00a\00l\00a\00n\00c\00e\00O\00f")
 (data $82 (i32.const 6124) "\1c")
 (data $82.1 (i32.const 6136) "\02\00\00\00\08\00\00\00b\00u\00r\00n")
 (data $83 (i32.const 6156) "\1c")
 (data $83.1 (i32.const 6168) "\02\00\00\00\08\00\00\00m\00i\00n\00t")
 (data $84 (i32.const 6188) ",")
 (data $84.1 (i32.const 6200) "\02\00\00\00\10\00\00\00t\00r\00a\00n\00s\00f\00e\00r")
 (data $85 (i32.const 6236) ",")
 (data $85.1 (i32.const 6248) "\02\00\00\00\18\00\00\00t\00r\00a\00n\00s\00f\00e\00r\00F\00r\00o\00m")
 (data $86 (i32.const 6284) ",")
 (data $86.1 (i32.const 6296) "\02\00\00\00\10\00\00\00d\00e\00c\00i\00m\00a\00l\00s")
 (data $87 (i32.const 6332) "\1c")
 (data $87.1 (i32.const 6344) "\02\00\00\00\08\00\00\00n\00a\00m\00e")
 (data $88 (i32.const 6364) "\1c")
 (data $88.1 (i32.const 6376) "\02\00\00\00\0c\00\00\00s\00y\00m\00b\00o\00l")
 (data $89 (i32.const 6396) ",")
 (data $89.1 (i32.const 6408) "\02\00\00\00\16\00\00\00t\00o\00t\00a\00l\00S\00u\00p\00p\00l\00y")
 (data $90 (i32.const 6444) "<")
 (data $90.1 (i32.const 6456) "\02\00\00\00\"\00\00\00c\00r\00a\00s\00h\00I\00n\00d\00e\00x\00e\00r\00T\00e\00s\00t\001")
 (data $91 (i32.const 6508) "<")
 (data $91.1 (i32.const 6520) "\02\00\00\00\"\00\00\00c\00r\00a\00s\00h\00I\00n\00d\00e\00x\00e\00r\00T\00e\00s\00t\002")
 (data $92 (i32.const 6572) "<")
 (data $92.1 (i32.const 6584) "\02\00\00\00\"\00\00\00c\00r\00a\00s\00h\00I\00n\00d\00e\00x\00e\00r\00T\00e\00s\00t\003")
 (data $93 (i32.const 6636) ",")
 (data $93.1 (i32.const 6648) "\02\00\00\00\1a\00\00\00a\00t\00t\00e\00m\00p\00t\00C\00r\00a\00s\00h\001")
 (data $94 (i32.const 6684) ",")
 (data $94.1 (i32.const 6696) "\02\00\00\00\1a\00\00\00a\00t\00t\00e\00m\00p\00t\00C\00r\00a\00s\00h\002")
 (data $95 (i32.const 6732) "<")
 (data $95.1 (i32.const 6744) "\02\00\00\00\"\00\00\00c\00r\00a\00s\00h\00I\00n\00d\00e\00x\00e\00r\00V\00i\00e\00w\001")
 (data $96 (i32.const 6796) "<")
 (data $96.1 (i32.const 6808) "\02\00\00\00\"\00\00\00c\00r\00a\00s\00h\00I\00n\00d\00e\00x\00e\00r\00V\00i\00e\00w\002")
 (data $97 (i32.const 6860) "<")
 (data $97.1 (i32.const 6872) "\02\00\00\00\"\00\00\00c\00r\00a\00s\00h\00I\00n\00d\00e\00x\00e\00r\00V\00i\00e\00w\003")
 (data $98 (i32.const 6924) ",")
 (data $98.1 (i32.const 6936) "\02\00\00\00\18\00\00\00a\00d\00d\00F\00r\00e\00e\00M\00o\00n\00e\00y")
 (data $99 (i32.const 6972) "L")
 (data $99.1 (i32.const 6984) "\02\00\00\006\00\00\00t\00e\00s\00t\00M\00e\00t\00h\00o\00d\00M\00u\00l\00t\00i\00p\00l\00e\00A\00d\00d\00r\00e\00s\00s\00e\00s")
 (data $100 (i32.const 7052) "L")
 (data $100.1 (i32.const 7064) "\02\00\00\006\00\00\00S\00a\00f\00e\00M\00a\00t\00h\00:\00 \00a\00d\00d\00i\00t\00i\00o\00n\00 \00o\00v\00e\00r\00f\00l\00o\00w")
 (data $101 (i32.const 7132) "L")
 (data $101.1 (i32.const 7144) "\02\00\00\002\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00t\00y\00p\00e\00s\00/\00S\00a\00f\00e\00M\00a\00t\00h\00.\00t\00s")
 (data $102 (i32.const 7212) "\1c")
 (data $102.1 (i32.const 7224) "\02\00\00\00\08\00\00\00M\00i\00n\00t")
 (data $103 (i32.const 7244) "L")
 (data $103.1 (i32.const 7256) "\02\00\00\00<\00\00\00S\00a\00f\00e\00M\00a\00t\00h\00:\00 \00s\00u\00b\00t\00r\00a\00c\00t\00i\00o\00n\00 \00o\00v\00e\00r\00f\00l\00o\00w")
 (data $104 (i32.const 7324) "<")
 (data $104.1 (i32.const 7336) "\02\00\00\00 \00\00\00D\00i\00v\00i\00s\00i\00o\00n\00 \00b\00y\00 \00z\00e\00r\00o")
 (data $105 (i32.const 7388) "\\")
 (data $105.1 (i32.const 7400) "\02\00\00\00B\00\00\00S\00a\00f\00e\00M\00a\00t\00h\00:\00 \00m\00u\00l\00t\00i\00p\00l\00i\00c\00a\00t\00i\00o\00n\00 \00o\00v\00e\00r\00f\00l\00o\00w")
 (data $106 (i32.const 7484) "\\")
 (data $106.1 (i32.const 7496) "\02\00\00\00H\00\00\00a\00b\00c\00d\00e\00f\00g\00h\00i\00j\00k\00l\00m\00n\00o\00p\00q\00r\00s\00t\00u\00v\00w\00x\00y\00z\000\001\002\003\004\005\006\007\008\009")
 (data $107 (i32.const 7580) "L")
 (data $107.1 (i32.const 7592) "\02\00\00\004\00\00\00s\00r\00c\00/\00b\00t\00c\00/\00c\00o\00n\00t\00r\00a\00c\00t\00s\00/\00O\00P\00_\002\000\00.\00t\00s")
 (data $108 (i32.const 7660) "L")
 (data $108.1 (i32.const 7672) "\02\00\00\002\00\00\00I\00n\00s\00u\00f\00f\00i\00c\00i\00e\00n\00t\00 \00t\00o\00t\00a\00l\00 \00s\00u\00p\00p\00l\00y")
 (data $109 (i32.const 7740) "L")
 (data $109.1 (i32.const 7752) "\02\00\00\00.\00\00\00C\00a\00n\00n\00o\00t\00 \00b\00u\00r\00n\00 \00t\00o\00k\00e\00n\00s\00 \00f\00o\00r\00 ")
 (data $110 (i32.const 7820) "\1c")
 (data $110.1 (i32.const 7832) "\02\00\00\00\0c\00\00\00 \00f\00r\00o\00m\00 ")
 (data $111 (i32.const 7852) ",")
 (data $111.1 (i32.const 7864) "\02\00\00\00\10\00\00\00 \00a\00c\00c\00o\00u\00n\00t")
 (data $112 (i32.const 7900) ",\00\00\00\03\00\00\00\00\00\00\00$\00\00\00\14\00\00\00P\1e\00\00\00\00\00\00\a0\1e\00\00\00\00\00\00\c0\1e")
 (data $113 (i32.const 7948) "<")
 (data $113.1 (i32.const 7960) "\02\00\00\00(\00\00\00C\00a\00n\00n\00o\00t\00 \00b\00u\00r\00n\00 \000\00 \00t\00o\00k\00e\00n\00s")
 (data $114 (i32.const 8012) "<")
 (data $114.1 (i32.const 8024) "\02\00\00\00(\00\00\00I\00n\00s\00u\00f\00f\00i\00c\00i\00e\00n\00t\00 \00b\00a\00l\00a\00n\00c\00e")
 (data $115 (i32.const 8076) "L")
 (data $115.1 (i32.const 8088) "\02\00\00\00<\00\00\00C\00a\00n\00 \00n\00o\00t\00 \00m\00i\00n\00t\00 \00f\00r\00o\00m\00 \00s\00e\00l\00f\00 \00a\00c\00c\00o\00u\00n\00t")
 (data $116 (i32.const 8156) "\\")
 (data $116.1 (i32.const 8168) "\02\00\00\00>\00\00\00O\00n\00l\00y\00 \00o\00w\00n\00e\00r\00 \00c\00a\00n\00 \00c\00a\00l\00l\00 \00t\00h\00i\00s\00 \00m\00e\00t\00h\00o\00d")
 (data $117 (i32.const 8252) "\\")
 (data $117.1 (i32.const 8264) "\02\00\00\00D\00\00\00C\00a\00n\00 \00n\00o\00t\00 \00t\00r\00a\00n\00s\00f\00e\00r\00 \00f\00r\00o\00m\00 \00s\00e\00l\00f\00 \00a\00c\00c\00o\00u\00n\00t")
 (data $118 (i32.const 8348) "L")
 (data $118.1 (i32.const 8360) "\02\00\00\004\00\00\00C\00a\00n\00n\00o\00t\00 \00t\00r\00a\00n\00s\00f\00e\00r\00 \00t\00o\00k\00e\00n\00s\00 \00t\00o\00 ")
 (data $119 (i32.const 8428) ",\00\00\00\03\00\00\00\00\00\00\00$\00\00\00\14\00\00\00\b0 \00\00\00\00\00\00\a0\1e\00\00\00\00\00\00\c0\1e")
 (data $120 (i32.const 8476) "L")
 (data $120.1 (i32.const 8488) "\02\00\00\000\00\00\00C\00a\00n\00n\00o\00t\00 \00t\00r\00a\00n\00s\00f\00e\00r\00 \000\00 \00t\00o\00k\00e\00n\00s")
 (data $121 (i32.const 8556) "<")
 (data $121.1 (i32.const 8568) "\02\00\00\00,\00\00\00I\00n\00s\00u\00f\00f\00i\00c\00i\00e\00n\00t\00 \00a\00l\00l\00o\00w\00a\00n\00c\00e")
 (data $122 (i32.const 8624) ".\00\00\00 \00\00\00 \00\00\00 \00\00\00\00\00\00\00\02\01\00\00B")
 (data $122.1 (i32.const 8660) " ")
 (data $122.2 (i32.const 8672) "\08\01\00\00\10A\82\00A\00\00\00\08A\00\00\10A\82")
 (data $122.3 (i32.const 8700) " \00\00\00\10A\82\00\10A\82\00\10A\82")
 (data $122.4 (i32.const 8724) "\02A\00\00\02\t")
 (data $122.5 (i32.const 8740) " \00\00\00\10A\82\00\00\00\00\00\10A\82\00\10A\82\00\10A\82\00\00\00\00\00$\01\00\00\04A\00\00\02A\00\00\02A\00\00\02A\00\00\00\00\00\00\02A\00\00\02A\00\00\02A\00\00\00\00\00\00\02\02")
 (export "getContract" (func $src/index/getContract))
 (export "getViewABI" (func $src/btc/exports/index/getViewABI))
 (export "getEvents" (func $src/btc/exports/index/getEvents))
 (export "getMethodABI" (func $src/btc/exports/index/getMethodABI))
 (export "getWriteMethods" (func $src/btc/exports/index/getWriteMethods))
 (export "getModifiedStorage" (func $src/btc/exports/index/getModifiedStorage))
 (export "initializeStorage" (func $src/btc/exports/index/initializeStorage))
 (export "allocateMemory" (func $src/btc/exports/index/allocateMemory))
 (export "deallocateMemory" (func $src/btc/exports/index/deallocateMemory))
 (export "isInitialized" (func $src/btc/exports/index/isInitialized))
 (export "purgeMemory" (func $src/btc/exports/index/purgeMemory))
 (export "__new" (func $~lib/rt/itcms/__new))
 (export "__pin" (func $~lib/rt/itcms/__pin))
 (export "__unpin" (func $~lib/rt/itcms/__unpin))
 (export "__collect" (func $~lib/rt/itcms/__collect))
 (export "__rtti_base" (global $~lib/rt/__rtti_base))
 (export "memory" (memory $0))
 (export "readMethod" (func $export:src/btc/exports/index/readMethod))
 (export "INIT" (func $export:src/btc/exports/index/INIT))
 (export "readView" (func $export:src/btc/exports/index/readView))
 (export "loadStorage" (func $export:src/btc/exports/index/loadStorage))
 (start $~start)
 (func $~lib/rt/itcms/Object#set:nextWithColor (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=4
 )
 (func $~lib/rt/itcms/Object#set:prev (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=8
 )
 (func $~lib/rt/itcms/initLazy (param $0 i32) (result i32)
  local.get $0
  local.get $0
  call $~lib/rt/itcms/Object#set:nextWithColor
  local.get $0
  local.get $0
  call $~lib/rt/itcms/Object#set:prev
  local.get $0
 )
 (func $~lib/rt/itcms/Object#get:next (param $0 i32) (result i32)
  local.get $0
  i32.load offset=4
  i32.const -4
  i32.and
 )
 (func $~lib/rt/itcms/Object#get:color (param $0 i32) (result i32)
  local.get $0
  i32.load offset=4
  i32.const 3
  i32.and
 )
 (func $~lib/rt/itcms/visitRoots
  (local $0 i32)
  (local $1 i32)
  i32.const 1696
  call $~lib/rt/itcms/__visit
  i32.const 1904
  call $~lib/rt/itcms/__visit
  i32.const 3632
  call $~lib/rt/itcms/__visit
  i32.const 3040
  call $~lib/rt/itcms/__visit
  i32.const 1504
  call $~lib/rt/itcms/__visit
  i32.const 5872
  call $~lib/rt/itcms/__visit
  i32.const 5936
  call $~lib/rt/itcms/__visit
  i32.const 2752
  call $~lib/rt/itcms/__visit
  i32.const 4496
  call $~lib/rt/itcms/__visit
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
  i32.const 2016
  call $~lib/rt/itcms/__visit
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
  global.get $src/btc/types/SafeMath/SafeMath.ZERO
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
  i32.const 1344
  call $~lib/rt/itcms/__visit
  i32.const 1456
  call $~lib/rt/itcms/__visit
  global.get $~lib/rt/itcms/pinSpace
  local.tee $1
  call $~lib/rt/itcms/Object#get:next
  local.set $0
  loop $while-continue|0
   local.get $0
   local.get $1
   i32.ne
   if
    local.get $0
    call $~lib/rt/itcms/Object#get:color
    drop
    local.get $0
    i32.const 20
    i32.add
    call $~lib/rt/__visit_members
    local.get $0
    call $~lib/rt/itcms/Object#get:next
    local.set $0
    br $while-continue|0
   end
  end
 )
 (func $~lib/rt/itcms/Object#set:color (param $0 i32) (param $1 i32)
  local.get $0
  local.get $0
  i32.load offset=4
  i32.const -4
  i32.and
  local.get $1
  i32.or
  call $~lib/rt/itcms/Object#set:nextWithColor
 )
 (func $~lib/rt/itcms/Object#set:next (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  local.get $0
  i32.load offset=4
  i32.const 3
  i32.and
  i32.or
  call $~lib/rt/itcms/Object#set:nextWithColor
 )
 (func $~lib/rt/itcms/Object#unlink (param $0 i32)
  (local $1 i32)
  local.get $0
  call $~lib/rt/itcms/Object#get:next
  local.tee $1
  i32.eqz
  if
   local.get $0
   i32.load offset=8
   drop
   return
  end
  local.get $1
  local.get $0
  i32.load offset=8
  local.tee $0
  call $~lib/rt/itcms/Object#set:prev
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:next
 )
 (func $~lib/rt/itcms/Object#linkTo (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  local.get $1
  i32.load offset=8
  local.set $3
  local.get $0
  local.get $1
  local.get $2
  i32.or
  call $~lib/rt/itcms/Object#set:nextWithColor
  local.get $0
  local.get $3
  call $~lib/rt/itcms/Object#set:prev
  local.get $3
  local.get $0
  call $~lib/rt/itcms/Object#set:next
  local.get $1
  local.get $0
  call $~lib/rt/itcms/Object#set:prev
 )
 (func $~lib/rt/itcms/Object#makeGray (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  local.get $0
  global.get $~lib/rt/itcms/iter
  i32.eq
  if
   local.get $0
   i32.load offset=8
   global.set $~lib/rt/itcms/iter
  end
  local.get $0
  call $~lib/rt/itcms/Object#unlink
  global.get $~lib/rt/itcms/toSpace
  local.set $1
  local.get $0
  i32.load offset=12
  local.tee $2
  i32.const 2
  i32.le_u
  if (result i32)
   i32.const 1
  else
   local.get $2
   i32.const 8624
   i32.load
   i32.gt_u
   if
    i32.const 1696
    i32.const 1760
    i32.const 21
    i32.const 28
    call $~lib/builtins/abort
    unreachable
   end
   local.get $2
   i32.const 2
   i32.shl
   i32.const 8628
   i32.add
   i32.load
   i32.const 32
   i32.and
  end
  local.set $2
  local.get $0
  local.get $1
  global.get $~lib/rt/itcms/white
  i32.eqz
  i32.const 2
  local.get $2
  select
  call $~lib/rt/itcms/Object#linkTo
 )
 (func $~lib/rt/itcms/__visit (param $0 i32)
  local.get $0
  i32.eqz
  if
   return
  end
  local.get $0
  i32.const 20
  i32.sub
  local.tee $0
  call $~lib/rt/itcms/Object#get:color
  global.get $~lib/rt/itcms/white
  i32.eq
  if
   local.get $0
   call $~lib/rt/itcms/Object#makeGray
   global.get $~lib/rt/itcms/visitCount
   i32.const 1
   i32.add
   global.set $~lib/rt/itcms/visitCount
  end
 )
 (func $~lib/rt/itcms/Object#get:size (param $0 i32) (result i32)
  local.get $0
  i32.load
  i32.const -4
  i32.and
  i32.const 4
  i32.add
 )
 (func $~lib/rt/tlsf/Root#set:flMap (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store
 )
 (func $~lib/rt/tlsf/removeBlock (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  local.get $1
  i32.load
  i32.const -4
  i32.and
  local.tee $3
  i32.const 256
  i32.lt_u
  if (result i32)
   local.get $3
   i32.const 4
   i32.shr_u
  else
   i32.const 31
   i32.const 1073741820
   local.get $3
   local.get $3
   i32.const 1073741820
   i32.ge_u
   select
   local.tee $3
   i32.clz
   i32.sub
   local.tee $4
   i32.const 7
   i32.sub
   local.set $2
   local.get $3
   local.get $4
   i32.const 4
   i32.sub
   i32.shr_u
   i32.const 16
   i32.xor
  end
  local.set $4
  local.get $1
  i32.load offset=8
  local.set $5
  local.get $1
  i32.load offset=4
  local.tee $3
  if
   local.get $3
   local.get $5
   call $~lib/rt/itcms/Object#set:prev
  end
  local.get $5
  if
   local.get $5
   local.get $3
   call $~lib/rt/itcms/Object#set:nextWithColor
  end
  local.get $1
  local.get $0
  local.get $2
  i32.const 4
  i32.shl
  local.get $4
  i32.add
  i32.const 2
  i32.shl
  i32.add
  local.tee $1
  i32.load offset=96
  i32.eq
  if
   local.get $1
   local.get $5
   i32.store offset=96
   local.get $5
   i32.eqz
   if
    local.get $0
    local.get $2
    i32.const 2
    i32.shl
    i32.add
    local.tee $1
    i32.load offset=4
    i32.const -2
    local.get $4
    i32.rotl
    i32.and
    local.set $3
    local.get $1
    local.get $3
    i32.store offset=4
    local.get $3
    i32.eqz
    if
     local.get $0
     local.get $0
     i32.load
     i32.const -2
     local.get $2
     i32.rotl
     i32.and
     call $~lib/rt/tlsf/Root#set:flMap
    end
   end
  end
 )
 (func $~lib/rt/tlsf/insertBlock (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  local.get $1
  i32.load
  local.set $3
  local.get $1
  i32.const 4
  i32.add
  local.get $1
  i32.load
  i32.const -4
  i32.and
  i32.add
  local.tee $4
  i32.load
  local.tee $2
  i32.const 1
  i32.and
  if
   local.get $0
   local.get $4
   call $~lib/rt/tlsf/removeBlock
   local.get $1
   local.get $3
   i32.const 4
   i32.add
   local.get $2
   i32.const -4
   i32.and
   i32.add
   local.tee $3
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $1
   i32.const 4
   i32.add
   local.get $1
   i32.load
   i32.const -4
   i32.and
   i32.add
   local.tee $4
   i32.load
   local.set $2
  end
  local.get $3
  i32.const 2
  i32.and
  if
   local.get $1
   i32.const 4
   i32.sub
   i32.load
   local.tee $1
   i32.load
   local.set $6
   local.get $0
   local.get $1
   call $~lib/rt/tlsf/removeBlock
   local.get $1
   local.get $6
   i32.const 4
   i32.add
   local.get $3
   i32.const -4
   i32.and
   i32.add
   local.tee $3
   call $~lib/rt/tlsf/Root#set:flMap
  end
  local.get $4
  local.get $2
  i32.const 2
  i32.or
  call $~lib/rt/tlsf/Root#set:flMap
  local.get $4
  i32.const 4
  i32.sub
  local.get $1
  i32.store
  local.get $0
  local.get $3
  i32.const -4
  i32.and
  local.tee $2
  i32.const 256
  i32.lt_u
  if (result i32)
   local.get $2
   i32.const 4
   i32.shr_u
  else
   i32.const 31
   i32.const 1073741820
   local.get $2
   local.get $2
   i32.const 1073741820
   i32.ge_u
   select
   local.tee $2
   i32.clz
   i32.sub
   local.tee $3
   i32.const 7
   i32.sub
   local.set $5
   local.get $2
   local.get $3
   i32.const 4
   i32.sub
   i32.shr_u
   i32.const 16
   i32.xor
  end
  local.tee $2
  local.get $5
  i32.const 4
  i32.shl
  i32.add
  i32.const 2
  i32.shl
  i32.add
  i32.load offset=96
  local.set $3
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  local.get $1
  local.get $3
  call $~lib/rt/itcms/Object#set:prev
  local.get $3
  if
   local.get $3
   local.get $1
   call $~lib/rt/itcms/Object#set:nextWithColor
  end
  local.get $0
  local.get $5
  i32.const 4
  i32.shl
  local.get $2
  i32.add
  i32.const 2
  i32.shl
  i32.add
  local.get $1
  i32.store offset=96
  local.get $0
  local.get $0
  i32.load
  i32.const 1
  local.get $5
  i32.shl
  i32.or
  call $~lib/rt/tlsf/Root#set:flMap
  local.get $0
  local.get $5
  i32.const 2
  i32.shl
  i32.add
  local.tee $0
  local.get $0
  i32.load offset=4
  i32.const 1
  local.get $2
  i32.shl
  i32.or
  i32.store offset=4
 )
 (func $~lib/rt/tlsf/addMemory (param $0 i32) (param $1 i32) (param $2 i64)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  local.get $1
  i32.const 19
  i32.add
  i32.const -16
  i32.and
  i32.const 4
  i32.sub
  local.set $1
  local.get $0
  i32.load offset=1568
  local.tee $3
  if
   local.get $1
   i32.const 16
   i32.sub
   local.tee $5
   local.get $3
   i32.eq
   if
    local.get $3
    i32.load
    local.set $4
    local.get $5
    local.set $1
   end
  end
  local.get $2
  i32.wrap_i64
  i32.const -16
  i32.and
  local.get $1
  i32.sub
  local.tee $3
  i32.const 20
  i32.lt_u
  if
   return
  end
  local.get $1
  local.get $4
  i32.const 2
  i32.and
  local.get $3
  i32.const 8
  i32.sub
  local.tee $3
  i32.const 1
  i32.or
  i32.or
  call $~lib/rt/tlsf/Root#set:flMap
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:prev
  local.get $1
  i32.const 4
  i32.add
  local.get $3
  i32.add
  local.tee $3
  i32.const 2
  call $~lib/rt/tlsf/Root#set:flMap
  local.get $0
  local.get $3
  i32.store offset=1568
  local.get $0
  local.get $1
  call $~lib/rt/tlsf/insertBlock
 )
 (func $~lib/rt/tlsf/initialize
  (local $0 i32)
  (local $1 i32)
  memory.size
  local.tee $0
  i32.const 0
  i32.le_s
  if (result i32)
   i32.const 1
   local.get $0
   i32.sub
   memory.grow
   i32.const 0
   i32.lt_s
  else
   i32.const 0
  end
  if
   unreachable
  end
  i32.const 41584
  i32.const 0
  call $~lib/rt/tlsf/Root#set:flMap
  i32.const 43152
  i32.const 0
  i32.store
  loop $for-loop|0
   local.get $1
   i32.const 23
   i32.lt_u
   if
    local.get $1
    i32.const 2
    i32.shl
    i32.const 41584
    i32.add
    i32.const 0
    i32.store offset=4
    i32.const 0
    local.set $0
    loop $for-loop|1
     local.get $0
     i32.const 16
     i32.lt_u
     if
      local.get $1
      i32.const 4
      i32.shl
      local.get $0
      i32.add
      i32.const 2
      i32.shl
      i32.const 41584
      i32.add
      i32.const 0
      i32.store offset=96
      local.get $0
      i32.const 1
      i32.add
      local.set $0
      br $for-loop|1
     end
    end
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|0
   end
  end
  i32.const 41584
  i32.const 43156
  memory.size
  i64.extend_i32_s
  i64.const 16
  i64.shl
  call $~lib/rt/tlsf/addMemory
  i32.const 41584
  global.set $~lib/rt/tlsf/ROOT
 )
 (func $~lib/rt/tlsf/__free (param $0 i32)
  (local $1 i32)
  local.get $0
  i32.const 41580
  i32.lt_u
  if
   return
  end
  global.get $~lib/rt/tlsf/ROOT
  i32.eqz
  if
   call $~lib/rt/tlsf/initialize
  end
  local.get $0
  i32.const 4
  i32.sub
  local.set $1
  local.get $0
  i32.const 15
  i32.and
  i32.const 1
  local.get $0
  select
  if (result i32)
   i32.const 1
  else
   local.get $1
   i32.load
   i32.const 1
   i32.and
  end
  drop
  global.get $~lib/rt/tlsf/ROOT
  local.set $0
  local.get $1
  local.get $1
  i32.load
  i32.const 1
  i32.or
  call $~lib/rt/tlsf/Root#set:flMap
  local.get $0
  local.get $1
  call $~lib/rt/tlsf/insertBlock
 )
 (func $~lib/rt/itcms/step (result i32)
  (local $0 i32)
  (local $1 i32)
  block $break|0
   block $case2|0
    block $case1|0
     block $case0|0
      global.get $~lib/rt/itcms/state
      br_table $case0|0 $case1|0 $case2|0 $break|0
     end
     i32.const 1
     global.set $~lib/rt/itcms/state
     i32.const 0
     global.set $~lib/rt/itcms/visitCount
     call $~lib/rt/itcms/visitRoots
     global.get $~lib/rt/itcms/toSpace
     global.set $~lib/rt/itcms/iter
     global.get $~lib/rt/itcms/visitCount
     return
    end
    global.get $~lib/rt/itcms/white
    i32.eqz
    local.set $1
    global.get $~lib/rt/itcms/iter
    call $~lib/rt/itcms/Object#get:next
    local.set $0
    loop $while-continue|1
     local.get $0
     global.get $~lib/rt/itcms/toSpace
     i32.ne
     if
      local.get $0
      global.set $~lib/rt/itcms/iter
      local.get $0
      call $~lib/rt/itcms/Object#get:color
      local.get $1
      i32.ne
      if
       local.get $0
       local.get $1
       call $~lib/rt/itcms/Object#set:color
       i32.const 0
       global.set $~lib/rt/itcms/visitCount
       local.get $0
       i32.const 20
       i32.add
       call $~lib/rt/__visit_members
       global.get $~lib/rt/itcms/visitCount
       return
      end
      local.get $0
      call $~lib/rt/itcms/Object#get:next
      local.set $0
      br $while-continue|1
     end
    end
    i32.const 0
    global.set $~lib/rt/itcms/visitCount
    call $~lib/rt/itcms/visitRoots
    global.get $~lib/rt/itcms/iter
    call $~lib/rt/itcms/Object#get:next
    global.get $~lib/rt/itcms/toSpace
    i32.eq
    if
     global.get $~lib/memory/__stack_pointer
     local.set $0
     loop $while-continue|0
      local.get $0
      i32.const 41580
      i32.lt_u
      if
       local.get $0
       i32.load
       call $~lib/rt/itcms/__visit
       local.get $0
       i32.const 4
       i32.add
       local.set $0
       br $while-continue|0
      end
     end
     global.get $~lib/rt/itcms/iter
     call $~lib/rt/itcms/Object#get:next
     local.set $0
     loop $while-continue|2
      local.get $0
      global.get $~lib/rt/itcms/toSpace
      i32.ne
      if
       local.get $0
       call $~lib/rt/itcms/Object#get:color
       local.get $1
       i32.ne
       if
        local.get $0
        local.get $1
        call $~lib/rt/itcms/Object#set:color
        local.get $0
        i32.const 20
        i32.add
        call $~lib/rt/__visit_members
       end
       local.get $0
       call $~lib/rt/itcms/Object#get:next
       local.set $0
       br $while-continue|2
      end
     end
     global.get $~lib/rt/itcms/fromSpace
     local.set $0
     global.get $~lib/rt/itcms/toSpace
     global.set $~lib/rt/itcms/fromSpace
     local.get $0
     global.set $~lib/rt/itcms/toSpace
     local.get $1
     global.set $~lib/rt/itcms/white
     local.get $0
     call $~lib/rt/itcms/Object#get:next
     global.set $~lib/rt/itcms/iter
     i32.const 2
     global.set $~lib/rt/itcms/state
    end
    global.get $~lib/rt/itcms/visitCount
    return
   end
   global.get $~lib/rt/itcms/iter
   local.tee $0
   global.get $~lib/rt/itcms/toSpace
   i32.ne
   if
    local.get $0
    call $~lib/rt/itcms/Object#get:next
    global.set $~lib/rt/itcms/iter
    local.get $0
    call $~lib/rt/itcms/Object#get:color
    drop
    local.get $0
    i32.const 41580
    i32.lt_u
    if
     local.get $0
     i32.const 0
     call $~lib/rt/itcms/Object#set:nextWithColor
     local.get $0
     i32.const 0
     call $~lib/rt/itcms/Object#set:prev
    else
     global.get $~lib/rt/itcms/total
     local.get $0
     call $~lib/rt/itcms/Object#get:size
     i32.sub
     global.set $~lib/rt/itcms/total
     local.get $0
     i32.const 4
     i32.add
     call $~lib/rt/tlsf/__free
    end
    i32.const 10
    return
   end
   global.get $~lib/rt/itcms/toSpace
   global.get $~lib/rt/itcms/toSpace
   call $~lib/rt/itcms/Object#set:nextWithColor
   global.get $~lib/rt/itcms/toSpace
   global.get $~lib/rt/itcms/toSpace
   call $~lib/rt/itcms/Object#set:prev
   i32.const 0
   global.set $~lib/rt/itcms/state
  end
  i32.const 0
 )
 (func $~lib/rt/tlsf/roundSize (param $0 i32) (result i32)
  local.get $0
  i32.const 1
  i32.const 27
  local.get $0
  i32.clz
  i32.sub
  i32.shl
  i32.add
  i32.const 1
  i32.sub
  local.get $0
  local.get $0
  i32.const 536870910
  i32.lt_u
  select
 )
 (func $~lib/rt/tlsf/searchBlock (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  local.get $1
  i32.const 256
  i32.lt_u
  if (result i32)
   local.get $1
   i32.const 4
   i32.shr_u
  else
   i32.const 31
   local.get $1
   call $~lib/rt/tlsf/roundSize
   local.tee $1
   i32.clz
   i32.sub
   local.tee $3
   i32.const 7
   i32.sub
   local.set $2
   local.get $1
   local.get $3
   i32.const 4
   i32.sub
   i32.shr_u
   i32.const 16
   i32.xor
  end
  local.set $1
  local.get $0
  local.get $2
  i32.const 2
  i32.shl
  i32.add
  i32.load offset=4
  i32.const -1
  local.get $1
  i32.shl
  i32.and
  local.tee $1
  if (result i32)
   local.get $0
   local.get $1
   i32.ctz
   local.get $2
   i32.const 4
   i32.shl
   i32.add
   i32.const 2
   i32.shl
   i32.add
   i32.load offset=96
  else
   local.get $0
   i32.load
   i32.const -1
   local.get $2
   i32.const 1
   i32.add
   i32.shl
   i32.and
   local.tee $1
   if (result i32)
    local.get $0
    local.get $0
    local.get $1
    i32.ctz
    local.tee $0
    i32.const 2
    i32.shl
    i32.add
    i32.load offset=4
    i32.ctz
    local.get $0
    i32.const 4
    i32.shl
    i32.add
    i32.const 2
    i32.shl
    i32.add
    i32.load offset=96
   else
    i32.const 0
   end
  end
 )
 (func $~lib/rt/tlsf/allocateBlock (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  local.get $1
  i32.const 1073741820
  i32.gt_u
  if
   i32.const 1504
   i32.const 1840
   i32.const 461
   i32.const 29
   call $~lib/builtins/abort
   unreachable
  end
  local.get $0
  i32.const 12
  local.get $1
  i32.const 19
  i32.add
  i32.const -16
  i32.and
  i32.const 4
  i32.sub
  local.get $1
  i32.const 12
  i32.le_u
  select
  local.tee $1
  call $~lib/rt/tlsf/searchBlock
  local.tee $2
  i32.eqz
  if
   local.get $1
   i32.const 256
   i32.ge_u
   if (result i32)
    local.get $1
    call $~lib/rt/tlsf/roundSize
   else
    local.get $1
   end
   local.set $2
   memory.size
   local.tee $3
   local.get $2
   i32.const 4
   local.get $0
   i32.load offset=1568
   local.get $3
   i32.const 16
   i32.shl
   i32.const 4
   i32.sub
   i32.ne
   i32.shl
   i32.add
   i32.const 65535
   i32.add
   i32.const -65536
   i32.and
   i32.const 16
   i32.shr_u
   local.tee $2
   local.get $2
   local.get $3
   i32.lt_s
   select
   memory.grow
   i32.const 0
   i32.lt_s
   if
    local.get $2
    memory.grow
    i32.const 0
    i32.lt_s
    if
     unreachable
    end
   end
   local.get $0
   local.get $3
   i32.const 16
   i32.shl
   memory.size
   i64.extend_i32_s
   i64.const 16
   i64.shl
   call $~lib/rt/tlsf/addMemory
   local.get $0
   local.get $1
   call $~lib/rt/tlsf/searchBlock
   local.set $2
  end
  local.get $2
  i32.load
  drop
  local.get $0
  local.get $2
  call $~lib/rt/tlsf/removeBlock
  local.get $2
  i32.load
  local.tee $3
  i32.const -4
  i32.and
  local.get $1
  i32.sub
  local.tee $4
  i32.const 16
  i32.ge_u
  if
   local.get $2
   local.get $1
   local.get $3
   i32.const 2
   i32.and
   i32.or
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $2
   i32.const 4
   i32.add
   local.get $1
   i32.add
   local.tee $1
   local.get $4
   i32.const 4
   i32.sub
   i32.const 1
   i32.or
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $0
   local.get $1
   call $~lib/rt/tlsf/insertBlock
  else
   local.get $2
   local.get $3
   i32.const -2
   i32.and
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $2
   i32.const 4
   i32.add
   local.get $2
   i32.load
   i32.const -4
   i32.and
   i32.add
   local.tee $0
   local.get $0
   i32.load
   i32.const -3
   i32.and
   call $~lib/rt/tlsf/Root#set:flMap
  end
  local.get $2
 )
 (func $~lib/rt/tlsf/__alloc (param $0 i32) (result i32)
  global.get $~lib/rt/tlsf/ROOT
  i32.eqz
  if
   call $~lib/rt/tlsf/initialize
  end
  global.get $~lib/rt/tlsf/ROOT
  local.get $0
  call $~lib/rt/tlsf/allocateBlock
  i32.const 4
  i32.add
 )
 (func $~lib/rt/itcms/Object#set:rtId (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=12
 )
 (func $~lib/rt/itcms/Object#set:rtSize (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=16
 )
 (func $~lib/rt/itcms/__new (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  local.get $0
  i32.const 1073741804
  i32.ge_u
  if
   i32.const 1504
   i32.const 1568
   i32.const 261
   i32.const 31
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/rt/itcms/total
  global.get $~lib/rt/itcms/threshold
  i32.ge_u
  if
   block $__inlined_func$~lib/rt/itcms/interrupt$36
    i32.const 2048
    local.set $2
    loop $do-loop|0
     local.get $2
     call $~lib/rt/itcms/step
     i32.sub
     local.set $2
     global.get $~lib/rt/itcms/state
     i32.eqz
     if
      global.get $~lib/rt/itcms/total
      i64.extend_i32_u
      i64.const 200
      i64.mul
      i64.const 100
      i64.div_u
      i32.wrap_i64
      i32.const 1024
      i32.add
      global.set $~lib/rt/itcms/threshold
      br $__inlined_func$~lib/rt/itcms/interrupt$36
     end
     local.get $2
     i32.const 0
     i32.gt_s
     br_if $do-loop|0
    end
    global.get $~lib/rt/itcms/total
    global.get $~lib/rt/itcms/total
    global.get $~lib/rt/itcms/threshold
    i32.sub
    i32.const 1024
    i32.lt_u
    i32.const 10
    i32.shl
    i32.add
    global.set $~lib/rt/itcms/threshold
   end
  end
  local.get $0
  i32.const 16
  i32.add
  call $~lib/rt/tlsf/__alloc
  i32.const 4
  i32.sub
  local.tee $2
  local.get $1
  call $~lib/rt/itcms/Object#set:rtId
  local.get $2
  local.get $0
  call $~lib/rt/itcms/Object#set:rtSize
  local.get $2
  global.get $~lib/rt/itcms/fromSpace
  global.get $~lib/rt/itcms/white
  call $~lib/rt/itcms/Object#linkTo
  global.get $~lib/rt/itcms/total
  local.get $2
  call $~lib/rt/itcms/Object#get:size
  i32.add
  global.set $~lib/rt/itcms/total
  local.get $2
  i32.const 20
  i32.add
  local.tee $1
  i32.const 0
  local.get $0
  memory.fill
  local.get $1
 )
 (func $~lib/rt/itcms/__link (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  local.get $1
  i32.eqz
  if
   return
  end
  local.get $1
  i32.const 20
  i32.sub
  local.tee $1
  call $~lib/rt/itcms/Object#get:color
  global.get $~lib/rt/itcms/white
  i32.eq
  if
   local.get $0
   i32.const 20
   i32.sub
   local.tee $0
   call $~lib/rt/itcms/Object#get:color
   local.tee $3
   global.get $~lib/rt/itcms/white
   i32.eqz
   i32.eq
   if
    local.get $0
    local.get $1
    local.get $2
    select
    call $~lib/rt/itcms/Object#makeGray
   else
    global.get $~lib/rt/itcms/state
    i32.const 1
    i32.eq
    local.get $3
    i32.const 3
    i32.eq
    i32.and
    if
     local.get $1
     call $~lib/rt/itcms/Object#makeGray
    end
   end
  end
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets" (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries" (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount" (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=20
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#set:allowedWriteMethods (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=12
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:isInitialized (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store8
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=16
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:events (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=20
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $~lib/string/String#get:length (param $0 i32) (result i32)
  local.get $0
  i32.const 20
  i32.sub
  i32.load offset=16
  i32.const 1
  i32.shr_u
 )
 (func $~lib/string/String.UTF8.encodeUnsafe (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  local.get $0
  local.get $1
  i32.const 1
  i32.shl
  i32.add
  local.set $3
  local.get $2
  local.set $1
  loop $while-continue|0
   local.get $0
   local.get $3
   i32.lt_u
   if
    local.get $0
    i32.load16_u
    local.tee $2
    i32.const 128
    i32.lt_u
    if (result i32)
     local.get $1
     local.get $2
     i32.store8
     local.get $1
     i32.const 1
     i32.add
    else
     local.get $2
     i32.const 2048
     i32.lt_u
     if (result i32)
      local.get $1
      local.get $2
      i32.const 6
      i32.shr_u
      i32.const 192
      i32.or
      local.get $2
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.const 8
      i32.shl
      i32.or
      i32.store16
      local.get $1
      i32.const 2
      i32.add
     else
      local.get $2
      i32.const 56320
      i32.lt_u
      local.get $0
      i32.const 2
      i32.add
      local.get $3
      i32.lt_u
      i32.and
      local.get $2
      i32.const 63488
      i32.and
      i32.const 55296
      i32.eq
      i32.and
      if
       local.get $0
       i32.load16_u offset=2
       local.tee $4
       i32.const 64512
       i32.and
       i32.const 56320
       i32.eq
       if
        local.get $1
        local.get $2
        i32.const 1023
        i32.and
        i32.const 10
        i32.shl
        i32.const 65536
        i32.add
        local.get $4
        i32.const 1023
        i32.and
        i32.or
        local.tee $2
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.const 24
        i32.shl
        local.get $2
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.const 16
        i32.shl
        i32.or
        local.get $2
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.const 8
        i32.shl
        i32.or
        local.get $2
        i32.const 18
        i32.shr_u
        i32.const 240
        i32.or
        i32.or
        i32.store
        local.get $1
        i32.const 4
        i32.add
        local.set $1
        local.get $0
        i32.const 4
        i32.add
        local.set $0
        br $while-continue|0
       end
      end
      local.get $1
      local.get $2
      i32.const 12
      i32.shr_u
      i32.const 224
      i32.or
      local.get $2
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.const 8
      i32.shl
      i32.or
      i32.store16
      local.get $1
      local.get $2
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=2
      local.get $1
      i32.const 3
      i32.add
     end
    end
    local.set $1
    local.get $0
    i32.const 2
    i32.add
    local.set $0
    br $while-continue|0
   end
  end
 )
 (func $~lib/arraybuffer/ArrayBuffer#get:byteLength (param $0 i32) (result i32)
  local.get $0
  i32.const 20
  i32.sub
  i32.load offset=16
 )
 (func $~lib/polyfills/bswap<u32> (param $0 i32) (result i32)
  local.get $0
  i32.const -16711936
  i32.and
  i32.const 8
  i32.rotl
  local.get $0
  i32.const 16711935
  i32.and
  i32.const 8
  i32.rotr
  i32.or
 )
 (func $~lib/staticarray/StaticArray<u32>#__uset (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  local.get $2
  i32.store
 )
 (func $~lib/staticarray/StaticArray<u32>#__uget (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  i32.load
 )
 (func $~lib/staticarray/StaticArray<u32>#get:length (param $0 i32) (result i32)
  local.get $0
  i32.const 20
  i32.sub
  i32.load offset=16
  i32.const 2
  i32.shr_u
 )
 (func $~lib/polyfills/bswap<u64> (param $0 i64) (result i64)
  local.get $0
  i64.const 8
  i64.shr_u
  i64.const 71777214294589695
  i64.and
  local.get $0
  i64.const 71777214294589695
  i64.and
  i64.const 8
  i64.shl
  i64.or
  local.tee $0
  i64.const 16
  i64.shr_u
  i64.const 281470681808895
  i64.and
  local.get $0
  i64.const 281470681808895
  i64.and
  i64.const 16
  i64.shl
  i64.or
  i64.const 32
  i64.rotr
 )
 (func $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET> (param $0 i32) (result i32)
  local.get $0
  i32.const -1028477379
  i32.mul
  i32.const 374761397
  i32.add
  i32.const 17
  i32.rotl
  i32.const 668265263
  i32.mul
  local.tee $0
  i32.const 15
  i32.shr_u
  local.get $0
  i32.xor
  i32.const -2048144777
  i32.mul
  local.tee $0
  i32.const 13
  i32.shr_u
  local.get $0
  i32.xor
  i32.const -1028477379
  i32.mul
  local.tee $0
  i32.const 16
  i32.shr_u
  local.get $0
  i32.xor
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#set:trackDataTypes (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store8 offset=12
 )
 (func $~lib/rt/itcms/__renew (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  local.get $1
  local.get $0
  i32.const 20
  i32.sub
  local.tee $3
  i32.load
  i32.const -4
  i32.and
  i32.const 16
  i32.sub
  i32.le_u
  if
   local.get $3
   local.get $1
   call $~lib/rt/itcms/Object#set:rtSize
   local.get $0
   return
  end
  local.get $1
  local.get $3
  i32.load offset=12
  call $~lib/rt/itcms/__new
  local.tee $2
  local.get $0
  local.get $1
  local.get $3
  i32.load offset=16
  local.tee $0
  local.get $0
  local.get $1
  i32.gt_u
  select
  memory.copy
  local.get $2
 )
 (func $~lib/staticarray/StaticArray<~lib/string/String>#__uset (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  local.get $2
  i32.store
  local.get $0
  local.get $2
  i32.const 1
  call $~lib/rt/itcms/__link
 )
 (func $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:pointer" (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store16 offset=24
 )
 (func $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:memoryAllocatorAddress" (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=28
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue" (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=32
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $src/btc/contracts/OP_20/OP_20#set:allowanceMap (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store offset=24
  local.get $0
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
 )
 (func $src/btc/storage/StoredU256/StoredU256#set:pointer (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store16 offset=4
 )
 (func $~lib/util/number/decimalCount32 (param $0 i32) (result i32)
  local.get $0
  i32.const 10
  i32.ge_u
  i32.const 1
  i32.add
  local.get $0
  i32.const 10000
  i32.ge_u
  i32.const 3
  i32.add
  local.get $0
  i32.const 1000
  i32.ge_u
  i32.add
  local.get $0
  i32.const 100
  i32.lt_u
  select
  local.get $0
  i32.const 1000000
  i32.ge_u
  i32.const 6
  i32.add
  local.get $0
  i32.const 1000000000
  i32.ge_u
  i32.const 8
  i32.add
  local.get $0
  i32.const 100000000
  i32.ge_u
  i32.add
  local.get $0
  i32.const 10000000
  i32.lt_u
  select
  local.get $0
  i32.const 100000
  i32.lt_u
  select
 )
 (func $~lib/util/number/utoa_dec_simple<u32> (param $0 i32) (param $1 i32) (param $2 i32)
  loop $do-loop|0
   local.get $0
   local.get $2
   i32.const 1
   i32.sub
   local.tee $2
   i32.const 1
   i32.shl
   i32.add
   local.get $1
   i32.const 10
   i32.rem_u
   i32.const 48
   i32.add
   i32.store16
   local.get $1
   i32.const 10
   i32.div_u
   local.tee $1
   br_if $do-loop|0
  end
 )
 (func $~lib/number/U32#toString (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $folding-inner0
   local.get $0
   i32.eqz
   if
    i32.const 4464
    local.set $1
    br $folding-inner0
   end
   local.get $0
   call $~lib/util/number/decimalCount32
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.const 1
   i32.shl
   i32.const 2
   call $~lib/rt/itcms/__new
   local.tee $1
   i32.store
   local.get $1
   local.get $0
   local.get $2
   call $~lib/util/number/utoa_dec_simple<u32>
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $~lib/number/I32#toString (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $folding-inner0
   local.get $0
   i32.eqz
   if
    i32.const 4464
    local.set $0
    br $folding-inner0
   end
   i32.const 0
   local.get $0
   i32.sub
   local.get $0
   local.get $0
   i32.const 31
   i32.shr_u
   i32.const 1
   i32.shl
   local.tee $1
   select
   local.tee $3
   call $~lib/util/number/decimalCount32
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.const 1
   i32.shl
   local.get $1
   i32.add
   i32.const 2
   call $~lib/rt/itcms/__new
   local.tee $0
   i32.store
   local.get $0
   local.get $1
   i32.add
   local.get $3
   local.get $2
   call $~lib/util/number/utoa_dec_simple<u32>
   local.get $1
   if
    local.get $0
    i32.const 45
    i32.store16
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/math/cyrb53/imul64 (param $0 i64) (param $1 i64) (result i64)
  (local $2 i64)
  (local $3 i64)
  local.get $0
  i64.const 4294967295
  i64.and
  local.tee $2
  local.get $1
  i64.const 4294967295
  i64.and
  local.tee $3
  i64.mul
  local.get $0
  i64.const 32
  i64.shr_u
  local.tee $0
  local.get $3
  i64.mul
  i64.const 32
  i64.shl
  i64.add
  local.get $1
  i64.const 32
  i64.shr_u
  local.tee $1
  local.get $2
  i64.mul
  i64.const 32
  i64.shl
  i64.add
  local.get $0
  local.get $1
  i64.mul
  i64.add
 )
 (func $~lib/rt/itcms/__pin (param $0 i32) (result i32)
  (local $1 i32)
  local.get $0
  if
   local.get $0
   i32.const 20
   i32.sub
   local.tee $1
   call $~lib/rt/itcms/Object#get:color
   i32.const 3
   i32.eq
   if
    i32.const 5872
    i32.const 1568
    i32.const 338
    i32.const 7
    call $~lib/builtins/abort
    unreachable
   end
   local.get $1
   call $~lib/rt/itcms/Object#unlink
   local.get $1
   global.get $~lib/rt/itcms/pinSpace
   i32.const 3
   call $~lib/rt/itcms/Object#linkTo
  end
  local.get $0
 )
 (func $~lib/rt/itcms/__unpin (param $0 i32)
  local.get $0
  i32.eqz
  if
   return
  end
  local.get $0
  i32.const 20
  i32.sub
  local.tee $0
  call $~lib/rt/itcms/Object#get:color
  i32.const 3
  i32.ne
  if
   i32.const 5936
   i32.const 1568
   i32.const 352
   i32.const 5
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/rt/itcms/state
  i32.const 1
  i32.eq
  if
   local.get $0
   call $~lib/rt/itcms/Object#makeGray
  else
   local.get $0
   call $~lib/rt/itcms/Object#unlink
   local.get $0
   global.get $~lib/rt/itcms/fromSpace
   global.get $~lib/rt/itcms/white
   call $~lib/rt/itcms/Object#linkTo
  end
 )
 (func $~lib/rt/itcms/__collect
  global.get $~lib/rt/itcms/state
  i32.const 0
  i32.gt_s
  if
   loop $while-continue|0
    global.get $~lib/rt/itcms/state
    if
     call $~lib/rt/itcms/step
     drop
     br $while-continue|0
    end
   end
  end
  call $~lib/rt/itcms/step
  drop
  loop $while-continue|1
   global.get $~lib/rt/itcms/state
   if
    call $~lib/rt/itcms/step
    drop
    br $while-continue|1
   end
  end
  global.get $~lib/rt/itcms/total
  i64.extend_i32_u
  i64.const 200
  i64.mul
  i64.const 100
  i64.div_u
  i32.wrap_i64
  i32.const 1024
  i32.add
  global.set $~lib/rt/itcms/threshold
 )
 (func $~lib/as-bignum/assembly/globals/__umulq64 (param $0 i64) (param $1 i64) (result i64)
  (local $2 i64)
  (local $3 i64)
  (local $4 i64)
  local.get $0
  i64.const 4294967295
  i64.and
  local.tee $3
  local.get $1
  i64.const 4294967295
  i64.and
  local.tee $4
  i64.mul
  local.set $2
  local.get $1
  i64.const 32
  i64.shr_u
  local.tee $1
  local.get $3
  i64.mul
  local.get $0
  i64.const 32
  i64.shr_u
  local.tee $0
  local.get $4
  i64.mul
  local.get $2
  i64.const 32
  i64.shr_u
  i64.add
  local.tee $3
  i64.const 4294967295
  i64.and
  i64.add
  local.set $4
  local.get $0
  local.get $1
  i64.mul
  local.get $3
  i64.const 32
  i64.shr_u
  i64.add
  local.get $4
  i64.const 32
  i64.shr_u
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $2
  i64.const 4294967295
  i64.and
  local.get $4
  i64.const 32
  i64.shl
  i64.or
 )
 (func $~lib/as-bignum/assembly/globals/__mul256 (param $0 i64) (param $1 i64) (param $2 i64) (param $3 i64) (param $4 i64) (param $5 i64) (param $6 i64) (param $7 i64) (result i32)
  (local $8 i64)
  (local $9 i64)
  (local $10 i64)
  (local $11 i64)
  (local $12 i64)
  local.get $0
  local.get $4
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.set $8
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.tee $9
  local.get $1
  local.get $4
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.tee $10
  i64.add
  local.set $11
  local.get $9
  local.get $10
  i64.and
  local.get $9
  local.get $10
  i64.or
  local.get $11
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.tee $10
  local.get $2
  local.get $4
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.tee $12
  i64.add
  local.set $9
  local.get $10
  local.get $12
  i64.and
  local.get $10
  local.get $12
  i64.or
  local.get $9
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $3
  local.get $4
  i64.mul
  i64.add
  local.set $3
  local.get $0
  local.get $5
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.tee $4
  local.get $11
  i64.add
  local.set $10
  local.get $4
  local.get $11
  i64.and
  local.get $4
  local.get $11
  i64.or
  local.get $10
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.tee $4
  local.get $1
  local.get $5
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.tee $11
  i64.add
  local.set $12
  local.get $4
  local.get $11
  i64.and
  local.get $4
  local.get $11
  i64.or
  local.get $12
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.tee $4
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  local.tee $11
  i64.const -1
  i64.xor
  local.get $4
  i64.and
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  local.get $9
  local.get $12
  i64.and
  local.get $9
  local.get $12
  i64.or
  local.get $9
  local.get $12
  i64.add
  local.tee $4
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  local.get $11
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $3
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $2
  local.get $5
  i64.mul
  i64.add
  i64.add
  local.set $2
  local.get $0
  local.get $6
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.tee $3
  local.get $4
  i64.add
  local.set $5
  local.get $3
  local.get $4
  i64.and
  local.get $3
  local.get $4
  i64.or
  local.get $5
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $2
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $1
  local.get $6
  i64.mul
  i64.add
  i64.add
  local.tee $1
  local.get $0
  local.get $7
  call $~lib/as-bignum/assembly/globals/__umulq64
  local.tee $0
  i64.add
  local.set $2
  local.get $0
  local.get $1
  i64.and
  local.get $0
  local.get $1
  i64.or
  local.get $2
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  global.set $~lib/as-bignum/assembly/globals/__carry
  global.get $~lib/as-bignum/assembly/globals/__res128_hi
  global.get $~lib/as-bignum/assembly/globals/__carry
  i64.add
  global.set $~lib/as-bignum/assembly/globals/__res128_hi
  local.get $8
  local.get $10
  local.get $5
  local.get $2
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
 )
 (func $src/btc/math/rnd/murmurHash3 (param $0 i64) (result i64)
  local.get $0
  local.get $0
  i64.const 33
  i64.shr_u
  i64.xor
  i64.const -49064778989728563
  i64.mul
  local.tee $0
  i64.const 33
  i64.shr_u
  local.get $0
  i64.xor
  i64.const -4265267296055464877
  i64.mul
  local.tee $0
  i64.const 33
  i64.shr_u
  local.get $0
  i64.xor
 )
 (func $src/btc/math/rnd/splitMix32 (param $0 i32) (result i32)
  local.get $0
  i32.const 1831565813
  i32.add
  local.tee $0
  i32.const 1
  i32.or
  local.get $0
  local.get $0
  i32.const 15
  i32.shr_u
  i32.xor
  i32.mul
  local.tee $0
  local.get $0
  i32.const 61
  i32.or
  local.get $0
  local.get $0
  i32.const 7
  i32.shr_u
  i32.xor
  i32.mul
  i32.add
  local.get $0
  i32.xor
  local.tee $0
  i32.const 14
  i32.shr_u
  local.get $0
  i32.xor
 )
 (func $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has@override" (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has"
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override" (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#find"
  i32.const 0
  i32.ne
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override" (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#find"
  local.tee $0
  i32.eqz
  if
   i32.const 3040
   i32.const 3104
   i32.const 105
   i32.const 17
   call $~lib/builtins/abort
   unreachable
  end
  local.get $0
  i32.load offset=4
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set@override" (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  local.get $0
  local.get $1
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set"
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set@override" (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  local.get $0
  local.get $1
  local.get $2
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set"
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get@override" (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get"
 )
 (func $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#set@override" (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  local.get $0
  local.get $1
  local.get $2
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#set"
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#clear@override" (param $0 i32)
  (local $1 i32)
  local.get $0
  i32.const 8
  i32.sub
  i32.load
  drop
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 16
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 3
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 48
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 4
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/arraybuffer/ArrayBufferView~visit (param $0 i32)
  local.get $0
  i32.load
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
 )
 (func $src/btc/contracts/OP_NET/OP_NET~visit (param $0 i32)
  (local $1 i32)
  local.get $0
  i32.load
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=4
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=8
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>~visit" (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  call $~lib/rt/itcms/__visit
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.tee $2
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=16
  i32.const 12
  i32.mul
  i32.add
  local.set $0
  loop $while-continue|0
   local.get $0
   local.get $1
   i32.gt_u
   if
    local.get $1
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     local.get $1
     i32.load
     call $~lib/rt/itcms/__visit
     local.get $1
     i32.load offset=4
     call $~lib/rt/itcms/__visit
    end
    local.get $1
    i32.const 12
    i32.add
    local.set $1
    br $while-continue|0
   end
  end
  local.get $2
  call $~lib/rt/itcms/__visit
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/lang/ContractDefaults/ContractDefaults~visit (param $0 i32)
  (local $1 i32)
  local.get $0
  i32.load
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=4
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
 )
 (func $src/btc/contracts/OP_20/OP_20~visit (param $0 i32)
  (local $1 i32)
  local.get $0
  call $src/btc/contracts/OP_NET/OP_NET~visit
  local.get $0
  i32.load offset=16
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=20
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=32
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=24
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=28
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
 )
 (func $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>~visit" (param $0 i32)
  (local $1 i32)
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>~visit"
  local.get $0
  i32.load offset=28
  local.tee $1
  if
   local.get $1
   call $~lib/rt/itcms/__visit
  end
  local.get $0
  i32.load offset=32
  local.tee $0
  if
   local.get $0
   call $~lib/rt/itcms/__visit
  end
 )
 (func $~lib/rt/__visit_members (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  block $folding-inner7
   block $folding-inner6
    block $folding-inner5
     block $folding-inner4
      block $folding-inner3
       block $folding-inner2
        block $folding-inner1
         block $folding-inner0
          block $invalid
           block $src/contract/events/MintEvent/MintEvent
            block $~lib/staticarray/StaticArray<~lib/string/String>
             block $~lib/staticarray/StaticArray<u32>
              block $src/btc/storage/StoredU256/StoredU256
               block $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>"
                block $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>"
                 block $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>"
                  block $src/btc/contracts/interfaces/IOP_20/IOP_20
                   block $src/btc/contracts/OP_20/OP_20
                    block $src/contract/TestingContract/TestingContract
                     block $src/btc/events/NetEvent/NetEvent
                      block $src/btc/lang/ContractDefaults/ContractDefaults
                       block $~lib/as-bignum/assembly/integer/u256/u256
                        block $src/btc/env/BTCEnvironment/BlockchainEnvironment
                         block $~lib/function/Function<%28~lib/string/String%29=>void>
                          block $~lib/set/Set<~lib/typedarray/Uint8Array>
                           block $~lib/typedarray/Uint8Array
                            block $~lib/set/Set<u32>
                             block $src/btc/buffer/BytesWriter/BytesWriter
                              block $src/btc/interfaces/IBTC/IBTC
                               block $src/btc/contracts/OP_NET/OP_NET
                                block $src/btc/universal/ABIRegistry/ABIRegistryBase
                                 block $~lib/string/String
                                  block $~lib/arraybuffer/ArrayBuffer
                                   block $~lib/object/Object
                                    local.get $0
                                    i32.const 8
                                    i32.sub
                                    i32.load
                                    br_table $~lib/object/Object $~lib/arraybuffer/ArrayBuffer $~lib/string/String $folding-inner0 $folding-inner1 $folding-inner1 $src/btc/universal/ABIRegistry/ABIRegistryBase $src/btc/contracts/OP_NET/OP_NET $src/btc/interfaces/IBTC/IBTC $src/btc/buffer/BytesWriter/BytesWriter $folding-inner0 $~lib/set/Set<u32> $folding-inner2 $~lib/typedarray/Uint8Array $~lib/set/Set<~lib/typedarray/Uint8Array> $folding-inner2 $~lib/function/Function<%28~lib/string/String%29=>void> $src/btc/env/BTCEnvironment/BlockchainEnvironment $~lib/as-bignum/assembly/integer/u256/u256 $folding-inner2 $folding-inner2 $folding-inner2 $src/btc/lang/ContractDefaults/ContractDefaults $src/btc/events/NetEvent/NetEvent $folding-inner3 $folding-inner1 $src/contract/TestingContract/TestingContract $src/btc/contracts/OP_20/OP_20 $src/btc/contracts/interfaces/IOP_20/IOP_20 $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>" $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>" $folding-inner2 $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>" $folding-inner2 $src/btc/storage/StoredU256/StoredU256 $~lib/staticarray/StaticArray<u32> $~lib/staticarray/StaticArray<~lib/string/String> $folding-inner3 $folding-inner3 $folding-inner3 $folding-inner0 $folding-inner3 $folding-inner3 $folding-inner3 $src/contract/events/MintEvent/MintEvent $folding-inner1 $invalid
                                   end
                                   return
                                  end
                                  return
                                 end
                                 return
                                end
                                local.get $0
                                i32.load
                                local.tee $1
                                if
                                 local.get $1
                                 call $~lib/rt/itcms/__visit
                                end
                                local.get $0
                                i32.load offset=4
                                local.tee $1
                                if
                                 local.get $1
                                 call $~lib/rt/itcms/__visit
                                end
                                br $folding-inner4
                               end
                               local.get $0
                               call $src/btc/contracts/OP_NET/OP_NET~visit
                               return
                              end
                              return
                             end
                             local.get $0
                             i32.load offset=4
                             local.tee $1
                             if
                              local.get $1
                              call $~lib/rt/itcms/__visit
                             end
                             local.get $0
                             i32.load offset=8
                             local.tee $0
                             if
                              local.get $0
                              call $~lib/rt/itcms/__visit
                             end
                             return
                            end
                            global.get $~lib/memory/__stack_pointer
                            i32.const 4
                            i32.sub
                            global.set $~lib/memory/__stack_pointer
                            call $~stack_check
                            global.get $~lib/memory/__stack_pointer
                            i32.const 0
                            i32.store
                            global.get $~lib/memory/__stack_pointer
                            local.get $0
                            i32.store
                            local.get $0
                            i32.load
                            call $~lib/rt/itcms/__visit
                            global.get $~lib/memory/__stack_pointer
                            local.get $0
                            i32.store
                            local.get $0
                            i32.load offset=8
                            call $~lib/rt/itcms/__visit
                            br $folding-inner7
                           end
                           local.get $0
                           call $~lib/arraybuffer/ArrayBufferView~visit
                           return
                          end
                          global.get $~lib/memory/__stack_pointer
                          i32.const 4
                          i32.sub
                          global.set $~lib/memory/__stack_pointer
                          call $~stack_check
                          global.get $~lib/memory/__stack_pointer
                          i32.const 0
                          i32.store
                          global.get $~lib/memory/__stack_pointer
                          local.get $0
                          i32.store
                          local.get $0
                          i32.load
                          call $~lib/rt/itcms/__visit
                          global.get $~lib/memory/__stack_pointer
                          local.get $0
                          i32.store
                          local.get $0
                          i32.load offset=8
                          local.tee $2
                          local.set $1
                          global.get $~lib/memory/__stack_pointer
                          local.get $0
                          i32.store
                          local.get $1
                          local.get $0
                          i32.load offset=16
                          i32.const 3
                          i32.shl
                          i32.add
                          local.set $0
                          loop $while-continue|0
                           local.get $0
                           local.get $1
                           i32.gt_u
                           if
                            local.get $1
                            i32.load offset=4
                            i32.const 1
                            i32.and
                            i32.eqz
                            if
                             local.get $1
                             i32.load
                             call $~lib/rt/itcms/__visit
                            end
                            local.get $1
                            i32.const 8
                            i32.add
                            local.set $1
                            br $while-continue|0
                           end
                          end
                          local.get $2
                          call $~lib/rt/itcms/__visit
                          br $folding-inner7
                         end
                         global.get $~lib/memory/__stack_pointer
                         i32.const 4
                         i32.sub
                         global.set $~lib/memory/__stack_pointer
                         call $~stack_check
                         global.get $~lib/memory/__stack_pointer
                         i32.const 0
                         i32.store
                         global.get $~lib/memory/__stack_pointer
                         local.get $0
                         i32.store
                         local.get $0
                         i32.load offset=4
                         call $~lib/rt/itcms/__visit
                         br $folding-inner7
                        end
                        local.get $0
                        i32.load offset=4
                        local.tee $1
                        if
                         local.get $1
                         call $~lib/rt/itcms/__visit
                        end
                        local.get $0
                        i32.load offset=8
                        local.tee $1
                        if
                         local.get $1
                         call $~lib/rt/itcms/__visit
                        end
                        local.get $0
                        i32.load offset=12
                        local.tee $1
                        if
                         local.get $1
                         call $~lib/rt/itcms/__visit
                        end
                        br $folding-inner5
                       end
                       return
                      end
                      local.get $0
                      call $src/btc/lang/ContractDefaults/ContractDefaults~visit
                      return
                     end
                     local.get $0
                     call $src/btc/lang/ContractDefaults/ContractDefaults~visit
                     return
                    end
                    local.get $0
                    call $src/btc/contracts/OP_20/OP_20~visit
                    br $folding-inner5
                   end
                   local.get $0
                   call $src/btc/contracts/OP_20/OP_20~visit
                   return
                  end
                  return
                 end
                 local.get $0
                 call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>~visit"
                 return
                end
                local.get $0
                i32.load
                local.tee $1
                if
                 local.get $1
                 call $~lib/rt/itcms/__visit
                end
                br $folding-inner4
               end
               local.get $0
               call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>~visit"
               return
              end
              local.get $0
              i32.load
              local.tee $1
              if
               local.get $1
               call $~lib/rt/itcms/__visit
              end
              local.get $0
              i32.load offset=8
              local.tee $1
              if
               local.get $1
               call $~lib/rt/itcms/__visit
              end
              local.get $0
              i32.load offset=12
              local.tee $1
              if
               local.get $1
               call $~lib/rt/itcms/__visit
              end
              local.get $0
              i32.load offset=16
              local.tee $0
              if
               local.get $0
               call $~lib/rt/itcms/__visit
              end
              return
             end
             return
            end
            local.get $0
            local.get $0
            i32.const 20
            i32.sub
            i32.load offset=16
            i32.add
            local.set $1
            loop $while-continue|00
             local.get $0
             local.get $1
             i32.lt_u
             if
              local.get $0
              i32.load
              local.tee $2
              if
               local.get $2
               call $~lib/rt/itcms/__visit
              end
              local.get $0
              i32.const 4
              i32.add
              local.set $0
              br $while-continue|00
             end
            end
            return
           end
           local.get $0
           call $src/btc/lang/ContractDefaults/ContractDefaults~visit
           return
          end
          unreachable
         end
         local.get $0
         call $~lib/arraybuffer/ArrayBufferView~visit
         return
        end
        global.get $~lib/memory/__stack_pointer
        i32.const 4
        i32.sub
        global.set $~lib/memory/__stack_pointer
        call $~stack_check
        global.get $~lib/memory/__stack_pointer
        i32.const 0
        i32.store
        br $folding-inner6
       end
       local.get $0
       call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>~visit"
       return
      end
      global.get $~lib/memory/__stack_pointer
      i32.const 4
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i32.load offset=4
      local.set $1
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $1
      local.get $0
      i32.load offset=12
      i32.const 2
      i32.shl
      i32.add
      local.set $2
      loop $while-continue|01
       local.get $1
       local.get $2
       i32.lt_u
       if
        local.get $1
        i32.load
        local.tee $3
        if
         local.get $3
         call $~lib/rt/itcms/__visit
        end
        local.get $1
        i32.const 4
        i32.add
        local.set $1
        br $while-continue|01
       end
      end
      br $folding-inner6
     end
     local.get $0
     i32.load offset=8
     local.tee $1
     if
      local.get $1
      call $~lib/rt/itcms/__visit
     end
     local.get $0
     i32.load offset=12
     local.tee $0
     if
      local.get $0
      call $~lib/rt/itcms/__visit
     end
     return
    end
    local.get $0
    i32.load offset=16
    local.tee $1
    if
     local.get $1
     call $~lib/rt/itcms/__visit
    end
    local.get $0
    i32.load offset=20
    local.tee $0
    if
     local.get $0
     call $~lib/rt/itcms/__visit
    end
    return
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   call $~lib/rt/itcms/__visit
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~start
  memory.size
  i32.const 16
  i32.shl
  i32.const 41580
  i32.sub
  i32.const 1
  i32.shr_u
  global.set $~lib/rt/itcms/threshold
  i32.const 1616
  call $~lib/rt/itcms/initLazy
  global.set $~lib/rt/itcms/pinSpace
  i32.const 1648
  call $~lib/rt/itcms/initLazy
  global.set $~lib/rt/itcms/toSpace
  i32.const 1792
  call $~lib/rt/itcms/initLazy
  global.set $~lib/rt/itcms/fromSpace
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#constructor
  global.set $src/btc/universal/ABIRegistry/ABIRegistry
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#constructor
  global.set $src/btc/env/index/Blockchain
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  global.set $src/btc/types/SafeMath/SafeMath.ZERO
 )
 (func $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 8812
  i32.lt_s
  if
   i32.const 41600
   i32.const 41648
   i32.const 1
   i32.const 1
   call $~lib/builtins/abort
   unreachable
  end
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor" (result i32)
  i32.const 48
  i32.const 12
  call $"byn$mgfn-shared$~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#constructor (result i32)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.const 6
  call $~lib/rt/itcms/__new
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  local.get $0
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 0
   i32.const 0
   call $~lib/rt/itcms/__new
   local.tee $0
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.const 15
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 16
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 3
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 48
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 4
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:allowedWriteMethods
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#constructor" (result i32)
  i32.const 48
  i32.const 20
  call $"byn$mgfn-shared$~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#constructor (result i32)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.const 17
  call $~lib/rt/itcms/__new
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:isInitialized
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#constructor"
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#constructor"
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.const 21
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 16
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 3
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 48
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 4
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:allowedWriteMethods
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 9
  i32.const 22
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  i32.store8 offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  i32.const 0
  i32.const 2
  i32.const 24
  i32.const 2112
  call $~lib/rt/__newArray
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:events
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/as-bignum/assembly/integer/u256/u256#constructor (param $0 i64) (param $1 i64) (param $2 i64) (param $3 i64) (result i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.const 18
  call $~lib/rt/itcms/__new
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  local.get $0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  local.get $1
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  local.get $2
  i64.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  local.get $3
  i64.store offset=24
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $4
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization (param $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u
  i32.eqz
  if
   i32.const 0
   i32.const 2144
   i32.const 34
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/string/String.__not (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  local.get $0
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/string/String#get:length
   i32.eqz
  else
   i32.const 1
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/util/hash/HASH<~lib/string/String> (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  block $~lib/util/hash/hashStr|inlined.0 (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   i32.const 0
   local.get $0
   i32.eqz
   br_if $~lib/util/hash/hashStr|inlined.0
   drop
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.tee $1
   call $~lib/string/String#get:length
   i32.const 1
   i32.shl
   local.tee $3
   i32.const 16
   i32.ge_u
   if (result i32)
    i32.const 606290984
    local.set $2
    i32.const -2048144777
    local.set $4
    i32.const 1640531535
    local.set $5
    local.get $1
    local.get $3
    i32.add
    i32.const 16
    i32.sub
    local.set $7
    loop $while-continue|0
     local.get $1
     local.get $7
     i32.le_u
     if
      local.get $2
      local.get $1
      i32.load
      i32.const -2048144777
      i32.mul
      i32.add
      i32.const 13
      i32.rotl
      i32.const -1640531535
      i32.mul
      local.set $2
      local.get $4
      local.get $1
      i32.load offset=4
      i32.const -2048144777
      i32.mul
      i32.add
      i32.const 13
      i32.rotl
      i32.const -1640531535
      i32.mul
      local.set $4
      local.get $6
      local.get $1
      i32.load offset=8
      i32.const -2048144777
      i32.mul
      i32.add
      i32.const 13
      i32.rotl
      i32.const -1640531535
      i32.mul
      local.set $6
      local.get $5
      local.get $1
      i32.load offset=12
      i32.const -2048144777
      i32.mul
      i32.add
      i32.const 13
      i32.rotl
      i32.const -1640531535
      i32.mul
      local.set $5
      local.get $1
      i32.const 16
      i32.add
      local.set $1
      br $while-continue|0
     end
    end
    local.get $3
    local.get $2
    i32.const 1
    i32.rotl
    local.get $4
    i32.const 7
    i32.rotl
    i32.add
    local.get $6
    i32.const 12
    i32.rotl
    i32.add
    local.get $5
    i32.const 18
    i32.rotl
    i32.add
    i32.add
   else
    local.get $3
    i32.const 374761393
    i32.add
   end
   local.set $2
   local.get $0
   local.get $3
   i32.add
   i32.const 4
   i32.sub
   local.set $4
   loop $while-continue|1
    local.get $1
    local.get $4
    i32.le_u
    if
     local.get $2
     local.get $1
     i32.load
     i32.const -1028477379
     i32.mul
     i32.add
     i32.const 17
     i32.rotl
     i32.const 668265263
     i32.mul
     local.set $2
     local.get $1
     i32.const 4
     i32.add
     local.set $1
     br $while-continue|1
    end
   end
   local.get $0
   local.get $3
   i32.add
   local.set $0
   loop $while-continue|2
    local.get $0
    local.get $1
    i32.gt_u
    if
     local.get $2
     local.get $1
     i32.load8_u
     i32.const 374761393
     i32.mul
     i32.add
     i32.const 11
     i32.rotl
     i32.const -1640531535
     i32.mul
     local.set $2
     local.get $1
     i32.const 1
     i32.add
     local.set $1
     br $while-continue|2
    end
   end
   local.get $2
   local.get $2
   i32.const 15
   i32.shr_u
   i32.xor
   i32.const -2048144777
   i32.mul
   local.tee $0
   i32.const 13
   i32.shr_u
   local.get $0
   i32.xor
   i32.const -1028477379
   i32.mul
   local.tee $0
   i32.const 16
   i32.shr_u
   local.get $0
   i32.xor
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/string/String.__eq (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  block $folding-inner0
   local.get $0
   local.get $1
   i32.eq
   if
    i32.const 1
    local.set $2
    br $folding-inner0
   end
   local.get $1
   i32.eqz
   local.get $0
   i32.eqz
   i32.or
   br_if $folding-inner0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/string/String#get:length
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $1
   call $~lib/string/String#get:length
   local.get $2
   i32.ne
   if
    i32.const 0
    local.set $2
    br $folding-inner0
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   block $__inlined_func$~lib/util/string/compareImpl$84
    loop $while-continue|0
     local.get $2
     local.tee $3
     i32.const 1
     i32.sub
     local.set $2
     local.get $3
     if
      local.get $0
      i32.load16_u
      local.tee $3
      local.get $1
      i32.load16_u
      local.tee $5
      i32.sub
      local.set $4
      local.get $3
      local.get $5
      i32.ne
      br_if $__inlined_func$~lib/util/string/compareImpl$84
      local.get $0
      i32.const 2
      i32.add
      local.set $0
      local.get $1
      i32.const 2
      i32.add
      local.set $1
      br $while-continue|0
     end
    end
    i32.const 0
    local.set $4
   end
   local.get $4
   i32.eqz
   local.set $2
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#find" (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $3
  local.get $2
  local.get $0
  i32.load offset=4
  i32.and
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.set $0
  block $folding-inner0
   loop $while-continue|0
    local.get $0
    if
     local.get $0
     i32.load offset=8
     local.tee $2
     i32.const 1
     i32.and
     if (result i32)
      i32.const 0
     else
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.load
      local.tee $3
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=4
      local.get $3
      local.get $1
      call $~lib/string/String.__eq
     end
     br_if $folding-inner0
     local.get $2
     i32.const -2
     i32.and
     local.set $0
     br $while-continue|0
    end
   end
   i32.const 0
   local.set $0
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has" (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $1
  call $~lib/util/hash/HASH<~lib/string/String>
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#find"
  i32.const 0
  i32.ne
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/string/String.UTF8.encode@varargs (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $2of2
   block $outOfRange
    global.get $~argumentsLength
    i32.const 1
    i32.sub
    br_table $2of2 $2of2 $2of2 $outOfRange
   end
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.tee $1
  i32.const 20
  i32.sub
  i32.load offset=16
  local.get $1
  i32.add
  local.set $3
  loop $while-continue|0
   local.get $1
   local.get $3
   i32.lt_u
   if
    local.get $1
    i32.load16_u
    local.tee $4
    i32.const 128
    i32.lt_u
    if (result i32)
     local.get $2
     i32.const 1
     i32.add
    else
     local.get $4
     i32.const 2048
     i32.lt_u
     if (result i32)
      local.get $2
      i32.const 2
      i32.add
     else
      local.get $4
      i32.const 64512
      i32.and
      i32.const 55296
      i32.eq
      local.get $1
      i32.const 2
      i32.add
      local.get $3
      i32.lt_u
      i32.and
      if
       local.get $1
       i32.load16_u offset=2
       i32.const 64512
       i32.and
       i32.const 56320
       i32.eq
       if
        local.get $2
        i32.const 4
        i32.add
        local.set $2
        local.get $1
        i32.const 4
        i32.add
        local.set $1
        br $while-continue|0
       end
      end
      local.get $2
      i32.const 3
      i32.add
     end
    end
    local.set $2
    local.get $1
    i32.const 2
    i32.add
    local.set $1
    br $while-continue|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.const 1
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $0
  call $~lib/string/String#get:length
  local.get $1
  call $~lib/string/String.UTF8.encodeUnsafe
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $~lib/typedarray/Uint8Array.wrap@varargs (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $2of2
   block $1of2
    block $outOfRange
     global.get $~argumentsLength
     i32.const 1
     i32.sub
     br_table $1of2 $1of2 $2of2 $outOfRange
    end
    unreachable
   end
   i32.const -1
   local.set $2
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $~lib/arraybuffer/ArrayBuffer#get:byteLength
  local.set $3
  local.get $2
  local.tee $0
  i32.const 0
  i32.lt_s
  if
   local.get $0
   i32.const -1
   i32.eq
   if (result i32)
    local.get $3
   else
    i32.const 1904
    i32.const 2864
    i32.const 1869
    i32.const 7
    call $~lib/builtins/abort
    unreachable
   end
   local.set $0
  else
   local.get $0
   local.get $3
   i32.gt_s
   if
    i32.const 1904
    i32.const 2864
    i32.const 1874
    i32.const 7
    call $~lib/builtins/abort
    unreachable
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.const 13
  call $~lib/rt/itcms/__new
  local.tee $2
  i32.store offset=8
  local.get $2
  local.get $1
  i32.store
  local.get $2
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/__link
  local.get $2
  local.get $0
  i32.store offset=8
  local.get $2
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $~lib/arraybuffer/ArrayBufferView#constructor (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  local.get $0
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 12
   i32.const 3
   call $~lib/rt/itcms/__new
   local.tee $0
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $~lib/rt/itcms/Object#set:prev
  local.get $1
  i32.const 1073741820
  i32.gt_u
  if
   i32.const 1904
   i32.const 1952
   i32.const 19
   i32.const 57
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 1
  call $~lib/rt/itcms/__new
  local.tee $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  local.get $0
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $2
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/typedarray/Uint8Array#constructor (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.const 13
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  local.get $0
  call $~lib/arraybuffer/ArrayBufferView#constructor
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/typedarray/Uint8Array#get:length (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<u8>#__get (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   i32.const 1696
   i32.const 2928
   i32.const 114
   i32.const 42
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.add
  i32.load8_u
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/typedarray/Uint8Array#__set (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  local.get $1
  local.get $2
  i32.const 45
  i32.const 178
  i32.const 2864
  call $byn$mgfn-shared$~lib/typedarray/Uint8Array#__set
 )
 (func $~lib/typedarray/Uint8Array#subarray (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $~lib/typedarray/Uint8Array#get:length
  local.set $3
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.const 13
  call $~lib/rt/itcms/__new
  local.tee $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $4
  local.get $0
  i32.load
  local.tee $5
  i32.store
  local.get $4
  local.get $5
  i32.const 0
  call $~lib/rt/itcms/__link
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $4
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 0
  i32.lt_s
  if (result i32)
   local.get $1
   local.get $3
   i32.add
   local.tee $0
   i32.const 0
   local.get $0
   i32.const 0
   i32.gt_s
   select
  else
   local.get $1
   local.get $3
   local.get $1
   local.get $3
   i32.lt_s
   select
  end
  local.tee $0
  i32.add
  i32.store offset=4
  local.get $4
  local.get $2
  i32.const 0
  i32.lt_s
  if (result i32)
   local.get $2
   local.get $3
   i32.add
   local.tee $1
   i32.const 0
   local.get $1
   i32.const 0
   i32.gt_s
   select
  else
   local.get $2
   local.get $3
   local.get $2
   local.get $3
   i32.lt_s
   select
  end
  local.tee $1
  local.get $0
  local.get $0
  local.get $1
  i32.lt_s
  select
  local.get $0
  i32.sub
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $4
 )
 (func $~lib/typedarray/Uint8Array#subarray@varargs (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $2of2
   block $1of2
    block $0of2
     block $outOfRange
      global.get $~argumentsLength
      br_table $0of2 $1of2 $2of2 $outOfRange
     end
     unreachable
    end
    i32.const 0
    local.set $1
   end
   i32.const 2147483647
   local.set $2
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  local.get $2
  call $~lib/typedarray/Uint8Array#subarray
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/math/sha256/setU8 (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i32.load offset=4
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $0
  local.get $2
  local.get $1
  call $~lib/typedarray/Uint8Array#get:length
  memory.copy
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/math/sha256/load32_be (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.get $1
  i32.add
  i32.load
  call $~lib/polyfills/bswap<u32>
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<u32>#slice (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 1344
  i32.store
  i32.const 1356
  i32.load
  local.set $2
  local.get $0
  i32.const 0
  i32.lt_s
  if (result i32)
   local.get $0
   local.get $2
   i32.add
   local.tee $0
   i32.const 0
   local.get $0
   i32.const 0
   i32.gt_s
   select
  else
   local.get $0
   local.get $2
   local.get $0
   local.get $2
   i32.lt_s
   select
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 0
  i32.lt_s
  if (result i32)
   local.get $1
   local.get $2
   i32.add
   local.tee $1
   i32.const 0
   local.get $1
   i32.const 0
   i32.gt_s
   select
  else
   local.get $1
   local.get $2
   local.get $1
   local.get $2
   i32.lt_s
   select
  end
  local.get $0
  i32.sub
  local.tee $1
  i32.const 0
  local.get $1
  i32.const 0
  i32.gt_s
  select
  local.tee $1
  i32.const 2
  i32.const 4
  i32.const 0
  call $~lib/rt/__newArray
  local.tee $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  i32.load offset=4
  local.set $3
  global.get $~lib/memory/__stack_pointer
  i32.const 1344
  i32.store
  local.get $3
  i32.const 1348
  i32.load
  local.get $0
  i32.const 2
  i32.shl
  i32.add
  local.get $1
  i32.const 2
  i32.shl
  memory.copy
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $~lib/array/Array<u32>#slice@varargs (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $2of2
   block $1of2
    block $0of2
     block $outOfRange
      global.get $~argumentsLength
      br_table $0of2 $1of2 $2of2 $outOfRange
     end
     unreachable
    end
    i32.const 0
    local.set $0
   end
   i32.const 2147483647
   local.set $1
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 1344
  i32.store
  local.get $0
  local.get $1
  call $~lib/array/Array<u32>#slice
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/staticarray/StaticArray<u32>#__get (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $~lib/staticarray/StaticArray<u32>#get:length
  local.get $1
  i32.le_u
  if
   i32.const 1696
   i32.const 2976
   i32.const 78
   i32.const 41
   call $~lib/builtins/abort
   unreachable
  end
  local.get $0
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<u32>#__get (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   i32.const 1696
   i32.const 2928
   i32.const 114
   i32.const 42
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/math/sha256/Internal.handle (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  loop $for-loop|0
   local.get $3
   i32.const 16
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 7
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    local.tee $4
    call $~lib/staticarray/StaticArray<u32>#__get
    local.set $5
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store
    local.get $1
    local.get $3
    call $~lib/staticarray/StaticArray<u32>#__get
    local.get $5
    i32.add
    local.set $5
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    local.get $2
    local.get $3
    call $~lib/array/Array<u32>#__get
    local.get $5
    i32.add
    local.set $5
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 4
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $6
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 4
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $7
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 5
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $5
    local.get $6
    i32.const 6
    i32.rotr
    local.get $6
    i32.const 11
    i32.rotr
    i32.xor
    local.get $6
    i32.const 25
    i32.rotr
    i32.xor
    i32.add
    local.get $7
    local.get $8
    local.get $0
    i32.const 6
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.tee $5
    i32.xor
    i32.and
    local.get $5
    i32.xor
    i32.add
    local.set $6
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.const 3
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    local.tee $5
    local.get $0
    local.get $5
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.get $6
    i32.add
    call $~lib/staticarray/StaticArray<u32>#__uset
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 0
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $7
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 0
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 1
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $9
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.const 2
    local.get $3
    i32.sub
    i32.const 7
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $5
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    local.get $4
    local.get $6
    local.get $7
    i32.const 2
    i32.rotr
    local.get $7
    i32.const 13
    i32.rotr
    i32.xor
    local.get $7
    i32.const 22
    i32.rotr
    i32.xor
    i32.add
    local.get $8
    local.get $5
    local.get $9
    i32.xor
    i32.and
    local.get $5
    local.get $9
    i32.and
    i32.xor
    i32.add
    call $~lib/staticarray/StaticArray<u32>#__uset
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/math/sha256/Internal.expand (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  loop $for-loop|0
   local.get $1
   i32.const 16
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $1
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $1
    i32.const 9
    i32.add
    i32.const 15
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $3
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $1
    i32.const 14
    i32.add
    i32.const 15
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $1
    local.get $2
    local.get $3
    local.get $4
    i32.const 17
    i32.rotr
    local.get $4
    i32.const 19
    i32.rotr
    i32.xor
    local.get $4
    i32.const 10
    i32.shr_u
    i32.xor
    i32.add
    local.get $0
    local.get $1
    i32.const 1
    i32.add
    local.tee $1
    i32.const 15
    i32.and
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.tee $2
    i32.const 7
    i32.rotr
    local.get $2
    i32.const 18
    i32.rotr
    i32.xor
    local.get $2
    i32.const 3
    i32.shr_u
    i32.xor
    i32.add
    i32.add
    call $~lib/staticarray/StaticArray<u32>#__uset
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/math/sha256/Internal._hashblocks (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  call $~lib/staticarray/StaticArray<u32>#constructor
  local.tee $6
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  call $~lib/staticarray/StaticArray<u32>#constructor
  local.tee $5
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  call $~lib/staticarray/StaticArray<u32>#constructor
  local.tee $4
  i32.store offset=8
  loop $for-loop|0
   local.get $3
   i32.const 8
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=24
    local.get $5
    local.get $3
    local.get $0
    local.get $3
    i32.const 2
    i32.shl
    call $src/btc/math/sha256/load32_be
    call $~lib/staticarray/StaticArray<u32>#__uset
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=16
    local.get $6
    local.get $3
    local.get $5
    local.get $3
    call $~lib/staticarray/StaticArray<u32>#__uget
    call $~lib/staticarray/StaticArray<u32>#__uset
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  local.get $2
  local.set $3
  loop $while-continue|1
   local.get $3
   i32.const 64
   i32.ge_s
   if
    i32.const 0
    local.set $2
    loop $for-loop|2
     local.get $2
     i32.const 16
     i32.lt_s
     if
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store offset=12
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=16
      local.get $1
      local.get $2
      i32.const 2
      i32.shl
      local.get $7
      i32.add
      call $src/btc/math/sha256/load32_be
      local.set $8
      global.get $~lib/memory/__stack_pointer
      i32.const 4
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store
      local.get $4
      call $~lib/staticarray/StaticArray<u32>#get:length
      local.get $2
      i32.le_u
      if
       i32.const 1696
       i32.const 2976
       i32.const 93
       i32.const 41
       call $~lib/builtins/abort
       unreachable
      end
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store
      local.get $4
      local.get $2
      local.get $8
      call $~lib/staticarray/StaticArray<u32>#__uset
      global.get $~lib/memory/__stack_pointer
      i32.const 4
      i32.add
      global.set $~lib/memory/__stack_pointer
      local.get $2
      i32.const 1
      i32.add
      local.set $2
      br $for-loop|2
     end
    end
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    i32.const 1344
    i32.store offset=28
    i32.const 1
    global.set $~argumentsLength
    i32.const 0
    call $~lib/array/Array<u32>#slice@varargs
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=24
    local.get $5
    local.get $4
    local.get $2
    call $src/btc/math/sha256/Internal.handle
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=12
    local.get $4
    call $src/btc/math/sha256/Internal.expand
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    i32.const 1344
    i32.store offset=28
    i32.const 1
    global.set $~argumentsLength
    i32.const 16
    call $~lib/array/Array<u32>#slice@varargs
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=24
    local.get $5
    local.get $4
    local.get $2
    call $src/btc/math/sha256/Internal.handle
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=12
    local.get $4
    call $src/btc/math/sha256/Internal.expand
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    i32.const 1344
    i32.store offset=28
    i32.const 1
    global.set $~argumentsLength
    i32.const 32
    call $~lib/array/Array<u32>#slice@varargs
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=24
    local.get $5
    local.get $4
    local.get $2
    call $src/btc/math/sha256/Internal.handle
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=12
    local.get $4
    call $src/btc/math/sha256/Internal.expand
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    i32.const 1344
    i32.store offset=28
    i32.const 1
    global.set $~argumentsLength
    i32.const 48
    call $~lib/array/Array<u32>#slice@varargs
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=24
    local.get $5
    local.get $4
    local.get $2
    call $src/btc/math/sha256/Internal.handle
    i32.const 0
    local.set $2
    loop $for-loop|3
     local.get $2
     i32.const 8
     i32.lt_s
     if
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=12
      local.get $5
      local.get $2
      call $~lib/staticarray/StaticArray<u32>#__uget
      local.set $8
      global.get $~lib/memory/__stack_pointer
      local.get $6
      i32.store offset=12
      local.get $6
      local.get $2
      call $~lib/staticarray/StaticArray<u32>#__uget
      local.get $8
      i32.add
      local.set $8
      global.get $~lib/memory/__stack_pointer
      local.get $6
      i32.store offset=12
      local.get $6
      local.get $2
      local.get $8
      call $~lib/staticarray/StaticArray<u32>#__uset
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=12
      local.get $5
      local.get $2
      local.get $8
      call $~lib/staticarray/StaticArray<u32>#__uset
      local.get $2
      i32.const 1
      i32.add
      local.set $2
      br $for-loop|3
     end
    end
    local.get $7
    i32.const -64
    i32.sub
    local.set $7
    local.get $3
    i32.const -64
    i32.add
    local.set $3
    br $while-continue|1
   end
  end
  i32.const 0
  local.set $1
  loop $for-loop|4
   local.get $1
   i32.const 8
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store offset=16
    local.get $6
    local.get $1
    call $~lib/staticarray/StaticArray<u32>#__uget
    local.set $2
    global.get $~lib/memory/__stack_pointer
    i32.const 4
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i32.const 0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.load
    local.get $1
    i32.const 2
    i32.shl
    i32.add
    local.get $2
    call $~lib/polyfills/bswap<u32>
    i32.store
    global.get $~lib/memory/__stack_pointer
    i32.const 4
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|4
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
 )
 (func $src/btc/math/sha256/Internal._hashUpdate (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 1
  global.set $~argumentsLength
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.const 32
  call $~lib/typedarray/Uint8Array#subarray@varargs
  local.tee $6
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 64
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $5
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=16
  local.get $6
  i32.const 0
  i32.const 64
  call $~lib/typedarray/Uint8Array#subarray
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=12
  local.get $5
  local.get $3
  call $src/btc/math/sha256/setU8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $1
  i32.const 0
  i32.const 64
  local.get $2
  local.get $2
  i32.const 64
  i32.ge_s
  select
  local.tee $3
  call $~lib/typedarray/Uint8Array#subarray
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $7
  i32.store offset=12
  local.get $5
  local.get $7
  call $src/btc/math/sha256/setU8
  local.get $2
  local.get $3
  i32.sub
  local.set $7
  local.get $3
  local.tee $2
  i32.const 64
  i32.eq
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $5
   i32.store offset=12
   local.get $0
   local.get $5
   i32.const 64
   call $src/btc/math/sha256/Internal._hashblocks
   drop
   local.get $3
   local.set $4
   i32.const 0
   local.set $2
  end
  block $folding-inner0
   local.get $7
   i32.eqz
   if
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=12
    local.get $6
    local.get $5
    call $src/btc/math/sha256/setU8
    br $folding-inner0
   end
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   i32.const 1
   global.set $~argumentsLength
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $4
   call $~lib/typedarray/Uint8Array#subarray@varargs
   local.tee $1
   i32.store offset=20
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=16
   local.get $0
   local.get $1
   local.get $1
   call $~lib/typedarray/Uint8Array#get:length
   call $src/btc/math/sha256/Internal._hashblocks
   local.tee $2
   i32.const 0
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=24
    local.get $1
    call $~lib/typedarray/Uint8Array#get:length
    local.get $2
    i32.sub
    local.set $0
    i32.const 1
    global.set $~argumentsLength
    local.get $1
    local.get $0
    call $~lib/typedarray/Uint8Array#subarray@varargs
    local.set $0
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=12
    local.get $6
    local.get $0
    call $src/btc/math/sha256/setU8
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/btc/math/sha256/store64_be (param $0 i32) (param $1 i32) (param $2 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.get $1
  i32.add
  local.get $2
  call $~lib/polyfills/bswap<u64>
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/typedarray/Uint8Array#__get (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  local.get $1
  i32.const 45
  i32.const 167
  i32.const 2864
  call $byn$mgfn-shared$~lib/typedarray/Uint8Array#__get
 )
 (func $src/btc/math/sha256/Internal._hashFinal (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 1
  global.set $~argumentsLength
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.const 32
  call $~lib/typedarray/Uint8Array#subarray@varargs
  local.tee $6
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 128
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=16
  local.get $6
  i32.const 0
  local.get $3
  call $~lib/typedarray/Uint8Array#subarray
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=12
  local.get $4
  local.get $6
  call $src/btc/math/sha256/setU8
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store
  local.get $4
  local.get $3
  i32.const 128
  call $~lib/typedarray/Uint8Array#__set
  local.get $3
  i32.const 56
  i32.lt_s
  if
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store
   local.get $4
   i32.const 56
   local.get $2
   i32.const 3
   i32.shl
   i64.extend_i32_s
   call $src/btc/math/sha256/store64_be
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=12
   local.get $0
   local.get $4
   i32.const 64
   call $src/btc/math/sha256/Internal._hashblocks
   drop
  else
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store
   local.get $4
   i32.const 120
   local.get $2
   i32.const 3
   i32.shl
   i64.extend_i32_s
   call $src/btc/math/sha256/store64_be
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=12
   local.get $0
   local.get $4
   i32.const 128
   call $src/btc/math/sha256/Internal._hashblocks
   drop
  end
  loop $for-loop|0
   local.get $5
   i32.const 32
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=12
    local.get $1
    local.get $5
    local.get $0
    local.get $5
    call $~lib/typedarray/Uint8Array#__get
    call $~lib/typedarray/Uint8Array#__set
    local.get $5
    i32.const 1
    i32.add
    local.set $5
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/math/sha256/Internal._hash (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.set $5
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 96
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $3
  i32.store
  loop $for-loop|0
   local.get $4
   i32.const 32
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    i32.const 1456
    i32.store offset=8
    local.get $3
    local.get $4
    i32.const 1456
    local.get $4
    call $~lib/array/Array<u8>#__get
    call $~lib/typedarray/Uint8Array#__set
    local.get $4
    i32.const 1
    i32.add
    local.set $4
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $5
  local.get $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $3
  local.get $1
  local.get $2
  call $src/btc/math/sha256/Internal._hashUpdate
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $3
  local.get $0
  local.get $2
  local.get $1
  call $src/btc/math/sha256/Internal._hashFinal
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/math/sha256/Sha256.hash (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  local.get $1
  local.get $0
  local.get $0
  call $~lib/typedarray/Uint8Array#get:length
  call $src/btc/math/sha256/Internal._hash
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $src/btc/math/abi/encodeSelector (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  i32.const 1
  global.set $~argumentsLength
  local.get $0
  call $~lib/string/String.UTF8.encode@varargs
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 1
  global.set $~argumentsLength
  local.get $1
  local.get $0
  call $~lib/typedarray/Uint8Array.wrap@varargs
  local.tee $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $src/btc/math/sha256/Sha256.hash
  local.tee $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 0
  call $~lib/typedarray/Uint8Array#__get
  i32.const 24
  i32.shl
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.const 1
  call $~lib/typedarray/Uint8Array#__get
  i32.const 16
  i32.shl
  i32.or
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.const 2
  call $~lib/typedarray/Uint8Array#__get
  i32.const 8
  i32.shl
  i32.or
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 3
  call $~lib/typedarray/Uint8Array#__get
  local.get $1
  i32.or
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#find" (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $3
  local.get $2
  local.get $0
  i32.load offset=4
  i32.and
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.set $0
  block $folding-inner0
   loop $while-continue|0
    local.get $0
    if
     local.get $0
     i32.load offset=8
     local.tee $2
     i32.const 1
     i32.and
     if (result i32)
      i32.const 0
     else
      local.get $0
      i32.load
      local.get $1
      i32.eq
     end
     br_if $folding-inner0
     local.get $2
     i32.const -2
     i32.and
     local.set $0
     br $while-continue|0
    end
   end
   i32.const 0
   local.set $0
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/dataview/DataView#constructor (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.const 10
  call $~lib/rt/itcms/__new
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $~lib/arraybuffer/ArrayBuffer#get:byteLength
  local.get $1
  i32.lt_u
  local.get $1
  i32.const 1073741820
  i32.gt_u
  i32.or
  if
   i32.const 1904
   i32.const 3152
   i32.const 25
   i32.const 7
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $2
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  local.get $0
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  local.get $1
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $~lib/dataview/DataView#constructor@varargs (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  block $2of2
   block $1of2
    block $outOfRange
     global.get $~argumentsLength
     i32.const 1
     i32.sub
     br_table $1of2 $1of2 $2of2 $outOfRange
    end
    unreachable
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/arraybuffer/ArrayBuffer#get:byteLength
   local.set $1
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/dataview/DataView#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#constructor (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 13
  i32.const 9
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#set:trackDataTypes
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $~lib/rt/tlsf/Root#set:flMap
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 0
  i32.const 0
  i32.const 5
  i32.const 3216
  call $~lib/rt/__newArray
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $1
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 1
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  i32.const 1
  global.set $~argumentsLength
  local.get $0
  call $~lib/dataview/DataView#constructor@varargs
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $1
  local.get $0
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $~lib/array/ensureCapacity (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=8
  local.tee $4
  local.get $2
  i32.shr_u
  i32.gt_u
  if
   local.get $1
   i32.const 1073741820
   local.get $2
   i32.shr_u
   i32.gt_u
   if
    i32.const 1904
    i32.const 2928
    i32.const 19
    i32.const 48
    call $~lib/builtins/abort
    unreachable
   end
   i32.const 8
   local.get $1
   local.get $1
   i32.const 8
   i32.le_u
   select
   local.get $2
   i32.shl
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $3
   if
    i32.const 1073741820
    local.get $4
    i32.const 1
    i32.shl
    local.tee $2
    local.get $2
    i32.const 1073741820
    i32.ge_u
    select
    local.tee $2
    local.get $1
    local.get $1
    local.get $2
    i32.lt_u
    select
    local.set $1
   end
   local.get $0
   i32.load
   local.tee $5
   local.get $1
   call $~lib/rt/itcms/__renew
   local.tee $2
   local.get $5
   i32.ne
   if
    local.get $0
    local.get $2
    i32.store
    local.get $0
    local.get $2
    i32.store offset=4
    local.get $0
    local.get $2
    i32.const 0
    call $~lib/rt/itcms/__link
   end
   local.get $0
   local.get $1
   i32.store offset=8
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/array/Array<u8>#push (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $0
  i32.load offset=12
  local.tee $2
  i32.const 1
  i32.add
  local.tee $3
  i32.const 0
  i32.const 1
  call $~lib/array/ensureCapacity
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $2
  i32.add
  local.get $1
  i32.store8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $3
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/dataview/DataView#getUint8 (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  local.get $1
  i32.const 50
  i32.const 72
  i32.const 3152
  call $byn$mgfn-shared$~lib/typedarray/Uint8Array#__get
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#resize (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=8
  local.get $1
  i32.add
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $1
  i32.store offset=8
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   local.tee $3
   i32.store
   local.get $2
   local.get $3
   i32.load offset=8
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=4
    local.tee $3
    i32.store offset=4
    local.get $1
    local.get $2
    local.get $3
    local.get $2
    call $~lib/dataview/DataView#getUint8
    call $~lib/typedarray/Uint8Array#__set
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load
  local.tee $1
  i32.store offset=12
  i32.const 1
  global.set $~argumentsLength
  local.get $1
  call $~lib/dataview/DataView#constructor@varargs
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#allocSafe (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.get $1
  i32.add
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store
  local.get $3
  local.get $2
  i32.load offset=8
  i32.gt_u
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   local.tee $2
   i32.store
   local.get $2
   i32.load offset=8
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $1
   local.get $2
   local.get $0
   i32.load
   i32.sub
   i32.sub
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   local.get $1
   call $src/btc/buffer/BytesWriter/BytesWriter#resize
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeU16 (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   local.get $2
   i32.const 1
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 2
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.load
  local.set $3
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $3
  i32.const 31
  i32.shr_u
  local.get $2
  i32.load offset=8
  local.get $3
  i32.const 2
  i32.add
  i32.lt_s
  i32.or
  if
   i32.const 1696
   i32.const 3152
   i32.const 135
   i32.const 7
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  i32.load offset=4
  local.get $3
  i32.add
  local.get $1
  i32.store16
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load
  i32.const 2
  i32.add
  call $~lib/rt/tlsf/Root#set:flMap
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/string/String#charCodeAt (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $~lib/string/String#get:length
  local.get $1
  i32.le_u
  if (result i32)
   i32.const -1
  else
   local.get $0
   local.get $1
   i32.const 1
   i32.shl
   i32.add
   i32.load16_u
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/dataview/DataView#setUint8 (param $0 i32) (param $1 i32) (param $2 i32)
  local.get $0
  local.get $1
  local.get $2
  i32.const 50
  i32.const 128
  i32.const 3152
  call $byn$mgfn-shared$~lib/typedarray/Uint8Array#__set
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeU8 (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   local.get $2
   i32.const 0
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 1
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $0
  i32.load
  local.tee $0
  i32.const 1
  i32.add
  call $~lib/rt/tlsf/Root#set:flMap
  local.get $2
  local.get $0
  local.get $1
  call $~lib/dataview/DataView#setUint8
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeString (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $3
   i32.store
   local.get $3
   i32.const 6
   call $~lib/array/Array<u8>#push
  end
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $1
   call $~lib/string/String#get:length
   local.get $2
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    local.get $0
    local.get $1
    local.get $2
    call $~lib/string/String#charCodeAt
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeStringWithLength (param $0 i32) (param $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/string/String#get:length
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeString
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeU32 (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $3
   i32.store
   local.get $3
   i32.const 2
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 4
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $4
  i32.const 31
  i32.shr_u
  local.get $3
  i32.load offset=8
  local.get $4
  i32.const 4
  i32.add
  i32.lt_s
  i32.or
  if
   i32.const 1696
   i32.const 3152
   i32.const 142
   i32.const 7
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $4
  local.get $3
  i32.load offset=4
  i32.add
  local.get $2
  if (result i32)
   local.get $1
  else
   local.get $1
   call $~lib/polyfills/bswap<u32>
  end
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load
  i32.const 4
  i32.add
  call $~lib/rt/tlsf/Root#set:flMap
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeSelector (param $0 i32) (param $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU32
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#getBuffer (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.load offset=8
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $2
  i32.store offset=8
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   local.tee $3
   i32.store
   local.get $1
   local.get $3
   i32.load offset=8
   i32.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=4
    local.tee $3
    i32.store offset=4
    local.get $2
    local.get $1
    local.get $3
    local.get $1
    call $~lib/dataview/DataView#getUint8
    call $~lib/typedarray/Uint8Array#__set
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 0
  call $~lib/rt/tlsf/Root#set:flMap
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  i32.const 1
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  i32.const 1
  global.set $~argumentsLength
  local.get $1
  call $~lib/dataview/DataView#constructor@varargs
  local.set $1
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 0
  i32.const 0
  i32.const 5
  i32.const 3248
  call $~lib/rt/__newArray
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $~lib/set/Set<~lib/typedarray/Uint8Array>#find (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $3
  local.get $2
  local.get $0
  i32.load offset=4
  i32.and
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.set $0
  block $folding-inner0
   loop $while-continue|0
    local.get $0
    if
     local.get $0
     i32.load offset=4
     local.tee $2
     i32.const 1
     i32.and
     if (result i32)
      i32.const 0
     else
      local.get $0
      i32.load
      local.get $1
      i32.eq
     end
     br_if $folding-inner0
     local.get $2
     i32.const -2
     i32.and
     local.set $0
     br $while-continue|0
    end
   end
   i32.const 0
   local.set $0
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/set/Set<~lib/typedarray/Uint8Array>#rehash (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 1
  i32.add
  local.tee $2
  i32.const 2
  i32.shl
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $6
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.const 3
  i32.shl
  i32.const 3
  i32.div_s
  local.tee $5
  i32.const 3
  i32.shl
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  i32.load offset=8
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $7
  local.get $0
  i32.load offset=16
  i32.const 3
  i32.shl
  i32.add
  local.set $4
  local.get $3
  local.set $2
  loop $while-continue|0
   local.get $4
   local.get $7
   i32.ne
   if
    local.get $7
    i32.load offset=4
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $7
     i32.load
     local.tee $8
     i32.store offset=12
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.store offset=8
     local.get $2
     local.get $8
     call $~lib/rt/tlsf/Root#set:flMap
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.store offset=8
     local.get $2
     local.get $6
     local.get $8
     call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
     local.get $1
     i32.and
     i32.const 2
     i32.shl
     i32.add
     local.tee $8
     i32.load
     call $~lib/rt/itcms/Object#set:nextWithColor
     local.get $8
     local.get $2
     i32.store
     local.get $2
     i32.const 8
     i32.add
     local.set $2
    end
    local.get $7
    i32.const 8
    i32.add
    local.set $7
    br $while-continue|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=16
  local.get $0
  local.get $6
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=16
  local.get $0
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $5
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  local.get $0
  local.get $0
  i32.load offset=20
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/set/Set<~lib/typedarray/Uint8Array>#add (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  local.get $2
  call $~lib/set/Set<~lib/typedarray/Uint8Array>#find
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=16
   local.set $3
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $3
   local.get $0
   i32.load offset=12
   i32.eq
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.load offset=20
    local.set $3
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $3
    local.get $0
    i32.load offset=12
    i32.const 3
    i32.mul
    i32.const 4
    i32.div_s
    i32.lt_s
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
    else
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
     i32.const 1
     i32.shl
     i32.const 1
     i32.or
    end
    call $~lib/set/Set<~lib/typedarray/Uint8Array>#rehash
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=8
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=16
   local.tee $3
   i32.const 1
   i32.add
   call $~lib/rt/itcms/Object#set:rtSize
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $3
   i32.const 3
   i32.shl
   i32.add
   local.tee $3
   local.get $1
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $0
   local.get $1
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=20
   i32.const 1
   i32.add
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $3
   local.get $1
   local.get $2
   local.get $0
   i32.load offset=4
   i32.and
   i32.const 2
   i32.shl
   i32.add
   local.tee $0
   i32.load
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $3
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/set/Set<u32>#constructor (result i32)
  i32.const 32
  i32.const 11
  call $"byn$mgfn-shared$~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
 )
 (func $~lib/set/Set<u32>#has (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  call $~lib/set/Set<~lib/typedarray/Uint8Array>#find
  i32.const 0
  i32.ne
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/set/Set<u32>#rehash (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 1
  i32.add
  local.tee $2
  i32.const 2
  i32.shl
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $6
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.const 3
  i32.shl
  i32.const 3
  i32.div_s
  local.tee $5
  i32.const 3
  i32.shl
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  i32.load offset=8
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $7
  local.get $0
  i32.load offset=16
  i32.const 3
  i32.shl
  i32.add
  local.set $4
  local.get $3
  local.set $2
  loop $while-continue|0
   local.get $4
   local.get $7
   i32.ne
   if
    local.get $7
    i32.load offset=4
    i32.const 1
    i32.and
    i32.eqz
    if
     local.get $2
     local.get $7
     i32.load
     local.tee $8
     call $~lib/rt/tlsf/Root#set:flMap
     local.get $2
     local.get $6
     local.get $8
     call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
     local.get $1
     i32.and
     i32.const 2
     i32.shl
     i32.add
     local.tee $8
     i32.load
     call $~lib/rt/itcms/Object#set:nextWithColor
     local.get $8
     local.get $2
     i32.store
     local.get $2
     i32.const 8
     i32.add
     local.set $2
    end
    local.get $7
    i32.const 8
    i32.add
    local.set $7
    br $while-continue|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=12
  local.get $0
  local.get $6
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=12
  local.get $0
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $5
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  local.get $0
  local.get $0
  i32.load offset=20
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/set/Set<u32>#add (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  local.get $2
  call $~lib/set/Set<~lib/typedarray/Uint8Array>#find
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=16
   local.set $3
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $3
   local.get $0
   i32.load offset=12
   i32.eq
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.load offset=20
    local.set $3
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $3
    local.get $0
    i32.load offset=12
    i32.const 3
    i32.mul
    i32.const 4
    i32.div_s
    i32.lt_s
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
    else
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
     i32.const 1
     i32.shl
     i32.const 1
     i32.or
    end
    call $~lib/set/Set<u32>#rehash
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=8
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=16
   local.tee $3
   i32.const 1
   i32.add
   call $~lib/rt/itcms/Object#set:rtSize
   local.get $4
   local.get $3
   i32.const 3
   i32.shl
   i32.add
   local.tee $3
   local.get $1
   call $~lib/rt/tlsf/Root#set:flMap
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=20
   i32.const 1
   i32.add
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $3
   local.get $1
   local.get $2
   local.get $0
   i32.load offset=4
   i32.and
   i32.const 2
   i32.shl
   i32.add
   local.tee $1
   i32.load
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $1
   local.get $3
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#rehash" (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 1
  i32.add
  local.tee $2
  i32.const 2
  i32.shl
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $7
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.const 3
  i32.shl
  i32.const 3
  i32.div_s
  local.tee $6
  i32.const 12
  i32.mul
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  i32.load offset=8
  local.set $8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $8
  local.get $0
  i32.load offset=16
  i32.const 12
  i32.mul
  i32.add
  local.set $5
  local.get $3
  local.set $2
  loop $while-continue|0
   local.get $5
   local.get $8
   i32.ne
   if
    local.get $8
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.load
     local.tee $9
     i32.store offset=12
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.store offset=8
     local.get $2
     local.get $9
     call $~lib/rt/tlsf/Root#set:flMap
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.load offset=4
     local.tee $4
     i32.store offset=8
     local.get $2
     local.get $4
     call $~lib/rt/itcms/Object#set:nextWithColor
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.store offset=8
     local.get $2
     local.get $7
     local.get $9
     call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
     local.get $1
     i32.and
     i32.const 2
     i32.shl
     i32.add
     local.tee $4
     i32.load
     call $~lib/rt/itcms/Object#set:prev
     local.get $4
     local.get $2
     i32.store
     local.get $2
     i32.const 12
     i32.add
     local.set $2
    end
    local.get $8
    i32.const 12
    i32.add
    local.set $8
    br $while-continue|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $7
  i32.store offset=16
  local.get $0
  local.get $7
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=16
  local.get $0
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $6
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  local.get $0
  local.get $0
  i32.load offset=20
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set" (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#find"
  local.tee $4
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $4
   local.get $2
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $2
   i32.const 1
   call $~lib/rt/itcms/__link
  else
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=16
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $4
   local.get $0
   i32.load offset=12
   i32.eq
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.load offset=20
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $4
    local.get $0
    i32.load offset=12
    i32.const 3
    i32.mul
    i32.const 4
    i32.div_s
    i32.lt_s
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
    else
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
     i32.const 1
     i32.shl
     i32.const 1
     i32.or
    end
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#rehash"
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $4
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=16
   local.tee $5
   i32.const 1
   i32.add
   call $~lib/rt/itcms/Object#set:rtSize
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $5
   i32.const 12
   i32.mul
   i32.add
   local.tee $4
   local.get $1
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $0
   local.get $1
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $4
   local.get $2
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $2
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=20
   i32.const 1
   i32.add
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $4
   local.get $1
   local.get $3
   local.get $0
   i32.load offset=4
   i32.and
   i32.const 2
   i32.shl
   i32.add
   local.tee $0
   i32.load
   call $~lib/rt/itcms/Object#set:prev
   local.get $0
   local.get $4
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#addToWriteMethods (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=12
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $3
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override"
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=12
   local.tee $3
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
   local.tee $3
   i32.store offset=12
  else
   global.get $~lib/memory/__stack_pointer
   call $~lib/set/Set<u32>#constructor
   local.tee $3
   i32.store offset=12
  end
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  local.get $2
  call $~lib/set/Set<u32>#has
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   local.get $2
   call $~lib/set/Set<u32>#add
   drop
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=12
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $0
   local.get $1
   local.get $3
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set@override"
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#defineGetterSelector (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  call $src/btc/math/abi/encodeSelector
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $3
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override"
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   local.tee $3
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
   local.tee $3
   i32.store offset=12
  else
   global.get $~lib/memory/__stack_pointer
   local.set $5
   global.get $~lib/memory/__stack_pointer
   i32.const 12
   i32.sub
   global.set $~lib/memory/__stack_pointer
   call $~stack_check
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store
   global.get $~lib/memory/__stack_pointer
   i32.const 0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   i32.const 24
   i32.const 14
   call $~lib/rt/itcms/__new
   local.tee $3
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   i32.const 16
   call $~lib/arraybuffer/ArrayBuffer#constructor
   local.set $6
   global.get $~lib/memory/__stack_pointer
   local.get $6
   i32.store offset=8
   local.get $3
   local.get $6
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   local.get $3
   i32.const 3
   call $~lib/rt/itcms/Object#set:nextWithColor
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   i32.const 32
   call $~lib/arraybuffer/ArrayBuffer#constructor
   local.set $6
   global.get $~lib/memory/__stack_pointer
   local.get $6
   i32.store offset=8
   local.get $3
   local.get $6
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   local.get $3
   i32.const 4
   call $~lib/rt/itcms/Object#set:rtId
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   local.get $3
   i32.const 0
   call $~lib/rt/itcms/Object#set:rtSize
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   local.get $3
   i32.const 0
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   i32.const 12
   i32.add
   global.set $~lib/memory/__stack_pointer
   local.get $5
   local.get $3
   i32.store offset=12
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $5
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $5
  local.get $2
  call $src/btc/buffer/BytesWriter/BytesWriter#writeStringWithLength
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  local.get $5
  local.get $4
  call $src/btc/buffer/BytesWriter/BytesWriter#writeSelector
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  local.get $5
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $3
  local.get $2
  call $~lib/set/Set<~lib/typedarray/Uint8Array>#add
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $2
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override"
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $2
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
   local.tee $2
   i32.store offset=20
  else
   global.get $~lib/memory/__stack_pointer
   call $~lib/set/Set<u32>#constructor
   local.tee $2
   i32.store offset=20
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  local.get $4
  call $~lib/set/Set<u32>#has
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $2
   local.get $4
   call $~lib/set/Set<u32>#add
   drop
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=24
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $4
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=8
   local.get $4
   local.get $1
   local.get $2
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set@override"
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set@override"
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $2
  local.get $0
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/string/String.__ne (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/string/String.__eq
  i32.eqz
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/string/String#concat (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $~lib/string/String#get:length
  i32.const 1
  i32.shl
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  block $folding-inner0
   local.get $1
   call $~lib/string/String#get:length
   i32.const 1
   i32.shl
   local.tee $4
   local.get $3
   i32.add
   local.tee $2
   i32.eqz
   if
    i32.const 3504
    local.set $2
    br $folding-inner0
   end
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.const 2
   call $~lib/rt/itcms/__new
   local.tee $2
   i32.store offset=4
   local.get $2
   local.get $0
   local.get $3
   memory.copy
   local.get $2
   local.get $3
   i32.add
   local.get $1
   local.get $4
   memory.copy
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $~lib/string/String.__concat (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/string/String#concat
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/util/string/joinReferenceArray<~lib/string/String> (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  block $folding-inner0
   local.get $1
   i32.const 1
   i32.sub
   local.tee $3
   i32.const 0
   i32.lt_s
   if
    i32.const 3504
    local.set $0
    br $folding-inner0
   end
   local.get $3
   i32.eqz
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load
    local.tee $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.const 0
    call $~lib/string/String.__ne
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
    else
     i32.const 3504
     local.set $0
    end
    br $folding-inner0
   end
   i32.const 3504
   local.set $1
   global.get $~lib/memory/__stack_pointer
   i32.const 3504
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   i32.const 3504
   i32.store offset=4
   i32.const 3504
   call $~lib/string/String#get:length
   local.set $5
   loop $for-loop|0
    local.get $3
    local.get $4
    i32.gt_s
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     local.get $4
     i32.const 2
     i32.shl
     i32.add
     i32.load
     local.tee $2
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=4
     local.get $2
     i32.const 0
     call $~lib/string/String.__ne
     if
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=16
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=12
      global.get $~lib/memory/__stack_pointer
      local.get $1
      local.get $2
      call $~lib/string/String.__concat
      local.tee $1
      i32.store offset=8
     end
     local.get $5
     if
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      i32.const 3504
      i32.store offset=12
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.const 3504
      call $~lib/string/String.__concat
      local.tee $1
      i32.store offset=8
     end
     local.get $4
     i32.const 1
     i32.add
     local.set $4
     br $for-loop|0
    end
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   local.get $3
   i32.const 2
   i32.shl
   i32.add
   i32.load
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   i32.const 0
   call $~lib/string/String.__ne
   if
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $1
    local.get $0
    call $~lib/string/String.__concat
    local.tee $1
    i32.store offset=8
   end
   local.get $1
   local.set $0
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/staticarray/StaticArray<~lib/string/String>#join (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $~lib/staticarray/StaticArray<u32>#get:length
  local.set $1
  global.get $~lib/memory/__stack_pointer
  i32.const 3504
  i32.store
  local.get $0
  local.get $1
  call $~lib/util/string/joinReferenceArray<~lib/string/String>
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#defineMethodSelector (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 24
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  call $src/btc/math/abi/encodeSelector
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $5
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override"
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load
   local.tee $5
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   call $~lib/set/Set<u32>#constructor
   local.set $6
   global.get $~lib/memory/__stack_pointer
   local.get $6
   i32.store offset=8
   local.get $5
   local.get $1
   local.get $6
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set@override"
  end
  local.get $3
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   local.get $0
   local.get $1
   local.get $4
   call $src/btc/universal/ABIRegistry/ABIRegistryBase#addToWriteMethods
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
  local.tee $3
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  local.get $4
  call $~lib/set/Set<u32>#has
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=20
   global.get $~lib/memory/__stack_pointer
   i32.const 3472
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   i32.const 3472
   i32.const 1
   local.get $2
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 3472
   i32.store
   global.get $~lib/memory/__stack_pointer
   i32.const 3504
   i32.store offset=4
   i32.const 3472
   call $~lib/staticarray/StaticArray<~lib/string/String>#join
   i32.const 3536
   i32.const 136
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  local.get $4
  call $~lib/set/Set<u32>#add
  drop
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set@override"
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $3
  local.get $0
  local.get $1
  local.get $2
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_NET/OP_NET#constructor (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  local.get $0
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 12
   i32.const 7
   call $~lib/rt/itcms/__new
   local.tee $0
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $0
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  call $~lib/string/String.__not
  if
   i32.const 2224
   i32.const 2336
   i32.const 18
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  call $~lib/string/String.__not
  if
   i32.const 2416
   i32.const 2336
   i32.const 22
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=12
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $3
  local.get $1
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has@override"
  local.set $3
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
  if
   i32.const 2496
   i32.const 2336
   i32.const 26
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $0
  local.get $2
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $1
  i32.store offset=4
  local.get $1
  call $~lib/string/String.__not
  if
   i32.const 2576
   i32.const 2336
   i32.const 33
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $1
  i32.store offset=4
  local.get $1
  call $~lib/string/String.__not
  if
   i32.const 2640
   i32.const 2336
   i32.const 37
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 2704
  i32.store offset=4
  local.get $0
  i32.const 2704
  call $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 3280
  i32.store offset=4
  local.get $0
  i32.const 3280
  call $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 3312
  i32.store offset=4
  local.get $0
  i32.const 3312
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  block $__inlined_func$src/btc/contracts/OP_NET/OP_NET#defineSelectors@override$234
   block $default
    block $case1
     local.get $0
     i32.const 8
     i32.sub
     i32.load
     local.tee $1
     i32.const 26
     i32.ne
     if
      local.get $1
      i32.const 27
      i32.eq
      br_if $case1
      br $default
     end
     local.get $0
     call $src/contract/TestingContract/TestingContract#defineSelectors
     br $__inlined_func$src/btc/contracts/OP_NET/OP_NET#defineSelectors@override$234
    end
    local.get $0
    call $src/btc/contracts/OP_20/OP_20#defineSelectors
    br $__inlined_func$src/btc/contracts/OP_NET/OP_NET#defineSelectors@override$234
   end
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#constructor" (result i32)
  i32.const 48
  i32.const 19
  call $"byn$mgfn-shared$~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor"
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#rehash" (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 1
  i32.add
  local.tee $2
  i32.const 2
  i32.shl
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $7
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.const 3
  i32.shl
  i32.const 3
  i32.div_s
  local.tee $6
  i32.const 12
  i32.mul
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  i32.load offset=8
  local.set $8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $8
  local.get $0
  i32.load offset=16
  i32.const 12
  i32.mul
  i32.add
  local.set $5
  local.get $3
  local.set $2
  loop $while-continue|0
   local.get $5
   local.get $8
   i32.ne
   if
    local.get $8
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.load
     local.tee $9
     i32.store offset=12
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.store offset=8
     local.get $2
     local.get $9
     call $~lib/rt/tlsf/Root#set:flMap
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.load offset=4
     local.tee $4
     i32.store offset=8
     local.get $2
     local.get $4
     call $~lib/rt/itcms/Object#set:nextWithColor
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.store offset=8
     local.get $2
     local.get $7
     local.get $9
     call $~lib/util/hash/HASH<~lib/string/String>
     local.get $1
     i32.and
     i32.const 2
     i32.shl
     i32.add
     local.tee $4
     i32.load
     call $~lib/rt/itcms/Object#set:prev
     local.get $4
     local.get $2
     i32.store
     local.get $2
     i32.const 12
     i32.add
     local.set $2
    end
    local.get $8
    i32.const 12
    i32.add
    local.set $8
    br $while-continue|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $7
  i32.store offset=16
  local.get $0
  local.get $7
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=16
  local.get $0
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $6
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  local.get $0
  local.get $0
  i32.load offset=20
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set" (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  call $~lib/util/hash/HASH<~lib/string/String>
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  local.get $3
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#find"
  local.tee $4
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $4
   local.get $2
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $2
   i32.const 1
   call $~lib/rt/itcms/__link
  else
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=16
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $4
   local.get $0
   i32.load offset=12
   i32.eq
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.load offset=20
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $4
    local.get $0
    i32.load offset=12
    i32.const 3
    i32.mul
    i32.const 4
    i32.div_s
    i32.lt_s
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
    else
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
     i32.const 1
     i32.shl
     i32.const 1
     i32.or
    end
    call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#rehash"
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $4
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=16
   local.tee $5
   i32.const 1
   i32.add
   call $~lib/rt/itcms/Object#set:rtSize
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $5
   i32.const 12
   i32.mul
   i32.add
   local.tee $4
   local.get $1
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $0
   local.get $1
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $4
   local.get $2
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $2
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=20
   i32.const 1
   i32.add
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $4
   local.get $1
   local.get $3
   local.get $0
   i32.load offset=4
   i32.and
   i32.const 2
   i32.shl
   i32.add
   local.tee $0
   i32.load
   call $~lib/rt/itcms/Object#set:prev
   local.get $0
   local.get $4
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtAddress (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $2
  local.get $1
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has@override"
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#constructor"
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=8
   local.get $0
   local.get $1
   local.get $2
   call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set@override"
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/array/Array<u8>#__set (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   local.get $1
   i32.const 0
   i32.lt_s
   if
    i32.const 1696
    i32.const 2928
    i32.const 130
    i32.const 22
    call $~lib/builtins/abort
    unreachable
   end
   local.get $0
   local.get $1
   i32.const 1
   i32.add
   local.tee $3
   i32.const 0
   i32.const 1
   call $~lib/array/ensureCapacity
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   local.get $3
   call $~lib/rt/itcms/Object#set:rtId
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.add
  local.get $2
  i32.store8
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/array/Array<u8>#get:length (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=12
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/math/bytes/bytes32 (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  call $~lib/typedarray/Uint8Array#get:length
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   call $~lib/typedarray/Uint8Array#get:length
   drop
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  i32.load offset=4
  local.tee $0
  i64.load
  local.get $0
  i64.load offset=8
  local.get $0
  i64.load offset=16
  local.get $0
  i64.load offset=24
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/math/abi/encodePointerHash (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 48
  memory.fill
  global.get $~lib/memory/__stack_pointer
  i32.const 34
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.set $6
  global.get $~lib/memory/__stack_pointer
  i32.const 2
  i32.const 0
  i32.const 5
  i32.const 0
  call $~lib/rt/__newArray
  local.tee $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.load offset=4
  i32.store offset=8
  local.get $2
  i32.const 0
  local.get $0
  i32.const 255
  i32.and
  call $~lib/array/Array<u8>#__set
  local.get $2
  i32.const 1
  local.get $0
  i32.const 65535
  i32.and
  i32.const 8
  i32.shr_u
  call $~lib/array/Array<u8>#__set
  local.get $6
  local.get $2
  i32.store offset=12
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=16
   local.get $2
   call $~lib/array/Array<u8>#get:length
   local.get $3
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $4
    local.get $3
    local.get $2
    local.get $3
    call $~lib/array/Array<u8>#__get
    call $~lib/typedarray/Uint8Array#__set
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $3
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=16
  local.get $3
  i32.load offset=4
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $6
  local.get $1
  i64.load
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $6
  local.get $1
  i64.load offset=8
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $6
  local.get $1
  i64.load offset=16
  i64.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $6
  local.get $1
  i64.load offset=24
  i64.store offset=24
  local.get $0
  local.get $3
  i32.store offset=44
  loop $for-loop|1
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=16
   local.get $3
   call $~lib/typedarray/Uint8Array#get:length
   local.get $5
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $2
    call $~lib/array/Array<u8>#get:length
    local.get $5
    i32.add
    local.set $0
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=20
    local.get $4
    local.get $0
    local.get $3
    local.get $5
    call $~lib/typedarray/Uint8Array#__get
    call $~lib/typedarray/Uint8Array#__set
    local.get $5
    i32.const 1
    i32.add
    local.set $5
    br $for-loop|1
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=20
  local.get $4
  call $src/btc/math/sha256/Sha256.hash
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  local.get $0
  call $src/btc/math/bytes/bytes32
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get" (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $1
  call $~lib/util/hash/HASH<~lib/string/String>
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#find"
  local.tee $0
  i32.eqz
  if
   i32.const 3040
   i32.const 3104
   i32.const 105
   i32.const 17
   call $~lib/builtins/abort
   unreachable
  end
  local.get $0
  i32.load offset=4
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#find" (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i64)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $4
  local.get $2
  local.get $0
  i32.load offset=4
  i32.and
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.set $0
  block $folding-inner0
   loop $while-continue|0
    local.get $0
    if
     local.get $0
     i32.load offset=8
     local.tee $2
     i32.const 1
     i32.and
     if (result i32)
      i32.const 0
     else
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.load
      local.tee $4
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store
      local.get $4
      i64.load
      local.set $3
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store
      local.get $3
      local.get $1
      i64.load
      i64.eq
      if (result i32)
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store
       local.get $4
       i64.load offset=8
       local.set $3
       global.get $~lib/memory/__stack_pointer
       local.get $1
       i32.store
       local.get $3
       local.get $1
       i64.load offset=8
       i64.eq
      else
       i32.const 0
      end
      if (result i32)
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store
       local.get $4
       i64.load offset=16
       local.set $3
       global.get $~lib/memory/__stack_pointer
       local.get $1
       i32.store
       local.get $3
       local.get $1
       i64.load offset=16
       i64.eq
      else
       i32.const 0
      end
      if (result i32)
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store
       local.get $4
       i64.load offset=24
       local.set $3
       global.get $~lib/memory/__stack_pointer
       local.get $1
       i32.store
       local.get $3
       local.get $1
       i64.load offset=24
       i64.eq
      else
       i32.const 0
      end
     end
     br_if $folding-inner0
     local.get $2
     i32.const -2
     i32.and
     local.set $0
     br $while-continue|0
    end
   end
   i32.const 0
   local.set $0
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#set" (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  local.get $3
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#find"
  local.tee $4
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $4
   local.get $2
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $2
   i32.const 1
   call $~lib/rt/itcms/__link
  else
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=16
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $4
   local.get $0
   i32.load offset=12
   i32.eq
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    i32.load offset=20
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    local.get $0
    local.get $4
    local.get $0
    i32.load offset=12
    i32.const 3
    i32.mul
    i32.const 4
    i32.div_s
    i32.lt_s
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
    else
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $0
     i32.load offset=4
     i32.const 1
     i32.shl
     i32.const 1
     i32.or
    end
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#rehash"
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $4
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=16
   local.tee $5
   i32.const 1
   i32.add
   call $~lib/rt/itcms/Object#set:rtSize
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $5
   i32.const 12
   i32.mul
   i32.add
   local.tee $4
   local.get $1
   call $~lib/rt/tlsf/Root#set:flMap
   local.get $0
   local.get $1
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   local.get $4
   local.get $2
   call $~lib/rt/itcms/Object#set:nextWithColor
   local.get $0
   local.get $2
   i32.const 1
   call $~lib/rt/itcms/__link
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=20
   i32.const 1
   i32.add
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $4
   local.get $1
   local.get $3
   local.get $0
   i32.load offset=4
   i32.and
   i32.const 2
   i32.shl
   i32.add
   local.tee $0
   i32.load
   call $~lib/rt/itcms/Object#set:prev
   local.get $0
   local.get $4
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialStorage (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $4
  local.get $1
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has@override"
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $4
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#constructor"
   local.set $5
   global.get $~lib/memory/__stack_pointer
   local.get $5
   i32.store offset=8
   local.get $4
   local.get $1
   local.get $5
   call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set@override"
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get@override"
  local.tee $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $0
  local.get $2
  local.get $3
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#set@override"
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor (param $0 i32) (result i32)
  local.get $0
  i32.const 37
  call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
 )
 (func $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   local.get $1
   i32.const 0
   i32.lt_s
   if
    i32.const 1696
    i32.const 2928
    i32.const 130
    i32.const 22
    call $~lib/builtins/abort
    unreachable
   end
   local.get $0
   local.get $1
   i32.const 1
   i32.add
   local.tee $3
   i32.const 2
   i32.const 1
   call $~lib/array/ensureCapacity
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   local.get $3
   call $~lib/rt/itcms/Object#set:rtId
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  local.get $2
  i32.store
  local.get $0
  local.get $2
  i32.const 1
  call $~lib/rt/itcms/__link
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length (param $0 i32) (param $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  local.get $0
  local.get $1
  i32.const 2
  i32.const 0
  call $~lib/array/ensureCapacity
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#keys" (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=16
  local.tee $4
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $0
  i32.store offset=4
  loop $for-loop|0
   local.get $2
   local.get $4
   i32.lt_s
   if
    local.get $3
    local.get $2
    i32.const 12
    i32.mul
    i32.add
    local.tee $5
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.load
     local.tee $5
     i32.store offset=8
     local.get $0
     local.get $1
     local.get $5
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
     local.get $1
     i32.const 1
     i32.add
     local.set $1
    end
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   i32.const 1696
   i32.const 2928
   i32.const 114
   i32.const 42
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  i32.load
  local.tee $0
  i32.store offset=4
  local.get $0
  i32.eqz
  if
   i32.const 3632
   i32.const 2928
   i32.const 118
   i32.const 40
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#hasPointerStorageHash (param $0 i32) (param $1 i32) (result i32)
  (local $2 i64)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#keys"
  local.tee $0
  i32.store offset=4
  block $folding-inner0 (result i32)
   loop $for-loop|0
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    call $~lib/array/Array<u8>#get:length
    local.get $3
    i32.gt_s
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     local.get $3
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
     local.tee $4
     i32.store offset=8
     global.get $~lib/memory/__stack_pointer
     local.get $4
     i32.store offset=12
     global.get $~lib/memory/__stack_pointer
     local.get $1
     i32.store offset=16
     global.get $~lib/memory/__stack_pointer
     local.get $4
     i32.store
     local.get $4
     i64.load
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $1
     i32.store
     i32.const 1
     local.get $2
     local.get $1
     i64.load
     i64.eq
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store
      local.get $4
      i64.load offset=8
      local.set $2
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store
      local.get $2
      local.get $1
      i64.load offset=8
      i64.eq
     else
      i32.const 0
     end
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store
      local.get $4
      i64.load offset=16
      local.set $2
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store
      local.get $2
      local.get $1
      i64.load offset=16
      i64.eq
     else
      i32.const 0
     end
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $4
      i32.store
      local.get $4
      i64.load offset=24
      local.set $2
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store
      local.get $2
      local.get $1
      i64.load offset=24
      i64.eq
     else
      i32.const 0
     end
     br_if $folding-inner0
     drop
     local.get $3
     i32.const 1
     i32.add
     local.set $3
     br $for-loop|0
    end
   end
   i32.const 0
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#delete" (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $1
  call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#find"
  local.tee $1
  if
   local.get $1
   local.get $1
   i32.load offset=8
   i32.const 1
   i32.or
   call $~lib/rt/itcms/Object#set:prev
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   local.get $0
   local.get $0
   i32.load offset=20
   i32.const 1
   i32.sub
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load offset=4
   i32.const 1
   i32.shr_u
   local.set $1
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $1
   i32.const 1
   i32.add
   i32.const 4
   local.get $0
   i32.load offset=20
   local.tee $2
   local.get $2
   i32.const 4
   i32.lt_u
   select
   i32.ge_u
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.load offset=20
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $2
    local.get $0
    i32.load offset=12
    i32.const 3
    i32.mul
    i32.const 4
    i32.div_s
    i32.lt_s
   else
    i32.const 0
   end
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    local.get $1
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#rehash"
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#_internalSetStorageAt (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i64)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get@override"
  local.tee $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#keys"
  local.tee $6
  i32.store offset=16
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $6
   i32.store
   local.get $6
   call $~lib/array/Array<u8>#get:length
   local.get $5
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $6
    local.get $5
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $0
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=24
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=28
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i64.load
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    local.get $4
    local.get $2
    i64.load
    i64.eq
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     i64.load offset=8
     local.set $4
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $4
     local.get $2
     i64.load offset=8
     i64.eq
    else
     i32.const 0
    end
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     i64.load offset=16
     local.set $4
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $4
     local.get $2
     i64.load offset=16
     i64.eq
    else
     i32.const 0
    end
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     i64.load offset=24
     local.set $4
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $4
     local.get $2
     i64.load offset=24
     i64.eq
    else
     i32.const 0
    end
    if
     global.get $~lib/memory/__stack_pointer
     local.get $1
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     local.get $1
     i32.const 8
     i32.sub
     i32.load
     drop
     local.get $1
     local.get $0
     call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#delete"
    end
    local.get $5
    i32.const 1
    i32.add
    local.set $5
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $1
  local.get $2
  local.get $3
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#set@override"
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtPointer (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $4
  local.get $1
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has@override"
  i32.eqz
  if
   i32.const 0
   i32.const 2144
   i32.const 272
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=12
  local.get $0
  local.get $1
  local.get $2
  local.get $3
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialStorage
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $4
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get@override"
  local.tee $4
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $4
  local.get $2
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#hasPointerStorageHash
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=12
   local.get $0
   local.get $1
   local.get $2
   local.get $3
   call $src/btc/env/BTCEnvironment/BlockchainEnvironment#_internalSetStorageAt
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (param $4 i32) (result i32)
  (local $5 i64)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 40
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtAddress
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  local.get $3
  call $src/btc/math/abi/encodePointerHash
  local.tee $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=16
  local.get $0
  local.get $1
  local.get $2
  local.get $4
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtPointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get@override"
  local.tee $0
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#keys"
  local.tee $1
  i32.store offset=24
  loop $for-loop|0
   block $folding-inner0
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store
    local.get $1
    call $~lib/array/Array<u8>#get:length
    local.get $6
    i32.gt_s
    if
     global.get $~lib/memory/__stack_pointer
     local.get $1
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $1
     local.get $6
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
     local.tee $3
     i32.store offset=28
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store offset=32
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=36
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store
     local.get $3
     i64.load
     local.set $5
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $5
     local.get $2
     i64.load
     i64.eq
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store
      local.get $3
      i64.load offset=8
      local.set $5
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      local.get $5
      local.get $2
      i64.load offset=8
      i64.eq
     else
      i32.const 0
     end
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store
      local.get $3
      i64.load offset=16
      local.set $5
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      local.get $5
      local.get $2
      i64.load offset=16
      i64.eq
     else
      i32.const 0
     end
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store
      local.get $3
      i64.load offset=24
      local.set $5
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      local.get $5
      local.get $2
      i64.load offset=24
      i64.eq
     else
      i32.const 0
     end
     if
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store offset=4
      local.get $0
      i32.const 8
      i32.sub
      i32.load
      drop
      global.get $~lib/memory/__stack_pointer
      i32.const 12
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i64.const 0
      i64.store
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store offset=8
      local.get $0
      local.get $3
      local.get $3
      call $~lib/util/hash/HASH<src/btc/contracts/OP_NET/OP_NET>
      call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#find"
      local.tee $0
      i32.eqz
      if
       i32.const 3040
       i32.const 3104
       i32.const 105
       i32.const 17
       call $~lib/builtins/abort
       unreachable
      end
      local.get $0
      i32.load offset=4
      local.set $4
      global.get $~lib/memory/__stack_pointer
      i32.const 12
      i32.add
      global.set $~lib/memory/__stack_pointer
      br $folding-inner0
     end
     local.get $6
     i32.const 1
     i32.add
     local.set $6
     br $for-loop|0
    end
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $4
 )
 (func $src/btc/storage/StoredU256/StoredU256#constructor (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.const 34
  call $~lib/rt/itcms/__new
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $3
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  local.get $3
  i32.const 3
  call $src/btc/storage/StoredU256/StoredU256#set:pointer
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $3
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $3
  local.get $2
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:allowedWriteMethods
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $3
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load
  local.tee $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=28
  local.get $3
  i32.load16_u offset=4
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=8
  local.tee $1
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=12
  local.tee $5
  i32.store offset=24
  local.get $2
  local.get $0
  local.get $4
  local.get $1
  local.get $5
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $3
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
 )
 (func $src/btc/contracts/OP_20/OP_20#constructor (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  local.get $0
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 36
   i32.const 27
   call $~lib/rt/itcms/__new
   local.tee $0
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 3760
  i32.store offset=8
  local.get $0
  i32.const 3760
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 3792
  i32.store offset=8
  local.get $0
  i32.const 3792
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:events
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 8
  call $src/btc/buffer/BytesWriter/BytesWriter#set:trackDataTypes
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $src/btc/contracts/OP_20/OP_20#set:allowanceMap
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:memoryAllocatorAddress"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $1
  local.get $2
  call $src/btc/contracts/OP_NET/OP_NET#constructor
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=16
  local.get $1
  local.get $2
  i32.const 1
  i32.const 31
  i32.const 29
  call $"byn$mgfn-shared$src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#constructor"
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $0
  local.get $2
  call $src/btc/contracts/OP_20/OP_20#set:allowanceMap
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=16
  local.get $1
  local.get $2
  i32.const 2
  i32.const 33
  i32.const 32
  call $"byn$mgfn-shared$src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#constructor"
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $0
  local.get $2
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:memoryAllocatorAddress"
  global.get $~lib/memory/__stack_pointer
  local.set $4
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=12
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $5
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=16
  local.get $4
  local.get $2
  local.get $1
  i32.const 3
  local.get $3
  local.get $5
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt
  local.tee $2
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=24
  local.get $1
  local.get $3
  local.get $2
  call $src/btc/storage/StoredU256/StoredU256#constructor
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/lang/ContractDefaults/ContractDefaults#get:selfAddress (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $1
  i32.store
  local.get $1
  call $~lib/string/String.__not
  if
   i32.const 3904
   i32.const 3968
   i32.const 15
   i32.const 36
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/index/getContract (result i32)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 24
  memory.fill
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i32.load offset=16
  local.set $2
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  local.get $2
  call $src/btc/lang/ContractDefaults/ContractDefaults#get:selfAddress
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.load
  local.tee $1
  i32.store
  local.get $1
  call $~lib/string/String.__not
  if
   i32.const 4064
   i32.const 3968
   i32.const 21
   i32.const 26
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  i32.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 36
  i32.const 26
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 3824
  i32.store offset=8
  local.get $1
  i32.const 3824
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 3872
  i32.store offset=8
  local.get $1
  i32.const 3872
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:events
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 8
  call $src/btc/buffer/BytesWriter/BytesWriter#set:trackDataTypes
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  local.get $0
  local.get $4
  call $src/btc/contracts/OP_20/OP_20#constructor
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
  local.get $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=20
  local.get $2
  call $src/btc/lang/ContractDefaults/ContractDefaults#get:selfAddress
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=12
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $1
  local.get $2
  local.get $0
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set@override"
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#keys" (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=16
  local.tee $4
  i32.const 38
  call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $0
  i32.store offset=4
  loop $for-loop|0
   local.get $2
   local.get $4
   i32.lt_s
   if
    local.get $3
    local.get $2
    i32.const 12
    i32.mul
    i32.add
    local.tee $5
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.load
     local.tee $5
     i32.store offset=8
     local.get $0
     local.get $1
     local.get $5
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
     local.get $1
     i32.const 1
     i32.add
     local.set $1
    end
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#values" (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=16
  local.tee $4
  i32.const 39
  call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $0
  i32.store offset=4
  loop $for-loop|0
   local.get $2
   local.get $4
   i32.lt_s
   if
    local.get $3
    local.get $2
    i32.const 12
    i32.mul
    i32.add
    local.tee $5
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.load offset=4
     local.tee $5
     i32.store offset=8
     local.get $0
     local.get $1
     local.get $5
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
     local.get $1
     i32.const 1
     i32.add
     local.set $1
    end
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#hasMethodBySelectorInAllContracts (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#keys"
  local.tee $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#values"
  local.tee $0
  i32.store offset=12
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/array/Array<u8>#get:length
   local.get $2
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $3
    local.get $2
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $4
    i32.store offset=16
    local.get $4
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     local.get $2
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
     local.tee $5
     i32.store offset=20
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $5
     local.get $1
     call $~lib/set/Set<u32>#add
     local.tee $5
     i32.store offset=24
     local.get $5
     if
      global.get $~lib/memory/__stack_pointer
      i32.const 32
      i32.add
      global.set $~lib/memory/__stack_pointer
      local.get $4
      return
     end
    end
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $~lib/number/U32#toString
  local.tee $0
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  i32.const 4240
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  i32.const 4240
  i32.const 1
  local.get $0
  call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
  global.get $~lib/memory/__stack_pointer
  i32.const 4240
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 3504
  i32.store offset=4
  i32.const 4240
  call $~lib/staticarray/StaticArray<~lib/string/String>#join
  i32.const 3536
  i32.const 200
  i32.const 9
  call $~lib/builtins/abort
  unreachable
 )
 (func $src/btc/buffer/BytesReader/BytesReader#constructor (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.const 40
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $0
  i32.store offset=12
  i32.const 1
  global.set $~argumentsLength
  local.get $0
  call $~lib/dataview/DataView#constructor@varargs
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $1
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $src/btc/buffer/BytesReader/BytesReader#verifyEnd (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $3
  i32.store
  local.get $2
  local.get $3
  i32.load offset=8
  i32.gt_s
  if
   global.get $~lib/memory/__stack_pointer
   local.get $1
   call $~lib/number/I32#toString
   local.tee $1
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   call $~lib/number/I32#toString
   local.tee $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   i32.const 4976
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   i32.const 4976
   i32.const 1
   local.get $1
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 4976
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   i32.const 4976
   i32.const 3
   local.get $0
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 4976
   i32.store
   global.get $~lib/memory/__stack_pointer
   i32.const 3504
   i32.store offset=4
   i32.const 4976
   call $~lib/staticarray/StaticArray<~lib/string/String>#join
   i32.const 5024
   i32.const 135
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesReader/BytesReader#readU8 (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load offset=4
  i32.const 1
  i32.add
  call $src/btc/buffer/BytesReader/BytesReader#verifyEnd
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  local.get $0
  i32.load offset=4
  local.tee $0
  i32.const 1
  i32.add
  call $~lib/rt/itcms/Object#set:nextWithColor
  local.get $1
  local.get $0
  call $~lib/dataview/DataView#getUint8
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/buffer/BytesReader/BytesReader#readBytes (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load offset=4
  i32.const -64
  i32.sub
  call $src/btc/buffer/BytesReader/BytesReader#verifyEnd
  global.get $~lib/memory/__stack_pointer
  i32.const 64
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $2
  i32.store offset=8
  loop $for-loop|0
   local.get $7
   i32.const 64
   i32.lt_u
   if
    block $for-break0
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     call $src/btc/buffer/BytesReader/BytesReader#readU8
     local.tee $1
     i32.eqz
     if
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.set $5
      global.get $~lib/memory/__stack_pointer
      i32.const 12
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i64.const 0
      i64.store
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=4
      local.get $2
      call $~lib/typedarray/Uint8Array#get:length
      local.tee $1
      i32.const 0
      local.get $1
      i32.const 0
      i32.le_s
      select
      local.set $6
      global.get $~lib/memory/__stack_pointer
      local.get $7
      i32.const 0
      i32.lt_s
      if (result i32)
       local.get $1
       local.get $7
       i32.add
       local.tee $1
       i32.const 0
       local.get $1
       i32.const 0
       i32.gt_s
       select
      else
       local.get $7
       local.get $1
       local.get $1
       local.get $7
       i32.gt_s
       select
      end
      local.get $6
      i32.sub
      local.tee $1
      i32.const 0
      local.get $1
      i32.const 0
      i32.gt_s
      select
      local.tee $4
      call $~lib/typedarray/Uint8Array#constructor
      local.tee $1
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=4
      local.get $1
      i32.load offset=4
      local.set $3
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=4
      local.get $3
      local.get $2
      i32.load offset=4
      local.get $6
      i32.add
      local.get $4
      memory.copy
      global.get $~lib/memory/__stack_pointer
      i32.const 12
      i32.add
      global.set $~lib/memory/__stack_pointer
      local.get $5
      local.get $1
      local.tee $2
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=4
      local.get $0
      local.get $0
      i32.load offset=4
      i32.const 63
      local.get $7
      i32.sub
      i32.add
      call $~lib/rt/itcms/Object#set:nextWithColor
      br $for-break0
     end
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $2
     local.get $7
     local.get $1
     call $~lib/typedarray/Uint8Array#__set
     local.get $7
     i32.const 1
     i32.add
     local.set $7
     br $for-loop|0
    end
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/btc/buffer/BytesReader/BytesReader#readAddress (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $src/btc/buffer/BytesReader/BytesReader#readBytes
  local.tee $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $0
  call $~lib/arraybuffer/ArrayBuffer#get:byteLength
  call $~lib/string/String.UTF8.decodeUnsafe
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   local.get $2
   i32.const 7
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  local.get $1
  i32.eqz
  i32.eqz
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_NET/OP_NET#callMethod (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 3312
  i32.store
  i32.const 3312
  call $src/btc/math/abi/encodeSelector
  local.get $1
  i32.eq
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   i32.const 20
   i32.sub
   global.set $~lib/memory/__stack_pointer
   call $~stack_check
   global.get $~lib/memory/__stack_pointer
   i32.const 0
   i32.const 20
   memory.fill
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   call $src/btc/buffer/BytesReader/BytesReader#readAddress
   local.tee $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load
   local.tee $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=4
   local.tee $3
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=12
   local.get $1
   local.get $3
   local.get $2
   call $~lib/string/String.__eq
   call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $0
   global.get $~lib/memory/__stack_pointer
   i32.const 20
   i32.add
   global.set $~lib/memory/__stack_pointer
   global.get $~lib/memory/__stack_pointer
   i32.const 8
   i32.add
   global.set $~lib/memory/__stack_pointer
   local.get $0
   return
  end
  i32.const 5104
  i32.const 2336
  i32.const 58
  i32.const 17
  call $~lib/builtins/abort
  unreachable
 )
 (func $src/btc/exports/index/readMethod (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 24
  memory.fill
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $4
  i32.store
  local.get $4
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  local.set $5
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  block $__inlined_func$src/btc/universal/ABIRegistry/ABIRegistryBase#hasMethodBySelector$380
   local.get $1
   i32.eqz
   if
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store
    local.get $4
    local.get $0
    call $src/btc/universal/ABIRegistry/ABIRegistryBase#hasMethodBySelectorInAllContracts
    local.set $1
    br $__inlined_func$src/btc/universal/ABIRegistry/ABIRegistryBase#hasMethodBySelector$380
   end
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.sub
   global.set $~lib/memory/__stack_pointer
   call $~stack_check
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.load
   local.tee $6
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   local.get $6
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override"
   i32.eqz
   if
    i32.const 4592
    i32.const 3536
    i32.const 146
    i32.const 13
    call $~lib/builtins/abort
    unreachable
   end
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.load
   local.tee $4
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $4
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
   local.tee $4
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store
   local.get $1
   i32.const 0
   local.get $4
   local.get $0
   call $~lib/set/Set<u32>#has
   select
   local.set $1
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.add
   global.set $~lib/memory/__stack_pointer
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $5
  local.get $1
  i32.store offset=8
  local.get $1
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 4656
   i32.store
   local.get $0
   call $~lib/number/U32#toString
   local.set $0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   i32.const 4656
   local.get $0
   call $~lib/string/String#concat
   i32.const 4736
   i32.const 14
   i32.const 9
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  call $src/btc/buffer/BytesReader/BytesReader#constructor
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  block $__inlined_func$src/btc/contracts/OP_NET/OP_NET#callMethod@override$381 (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=16
   block $default
    block $case1
     local.get $1
     i32.const 8
     i32.sub
     i32.load
     local.tee $4
     i32.const 26
     i32.ne
     if
      local.get $4
      i32.const 27
      i32.eq
      br_if $case1
      br $default
     end
     local.get $1
     local.get $0
     local.get $2
     local.get $3
     call $src/contract/TestingContract/TestingContract#callMethod
     br $__inlined_func$src/btc/contracts/OP_NET/OP_NET#callMethod@override$381
    end
    local.get $1
    local.get $0
    local.get $2
    local.get $3
    call $src/btc/contracts/OP_20/OP_20#callMethod
    br $__inlined_func$src/btc/contracts/OP_NET/OP_NET#callMethod@override$381
   end
   local.get $1
   local.get $0
   local.get $2
   call $src/btc/contracts/OP_NET/OP_NET#callMethod
  end
  local.tee $0
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/universal/ABIRegistry/ABIRegistryBase#hasViewSelectorInAllContracts (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#keys"
  local.tee $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#values"
  local.tee $0
  i32.store offset=12
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/array/Array<u8>#get:length
   local.get $2
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $3
    local.get $2
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $4
    i32.store offset=16
    local.get $4
    if
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     local.get $2
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
     local.tee $5
     i32.store offset=20
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.store
     local.get $5
     local.get $1
     call $~lib/set/Set<u32>#has
     if
      global.get $~lib/memory/__stack_pointer
      i32.const 28
      i32.add
      global.set $~lib/memory/__stack_pointer
      local.get $4
      return
     end
    end
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $~lib/number/U32#toString
  local.tee $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  i32.const 5328
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  i32.const 5328
  i32.const 1
  local.get $0
  call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
  global.get $~lib/memory/__stack_pointer
  i32.const 5328
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 3504
  i32.store offset=4
  i32.const 5328
  call $~lib/staticarray/StaticArray<~lib/string/String>#join
  i32.const 3536
  i32.const 83
  i32.const 9
  call $~lib/builtins/abort
  unreachable
 )
 (func $src/btc/contracts/OP_NET/OP_NET#get:address (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#fromAddress (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $~lib/string/String#get:length
  i32.const 64
  i32.gt_s
  if
   i32.const 5440
   i32.const 5504
   i32.const 271
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 64
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $2
  i32.store offset=4
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/string/String#get:length
   local.get $1
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    local.get $2
    local.get $1
    local.get $0
    local.get $1
    call $~lib/string/String#charCodeAt
    call $~lib/typedarray/Uint8Array#__set
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $~lib/string/String#get:length
  local.set $0
  loop $for-loop|1
   local.get $0
   i32.const 255
   i32.and
   local.tee $1
   i32.const 64
   i32.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    local.get $2
    local.get $1
    i32.const 0
    call $~lib/typedarray/Uint8Array#__set
    local.get $0
    i32.const 1
    i32.add
    local.set $0
    br $for-loop|1
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeBytes (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/typedarray/Uint8Array#get:length
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $1
   call $~lib/typedarray/Uint8Array#get:length
   local.get $2
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    local.get $0
    local.get $1
    local.get $2
    call $~lib/typedarray/Uint8Array#__get
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeAddress (param $0 i32) (param $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   local.get $2
   i32.const 5
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#fromAddress
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeBytes
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_NET/OP_NET#callView (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 2704
  i32.store
  block $break|0
   block $case2|0
    block $case1|0
     i32.const 2704
     call $src/btc/math/abi/encodeSelector
     local.get $1
     i32.ne
     if
      global.get $~lib/memory/__stack_pointer
      i32.const 3280
      i32.store
      i32.const 3280
      call $src/btc/math/abi/encodeSelector
      local.get $1
      i32.eq
      br_if $case1|0
      br $case2|0
     end
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.load
     local.tee $1
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     call $src/btc/contracts/OP_NET/OP_NET#get:address
     local.set $2
     br $break|0
    end
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load
    local.tee $1
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    i32.const 4
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i32.const 0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.load offset=4
    local.set $2
    global.get $~lib/memory/__stack_pointer
    i32.const 4
    i32.add
    global.set $~lib/memory/__stack_pointer
    br $break|0
   end
   i32.const 5104
   i32.const 2336
   i32.const 71
   i32.const 17
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $1
  local.get $2
  call $src/btc/buffer/BytesWriter/BytesWriter#writeAddress
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/exports/index/readView (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $2
  i32.store
  local.get $2
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  local.set $4
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $3
  i32.store
  local.get $0
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  block $__inlined_func$src/btc/universal/ABIRegistry/ABIRegistryBase#hasSelectorForView$398
   local.get $1
   i32.eqz
   if
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store
    local.get $3
    local.get $0
    call $src/btc/universal/ABIRegistry/ABIRegistryBase#hasViewSelectorInAllContracts
    local.set $0
    br $__inlined_func$src/btc/universal/ABIRegistry/ABIRegistryBase#hasSelectorForView$398
   end
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.sub
   global.set $~lib/memory/__stack_pointer
   call $~stack_check
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.load offset=8
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   local.get $0
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#has@override"
   i32.eqz
   if
    i32.const 4592
    i32.const 3536
    i32.const 88
    i32.const 13
    call $~lib/builtins/abort
    unreachable
   end
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.load offset=8
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
   local.tee $0
   i32.store offset=12
   local.get $0
   i32.eqz
   if
    i32.const 4592
    i32.const 3536
    i32.const 93
    i32.const 13
    call $~lib/builtins/abort
    unreachable
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $1
   i32.const 0
   local.get $0
   local.get $2
   call $~lib/set/Set<u32>#has
   select
   local.set $0
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.add
   global.set $~lib/memory/__stack_pointer
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $4
  local.get $0
  i32.store offset=8
  local.get $0
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 5360
   i32.store
   local.get $2
   call $~lib/number/U32#toString
   local.set $0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   i32.const 5360
   local.get $0
   call $~lib/string/String#concat
   i32.const 4736
   i32.const 32
   i32.const 9
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  block $__inlined_func$src/btc/contracts/OP_NET/OP_NET#callView@override$399 (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.const 8
   i32.sub
   i32.load
   local.tee $1
   i32.const 27
   i32.eq
   local.get $1
   i32.const 26
   i32.eq
   i32.or
   if
    local.get $0
    local.get $2
    call $src/btc/contracts/OP_20/OP_20#callView
    br $__inlined_func$src/btc/contracts/OP_NET/OP_NET#callView@override$399
   end
   local.get $0
   local.get $2
   call $src/btc/contracts/OP_NET/OP_NET#callView
  end
  local.tee $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get:size" (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=20
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeViewSelectorMap (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  (local $10 i32)
  (local $11 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 24
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get:size"
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU16
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#keys"
  local.tee $5
  i32.store offset=8
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $5
   i32.store
   local.get $5
   call $~lib/array/Array<u8>#get:length
   local.get $6
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $5
    local.get $6
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $2
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $1
    local.get $2
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
    local.tee $9
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $2
    call $src/btc/contracts/OP_NET/OP_NET#get:address
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    local.get $0
    local.get $2
    call $src/btc/buffer/BytesWriter/BytesWriter#writeAddress
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.store offset=4
    i32.const 0
    local.set $3
    global.get $~lib/memory/__stack_pointer
    i32.const 16
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.store offset=4
    local.get $0
    local.get $9
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get:size"
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU16
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.set $7
    i32.const 0
    local.set $4
    i32.const 0
    local.set $2
    global.get $~lib/memory/__stack_pointer
    i32.const 12
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    i32.const 0
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.store
    local.get $9
    i32.load offset=8
    local.set $8
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.load offset=16
    local.tee $10
    i32.const 41
    call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
    local.tee $9
    i32.store offset=4
    loop $for-loop|00
     local.get $4
     local.get $10
     i32.lt_s
     if
      local.get $8
      local.get $4
      i32.const 3
      i32.shl
      i32.add
      local.tee $11
      i32.load offset=4
      i32.const 1
      i32.and
      i32.eqz
      if
       global.get $~lib/memory/__stack_pointer
       local.get $9
       i32.store
       global.get $~lib/memory/__stack_pointer
       local.get $11
       i32.load
       local.tee $11
       i32.store offset=8
       local.get $9
       local.get $2
       local.get $11
       call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
       local.get $2
       i32.const 1
       i32.add
       local.set $2
      end
      local.get $4
      i32.const 1
      i32.add
      local.set $4
      br $for-loop|00
     end
    end
    global.get $~lib/memory/__stack_pointer
    local.get $9
    i32.store
    local.get $9
    local.get $2
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
    global.get $~lib/memory/__stack_pointer
    i32.const 12
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $7
    local.get $9
    i32.store offset=8
    loop $for-loop|001
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.store
     local.get $9
     call $~lib/array/Array<u8>#get:length
     local.get $3
     i32.gt_s
     if
      global.get $~lib/memory/__stack_pointer
      local.get $9
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $9
      local.get $3
      call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
      local.tee $2
      i32.store offset=12
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=4
      local.get $0
      local.get $2
      call $src/btc/buffer/BytesWriter/BytesWriter#writeBytes
      local.get $3
      i32.const 1
      i32.add
      local.set $3
      br $for-loop|001
     end
    end
    global.get $~lib/memory/__stack_pointer
    i32.const 16
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $6
    i32.const 1
    i32.add
    local.set $6
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/exports/index/getViewABI (result i32)
  (local $0 i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=4
  local.tee $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeViewSelectorMap
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/math/cyrb53/cyrb53a (param $0 i32) (result i64)
  (local $1 i64)
  (local $2 i64)
  (local $3 i32)
  (local $4 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  i64.const 3735928559
  local.set $1
  i64.const 1103547991
  local.set $2
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   call $~lib/array/Array<u8>#get:length
   local.get $3
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    local.get $3
    call $~lib/array/Array<u8>#__get
    i64.extend_i32_u
    local.tee $4
    local.get $1
    i64.xor
    i64.const 2246822519
    call $src/btc/math/cyrb53/imul64
    local.set $1
    local.get $2
    local.get $4
    i64.xor
    i64.const 3266489917
    call $src/btc/math/cyrb53/imul64
    local.set $2
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  local.get $2
  local.get $1
  local.get $2
  i64.const 15
  i64.shr_u
  i64.xor
  i64.const 1935289751
  call $src/btc/math/cyrb53/imul64
  local.get $1
  i64.xor
  local.tee $1
  i64.const 15
  i64.shr_u
  i64.xor
  i64.const 3405138345
  call $src/btc/math/cyrb53/imul64
  local.get $2
  i64.xor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
  i64.const 16
  i64.shr_u
  local.get $1
  i64.xor
  local.tee $1
  i64.const 16
  i64.shr_u
  local.get $2
  i64.xor
  i64.const 21
  i64.shl
  local.get $1
  i64.const 11
  i64.shr_u
  i64.add
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeU64 (param $0 i32) (param $1 i64)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   local.get $2
   i32.const 3
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 8
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.load
  local.set $3
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $3
  i32.const 31
  i32.shr_u
  local.get $2
  i32.load offset=8
  local.get $3
  i32.const 8
  i32.add
  i32.lt_s
  i32.or
  if
   i32.const 1696
   i32.const 3152
   i32.const 174
   i32.const 7
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $3
  local.get $2
  i32.load offset=4
  i32.add
  local.get $1
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load
  i32.const 8
  i32.add
  call $~lib/rt/tlsf/Root#set:flMap
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#bufferLength (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#getEvents (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i64)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=20
  local.tee $1
  i32.store
  local.get $1
  call $~lib/array/Array<u8>#get:length
  local.tee $3
  i32.const 255
  i32.and
  i32.const 8
  i32.gt_u
  if
   i32.const 0
   i32.const 2144
   i32.const 88
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $5
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  local.get $5
  local.get $3
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
  loop $for-loop|0
   local.get $2
   local.get $3
   i32.const 255
   i32.and
   i32.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=20
    local.tee $1
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    local.get $2
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $1
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.load
    local.tee $6
    i32.store offset=4
    local.get $5
    local.get $6
    call $src/btc/buffer/BytesWriter/BytesWriter#writeStringWithLength
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.load offset=4
    local.tee $6
    i32.store
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.load offset=8
    local.tee $7
    i32.store
    block $__inlined_func$src/btc/buffer/BytesWriter/BytesWriter#getSelectorDataType$840
     local.get $7
     call $~lib/array/Array<u8>#get:length
     i32.eqz
     if
      global.get $~lib/memory/__stack_pointer
      i32.const 8
      i32.add
      global.set $~lib/memory/__stack_pointer
      i64.const 0
      local.set $4
      br $__inlined_func$src/btc/buffer/BytesWriter/BytesWriter#getSelectorDataType$840
     end
     global.get $~lib/memory/__stack_pointer
     local.get $6
     i32.store offset=4
     global.get $~lib/memory/__stack_pointer
     local.get $6
     i32.load offset=8
     local.tee $6
     i32.store
     local.get $6
     call $src/btc/math/cyrb53/cyrb53a
     local.set $4
     global.get $~lib/memory/__stack_pointer
     i32.const 8
     i32.add
     global.set $~lib/memory/__stack_pointer
    end
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $5
    local.get $4
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU64
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.load offset=4
    local.tee $6
    i32.store
    local.get $6
    call $src/btc/buffer/BytesWriter/BytesWriter#bufferLength
    i32.const 256
    i32.gt_u
    if
     i32.const 5584
     i32.const 5696
     i32.const 20
     i32.const 13
     call $~lib/builtins/abort
     unreachable
    end
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.load offset=4
    local.tee $1
    i32.store
    local.get $1
    call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
    local.set $6
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.add
    global.set $~lib/memory/__stack_pointer
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store
    local.get $6
    i32.load offset=8
    local.set $7
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    local.get $5
    local.get $7
    i32.const 4
    i32.add
    call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    local.get $5
    local.get $7
    i32.const 1
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU32
    i32.const 0
    local.set $1
    loop $for-loop|00
     local.get $1
     local.get $7
     i32.lt_u
     if
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $6
      i32.store offset=4
      local.get $5
      local.get $6
      local.get $1
      call $~lib/typedarray/Uint8Array#__get
      call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
      local.get $1
      i32.const 1
      i32.add
      local.set $1
      br $for-loop|00
     end
    end
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 0
  i32.const 2
  i32.const 24
  i32.const 5776
  call $~lib/rt/__newArray
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:events
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  local.get $5
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/exports/index/getEvents (result i32)
  (local $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getEvents
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/set/Set<u32>#values (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=8
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=16
  local.tee $5
  i32.const 4
  call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $2
  i32.store offset=4
  loop $for-loop|0
   local.get $3
   local.get $5
   i32.lt_s
   if
    local.get $4
    local.get $3
    i32.const 3
    i32.shl
    i32.add
    local.tee $6
    i32.load offset=4
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $1
     local.tee $0
     i32.const 1
     i32.add
     local.set $1
     local.get $6
     i32.load
     local.set $7
     global.get $~lib/memory/__stack_pointer
     i32.const 4
     i32.sub
     global.set $~lib/memory/__stack_pointer
     call $~stack_check
     global.get $~lib/memory/__stack_pointer
     i32.const 0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $0
     local.get $2
     i32.load offset=12
     i32.ge_u
     if
      local.get $0
      i32.const 0
      i32.lt_s
      if
       i32.const 1696
       i32.const 2928
       i32.const 130
       i32.const 22
       call $~lib/builtins/abort
       unreachable
      end
      local.get $2
      local.get $0
      i32.const 1
      i32.add
      local.tee $6
      i32.const 2
      i32.const 1
      call $~lib/array/ensureCapacity
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      local.get $2
      local.get $6
      call $~lib/rt/itcms/Object#set:rtId
     end
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store
     local.get $2
     i32.load offset=4
     local.get $0
     i32.const 2
     i32.shl
     i32.add
     local.get $7
     i32.store
     global.get $~lib/memory/__stack_pointer
     i32.const 4
     i32.add
     global.set $~lib/memory/__stack_pointer
    end
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  local.get $1
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeMethodSelectorsMap (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 24
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get:size"
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU16
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#keys"
  local.tee $3
  i32.store offset=8
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   call $~lib/array/Array<u8>#get:length
   local.get $4
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $3
    local.get $4
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $2
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $1
    local.get $2
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get@override"
    local.tee $5
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $2
    call $src/btc/contracts/OP_NET/OP_NET#get:address
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    local.get $0
    local.get $2
    call $src/btc/buffer/BytesWriter/BytesWriter#writeAddress
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=4
    i32.const 0
    local.set $2
    global.get $~lib/memory/__stack_pointer
    i32.const 12
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    i32.const 0
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=4
    local.get $0
    local.get $5
    call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get:size"
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU16
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $5
    call $~lib/set/Set<u32>#values
    local.tee $5
    i32.store offset=8
    loop $for-loop|00
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.store
     local.get $5
     call $~lib/array/Array<u8>#get:length
     local.get $2
     i32.gt_s
     if
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store
      local.get $5
      local.get $2
      call $~lib/array/Array<u32>#__get
      local.set $6
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      local.get $6
      call $src/btc/buffer/BytesWriter/BytesWriter#writeSelector
      local.get $2
      i32.const 1
      i32.add
      local.set $2
      br $for-loop|00
     end
    end
    global.get $~lib/memory/__stack_pointer
    i32.const 12
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $4
    i32.const 1
    i32.add
    local.set $4
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/exports/index/getMethodABI (result i32)
  (local $0 i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load
  local.tee $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeMethodSelectorsMap
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/exports/index/getWriteMethods (result i32)
  (local $0 i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/universal/ABIRegistry/ABIRegistry
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=12
  local.tee $1
  i32.store offset=8
  local.get $0
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeMethodSelectorsMap
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeU256 (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load8_u offset=12
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=8
   local.tee $2
   i32.store
   local.get $2
   i32.const 4
   call $~lib/array/Array<u8>#push
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.const 32
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  global.get $~lib/memory/__stack_pointer
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  call $~lib/typedarray/Uint8Array#constructor
  local.tee $3
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  i32.load offset=4
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $4
  local.get $1
  i64.load offset=24
  call $~lib/polyfills/bswap<u64>
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $4
  local.get $1
  i64.load offset=16
  call $~lib/polyfills/bswap<u64>
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $4
  local.get $1
  i64.load offset=8
  call $~lib/polyfills/bswap<u64>
  i64.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $4
  local.get $1
  i64.load
  call $~lib/polyfills/bswap<u64>
  i64.store offset=24
  local.get $2
  local.get $3
  i32.store offset=28
  i32.const 0
  local.set $1
  loop $for-loop|0
   local.get $1
   i32.const 32
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=4
    local.get $0
    local.get $3
    local.get $1
    call $~lib/typedarray/Uint8Array#__get
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeStorage (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  (local $10 i32)
  (local $11 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 40
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<~lib/typedarray/Uint8Array>>#get:size"
  i32.const 1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU32
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.set $6
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i32.load offset=8
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=16
  local.tee $8
  i32.const 42
  call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $4
  i32.store offset=4
  loop $for-loop|0
   local.get $3
   local.get $8
   i32.lt_s
   if
    local.get $7
    local.get $3
    i32.const 12
    i32.mul
    i32.add
    local.tee $9
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $4
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.load
     local.tee $9
     i32.store offset=8
     local.get $4
     local.get $2
     local.get $9
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
     local.get $2
     i32.const 1
     i32.add
     local.set $2
    end
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store
  local.get $4
  local.get $2
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $6
  local.get $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.set $6
  i32.const 0
  local.set $3
  i32.const 0
  local.set $2
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i32.load offset=8
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=16
  local.tee $1
  i32.const 43
  call $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $8
  i32.store offset=4
  loop $for-loop|00
   local.get $1
   local.get $3
   i32.gt_s
   if
    local.get $7
    local.get $3
    i32.const 12
    i32.mul
    i32.add
    local.tee $9
    i32.load offset=8
    i32.const 1
    i32.and
    i32.eqz
    if
     global.get $~lib/memory/__stack_pointer
     local.get $8
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $9
     i32.load offset=4
     local.tee $9
     i32.store offset=8
     local.get $8
     local.get $2
     local.get $9
     call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
     local.get $2
     i32.const 1
     i32.add
     local.set $2
    end
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|00
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $8
  i32.store
  local.get $8
  local.get $2
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $6
  local.get $8
  i32.store offset=12
  loop $for-loop|001
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store
   local.get $4
   call $~lib/array/Array<u8>#get:length
   local.get $5
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $4
    local.get $5
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $1
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $8
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $8
    local.get $5
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.tee $2
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    local.get $0
    local.get $1
    call $src/btc/buffer/BytesWriter/BytesWriter#writeAddress
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#keys"
    local.tee $6
    i32.store offset=24
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.set $7
    i32.const 0
    local.set $1
    i32.const 0
    local.set $3
    global.get $~lib/memory/__stack_pointer
    i32.const 12
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    i32.const 0
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    local.get $2
    i32.load offset=8
    local.set $9
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.load offset=16
    local.tee $10
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
    local.tee $2
    i32.store offset=4
    loop $for-loop|01
     local.get $1
     local.get $10
     i32.lt_s
     if
      local.get $9
      local.get $1
      i32.const 12
      i32.mul
      i32.add
      local.tee $11
      i32.load offset=8
      i32.const 1
      i32.and
      i32.eqz
      if
       global.get $~lib/memory/__stack_pointer
       local.get $2
       i32.store
       global.get $~lib/memory/__stack_pointer
       local.get $11
       i32.load offset=4
       local.tee $11
       i32.store offset=8
       local.get $2
       local.get $3
       local.get $11
       call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
       local.get $3
       i32.const 1
       i32.add
       local.set $3
      end
      local.get $1
      i32.const 1
      i32.add
      local.set $1
      br $for-loop|01
     end
    end
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    local.get $2
    local.get $3
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#set:length
    global.get $~lib/memory/__stack_pointer
    i32.const 12
    i32.add
    global.set $~lib/memory/__stack_pointer
    local.get $7
    local.get $2
    i32.store offset=28
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $6
    i32.store offset=4
    local.get $0
    local.get $6
    call $~lib/array/Array<u8>#get:length
    i32.const 1
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU32
    i32.const 0
    local.set $1
    loop $for-loop|1
     global.get $~lib/memory/__stack_pointer
     local.get $6
     i32.store
     local.get $6
     call $~lib/array/Array<u8>#get:length
     local.get $1
     i32.gt_s
     if
      global.get $~lib/memory/__stack_pointer
      local.get $6
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $6
      local.get $1
      call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
      local.tee $3
      i32.store offset=32
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      local.get $1
      call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
      local.tee $7
      i32.store offset=36
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store offset=4
      local.get $0
      local.get $3
      call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $7
      i32.store offset=4
      local.get $0
      local.get $7
      call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
      local.get $1
      i32.const 1
      i32.add
      local.set $1
      br $for-loop|1
     end
    end
    local.get $5
    i32.const 1
    i32.add
    local.set $5
    br $for-loop|001
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/exports/index/getModifiedStorage (result i32)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $src/btc/buffer/BytesWriter/BytesWriter#writeStorage
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $0
  i32.store offset=4
  local.get $0
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#clear@override"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/exports/index/initializeStorage (result i32)
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#requireInitialization
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $src/btc/buffer/BytesWriter/BytesWriter#writeStorage
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $0
  i32.store offset=4
  local.get $0
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#clear@override"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#getBuffer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#purgeMemory (param $0 i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $1
  i32.store
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#clear@override"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $1
  i32.store
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#clear@override"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 0
  i32.const 2
  i32.const 24
  i32.const 5808
  call $~lib/rt/__newArray
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:events
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/buffer/BytesReader/BytesReader#readU32 (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load offset=4
  i32.const 4
  i32.add
  call $src/btc/buffer/BytesReader/BytesReader#verifyEnd
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.load offset=4
  local.set $2
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $2
  i32.const 31
  i32.shr_u
  local.get $1
  i32.load offset=8
  local.get $2
  i32.const 4
  i32.add
  i32.lt_s
  i32.or
  if
   i32.const 1696
   i32.const 3152
   i32.const 87
   i32.const 7
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i32.load offset=4
  local.get $2
  i32.add
  i32.load
  local.set $1
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $0
  i32.load offset=4
  i32.const 4
  i32.add
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $src/btc/buffer/BytesReader/BytesReader#readU256 (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 0
  i32.const 5
  i32.const 5840
  call $~lib/rt/__newArray
  local.tee $1
  i32.store
  loop $for-loop|0
   local.get $2
   i32.const 32
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    local.get $1
    local.get $2
    local.get $0
    call $src/btc/buffer/BytesReader/BytesReader#readU8
    call $~lib/array/Array<u8>#__set
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  call $~lib/array/Array<u8>#get:length
  if
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   local.get $1
   call $~lib/array/Array<u8>#get:length
   drop
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.load offset=4
  local.tee $0
  i64.load offset=24
  call $~lib/polyfills/bswap<u64>
  local.get $0
  i64.load offset=16
  call $~lib/polyfills/bswap<u64>
  local.get $0
  i64.load offset=8
  call $~lib/polyfills/bswap<u64>
  local.get $0
  i64.load
  call $~lib/polyfills/bswap<u64>
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#loadStorage (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#purgeMemory
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $src/btc/buffer/BytesReader/BytesReader#constructor
  local.tee $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  call $src/btc/buffer/BytesReader/BytesReader#readU32
  local.set $6
  loop $for-loop|0
   local.get $4
   local.get $6
   i32.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    call $src/btc/buffer/BytesReader/BytesReader#readAddress
    local.tee $1
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store
    local.get $2
    call $src/btc/buffer/BytesReader/BytesReader#readU32
    local.set $7
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=12
    local.get $0
    local.get $1
    call $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtAddress
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=4
    local.tee $5
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $5
    local.get $1
    call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get@override"
    local.tee $8
    i32.store offset=20
    i32.const 0
    local.set $1
    loop $for-loop|1
     local.get $1
     local.get $7
     i32.lt_u
     if
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      call $src/btc/buffer/BytesReader/BytesReader#readU256
      local.tee $9
      i32.store offset=24
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      call $src/btc/buffer/BytesReader/BytesReader#readU256
      local.tee $5
      i32.store offset=28
      global.get $~lib/memory/__stack_pointer
      local.get $8
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $9
      i32.store offset=12
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=16
      local.get $8
      local.get $9
      local.get $5
      call $"~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>#set@override"
      local.get $1
      i32.const 1
      i32.add
      local.set $1
      br $for-loop|1
     end
    end
    local.get $4
    i32.const 1
    i32.add
    local.set $4
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  loop $for-loop|00
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.load
   local.tee $0
   i32.store
   local.get $3
   local.get $0
   i32.load offset=8
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.load
    local.tee $0
    i32.store
    local.get $0
    local.get $3
    i32.const 0
    call $~lib/dataview/DataView#setUint8
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|00
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/exports/index/allocateMemory (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  i32.store
  local.get $0
  call $~lib/rt/tlsf/__alloc
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/exports/index/deallocateMemory (param $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  i32.store
  local.get $0
  call $~lib/rt/tlsf/__free
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/exports/index/isInitialized (result i32)
  (local $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  i32.load8_u
  i32.const 0
  i32.ne
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/exports/index/purgeMemory
  (local $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#purgeMemory
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_20/OP_20#defineSelectors (param $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6000
  i32.store offset=4
  local.get $0
  i32.const 6000
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6048
  i32.store offset=4
  local.get $0
  i32.const 6048
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6096
  i32.store offset=4
  local.get $0
  i32.const 6096
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6144
  i32.store offset=4
  local.get $0
  i32.const 6144
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6176
  i32.store offset=4
  local.get $0
  i32.const 6176
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6208
  i32.store offset=4
  local.get $0
  i32.const 6208
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6256
  i32.store offset=4
  local.get $0
  i32.const 6256
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6304
  i32.store offset=4
  local.get $0
  i32.const 6304
  call $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6352
  i32.store offset=4
  local.get $0
  i32.const 6352
  call $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6384
  i32.store offset=4
  local.get $0
  i32.const 6384
  call $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6416
  i32.store offset=4
  local.get $0
  i32.const 6416
  call $src/btc/contracts/OP_NET/OP_NET#defineGetterSelector
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#defineSelectors (param $0 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $src/btc/contracts/OP_20/OP_20#defineSelectors
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6464
  i32.store offset=4
  local.get $0
  i32.const 6464
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6528
  i32.store offset=4
  local.get $0
  i32.const 6528
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6592
  i32.store offset=4
  local.get $0
  i32.const 6592
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6656
  i32.store offset=4
  local.get $0
  i32.const 6656
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6704
  i32.store offset=4
  local.get $0
  i32.const 6704
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6752
  i32.store offset=4
  local.get $0
  i32.const 6752
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6816
  i32.store offset=4
  local.get $0
  i32.const 6816
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6880
  i32.store offset=4
  local.get $0
  i32.const 6880
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6944
  i32.store offset=4
  local.get $0
  i32.const 6944
  i32.const 1
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 6992
  i32.store offset=4
  local.get $0
  i32.const 6992
  i32.const 0
  call $src/btc/contracts/OP_NET/OP_NET#defineMethodSelector
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/math/abi/encodePointer (param $0 i32) (result i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  i32.const 1
  global.set $~argumentsLength
  local.get $0
  call $~lib/string/String.UTF8.encode@varargs
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  i32.const 1
  global.set $~argumentsLength
  local.get $1
  local.get $0
  call $~lib/typedarray/Uint8Array.wrap@varargs
  local.tee $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  call $src/btc/math/sha256/Sha256.hash
  local.tee $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $src/btc/math/bytes/bytes32
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get" (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  local.get $0
  i32.load16_u offset=24
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $1
  call $src/btc/math/abi/encodePointer
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $0
  i32.store offset=12
  local.get $2
  local.get $3
  local.get $4
  local.get $1
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/as-bignum/assembly/integer/u256/u256.add (param $0 i32) (param $1 i32) (result i32)
  (local $2 i64)
  (local $3 i64)
  (local $4 i64)
  (local $5 i64)
  (local $6 i64)
  (local $7 i64)
  (local $8 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=8
  local.set $5
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=16
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=24
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=8
  local.set $8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=16
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $2
  local.get $3
  i64.add
  local.get $5
  local.get $8
  i64.and
  local.get $5
  local.get $8
  i64.or
  local.get $6
  local.get $7
  i64.add
  local.tee $7
  local.get $6
  i64.lt_u
  i64.extend_i32_u
  local.get $5
  local.get $8
  i64.add
  i64.add
  local.tee $5
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  i64.add
  local.set $6
  local.get $7
  local.get $5
  local.get $6
  local.get $4
  local.get $1
  i64.load offset=24
  i64.add
  local.get $2
  local.get $3
  i64.and
  local.get $2
  local.get $3
  i64.or
  local.get $6
  i64.const -1
  i64.xor
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  i64.add
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/as-bignum/assembly/integer/u256/u256.lt (param $0 i32) (param $1 i32) (result i32)
  (local $2 i64)
  (local $3 i64)
  (local $4 i64)
  (local $5 i64)
  (local $6 i64)
  (local $7 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=24
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=16
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=24
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=16
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=8
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=8
  local.set $5
  local.get $2
  local.get $6
  i64.eq
  if (result i32)
   local.get $3
   local.get $7
   i64.eq
   if (result i32)
    local.get $4
    local.get $5
    i64.eq
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     i64.load
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $1
     i32.store
     local.get $2
     local.get $1
     i64.load
     i64.lt_u
    else
     local.get $4
     local.get $5
     i64.lt_u
    end
   else
    local.get $3
    local.get $7
    i64.gt_u
   end
  else
   local.get $2
   local.get $6
   i64.gt_u
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/types/SafeMath/SafeMath.add (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $1
  call $~lib/as-bignum/assembly/integer/u256/u256.add
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $1
  local.get $0
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 7072
   i32.const 7152
   i32.const 9
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (param $4 i32) (param $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtAddress
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  local.get $3
  call $src/btc/math/abi/encodePointerHash
  local.tee $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=16
  local.get $0
  local.get $1
  local.get $2
  local.get $5
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtPointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=16
  local.get $0
  local.get $1
  local.get $2
  local.get $4
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#_internalSetStorageAt
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set" (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $src/btc/math/abi/encodePointer
  local.tee $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  local.get $0
  i32.load16_u offset=24
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $0
  i32.store offset=20
  local.get $5
  local.get $1
  local.get $3
  local.get $4
  local.get $2
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/storage/StoredU256/StoredU256#ensureValue (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $5
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  local.get $0
  i32.load16_u offset=4
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $2
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=12
  local.tee $3
  i32.store offset=20
  local.get $4
  local.get $5
  local.get $1
  local.get $2
  local.get $3
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/events/MintEvent/MintEvent#constructor (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.const 44
  call $~lib/rt/itcms/__new
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 1
  call $src/btc/buffer/BytesWriter/BytesWriter#constructor
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  local.get $3
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#writeAddress
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $3
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
  global.get $~lib/memory/__stack_pointer
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 7232
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  local.get $2
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 8
   i32.const 23
   call $~lib/rt/itcms/__new
   local.tee $2
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 7232
  i32.store offset=8
  local.get $2
  i32.const 7232
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $2
  local.get $3
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#addEvent (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=20
  local.tee $2
  i32.store
  local.get $2
  call $~lib/array/Array<u8>#get:length
  i32.const 8
  i32.ge_s
  if
   i32.const 0
   i32.const 2144
   i32.const 79
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=20
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  local.get $2
  i32.load offset=12
  local.tee $0
  i32.const 1
  i32.add
  local.tee $3
  i32.const 2
  i32.const 1
  call $~lib/array/ensureCapacity
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  i32.load offset=4
  local.get $0
  i32.const 2
  i32.shl
  i32.add
  local.get $1
  i32.store
  local.get $2
  local.get $1
  i32.const 1
  call $~lib/rt/itcms/__link
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  local.get $3
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#createMintEvent (param $0 i32) (param $1 i32) (param $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  local.get $2
  call $src/contract/events/MintEvent/MintEvent#constructor
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=4
  local.tee $0
  i32.store
  local.get $0
  call $src/btc/buffer/BytesWriter/BytesWriter#bufferLength
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
  i32.const 256
  i32.gt_u
  if
   i32.const 5584
   i32.const 2336
   i32.const 79
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#addEvent
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#_addFreeMoney (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 48
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  local.get $1
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $3
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  local.get $2
  call $src/btc/types/SafeMath/SafeMath.add
  local.tee $3
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $4
  local.get $1
  local.get $3
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $6
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=8
  local.get $6
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.load offset=16
  local.tee $3
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=36
  local.get $3
  local.get $2
  call $src/btc/types/SafeMath/SafeMath.add
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=20
  local.get $6
  local.get $3
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $7
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.load
  local.tee $8
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=44
  local.get $6
  i32.load16_u offset=4
  local.set $9
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.load offset=8
  local.tee $3
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.load offset=16
  local.tee $4
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.load offset=12
  local.tee $5
  i32.store offset=40
  local.get $7
  local.get $8
  local.get $9
  local.get $3
  local.get $4
  local.get $5
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=4
  local.get $0
  local.get $6
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $2
  call $src/contract/TestingContract/TestingContract#createMintEvent
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/env/BTCEnvironment/BlockchainEnvironment#hasStorageAt (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  (local $4 i64)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 36
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 36
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#ensureStorageAtAddress
  global.get $~lib/memory/__stack_pointer
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $5
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=12
  local.get $6
  local.get $0
  local.get $1
  local.get $2
  local.get $3
  local.get $5
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt
  local.tee $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.tee $1
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $4
  local.get $1
  i64.load
  i64.eq
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i64.load offset=8
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $1
   i64.load offset=8
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i64.load offset=16
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $1
   i64.load offset=16
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i64.load offset=24
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $1
   i64.load offset=24
   i64.eq
  else
   i32.const 0
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 36
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
  i32.eqz
 )
 (func $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has" (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  local.get $0
  i32.load16_u offset=24
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $1
  call $src/btc/math/abi/encodePointer
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $2
  local.get $3
  local.get $0
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#hasStorageAt
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/contracts/OP_20/OP_20#_balanceOf (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  block $folding-inner0 (result i32)
   local.get $2
   local.get $1
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
   i32.eqz
   if
    i64.const 0
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    br $folding-inner0
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   local.get $0
   local.get $1
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/as-bignum/assembly/integer/u256/u256.sub (param $0 i32) (param $1 i32) (result i32)
  (local $2 i64)
  (local $3 i64)
  (local $4 i64)
  (local $5 i64)
  (local $6 i64)
  (local $7 i64)
  (local $8 i64)
  (local $9 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=8
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=16
  local.set $8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=24
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=8
  local.set $5
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $1
  i64.load offset=16
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $8
  local.get $2
  i64.sub
  local.get $7
  i64.const -1
  i64.xor
  local.tee $9
  local.get $5
  i64.and
  local.get $7
  local.get $5
  i64.sub
  local.get $6
  local.get $4
  i64.sub
  local.tee $4
  local.get $6
  i64.gt_u
  i64.extend_i32_u
  i64.sub
  local.tee $6
  local.get $5
  local.get $9
  i64.or
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  i64.sub
  local.set $5
  local.get $4
  local.get $6
  local.get $5
  local.get $3
  local.get $1
  i64.load offset=24
  i64.sub
  local.get $8
  i64.const -1
  i64.xor
  local.tee $3
  local.get $2
  i64.and
  local.get $2
  local.get $3
  i64.or
  local.get $5
  i64.and
  i64.or
  i64.const 63
  i64.shr_u
  i64.sub
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/types/SafeMath/SafeMath.sub (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 7264
   i32.const 7152
   i32.const 16
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/as-bignum/assembly/integer/u256/u256.sub
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/as-bignum/assembly/integer/u256/u256#clone (param $0 i32) (result i32)
  (local $1 i64)
  (local $2 i64)
  (local $3 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=8
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i64.load offset=16
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $2
  local.get $3
  local.get $0
  i64.load offset=24
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<u64>#__set (param $0 i32) (param $1 i32) (param $2 i64)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   local.get $1
   i32.const 0
   i32.lt_s
   if
    i32.const 1696
    i32.const 2928
    i32.const 130
    i32.const 22
    call $~lib/builtins/abort
    unreachable
   end
   local.get $0
   local.get $1
   i32.const 1
   i32.add
   local.tee $3
   i32.const 3
   i32.const 1
   call $~lib/array/ensureCapacity
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   local.get $3
   call $~lib/rt/itcms/Object#set:rtId
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 3
  i32.shl
  i32.add
  local.get $2
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $~lib/array/Array<u64>#constructor (result i32)
  (local $0 i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.const 45
  call $~lib/rt/itcms/__new
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  i32.const 64
  i32.const 1
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $0
  local.get $1
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 64
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $0
  i32.const 4
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<u64>#fill@varargs (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  block $2of2
   block $1of2
    block $outOfRange
     global.get $~argumentsLength
     i32.const 1
     i32.sub
     br_table $1of2 $1of2 $2of2 $outOfRange
    end
    unreachable
   end
   i32.const 2147483647
   local.set $2
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=12
  local.tee $4
  i32.const 0
  local.get $4
  i32.const 0
  i32.le_s
  select
  local.set $1
  local.get $2
  local.get $4
  local.get $2
  local.get $4
  i32.lt_s
  select
  local.set $2
  loop $for-loop|0
   local.get $1
   local.get $2
   i32.lt_s
   if
    local.get $3
    local.get $1
    i32.const 3
    i32.shl
    i32.add
    i64.const 0
    i64.store
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/array/Array<u64>#__get (param $0 i32) (param $1 i32) (result i64)
  (local $2 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=12
  i32.ge_u
  if
   i32.const 1696
   i32.const 2928
   i32.const 114
   i32.const 42
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 3
  i32.shl
  i32.add
  i64.load
  local.set $2
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/btc/types/SafeMath/SafeMath.mul (param $0 i32) (param $1 i32) (result i32)
  (local $2 i64)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i64)
  (local $10 i64)
  (local $11 i32)
  (local $12 i64)
  (local $13 i64)
  (local $14 i64)
  (local $15 i64)
  (local $16 i32)
  (local $17 i32)
  (local $18 i32)
  (local $19 i32)
  (local $20 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 156
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 156
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/types/SafeMath/SafeMath.ZERO
  local.tee $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $0
  i64.load
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=8
  global.get $src/btc/types/SafeMath/SafeMath.ZERO
  local.set $3
  local.get $2
  local.get $4
  i64.load
  i64.eq
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load offset=8
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $2
   local.get $4
   i64.load offset=8
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load offset=16
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $2
   local.get $4
   i64.load offset=16
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load offset=24
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $2
   local.get $4
   i64.load offset=24
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   i32.const 1
  else
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   global.get $src/btc/types/SafeMath/SafeMath.ZERO
   local.tee $4
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $1
   i64.load
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $2
   local.get $4
   i64.load
   i64.eq
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $1
    i64.load offset=8
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=8
    local.get $2
    local.get $4
    i64.load offset=8
    i64.eq
   else
    i32.const 0
   end
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $1
    i64.load offset=16
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=8
    local.get $2
    local.get $4
    i64.load offset=16
    i64.eq
   else
    i32.const 0
   end
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $1
    i64.load offset=24
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=8
    local.get $2
    local.get $4
    i64.load offset=24
    i64.eq
   else
    i32.const 0
   end
  end
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=20
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=24
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load
   local.set $12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load offset=8
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load offset=16
   local.set $9
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load offset=24
   local.set $10
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $1
   i64.load
   local.set $13
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $1
   i64.load offset=8
   local.set $14
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $1
   i64.load offset=16
   local.set $15
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $12
   local.get $2
   local.get $9
   local.get $10
   local.get $13
   local.get $14
   local.get $15
   local.get $1
   i64.load offset=24
   call $~lib/as-bignum/assembly/globals/__mul256
   local.tee $3
   i32.store offset=28
   global.get $~lib/memory/__stack_pointer
   local.set $8
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=32
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=36
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=40
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $0
   i64.load
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $2
   local.get $0
   i64.load offset=8
   i64.or
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $2
   local.get $0
   i64.load offset=16
   i64.or
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   local.get $2
   local.get $0
   i64.load offset=24
   i64.or
   i64.eqz
   if
    i32.const 7344
    i32.const 7152
    i32.const 40
    i32.const 13
    call $~lib/builtins/abort
    unreachable
   end
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=44
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $3
   i64.load
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $2
   local.get $3
   i64.load offset=8
   i64.or
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $2
   local.get $3
   i64.load offset=16
   i64.or
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   block $src/btc/types/SafeMath/SafeMath.div|inlined.0
    local.get $2
    local.get $3
    i64.load offset=24
    i64.or
    i64.eqz
    if
     i64.const 0
     i64.const 0
     i64.const 0
     i64.const 0
     call $~lib/as-bignum/assembly/integer/u256/u256#constructor
     local.set $4
     br $src/btc/types/SafeMath/SafeMath.div|inlined.0
    end
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=48
    local.get $3
    local.get $0
    call $~lib/as-bignum/assembly/integer/u256/u256.lt
    if
     i64.const 0
     i64.const 0
     i64.const 0
     i64.const 0
     call $~lib/as-bignum/assembly/integer/u256/u256#constructor
     local.set $4
     br $src/btc/types/SafeMath/SafeMath.div|inlined.0
    end
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=52
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=56
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=8
    local.get $3
    i64.load
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    local.get $2
    local.get $0
    i64.load
    i64.eq
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store offset=8
     local.get $3
     i64.load offset=8
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $2
     local.get $0
     i64.load offset=8
     i64.eq
    else
     i32.const 0
    end
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store offset=8
     local.get $3
     i64.load offset=16
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $2
     local.get $0
     i64.load offset=16
     i64.eq
    else
     i32.const 0
    end
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store offset=8
     local.get $3
     i64.load offset=24
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $2
     local.get $0
     i64.load offset=24
     i64.eq
    else
     i32.const 0
    end
    if
     i64.const 1
     i64.const 0
     i64.const 0
     i64.const 0
     call $~lib/as-bignum/assembly/integer/u256/u256#constructor
     local.set $4
     br $src/btc/types/SafeMath/SafeMath.div|inlined.0
    end
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $3
    call $~lib/as-bignum/assembly/integer/u256/u256#clone
    local.tee $5
    i32.store offset=60
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    call $~lib/as-bignum/assembly/integer/u256/u256#clone
    local.tee $0
    i32.store offset=64
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    local.tee $4
    i32.store offset=68
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=72
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=8
    local.get $0
    i64.load offset=24
    i64.const 0
    i64.ne
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     i64.load offset=24
     i64.clz
     i32.wrap_i64
    else
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     i64.load offset=16
     i64.const 0
     i64.ne
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      local.get $0
      i64.load offset=16
      i64.clz
      i64.const -64
      i64.sub
      i32.wrap_i64
     else
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      local.get $0
      i64.load offset=8
      i64.const 0
      i64.ne
      if (result i32)
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store offset=8
       local.get $0
       i64.load offset=8
       i64.clz
       i64.const 128
       i64.add
       i32.wrap_i64
      else
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store offset=8
       local.get $0
       i64.load
       i64.clz
       i64.const 192
       i64.add
       i32.wrap_i64
      end
     end
    end
    local.set $7
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=76
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=8
    local.get $5
    i64.load offset=24
    i64.const 0
    i64.ne
    if (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.store offset=8
     local.get $5
     i64.load offset=24
     i64.clz
     i32.wrap_i64
    else
     global.get $~lib/memory/__stack_pointer
     local.get $5
     i32.store offset=8
     local.get $5
     i64.load offset=16
     i64.const 0
     i64.ne
     if (result i32)
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=8
      local.get $5
      i64.load offset=16
      i64.clz
      i64.const -64
      i64.sub
      i32.wrap_i64
     else
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=8
      local.get $5
      i64.load offset=8
      i64.const 0
      i64.ne
      if (result i32)
       global.get $~lib/memory/__stack_pointer
       local.get $5
       i32.store offset=8
       local.get $5
       i64.load offset=8
       i64.clz
       i64.const 128
       i64.add
       i32.wrap_i64
      else
       global.get $~lib/memory/__stack_pointer
       local.get $5
       i32.store offset=8
       local.get $5
       i64.load
       i64.clz
       i64.const 192
       i64.add
       i32.wrap_i64
      end
     end
    end
    local.set $11
    global.get $~lib/memory/__stack_pointer
    block $src/btc/types/SafeMath/SafeMath.shl|inlined.0 (result i32)
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=80
     local.get $7
     local.get $11
     i32.sub
     local.tee $7
     i32.eqz
     if
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      local.get $0
      call $~lib/as-bignum/assembly/integer/u256/u256#clone
      br $src/btc/types/SafeMath/SafeMath.shl|inlined.0
     end
     local.get $7
     i32.const 255
     i32.and
     local.tee $11
     i32.const 6
     i32.shr_u
     local.set $16
     local.get $11
     i32.const 63
     i32.and
     local.set $11
     global.get $~lib/memory/__stack_pointer
     local.set $17
     global.get $~lib/memory/__stack_pointer
     i32.const 4
     i32.const 3
     i32.const 45
     i32.const 0
     call $~lib/rt/__newArray
     local.tee $18
     i32.store offset=84
     global.get $~lib/memory/__stack_pointer
     local.get $18
     i32.load offset=4
     i32.store offset=88
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $18
     i32.const 0
     local.get $0
     i64.load
     call $~lib/array/Array<u64>#__set
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $18
     i32.const 1
     local.get $0
     i64.load offset=8
     call $~lib/array/Array<u64>#__set
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $18
     i32.const 2
     local.get $0
     i64.load offset=16
     call $~lib/array/Array<u64>#__set
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $18
     i32.const 3
     local.get $0
     i64.load offset=24
     call $~lib/array/Array<u64>#__set
     local.get $17
     local.get $18
     i32.store offset=92
     global.get $~lib/memory/__stack_pointer
     local.set $0
     call $~lib/array/Array<u64>#constructor
     local.set $17
     global.get $~lib/memory/__stack_pointer
     local.get $17
     i32.store offset=8
     i32.const 1
     global.set $~argumentsLength
     local.get $0
     local.get $17
     call $~lib/array/Array<u64>#fill@varargs
     local.tee $0
     i32.store offset=96
     loop $for-loop|0
      global.get $~lib/memory/__stack_pointer
      local.get $18
      i32.store offset=8
      local.get $18
      call $~lib/array/Array<u8>#get:length
      local.get $6
      i32.gt_s
      if
       global.get $~lib/memory/__stack_pointer
       local.get $18
       i32.store offset=8
       local.get $18
       call $~lib/array/Array<u8>#get:length
       local.get $6
       local.get $16
       i32.add
       local.tee $17
       i32.gt_s
       if
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=8
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=48
        local.get $0
        local.get $17
        call $~lib/array/Array<u64>#__get
        local.set $2
        global.get $~lib/memory/__stack_pointer
        local.get $18
        i32.store offset=48
        local.get $0
        local.get $17
        local.get $2
        local.get $18
        local.get $6
        call $~lib/array/Array<u64>#__get
        local.get $11
        i64.extend_i32_s
        i64.shl
        i64.or
        call $~lib/array/Array<u64>#__set
       end
       local.get $11
       if (result i32)
        global.get $~lib/memory/__stack_pointer
        local.get $18
        i32.store offset=8
        local.get $18
        call $~lib/array/Array<u8>#get:length
        local.get $6
        local.get $16
        i32.add
        i32.const 1
        i32.add
        i32.gt_s
       else
        i32.const 0
       end
       if
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=8
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=48
        local.get $0
        local.get $6
        local.get $16
        i32.add
        i32.const 1
        i32.add
        local.tee $17
        call $~lib/array/Array<u64>#__get
        local.set $2
        global.get $~lib/memory/__stack_pointer
        local.get $18
        i32.store offset=48
        local.get $0
        local.get $17
        local.get $2
        local.get $18
        local.get $6
        call $~lib/array/Array<u64>#__get
        i32.const 64
        local.get $11
        i32.sub
        i64.extend_i32_s
        i64.shr_u
        i64.or
        call $~lib/array/Array<u64>#__set
       end
       local.get $6
       i32.const 1
       i32.add
       local.set $6
       br $for-loop|0
      end
     end
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     i32.const 0
     call $~lib/array/Array<u64>#__get
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     i32.const 1
     call $~lib/array/Array<u64>#__get
     local.set $9
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     i32.const 2
     call $~lib/array/Array<u64>#__get
     local.set $10
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $2
     local.get $9
     local.get $10
     local.get $0
     i32.const 3
     call $~lib/array/Array<u64>#__get
     call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    end
    local.tee $0
    i32.store offset=64
    loop $for-loop|1
     local.get $7
     i32.const 0
     i32.ge_s
     if
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=100
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=104
      global.get $~lib/memory/__stack_pointer
      local.get $5
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=48
      local.get $5
      local.get $0
      call $~lib/as-bignum/assembly/integer/u256/u256.lt
      i32.eqz
      if
       global.get $~lib/memory/__stack_pointer
       local.get $5
       i32.store offset=8
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store offset=48
       global.get $~lib/memory/__stack_pointer
       local.get $5
       local.get $0
       call $~lib/as-bignum/assembly/integer/u256/u256.sub
       local.tee $5
       i32.store offset=60
       global.get $~lib/memory/__stack_pointer
       local.set $16
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store offset=108
       global.get $~lib/memory/__stack_pointer
       block $src/btc/types/SafeMath/SafeMath.shl|inlined.1 (result i32)
        global.get $~lib/memory/__stack_pointer
        i64.const 1
        i64.const 0
        i64.const 0
        i64.const 0
        call $~lib/as-bignum/assembly/integer/u256/u256#constructor
        local.tee $6
        i32.store offset=112
        local.get $7
        i32.eqz
        if
         global.get $~lib/memory/__stack_pointer
         local.get $6
         i32.store offset=8
         local.get $6
         call $~lib/as-bignum/assembly/integer/u256/u256#clone
         br $src/btc/types/SafeMath/SafeMath.shl|inlined.1
        end
        local.get $7
        i32.const 255
        i32.and
        local.tee $11
        i32.const 6
        i32.shr_u
        local.set $17
        local.get $11
        i32.const 63
        i32.and
        local.set $18
        global.get $~lib/memory/__stack_pointer
        local.set $19
        global.get $~lib/memory/__stack_pointer
        i32.const 4
        i32.const 3
        i32.const 45
        i32.const 0
        call $~lib/rt/__newArray
        local.tee $11
        i32.store offset=116
        global.get $~lib/memory/__stack_pointer
        local.get $11
        i32.load offset=4
        i32.store offset=120
        global.get $~lib/memory/__stack_pointer
        local.get $6
        i32.store offset=8
        local.get $11
        i32.const 0
        local.get $6
        i64.load
        call $~lib/array/Array<u64>#__set
        global.get $~lib/memory/__stack_pointer
        local.get $6
        i32.store offset=8
        local.get $11
        i32.const 1
        local.get $6
        i64.load offset=8
        call $~lib/array/Array<u64>#__set
        global.get $~lib/memory/__stack_pointer
        local.get $6
        i32.store offset=8
        local.get $11
        i32.const 2
        local.get $6
        i64.load offset=16
        call $~lib/array/Array<u64>#__set
        global.get $~lib/memory/__stack_pointer
        local.get $6
        i32.store offset=8
        local.get $11
        i32.const 3
        local.get $6
        i64.load offset=24
        call $~lib/array/Array<u64>#__set
        local.get $19
        local.get $11
        i32.store offset=124
        global.get $~lib/memory/__stack_pointer
        local.set $6
        call $~lib/array/Array<u64>#constructor
        local.set $19
        global.get $~lib/memory/__stack_pointer
        local.get $19
        i32.store offset=8
        i32.const 1
        global.set $~argumentsLength
        local.get $6
        local.get $19
        call $~lib/array/Array<u64>#fill@varargs
        local.tee $19
        i32.store offset=128
        i32.const 0
        local.set $6
        loop $for-loop|2
         global.get $~lib/memory/__stack_pointer
         local.get $11
         i32.store offset=8
         local.get $11
         call $~lib/array/Array<u8>#get:length
         local.get $6
         i32.gt_s
         if
          global.get $~lib/memory/__stack_pointer
          local.get $11
          i32.store offset=8
          local.get $11
          call $~lib/array/Array<u8>#get:length
          local.get $6
          local.get $17
          i32.add
          local.tee $20
          i32.gt_s
          if
           global.get $~lib/memory/__stack_pointer
           local.get $19
           i32.store offset=8
           global.get $~lib/memory/__stack_pointer
           local.get $19
           i32.store offset=48
           local.get $19
           local.get $20
           call $~lib/array/Array<u64>#__get
           local.set $2
           global.get $~lib/memory/__stack_pointer
           local.get $11
           i32.store offset=48
           local.get $19
           local.get $20
           local.get $2
           local.get $11
           local.get $6
           call $~lib/array/Array<u64>#__get
           local.get $18
           i64.extend_i32_s
           i64.shl
           i64.or
           call $~lib/array/Array<u64>#__set
          end
          local.get $18
          if (result i32)
           global.get $~lib/memory/__stack_pointer
           local.get $11
           i32.store offset=8
           local.get $11
           call $~lib/array/Array<u8>#get:length
           local.get $6
           local.get $17
           i32.add
           i32.const 1
           i32.add
           i32.gt_s
          else
           i32.const 0
          end
          if
           global.get $~lib/memory/__stack_pointer
           local.get $19
           i32.store offset=8
           global.get $~lib/memory/__stack_pointer
           local.get $19
           i32.store offset=48
           local.get $19
           local.get $6
           local.get $17
           i32.add
           i32.const 1
           i32.add
           local.tee $20
           call $~lib/array/Array<u64>#__get
           local.set $2
           global.get $~lib/memory/__stack_pointer
           local.get $11
           i32.store offset=48
           local.get $19
           local.get $20
           local.get $2
           local.get $11
           local.get $6
           call $~lib/array/Array<u64>#__get
           i32.const 64
           local.get $18
           i32.sub
           i64.extend_i32_s
           i64.shr_u
           i64.or
           call $~lib/array/Array<u64>#__set
          end
          local.get $6
          i32.const 1
          i32.add
          local.set $6
          br $for-loop|2
         end
        end
        global.get $~lib/memory/__stack_pointer
        local.get $19
        i32.store offset=8
        local.get $19
        i32.const 0
        call $~lib/array/Array<u64>#__get
        local.set $2
        global.get $~lib/memory/__stack_pointer
        local.get $19
        i32.store offset=8
        local.get $19
        i32.const 1
        call $~lib/array/Array<u64>#__get
        local.set $9
        global.get $~lib/memory/__stack_pointer
        local.get $19
        i32.store offset=8
        local.get $19
        i32.const 2
        call $~lib/array/Array<u64>#__get
        local.set $10
        global.get $~lib/memory/__stack_pointer
        local.get $19
        i32.store offset=8
        local.get $2
        local.get $9
        local.get $10
        local.get $19
        i32.const 3
        call $~lib/array/Array<u64>#__get
        call $~lib/as-bignum/assembly/integer/u256/u256#constructor
       end
       local.tee $6
       i32.store offset=132
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store offset=8
       local.get $4
       i64.load
       local.set $2
       global.get $~lib/memory/__stack_pointer
       local.get $6
       i32.store offset=8
       local.get $2
       local.get $6
       i64.load
       i64.or
       local.set $2
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store offset=8
       local.get $4
       i64.load offset=8
       local.set $9
       global.get $~lib/memory/__stack_pointer
       local.get $6
       i32.store offset=8
       local.get $9
       local.get $6
       i64.load offset=8
       i64.or
       local.set $9
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store offset=8
       local.get $4
       i64.load offset=16
       local.set $10
       global.get $~lib/memory/__stack_pointer
       local.get $6
       i32.store offset=8
       local.get $10
       local.get $6
       i64.load offset=16
       i64.or
       local.set $10
       global.get $~lib/memory/__stack_pointer
       local.get $4
       i32.store offset=8
       local.get $4
       i64.load offset=24
       local.set $12
       global.get $~lib/memory/__stack_pointer
       local.get $6
       i32.store offset=8
       local.get $16
       local.get $2
       local.get $9
       local.get $10
       local.get $12
       local.get $6
       i64.load offset=24
       i64.or
       call $~lib/as-bignum/assembly/integer/u256/u256#constructor
       local.tee $4
       i32.store offset=68
      end
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.set $6
      global.get $~lib/memory/__stack_pointer
      i32.const 4
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i64.load offset=24
      i64.const 1
      i64.shr_u
      local.set $2
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i64.load offset=16
      i64.const 1
      i64.shr_u
      local.set $9
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $9
      local.get $0
      i64.load offset=24
      i64.const 63
      i64.shl
      i64.or
      local.set $9
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i64.load offset=8
      i64.const 1
      i64.shr_u
      local.set $10
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $10
      local.get $0
      i64.load offset=16
      i64.const 63
      i64.shl
      i64.or
      local.set $10
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i64.load
      i64.const 1
      i64.shr_u
      local.set $12
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $12
      local.get $0
      i64.load offset=8
      i64.const 63
      i64.shl
      i64.or
      local.get $10
      local.get $9
      local.get $2
      call $~lib/as-bignum/assembly/integer/u256/u256#constructor
      local.set $0
      global.get $~lib/memory/__stack_pointer
      i32.const 4
      i32.add
      global.set $~lib/memory/__stack_pointer
      local.get $6
      local.get $0
      i32.store offset=64
      local.get $7
      i32.const 1
      i32.sub
      local.set $7
      br $for-loop|1
     end
    end
   end
   local.get $8
   local.get $4
   i32.store offset=136
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=140
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=144
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=148
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=152
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $4
   i64.load
   local.set $2
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $2
   local.get $1
   i64.load
   i64.eq
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=8
    local.get $4
    i64.load offset=8
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $2
    local.get $1
    i64.load offset=8
    i64.eq
   else
    i32.const 0
   end
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=8
    local.get $4
    i64.load offset=16
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $2
    local.get $1
    i64.load offset=16
    i64.eq
   else
    i32.const 0
   end
   if (result i32)
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=8
    local.get $4
    i64.load offset=24
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $2
    local.get $1
    i64.load offset=24
    i64.eq
   else
    i32.const 0
   end
   i32.eqz
   if
    i32.const 7408
    i32.const 7152
    i32.const 31
    i32.const 13
    call $~lib/builtins/abort
    unreachable
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 156
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
 )
 (func $src/btc/contracts/OP_20/OP_20#get:totalSupply (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=16
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/contract/TestingContract/TestingContract#_testMethodMultipleAddresses (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $1
  call $src/btc/contracts/OP_20/OP_20#_balanceOf
  local.tee $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  local.get $2
  call $src/btc/contracts/OP_20/OP_20#_balanceOf
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $4
  local.get $2
  call $src/btc/types/SafeMath/SafeMath.sub
  local.tee $5
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $4
  local.get $2
  call $src/btc/types/SafeMath/SafeMath.mul
  local.tee $1
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  i32.const 5
  i32.const 2
  i32.const 37
  i32.const 0
  call $~lib/rt/__newArray
  local.tee $3
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=4
  i32.store offset=28
  local.get $3
  i32.const 0
  local.get $4
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
  local.get $3
  i32.const 1
  local.get $2
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
  local.get $3
  i32.const 2
  local.get $5
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $3
  i32.const 3
  local.get $0
  call $src/btc/contracts/OP_20/OP_20#get:totalSupply
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
  local.get $3
  i32.const 4
  local.get $1
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
 )
 (func $src/btc/buffer/BytesWriter/BytesWriter#writeTuple (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/array/Array<u8>#get:length
  i32.const 5
  i32.shl
  i32.const 4
  i32.add
  call $src/btc/buffer/BytesWriter/BytesWriter#allocSafe
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/array/Array<u8>#get:length
  i32.const 1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU32
  loop $for-loop|0
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $1
   call $~lib/array/Array<u8>#get:length
   local.get $2
   i32.gt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    local.get $1
    local.get $2
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__get
    local.set $3
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=4
    local.get $0
    local.get $3
    call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#_crashIndexerTest2 (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 40
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  i64.const 1
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=12
  local.get $1
  local.get $2
  call $~lib/as-bignum/assembly/integer/u256/u256.add
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/contract/TestingContract/TestingContract#_crashIndexerTest2
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $3
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  i64.const 1
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.tee $1
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $3
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=16
  local.tee $2
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=28
  local.get $2
  local.get $1
  call $src/btc/types/SafeMath/SafeMath.add
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $3
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load
  local.tee $5
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=36
  local.get $3
  i32.load16_u offset=4
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=8
  local.tee $7
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=16
  local.tee $1
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=12
  local.tee $2
  i32.store offset=32
  local.get $4
  local.get $5
  local.get $6
  local.get $7
  local.get $1
  local.get $2
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  local.get $0
  local.get $3
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#_crashIndexerTest3 (param $0 i32) (result i32)
  (local $1 i64)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 44
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 44
  memory.fill
  global.get $~lib/memory/__stack_pointer
  i32.const 600000
  call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor
  local.tee $2
  i32.store
  i64.const 10000000
  local.set $1
  loop $for-loop|0
   local.get $1
   i64.const 10600000
   i64.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    local.get $1
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    local.set $3
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=8
    local.get $2
    local.get $1
    i32.wrap_i64
    local.get $3
    call $~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#__set
    local.get $1
    i64.const 1
    i64.add
    local.set $1
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $5
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  i64.const 1
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.tee $3
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=12
  local.get $5
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=16
  local.tee $4
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=32
  local.get $4
  local.get $3
  call $src/btc/types/SafeMath/SafeMath.add
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=24
  local.get $5
  local.get $3
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $6
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load
  local.tee $7
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=40
  local.get $5
  i32.load16_u offset=4
  local.set $8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=8
  local.tee $9
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=16
  local.tee $3
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=12
  local.tee $4
  i32.store offset=36
  local.get $6
  local.get $7
  local.get $8
  local.get $9
  local.get $3
  local.get $4
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  local.get $0
  local.get $5
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  i32.const 44
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $src/contract/TestingContract/TestingContract#_crashIndexerView1 (param $0 i32)
  (local $1 i32)
  (local $2 i64)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 40
  memory.fill
  loop $while-continue|0
   local.get $2
   i64.const 10000000
   i64.lt_u
   if
    local.get $2
    i64.const 1
    i64.add
    local.set $2
    br $while-continue|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  i64.const 1
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.tee $3
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $1
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=16
  local.tee $4
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=28
  local.get $4
  local.get $3
  call $src/btc/types/SafeMath/SafeMath.add
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=20
  local.get $1
  local.get $3
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $5
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load
  local.tee $6
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=36
  local.get $1
  i32.load16_u offset=4
  local.set $7
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=8
  local.tee $8
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=16
  local.tee $3
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.load offset=12
  local.tee $4
  i32.store offset=32
  local.get $5
  local.get $6
  local.get $7
  local.get $8
  local.get $3
  local.get $4
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  i32.const 40
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#_attemptCrash1 (param $0 i32) (param $1 i32)
  (local $2 i64)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 48
  memory.fill
  i64.const 10000000
  local.set $2
  loop $for-loop|0
   local.get $2
   i64.const 10700000
   i64.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    local.get $1
    call $src/btc/contracts/OP_20/OP_20#_balanceOf
    local.tee $3
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store
    local.get $2
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    local.set $5
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=4
    local.get $4
    local.get $3
    local.get $5
    call $src/btc/types/SafeMath/SafeMath.add
    local.tee $3
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=28
    local.tee $4
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=16
    local.get $4
    local.get $1
    local.get $3
    call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=32
    local.tee $4
    i32.store offset=24
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    local.tee $3
    i32.store offset=28
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    local.get $4
    call $src/btc/storage/StoredU256/StoredU256#ensureValue
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=40
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.load offset=16
    local.tee $5
    i32.store offset=32
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=36
    local.get $5
    local.get $3
    call $src/btc/types/SafeMath/SafeMath.add
    local.set $3
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=20
    local.get $4
    local.get $3
    call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
    global.get $~lib/memory/__stack_pointer
    global.get $src/btc/env/index/Blockchain
    local.tee $5
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=44
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.load
    local.tee $6
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=44
    local.get $4
    i32.load16_u offset=4
    local.set $7
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=44
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.load offset=8
    local.tee $8
    i32.store offset=32
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=44
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.load offset=16
    local.tee $9
    i32.store offset=36
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=44
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.load offset=12
    local.tee $3
    i32.store offset=40
    local.get $5
    local.get $6
    local.get $7
    local.get $8
    local.get $9
    local.get $3
    call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=4
    local.get $0
    local.get $4
    call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
    local.get $2
    i64.const 1
    i64.add
    local.set $2
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/contract/TestingContract/TestingContract#generateRndString (result i32)
  (local $0 i32)
  (local $1 i64)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i64)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  i32.const 3504
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 3504
  i32.store
  loop $for-loop|0
   local.get $3
   i32.const 62
   i32.lt_s
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.set $5
    global.get $~lib/memory/__stack_pointer
    i32.const 7504
    i32.store offset=12
    global.get $src/btc/math/rnd/random_seeded
    i32.eqz
    if
     i64.const -7046029254386353131
     call $src/btc/math/rnd/murmurHash3
     global.set $src/btc/math/rnd/random_state0_64
     global.get $src/btc/math/rnd/random_state0_64
     i64.const -1
     i64.xor
     call $src/btc/math/rnd/murmurHash3
     global.set $src/btc/math/rnd/random_state1_64
     i32.const 2135587861
     call $src/btc/math/rnd/splitMix32
     global.set $src/btc/math/rnd/random_state0_32
     global.get $src/btc/math/rnd/random_state0_32
     call $src/btc/math/rnd/splitMix32
     drop
     i32.const 1
     global.set $src/btc/math/rnd/random_seeded
    end
    global.get $src/btc/math/rnd/random_state0_64
    local.set $6
    global.get $src/btc/math/rnd/random_state1_64
    local.tee $1
    global.set $src/btc/math/rnd/random_state0_64
    local.get $6
    local.get $6
    i64.const 23
    i64.shl
    i64.xor
    local.tee $6
    i64.const 17
    i64.shr_u
    local.get $6
    i64.xor
    local.get $1
    i64.xor
    local.get $1
    i64.const 26
    i64.shr_u
    i64.xor
    global.set $src/btc/math/rnd/random_state1_64
    global.get $~lib/memory/__stack_pointer
    i32.const 7504
    i32.store offset=16
    local.get $1
    i32.const 7504
    call $~lib/string/String#get:length
    i64.extend_i32_s
    i64.rem_u
    i32.wrap_i64
    local.set $4
    global.get $~lib/memory/__stack_pointer
    i32.const 8
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i64.const 0
    i64.store
    global.get $~lib/memory/__stack_pointer
    i32.const 7504
    i32.store
    block $__inlined_func$~lib/string/String#charAt$687
     i32.const 7504
     call $~lib/string/String#get:length
     local.get $4
     i32.le_u
     if
      global.get $~lib/memory/__stack_pointer
      i32.const 8
      i32.add
      global.set $~lib/memory/__stack_pointer
      i32.const 3504
      local.set $2
      br $__inlined_func$~lib/string/String#charAt$687
     end
     global.get $~lib/memory/__stack_pointer
     i32.const 2
     i32.const 2
     call $~lib/rt/itcms/__new
     local.tee $2
     i32.store offset=4
     local.get $2
     local.get $4
     i32.const 1
     i32.shl
     i32.const 7504
     i32.add
     i32.load16_u
     i32.store16
     global.get $~lib/memory/__stack_pointer
     i32.const 8
     i32.add
     global.set $~lib/memory/__stack_pointer
    end
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=8
    local.get $5
    local.get $0
    local.get $2
    call $~lib/string/String.__concat
    local.tee $0
    i32.store
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/contract/TestingContract/TestingContract#_attemptCrash2 (param $0 i32)
  (local $1 i64)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 52
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 52
  memory.fill
  i64.const 10000000
  local.set $1
  loop $for-loop|0
   local.get $1
   i64.const 10700000
   i64.lt_u
   if
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    call $src/contract/TestingContract/TestingContract#generateRndString
    local.tee $2
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $0
    local.get $2
    call $src/btc/contracts/OP_20/OP_20#_balanceOf
    local.tee $3
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store
    local.get $1
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    local.set $5
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=8
    local.get $4
    local.get $3
    local.get $5
    call $src/btc/types/SafeMath/SafeMath.add
    local.tee $3
    i32.store offset=16
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=24
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=28
    local.tee $4
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=20
    local.get $4
    local.get $2
    local.get $3
    call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load offset=32
    local.tee $5
    i32.store offset=28
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i64.const 0
    i64.const 0
    i64.const 0
    call $~lib/as-bignum/assembly/integer/u256/u256#constructor
    local.tee $2
    i32.store offset=32
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=20
    local.get $5
    call $src/btc/storage/StoredU256/StoredU256#ensureValue
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=44
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.load offset=16
    local.tee $3
    i32.store offset=36
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=40
    local.get $3
    local.get $2
    call $src/btc/types/SafeMath/SafeMath.add
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=24
    local.get $5
    local.get $2
    call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
    global.get $~lib/memory/__stack_pointer
    global.get $src/btc/env/index/Blockchain
    local.tee $2
    i32.store offset=20
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=48
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.load
    local.tee $3
    i32.store offset=24
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=48
    local.get $5
    i32.load16_u offset=4
    local.set $6
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=48
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.load offset=8
    local.tee $4
    i32.store offset=36
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=48
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.load offset=16
    local.tee $7
    i32.store offset=40
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=48
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.load offset=12
    local.tee $8
    i32.store offset=44
    local.get $2
    local.get $3
    local.get $6
    local.get $4
    local.get $7
    local.get $8
    call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
    global.get $~lib/memory/__stack_pointer
    local.get $5
    i32.store offset=8
    local.get $0
    local.get $5
    call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
    local.get $1
    i64.const 1
    i64.add
    local.set $1
    br $for-loop|0
   end
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 52
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#createKeyMerger" (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 28
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has"
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=24
   local.get $0
   i32.load16_u offset=24
   local.set $5
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=24
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $3
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=24
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=32
   local.tee $4
   i32.store offset=20
   global.get $~lib/memory/__stack_pointer
   i32.const 12
   i32.sub
   global.set $~lib/memory/__stack_pointer
   call $~stack_check
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store
   global.get $~lib/memory/__stack_pointer
   i32.const 0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.const 30
   call $~lib/rt/itcms/__new
   local.tee $2
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $2
   local.get $4
   call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:allowedWriteMethods
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   local.get $2
   i32.const 0
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   local.get $2
   i32.const 0
   call $src/btc/storage/StoredU256/StoredU256#set:pointer
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   local.get $2
   i32.const 0
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   local.get $2
   local.get $5
   call $src/btc/storage/StoredU256/StoredU256#set:pointer
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $2
   local.get $3
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $2
   local.get $1
   call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
   global.get $~lib/memory/__stack_pointer
   i32.const 12
   i32.add
   global.set $~lib/memory/__stack_pointer
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=8
   local.get $0
   local.get $1
   local.get $2
   call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#set"
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 28
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get" (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#createKeyMerger"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $"~lib/map/Map<~lib/string/String,~lib/map/Map<~lib/as-bignum/assembly/integer/u256/u256,~lib/as-bignum/assembly/integer/u256/u256>>#get"
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has" (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  local.get $1
  call $~lib/string/String#concat
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=16
  local.get $0
  i32.load16_u offset=4
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=16
  local.get $2
  call $src/btc/math/abi/encodePointer
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $1
  local.get $3
  local.get $0
  local.get $2
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#hasStorageAt
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get" (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 24
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  local.get $1
  call $~lib/string/String#concat
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=20
  local.get $0
  i32.load16_u offset=4
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=20
  local.get $2
  call $src/btc/math/abi/encodePointer
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=12
  local.tee $0
  i32.store offset=16
  local.get $1
  local.get $3
  local.get $4
  local.get $2
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#getStorageAt
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/contracts/OP_20/OP_20#allowance (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $1
  call $src/btc/buffer/BytesReader/BytesReader#readAddress
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $1
  call $src/btc/buffer/BytesReader/BytesReader#readAddress
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=24
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $4
  local.get $2
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $2
  local.get $1
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  i32.eqz
  if
   i32.const 0
   i32.const 7600
   i32.const 159
   i32.const 38
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $2
  local.get $1
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.set $1
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $3
  local.get $1
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $2
  local.get $1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set" (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 32
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  local.get $1
  call $~lib/string/String#concat
  local.tee $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  call $src/btc/math/abi/encodePointer
  local.tee $1
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=28
  local.get $0
  i32.load16_u offset=4
  local.set $5
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=12
  local.tee $0
  i32.store offset=24
  local.get $3
  local.get $4
  local.get $5
  local.get $1
  local.get $2
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_20/OP_20#approve (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 20
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $1
  call $src/btc/buffer/BytesReader/BytesReader#readAddress
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=16
  local.get $1
  call $src/btc/buffer/BytesReader/BytesReader#readU256
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=24
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $4
  local.get $2
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $2
  local.get $3
  local.get $1
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load
  local.tee $1
  i32.store
  local.get $1
  i32.const 1
  call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 20
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/contracts/OP_20/OP_20#_burn (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i64)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 68
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 68
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $5
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  local.get $5
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=16
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  local.get $5
  local.get $3
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 7680
   i32.const 7600
   i32.const 181
   i32.const 46
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $5
  local.get $2
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  i32.eqz
  if
   i32.const 0
   i32.const 7600
   i32.const 183
   i32.const 41
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $1
  local.get $2
  call $~lib/string/String.__eq
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=20
   global.get $~lib/memory/__stack_pointer
   i32.const 7920
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   i32.const 7920
   i32.const 1
   local.get $2
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 7920
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   i32.const 7920
   i32.const 3
   local.get $2
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 7920
   i32.store
   global.get $~lib/memory/__stack_pointer
   i32.const 3504
   i32.store offset=4
   i32.const 7920
   call $~lib/staticarray/StaticArray<~lib/string/String>#join
   i32.const 7600
   i32.const 186
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.tee $1
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  i64.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $4
  local.get $1
  i64.load
  i64.eq
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   i64.load offset=8
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $1
   i64.load offset=8
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   i64.load offset=16
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $1
   i64.load offset=16
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   i64.load offset=24
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   local.get $4
   local.get $1
   i64.load offset=24
   i64.eq
  else
   i32.const 0
  end
  if
   i32.const 7968
   i32.const 7600
   i32.const 190
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  local.get $2
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $1
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  local.get $1
  local.get $3
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 8032
   i32.const 7600
   i32.const 194
   i32.const 30
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  local.get $3
  call $src/btc/types/SafeMath/SafeMath.sub
  local.tee $1
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $5
  local.get $2
  local.get $1
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $5
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=48
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  local.get $5
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=60
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=16
  local.tee $1
  i32.store offset=52
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=56
  local.get $1
  local.get $3
  call $src/btc/types/SafeMath/SafeMath.sub
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=40
  local.get $5
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=64
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load
  local.tee $2
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=64
  local.get $5
  i32.load16_u offset=4
  local.set $6
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=64
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=8
  local.tee $3
  i32.store offset=52
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=64
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=16
  local.tee $7
  i32.store offset=56
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=64
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.load offset=12
  local.tee $8
  i32.store offset=60
  local.get $1
  local.get $2
  local.get $6
  local.get $3
  local.get $7
  local.get $8
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=4
  local.get $0
  local.get $5
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  i32.const 68
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_NET/OP_NET#isSelf (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=8
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $~lib/string/String.__eq
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/contracts/OP_20/OP_20#_mint (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 48
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/contracts/OP_NET/OP_NET#isSelf
  if
   i32.const 8096
   i32.const 7600
   i32.const 205
   i32.const 34
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=4
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $4
  local.get $1
  call $~lib/string/String.__ne
  if
   i32.const 8176
   i32.const 2336
   i32.const 95
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $1
  local.get $2
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $2
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
   local.tee $1
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $3
   call $src/btc/types/SafeMath/SafeMath.add
   local.tee $1
   i32.store offset=20
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $4
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $4
   local.get $2
   local.get $1
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  else
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $1
   local.get $2
   local.get $3
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=32
  local.tee $4
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=8
  local.get $4
  call $src/btc/storage/StoredU256/StoredU256#ensureValue
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.load offset=16
  local.tee $1
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=36
  local.get $1
  local.get $3
  call $src/btc/types/SafeMath/SafeMath.add
  local.set $1
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=12
  local.get $4
  local.get $1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:defaults
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.load
  local.tee $2
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=44
  local.get $4
  i32.load16_u offset=4
  local.set $5
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.load offset=8
  local.tee $3
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.load offset=16
  local.tee $6
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=44
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.load offset=12
  local.tee $7
  i32.store offset=40
  local.get $1
  local.get $2
  local.get $5
  local.get $3
  local.get $6
  local.get $7
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#setStorageAt
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $0
  local.get $4
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_20/OP_20#_transfer (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32)
  (local $4 i64)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 48
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $5
  local.get $1
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  i32.eqz
  if
   i32.const 0
   i32.const 7600
   i32.const 224
   i32.const 45
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/contracts/OP_NET/OP_NET#isSelf
  if
   i32.const 8272
   i32.const 7600
   i32.const 226
   i32.const 34
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $1
  local.get $2
  call $~lib/string/String.__eq
  if
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=16
   global.get $~lib/memory/__stack_pointer
   i32.const 8448
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   i32.const 8448
   i32.const 1
   local.get $2
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 8448
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   i32.const 8448
   i32.const 3
   local.get $2
   call $~lib/staticarray/StaticArray<~lib/string/String>#__uset
   global.get $~lib/memory/__stack_pointer
   i32.const 8448
   i32.store
   global.get $~lib/memory/__stack_pointer
   i32.const 3504
   i32.store offset=4
   i32.const 8448
   call $~lib/staticarray/StaticArray<~lib/string/String>#join
   i32.const 7600
   i32.const 229
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.const 0
  i64.const 0
  i64.const 0
  call $~lib/as-bignum/assembly/integer/u256/u256#constructor
  local.tee $5
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  i64.load
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  local.get $4
  local.get $5
  i64.load
  i64.eq
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   i64.load offset=8
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $5
   i32.store
   local.get $4
   local.get $5
   i64.load offset=8
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   i64.load offset=16
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $5
   i32.store
   local.get $4
   local.get $5
   i64.load offset=16
   i64.eq
  else
   i32.const 0
  end
  if (result i32)
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store
   local.get $3
   i64.load offset=24
   local.set $4
   global.get $~lib/memory/__stack_pointer
   local.get $5
   i32.store
   local.get $4
   local.get $5
   i64.load offset=24
   i64.eq
  else
   i32.const 0
  end
  if
   i32.const 8496
   i32.const 7600
   i32.const 233
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  local.get $1
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $5
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  local.get $5
  local.get $3
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 8032
   i32.const 7600
   i32.const 237
   i32.const 30
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  local.get $3
  call $src/btc/types/SafeMath/SafeMath.sub
  local.tee $5
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $6
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store offset=8
  local.get $6
  local.get $1
  local.get $5
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $1
  local.get $2
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $2
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
   local.tee $1
   i32.store offset=40
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $3
   call $src/btc/types/SafeMath/SafeMath.add
   local.tee $1
   i32.store offset=44
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=36
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $0
   local.get $2
   local.get $1
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  else
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=36
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $0
   local.get $2
   local.get $3
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 48
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_20/OP_20#_transferFrom (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (param $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 52
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.const 52
  memory.fill
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=24
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $5
  local.get $2
  call $"~lib/map/Map<~lib/string/String,src/btc/contracts/OP_NET/OP_NET>#has"
  local.set $5
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $5
  i32.eqz
  if
   i32.const 0
   i32.const 7600
   i32.const 256
   i32.const 43
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=24
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  local.get $2
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $5
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  local.get $1
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $5
  i32.store offset=16
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $5
  local.get $4
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 8576
   i32.const 7600
   i32.const 260
   i32.const 30
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  local.get $1
  call $src/btc/contracts/OP_NET/OP_NET#isSelf
  if
   i32.const 8272
   i32.const 7600
   i32.const 262
   i32.const 34
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=24
  local.tee $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  local.get $2
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $5
  i32.store offset=20
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $5
  local.get $1
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  i32.eqz
  if
   i32.const 0
   i32.const 7600
   i32.const 265
   i32.const 37
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $5
  local.get $1
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $6
  i32.store offset=24
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $6
  local.get $4
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 8576
   i32.const 7600
   i32.const 268
   i32.const 32
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $7
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $7
  local.get $2
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
  local.tee $7
  i32.store offset=28
  global.get $~lib/memory/__stack_pointer
  local.get $7
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $7
  local.get $4
  call $~lib/as-bignum/assembly/integer/u256/u256.lt
  if
   i32.const 8032
   i32.const 7600
   i32.const 271
   i32.const 30
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $6
  local.get $4
  call $src/btc/types/SafeMath/SafeMath.sub
  local.tee $6
  i32.store offset=32
  global.get $~lib/memory/__stack_pointer
  local.get $7
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $7
  local.get $4
  call $src/btc/types/SafeMath/SafeMath.sub
  local.tee $7
  i32.store offset=36
  global.get $~lib/memory/__stack_pointer
  local.get $5
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $6
  i32.store offset=8
  local.get $5
  local.get $1
  local.get $6
  call $"src/btc/memory/KeyMerger/KeyMerger<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=40
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $7
  i32.store offset=8
  local.get $1
  local.get $2
  local.get $7
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.load offset=28
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=4
  local.get $1
  local.get $3
  call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#has"
  if
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $3
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#get"
   local.tee $1
   i32.store offset=44
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   local.get $4
   call $src/btc/types/SafeMath/SafeMath.add
   local.tee $1
   i32.store offset=48
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=40
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $0
   local.get $3
   local.get $1
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  else
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=40
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load offset=28
   local.tee $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $4
   i32.store offset=8
   local.get $0
   local.get $3
   local.get $4
   call $"src/btc/memory/AddressMemoryMap/AddressMemoryMap<~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set"
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 52
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $src/btc/contracts/OP_20/OP_20#callMethod (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  (local $4 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 6000
  i32.store
  block $folding-inner0
   block $case7|0
    block $case6|0
     block $case5|0
      block $case4|0
       block $case3|0
        block $case2|0
         block $case1|0
          i32.const 6000
          call $src/btc/math/abi/encodeSelector
          local.get $1
          i32.ne
          if
           global.get $~lib/memory/__stack_pointer
           i32.const 6048
           i32.store
           i32.const 6048
           call $src/btc/math/abi/encodeSelector
           local.get $1
           i32.eq
           br_if $case1|0
           global.get $~lib/memory/__stack_pointer
           i32.const 6096
           i32.store
           i32.const 6096
           call $src/btc/math/abi/encodeSelector
           local.get $1
           i32.eq
           br_if $case2|0
           global.get $~lib/memory/__stack_pointer
           i32.const 6144
           i32.store
           i32.const 6144
           call $src/btc/math/abi/encodeSelector
           local.get $1
           i32.eq
           br_if $case3|0
           global.get $~lib/memory/__stack_pointer
           i32.const 6176
           i32.store
           i32.const 6176
           call $src/btc/math/abi/encodeSelector
           local.get $1
           i32.eq
           br_if $case4|0
           global.get $~lib/memory/__stack_pointer
           i32.const 6208
           i32.store
           i32.const 6208
           call $src/btc/math/abi/encodeSelector
           local.get $1
           i32.eq
           br_if $case5|0
           global.get $~lib/memory/__stack_pointer
           i32.const 6256
           i32.store
           i32.const 6256
           call $src/btc/math/abi/encodeSelector
           local.get $1
           i32.eq
           br_if $case6|0
           br $case7|0
          end
          global.get $~lib/memory/__stack_pointer
          local.get $0
          i32.store
          global.get $~lib/memory/__stack_pointer
          local.get $2
          i32.store offset=4
          local.get $0
          local.get $2
          call $src/btc/contracts/OP_20/OP_20#allowance
          local.set $0
          br $folding-inner0
         end
         global.get $~lib/memory/__stack_pointer
         local.get $0
         i32.store
         global.get $~lib/memory/__stack_pointer
         local.get $2
         i32.store offset=4
         global.get $~lib/memory/__stack_pointer
         local.get $3
         i32.store offset=8
         local.get $0
         local.get $2
         local.get $3
         call $src/btc/contracts/OP_20/OP_20#approve
         local.set $0
         br $folding-inner0
        end
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store
        global.get $~lib/memory/__stack_pointer
        local.get $2
        i32.store offset=4
        global.get $~lib/memory/__stack_pointer
        i32.const 20
        i32.sub
        global.set $~lib/memory/__stack_pointer
        call $~stack_check
        global.get $~lib/memory/__stack_pointer
        i32.const 0
        i32.const 20
        memory.fill
        global.get $~lib/memory/__stack_pointer
        local.get $2
        i32.store
        global.get $~lib/memory/__stack_pointer
        local.get $2
        call $src/btc/buffer/BytesReader/BytesReader#readAddress
        local.tee $1
        i32.store offset=4
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store
        global.get $~lib/memory/__stack_pointer
        local.get $1
        i32.store offset=8
        global.get $~lib/memory/__stack_pointer
        local.get $0
        local.get $1
        call $src/btc/contracts/OP_20/OP_20#_balanceOf
        local.tee $1
        i32.store offset=12
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=16
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.load
        local.tee $2
        i32.store
        global.get $~lib/memory/__stack_pointer
        local.get $1
        i32.store offset=8
        local.get $2
        local.get $1
        call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store
        local.get $0
        i32.load
        local.set $0
        global.get $~lib/memory/__stack_pointer
        i32.const 20
        i32.add
        global.set $~lib/memory/__stack_pointer
        br $folding-inner0
       end
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store
       global.get $~lib/memory/__stack_pointer
       local.get $2
       i32.store offset=4
       global.get $~lib/memory/__stack_pointer
       local.get $3
       i32.store offset=8
       global.get $~lib/memory/__stack_pointer
       i32.const 20
       i32.sub
       global.set $~lib/memory/__stack_pointer
       call $~stack_check
       global.get $~lib/memory/__stack_pointer
       i32.const 0
       i32.const 20
       memory.fill
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store
       global.get $~lib/memory/__stack_pointer
       local.get $3
       i32.store offset=4
       global.get $~lib/memory/__stack_pointer
       local.get $2
       i32.store offset=16
       local.get $2
       call $src/btc/buffer/BytesReader/BytesReader#readAddress
       local.set $1
       global.get $~lib/memory/__stack_pointer
       local.get $1
       i32.store offset=8
       global.get $~lib/memory/__stack_pointer
       local.get $2
       i32.store offset=16
       local.get $2
       call $src/btc/buffer/BytesReader/BytesReader#readU256
       local.set $2
       global.get $~lib/memory/__stack_pointer
       local.get $2
       i32.store offset=12
       local.get $0
       local.get $3
       local.get $1
       local.get $2
       call $src/btc/contracts/OP_20/OP_20#_burn
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store offset=4
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.load
       local.tee $1
       i32.store
       local.get $1
       i32.const 1
       call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store
       local.get $0
       i32.load
       local.set $0
       global.get $~lib/memory/__stack_pointer
       i32.const 20
       i32.add
       global.set $~lib/memory/__stack_pointer
       br $folding-inner0
      end
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      i32.const 20
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.const 20
      memory.fill
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $3
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=16
      local.get $2
      call $src/btc/buffer/BytesReader/BytesReader#readAddress
      local.set $1
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=16
      local.get $2
      call $src/btc/buffer/BytesReader/BytesReader#readU256
      local.set $2
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=12
      local.get $0
      local.get $3
      local.get $1
      local.get $2
      call $src/btc/contracts/OP_20/OP_20#_mint
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.load
      local.tee $1
      i32.store
      local.get $1
      i32.const 1
      call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i32.load
      local.set $0
      global.get $~lib/memory/__stack_pointer
      i32.const 20
      i32.add
      global.set $~lib/memory/__stack_pointer
      br $folding-inner0
     end
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=4
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store offset=8
     global.get $~lib/memory/__stack_pointer
     i32.const 20
     i32.sub
     global.set $~lib/memory/__stack_pointer
     call $~stack_check
     global.get $~lib/memory/__stack_pointer
     i32.const 0
     i32.const 20
     memory.fill
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $3
     i32.store offset=4
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=16
     local.get $2
     call $src/btc/buffer/BytesReader/BytesReader#readAddress
     local.set $1
     global.get $~lib/memory/__stack_pointer
     local.get $1
     i32.store offset=8
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=16
     local.get $2
     call $src/btc/buffer/BytesReader/BytesReader#readU256
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=12
     local.get $0
     local.get $3
     local.get $1
     local.get $2
     call $src/btc/contracts/OP_20/OP_20#_transfer
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.load
     local.tee $1
     i32.store
     local.get $1
     i32.const 1
     call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     i32.load
     local.set $0
     global.get $~lib/memory/__stack_pointer
     i32.const 20
     i32.add
     global.set $~lib/memory/__stack_pointer
     br $folding-inner0
    end
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    i32.const 24
    i32.sub
    global.set $~lib/memory/__stack_pointer
    call $~stack_check
    global.get $~lib/memory/__stack_pointer
    i32.const 0
    i32.const 24
    memory.fill
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $2
    call $src/btc/buffer/BytesReader/BytesReader#readAddress
    local.set $1
    global.get $~lib/memory/__stack_pointer
    local.get $1
    i32.store offset=8
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $2
    call $src/btc/buffer/BytesReader/BytesReader#readAddress
    local.set $4
    global.get $~lib/memory/__stack_pointer
    local.get $4
    i32.store offset=12
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=20
    local.get $2
    call $src/btc/buffer/BytesReader/BytesReader#readU256
    local.set $2
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=16
    local.get $0
    local.get $3
    local.get $1
    local.get $4
    local.get $2
    call $src/btc/contracts/OP_20/OP_20#_transferFrom
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.load
    local.tee $1
    i32.store
    local.get $1
    i32.const 1
    call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    i32.load
    local.set $0
    global.get $~lib/memory/__stack_pointer
    i32.const 24
    i32.add
    global.set $~lib/memory/__stack_pointer
    br $folding-inner0
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $2
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $3
   i32.store offset=8
   local.get $0
   local.get $1
   local.get $2
   call $src/btc/contracts/OP_NET/OP_NET#callMethod
   local.set $0
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/contract/TestingContract/TestingContract#callMethod (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 6944
  i32.store
  block $folding-inner1
   block $folding-inner0
    block $break|0
     block $case9|0
      block $case8|0
       block $case6|0
        block $case5|0
         block $case3|0
          block $case2|0
           block $case1|0
            i32.const 6944
            call $src/btc/math/abi/encodeSelector
            local.get $1
            i32.ne
            if
             global.get $~lib/memory/__stack_pointer
             i32.const 6992
             i32.store
             i32.const 6992
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case1|0
             global.get $~lib/memory/__stack_pointer
             i32.const 6464
             i32.store
             i32.const 6464
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case2|0
             global.get $~lib/memory/__stack_pointer
             i32.const 6528
             i32.store
             i32.const 6528
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case3|0
             global.get $~lib/memory/__stack_pointer
             i32.const 6592
             i32.store
             i32.const 6592
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $folding-inner0
             global.get $~lib/memory/__stack_pointer
             i32.const 6752
             i32.store
             i32.const 6752
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case5|0
             global.get $~lib/memory/__stack_pointer
             i32.const 6816
             i32.store
             i32.const 6816
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case6|0
             global.get $~lib/memory/__stack_pointer
             i32.const 6880
             i32.store
             i32.const 6880
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $folding-inner0
             global.get $~lib/memory/__stack_pointer
             i32.const 6656
             i32.store
             i32.const 6656
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case8|0
             global.get $~lib/memory/__stack_pointer
             i32.const 6704
             i32.store
             i32.const 6704
             call $src/btc/math/abi/encodeSelector
             local.get $1
             i32.eq
             br_if $case9|0
             br $break|0
            end
            global.get $~lib/memory/__stack_pointer
            local.get $0
            i32.store
            global.get $~lib/memory/__stack_pointer
            local.get $2
            i32.store offset=4
            global.get $~lib/memory/__stack_pointer
            local.get $3
            i32.store offset=8
            global.get $~lib/memory/__stack_pointer
            i32.const 24
            i32.sub
            global.set $~lib/memory/__stack_pointer
            call $~stack_check
            global.get $~lib/memory/__stack_pointer
            i32.const 0
            i32.const 24
            memory.fill
            global.get $~lib/memory/__stack_pointer
            local.get $2
            i32.store
            global.get $~lib/memory/__stack_pointer
            local.get $2
            call $src/btc/buffer/BytesReader/BytesReader#readAddress
            local.tee $1
            i32.store offset=4
            global.get $~lib/memory/__stack_pointer
            local.get $2
            i32.store
            global.get $~lib/memory/__stack_pointer
            local.get $2
            call $src/btc/buffer/BytesReader/BytesReader#readU256
            local.tee $2
            i32.store offset=8
            global.get $~lib/memory/__stack_pointer
            local.get $0
            i32.store
            global.get $~lib/memory/__stack_pointer
            local.get $1
            i32.store offset=12
            global.get $~lib/memory/__stack_pointer
            local.get $2
            i32.store offset=16
            global.get $~lib/memory/__stack_pointer
            local.get $3
            i32.store offset=20
            local.get $0
            local.get $1
            local.get $2
            call $src/contract/TestingContract/TestingContract#_addFreeMoney
            global.get $~lib/memory/__stack_pointer
            local.get $0
            i32.store offset=12
            global.get $~lib/memory/__stack_pointer
            local.get $0
            i32.load
            local.tee $1
            i32.store
            local.get $1
            i32.const 1
            call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
            global.get $~lib/memory/__stack_pointer
            local.get $0
            i32.store
            local.get $0
            i32.load
            local.set $0
            global.get $~lib/memory/__stack_pointer
            i32.const 24
            i32.add
            global.set $~lib/memory/__stack_pointer
            br $folding-inner1
           end
           global.get $~lib/memory/__stack_pointer
           local.get $0
           i32.store
           global.get $~lib/memory/__stack_pointer
           local.get $2
           i32.store offset=4
           global.get $~lib/memory/__stack_pointer
           i32.const 24
           i32.sub
           global.set $~lib/memory/__stack_pointer
           call $~stack_check
           global.get $~lib/memory/__stack_pointer
           i32.const 0
           i32.const 24
           memory.fill
           global.get $~lib/memory/__stack_pointer
           local.get $2
           i32.store
           global.get $~lib/memory/__stack_pointer
           local.get $2
           call $src/btc/buffer/BytesReader/BytesReader#readAddress
           local.tee $1
           i32.store offset=4
           global.get $~lib/memory/__stack_pointer
           local.get $2
           i32.store
           global.get $~lib/memory/__stack_pointer
           local.get $2
           call $src/btc/buffer/BytesReader/BytesReader#readAddress
           local.tee $2
           i32.store offset=8
           global.get $~lib/memory/__stack_pointer
           local.get $0
           i32.store
           global.get $~lib/memory/__stack_pointer
           local.get $1
           i32.store offset=12
           global.get $~lib/memory/__stack_pointer
           local.get $2
           i32.store offset=16
           global.get $~lib/memory/__stack_pointer
           local.get $0
           local.get $1
           local.get $2
           call $src/contract/TestingContract/TestingContract#_testMethodMultipleAddresses
           local.tee $1
           i32.store offset=20
           global.get $~lib/memory/__stack_pointer
           local.get $0
           i32.store offset=16
           global.get $~lib/memory/__stack_pointer
           local.get $0
           i32.load
           local.tee $2
           i32.store
           global.get $~lib/memory/__stack_pointer
           local.get $1
           i32.store offset=12
           local.get $2
           local.get $1
           call $src/btc/buffer/BytesWriter/BytesWriter#writeTuple
           global.get $~lib/memory/__stack_pointer
           local.get $0
           i32.store
           local.get $0
           i32.load
           local.set $0
           global.get $~lib/memory/__stack_pointer
           i32.const 24
           i32.add
           global.set $~lib/memory/__stack_pointer
           br $folding-inner1
          end
          global.get $~lib/memory/__stack_pointer
          local.get $0
          i32.store
          global.get $~lib/memory/__stack_pointer
          i32.const 4
          i32.sub
          global.set $~lib/memory/__stack_pointer
          call $~stack_check
          global.get $~lib/memory/__stack_pointer
          i32.const 0
          i32.store
          global.get $~lib/memory/__stack_pointer
          local.get $0
          i32.store
          loop $while-continue|0
           br $while-continue|0
          end
          unreachable
         end
         global.get $~lib/memory/__stack_pointer
         local.get $0
         i32.store
         global.get $~lib/memory/__stack_pointer
         i32.const 8
         i32.sub
         global.set $~lib/memory/__stack_pointer
         call $~stack_check
         global.get $~lib/memory/__stack_pointer
         i64.const 0
         i64.store
         global.get $~lib/memory/__stack_pointer
         local.get $0
         i32.store
         i64.const 0
         i64.const 0
         i64.const 0
         i64.const 0
         call $~lib/as-bignum/assembly/integer/u256/u256#constructor
         local.set $1
         global.get $~lib/memory/__stack_pointer
         local.get $1
         i32.store offset=4
         local.get $0
         local.get $1
         call $src/contract/TestingContract/TestingContract#_crashIndexerTest2
         global.get $~lib/memory/__stack_pointer
         local.get $0
         i32.store
         local.get $0
         i32.load
         local.set $0
         global.get $~lib/memory/__stack_pointer
         i32.const 8
         i32.add
         global.set $~lib/memory/__stack_pointer
         br $folding-inner1
        end
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store
        global.get $~lib/memory/__stack_pointer
        i32.const 4
        i32.sub
        global.set $~lib/memory/__stack_pointer
        call $~stack_check
        global.get $~lib/memory/__stack_pointer
        i32.const 0
        i32.store
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store
        local.get $0
        call $src/contract/TestingContract/TestingContract#_crashIndexerView1
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store
        local.get $0
        i32.load
        local.set $0
        global.get $~lib/memory/__stack_pointer
        i32.const 4
        i32.add
        global.set $~lib/memory/__stack_pointer
        br $folding-inner1
       end
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store
       global.get $~lib/memory/__stack_pointer
       i32.const 4
       i32.sub
       global.set $~lib/memory/__stack_pointer
       call $~stack_check
       global.get $~lib/memory/__stack_pointer
       i32.const 0
       i32.store
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store
       global.get $~lib/memory/__stack_pointer
       i32.const 12
       i32.sub
       global.set $~lib/memory/__stack_pointer
       call $~stack_check
       global.get $~lib/memory/__stack_pointer
       i64.const 0
       i64.store
       global.get $~lib/memory/__stack_pointer
       i32.const 0
       i32.store offset=8
       global.get $~lib/memory/__stack_pointer
       i64.const 0
       i64.const 0
       i64.const 0
       i64.const 0
       call $~lib/as-bignum/assembly/integer/u256/u256#constructor
       local.tee $0
       i32.store
       loop $while-continue|00
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=4
        i64.const 1
        i64.const 0
        i64.const 0
        i64.const 0
        call $~lib/as-bignum/assembly/integer/u256/u256#constructor
        local.set $1
        global.get $~lib/memory/__stack_pointer
        local.get $1
        i32.store offset=8
        local.get $0
        local.get $1
        call $~lib/as-bignum/assembly/integer/u256/u256.add
        drop
        br $while-continue|00
       end
       unreachable
      end
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      i32.const 12
      i32.sub
      global.set $~lib/memory/__stack_pointer
      call $~stack_check
      global.get $~lib/memory/__stack_pointer
      i64.const 0
      i64.store
      global.get $~lib/memory/__stack_pointer
      i32.const 0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $2
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $2
      call $src/btc/buffer/BytesReader/BytesReader#readAddress
      local.tee $1
      i32.store offset=4
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $1
      i32.store offset=8
      local.get $0
      local.get $1
      call $src/contract/TestingContract/TestingContract#_attemptCrash1
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.load
      local.tee $1
      i32.store
      local.get $1
      i32.const 1
      call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store
      local.get $0
      i32.load
      local.set $0
      global.get $~lib/memory/__stack_pointer
      i32.const 12
      i32.add
      global.set $~lib/memory/__stack_pointer
      br $folding-inner1
     end
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     global.get $~lib/memory/__stack_pointer
     i32.const 8
     i32.sub
     global.set $~lib/memory/__stack_pointer
     call $~stack_check
     global.get $~lib/memory/__stack_pointer
     i64.const 0
     i64.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     call $src/contract/TestingContract/TestingContract#_attemptCrash2
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=4
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.load
     local.tee $1
     i32.store
     local.get $1
     i32.const 1
     call $src/btc/buffer/BytesWriter/BytesWriter#writeBoolean
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store
     local.get $0
     i32.load
     local.set $0
     global.get $~lib/memory/__stack_pointer
     i32.const 8
     i32.add
     global.set $~lib/memory/__stack_pointer
     br $folding-inner1
    end
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    global.get $~lib/memory/__stack_pointer
    local.get $2
    i32.store offset=4
    global.get $~lib/memory/__stack_pointer
    local.get $3
    i32.store offset=8
    local.get $0
    local.get $1
    local.get $2
    local.get $3
    call $src/btc/contracts/OP_20/OP_20#callMethod
    local.set $0
    br $folding-inner1
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.sub
   global.set $~lib/memory/__stack_pointer
   call $~stack_check
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store
   global.get $~lib/memory/__stack_pointer
   i64.const 0
   i64.store offset=8
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $0
   call $src/contract/TestingContract/TestingContract#_crashIndexerTest3
   local.tee $1
   i32.store offset=4
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store offset=12
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.load
   local.tee $2
   i32.store
   global.get $~lib/memory/__stack_pointer
   local.get $1
   i32.store offset=8
   local.get $2
   local.get $1
   call $src/btc/buffer/BytesWriter/BytesWriter#writeTuple
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
   local.set $0
   global.get $~lib/memory/__stack_pointer
   i32.const 16
   i32.add
   global.set $~lib/memory/__stack_pointer
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $src/btc/contracts/OP_20/OP_20#callView (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 6304
  i32.store
  block $folding-inner0 (result i32)
   block $break|0
    block $case4|0
     block $case3|0
      block $case2|0
       block $case1|0
        i32.const 6304
        call $src/btc/math/abi/encodeSelector
        local.get $1
        i32.ne
        if
         global.get $~lib/memory/__stack_pointer
         i32.const 6352
         i32.store
         i32.const 6352
         call $src/btc/math/abi/encodeSelector
         local.get $1
         i32.eq
         br_if $case1|0
         global.get $~lib/memory/__stack_pointer
         i32.const 6384
         i32.store
         i32.const 6384
         call $src/btc/math/abi/encodeSelector
         local.get $1
         i32.eq
         br_if $case2|0
         global.get $~lib/memory/__stack_pointer
         i32.const 6416
         i32.store
         i32.const 6416
         call $src/btc/math/abi/encodeSelector
         local.get $1
         i32.eq
         br_if $case3|0
         br $case4|0
        end
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=4
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.load
        local.tee $1
        i32.store
        global.get $~lib/memory/__stack_pointer
        local.get $0
        i32.store offset=4
        local.get $1
        local.get $0
        i32.load8_u offset=12
        call $src/btc/buffer/BytesWriter/BytesWriter#writeU8
        br $break|0
       end
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store offset=8
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.load
       local.tee $1
       i32.store
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.store offset=8
       global.get $~lib/memory/__stack_pointer
       local.get $0
       i32.load offset=16
       local.tee $2
       i32.store offset=4
       local.get $1
       local.get $2
       call $src/btc/buffer/BytesWriter/BytesWriter#writeString
       br $break|0
      end
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.load
      local.tee $1
      i32.store
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.store offset=8
      global.get $~lib/memory/__stack_pointer
      local.get $0
      i32.load offset=20
      local.tee $2
      i32.store offset=4
      local.get $1
      local.get $2
      call $src/btc/buffer/BytesWriter/BytesWriter#writeString
      br $break|0
     end
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.load
     local.tee $1
     i32.store
     global.get $~lib/memory/__stack_pointer
     local.get $0
     i32.store offset=8
     local.get $0
     call $src/btc/contracts/OP_20/OP_20#get:totalSupply
     local.set $2
     global.get $~lib/memory/__stack_pointer
     local.get $2
     i32.store offset=4
     local.get $1
     local.get $2
     call $src/btc/buffer/BytesWriter/BytesWriter#writeU256
     br $break|0
    end
    global.get $~lib/memory/__stack_pointer
    local.get $0
    i32.store
    local.get $0
    local.get $1
    call $src/btc/contracts/OP_NET/OP_NET#callView
    br $folding-inner0
   end
   global.get $~lib/memory/__stack_pointer
   local.get $0
   i32.store
   local.get $0
   i32.load
  end
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/arraybuffer/ArrayBuffer#constructor (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  local.get $0
  i32.const 1073741820
  i32.gt_u
  if
   i32.const 1904
   i32.const 1952
   i32.const 52
   i32.const 43
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.const 1
  call $~lib/rt/itcms/__new
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/rt/__newArray (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  (local $4 i32)
  (local $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.set $4
  local.get $0
  local.get $1
  i32.shl
  local.tee $1
  i32.const 1
  call $~lib/rt/itcms/__new
  local.set $5
  local.get $3
  if
   local.get $5
   local.get $3
   local.get $1
   memory.copy
  end
  local.get $4
  local.get $5
  i32.store
  i32.const 16
  local.get $2
  call $~lib/rt/itcms/__new
  local.tee $2
  local.get $5
  i32.store
  local.get $2
  local.get $5
  i32.const 0
  call $~lib/rt/itcms/__link
  local.get $2
  local.get $5
  i32.store offset=4
  local.get $2
  local.get $1
  i32.store offset=8
  local.get $2
  local.get $0
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $~lib/staticarray/StaticArray<u32>#constructor (param $0 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  local.get $0
  i32.const 268435455
  i32.gt_u
  if
   i32.const 1904
   i32.const 2976
   i32.const 51
   i32.const 60
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.const 2
  i32.shl
  i32.const 35
  call $~lib/rt/itcms/__new
  local.tee $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $~lib/string/String.UTF8.decodeUnsafe (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  local.get $0
  local.get $1
  i32.add
  local.set $4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.const 1
  i32.shl
  i32.const 2
  call $~lib/rt/itcms/__new
  local.tee $2
  i32.store
  local.get $2
  local.set $1
  loop $while-continue|0
   local.get $0
   local.get $4
   i32.lt_u
   if
    block $while-break|0
     local.get $0
     i32.load8_u
     local.set $5
     local.get $0
     i32.const 1
     i32.add
     local.set $0
     local.get $5
     i32.const 128
     i32.and
     if
      local.get $0
      local.get $4
      i32.eq
      br_if $while-break|0
      local.get $0
      i32.load8_u
      i32.const 63
      i32.and
      local.set $6
      local.get $0
      i32.const 1
      i32.add
      local.set $0
      local.get $5
      i32.const 224
      i32.and
      i32.const 192
      i32.eq
      if
       local.get $1
       local.get $5
       i32.const 31
       i32.and
       i32.const 6
       i32.shl
       local.get $6
       i32.or
       i32.store16
      else
       local.get $0
       local.get $4
       i32.eq
       br_if $while-break|0
       local.get $0
       i32.load8_u
       i32.const 63
       i32.and
       local.set $3
       local.get $0
       i32.const 1
       i32.add
       local.set $0
       local.get $5
       i32.const 240
       i32.and
       i32.const 224
       i32.eq
       if
        local.get $5
        i32.const 15
        i32.and
        i32.const 12
        i32.shl
        local.get $6
        i32.const 6
        i32.shl
        i32.or
        local.get $3
        i32.or
        local.set $3
       else
        local.get $0
        local.get $4
        i32.eq
        br_if $while-break|0
        local.get $0
        i32.load8_u
        i32.const 63
        i32.and
        local.get $5
        i32.const 7
        i32.and
        i32.const 18
        i32.shl
        local.get $6
        i32.const 12
        i32.shl
        i32.or
        local.get $3
        i32.const 6
        i32.shl
        i32.or
        i32.or
        local.set $3
        local.get $0
        i32.const 1
        i32.add
        local.set $0
       end
       local.get $3
       i32.const 65536
       i32.lt_u
       if
        local.get $1
        local.get $3
        i32.store16
       else
        local.get $1
        local.get $3
        i32.const 65536
        i32.sub
        local.tee $3
        i32.const 10
        i32.shr_u
        i32.const 55296
        i32.or
        local.get $3
        i32.const 1023
        i32.and
        i32.const 56320
        i32.or
        i32.const 16
        i32.shl
        i32.or
        i32.store
        local.get $1
        i32.const 2
        i32.add
        local.set $1
       end
      end
     else
      local.get $1
      local.get $5
      i32.store16
     end
     local.get $1
     i32.const 2
     i32.add
     local.set $1
     br $while-continue|0
    end
   end
  end
  local.get $2
  local.get $1
  local.get $2
  i32.sub
  call $~lib/rt/itcms/__renew
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $export:src/btc/exports/index/readMethod (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $0
  local.get $1
  local.get $2
  local.get $3
  call $src/btc/exports/index/readMethod
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $export:src/btc/exports/index/INIT (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $3
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  i32.load8_u
  if
   i32.const 0
   i32.const 2144
   i32.const 47
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=12
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.load offset=16
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  local.get $2
  i32.load8_u offset=8
  if
   i32.const 5168
   i32.const 3968
   i32.const 27
   i32.const 26
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $2
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $2
  local.get $1
  call $src/btc/universal/ABIRegistry/ABIRegistryBase#set:selectors
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.load offset=4
  local.tee $0
  i32.store
  local.get $0
  call $~lib/string/String.__not
  if
   i32.const 2416
   i32.const 3968
   i32.const 33
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.load
  local.tee $0
  i32.store
  local.get $0
  call $~lib/string/String.__not
  if
   i32.const 5248
   i32.const 3968
   i32.const 37
   i32.const 13
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store
  local.get $3
  i32.const 1
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#set:isInitialized
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $export:src/btc/exports/index/readView (param $0 i32) (param $1 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store
  local.get $0
  local.get $1
  call $src/btc/exports/index/readView
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $export:src/btc/exports/index/loadStorage (param $0 i32)
  (local $1 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  global.get $src/btc/env/index/Blockchain
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=4
  local.get $1
  local.get $0
  call $src/btc/env/BTCEnvironment/BlockchainEnvironment#loadStorage
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  i32.add
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
 (func $"byn$mgfn-shared$src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#constructor" (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (param $4 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 36
  local.get $4
  call $~lib/rt/itcms/__new
  local.tee $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=8
  local.get $4
  local.get $1
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:defaultValue"
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  i32.const 0
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:pointer"
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  i32.const 0
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:memoryAllocatorAddress"
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.set $1
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  local.get $4
  i32.eqz
  if
   global.get $~lib/memory/__stack_pointer
   i32.const 24
   local.get $3
   call $~lib/rt/itcms/__new
   local.tee $4
   i32.store
  end
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  i32.const 16
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $4
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  i32.const 3
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  i32.const 48
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $3
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=8
  local.get $4
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  i32.const 4
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
  local.get $4
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  local.get $4
  local.get $2
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:pointer"
  global.get $~lib/memory/__stack_pointer
  local.get $4
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $4
  local.get $0
  call $"src/btc/memory/MultiAddressMemoryMap/MultiAddressMemoryMap<~lib/string/String,~lib/string/String,~lib/as-bignum/assembly/integer/u256/u256>#set:memoryAllocatorAddress"
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $4
 )
 (func $byn$mgfn-shared$~lib/array/Array<~lib/as-bignum/assembly/integer/u256/u256>#constructor (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  local.get $1
  call $~lib/rt/itcms/__new
  local.tee $2
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtId
  local.get $0
  i32.const 268435455
  i32.gt_u
  if
   i32.const 1904
   i32.const 2928
   i32.const 70
   i32.const 60
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 8
  local.get $0
  local.get $0
  i32.const 8
  i32.le_u
  select
  i32.const 2
  i32.shl
  local.tee $1
  i32.const 1
  call $~lib/rt/itcms/__new
  local.tee $3
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  global.get $~lib/memory/__stack_pointer
  local.get $3
  i32.store offset=12
  local.get $2
  local.get $3
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  local.get $3
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  local.get $1
  call $~lib/rt/itcms/Object#set:prev
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=4
  local.get $2
  local.get $0
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  i32.const 16
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $2
 )
 (func $"byn$mgfn-shared$~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#constructor" (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i64.const 0
  i64.store
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store offset=8
  global.get $~lib/memory/__stack_pointer
  i32.const 24
  local.get $1
  call $~lib/rt/itcms/__new
  local.tee $1
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  i32.const 16
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $2
  global.get $~lib/memory/__stack_pointer
  local.get $2
  i32.store offset=8
  local.get $1
  local.get $2
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:buckets"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 3
  call $~lib/rt/itcms/Object#set:nextWithColor
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $0
  call $~lib/arraybuffer/ArrayBuffer#constructor
  local.set $0
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store offset=8
  local.get $1
  local.get $0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entries"
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 4
  call $~lib/rt/itcms/Object#set:rtId
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $~lib/rt/itcms/Object#set:rtSize
  global.get $~lib/memory/__stack_pointer
  local.get $1
  i32.store offset=4
  local.get $1
  i32.const 0
  call $"~lib/map/Map<src/btc/contracts/OP_NET/OP_NET,~lib/set/Set<u32>>#set:entriesCount"
  global.get $~lib/memory/__stack_pointer
  i32.const 12
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $1
 )
 (func $byn$mgfn-shared$~lib/typedarray/Uint8Array#__get (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (param $4 i32) (result i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=8
  i32.ge_u
  if
   i32.const 1696
   local.get $4
   local.get $3
   local.get $2
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.add
  i32.load8_u
  local.set $0
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
  local.get $0
 )
 (func $byn$mgfn-shared$~lib/typedarray/Uint8Array#__set (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (param $4 i32) (param $5 i32)
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  call $~stack_check
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $1
  local.get $0
  i32.load offset=8
  i32.ge_u
  if
   i32.const 1696
   local.get $5
   local.get $4
   local.get $3
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  local.get $0
  i32.store
  local.get $0
  i32.load offset=4
  local.get $1
  i32.add
  local.get $2
  i32.store8
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
)
