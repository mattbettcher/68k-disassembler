#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate regex;
use regex::Regex;

use std::collections::HashMap;

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum SP {
    n, // normal
    s, // static operand
    r, // register operand
    rr, // register to register
    mm, // memory to memory
    er, // effective address to register
    re, // register to effective address
    dd, // data register to data register
    da, // data register to address register
    di, // address register indirect with displacement
    al, // absolute long address
    aa, // address register to address register
    cr, // control register to register
    rc, // register to control register
    aw, // absolute word address
    pd, // address register indirect with predecrement
    pi, // address register indirect with postincrement
    ix, // address register indirect with index
    ai, // address register indirect
    d, // data register
    pd7, // ????
    pi7, // ????
    toc, // to condition code register
    tos, // to status register
    tou, // to user stack pointer
    frc, // from condition code register
    frs, // from status register
    fru, // from user stack pointer
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum SE {
    n, // normal
    i, // immediate
    d, // data register
    a, // address register
    ai, // address register indirect
    pi, // address register indirect with postincrement
    pd, // address register indirect with predecrement
    di, // address register indirect with displacement
    ix, // address register indirect with index
    aw, // absolute word address
    al, // absolute long address
    pcdi, // program counter relative with displacement
    pcix, // program counter relative with index
    a7, // register specified in instruction is A7
    ax7, // register field X of instruction is A7
    ay7, // register field Y of instruction is A7
    axy7, // register fields X and Y of instruction are A7
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Inst {
    name: &'static str,
    size: u16,
    sp: SP,
    se: SE,
    bitp: Regex,
    aea: &'static [u8; 10],
    cycles: [u32; 3],
}

// allowed ea:  List of allowed addressing modes:
// .: not present
// A: address register indirect
// +: ARI with postincrement
// -: ARI with predecrement
// D: ARI with displacement
// X: ARI with index
// W: absolute word address
// L: absolute long address
// d: program counter indirect with displacement
// x: program counter indirect with index
// I: immediate
//

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// fn generate_ea(eae: &'static str) -> String {
// let modes = 0;
//
// for t in eae {
// if t = '.' {
// break;
// }
//
// modes += 1;
// }
//
// }
//

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct NewInst {
    name: &'static str,
    bitp: &'static str,
    aea: &'static str,
    cycles: [u32; 3],
}

// let inst_list = [
// NewInst { name: "add", bitp: "1101DDD000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
// NewInst { name: "add", bitp: "1101DDD000......", aea: "A+-DXWLdxI", cycles: [ 4,  4,  2] },
// ];
//
// A: address register indirect
// +: ARI with postincrement
// -: ARI with predecrement
// D: ARI with displacement
// X: ARI with index
// W: absolute word address
// L: absolute long address
// d: program counter indirect with displacement
// x: program counter indirect with index
// I: immediate
//

fn get_static_addressing_mode(aea: &[u8], index: usize) -> Option<&'static str> {
    match aea[index] {
        b'A' => Some("(a0)"),
        b'+' => Some("(a0)+"),
        b'-' => Some("-(a0)"),
        b'D' => Some("42(a0)"),
        _ => None,
    }
}

/// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

    let test = [
        
		Inst { name: "abcd",     size:  8, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1100[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::ax7,  	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1100111100001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::ay7,  	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1100(?:111[0-1]{3})100001111").unwrap()}, 													
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::axy7, 	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1100111100001111").unwrap()}, 																
		Inst { name: "abcd",     size:  8, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1100[0-1]{3}100001(?:111[0-1]{3})").unwrap()}, 												
		Inst { name: "add",      size:  8, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "add",      size:  8, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "add",      size:  8, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1101[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "add",      size: 16, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "add",      size: 16, sp: SP::er,  se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "add",      size: 16, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "add",      size: 16, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1101[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "add",      size: 32, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "add",      size: 32, sp: SP::er,  se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}010001[0-1]{3}").unwrap()}, 														
		Inst { name: "add",      size: 32, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "add",      size: 32, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"1101[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "adda",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"1101[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "adda",     size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"1101[0-1]{3}011001[0-1]{3}").unwrap()}, 														
		Inst { name: "adda",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  2],	bitp: Regex::new(r"1101[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "adda",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}111000[0-1]{3}").unwrap()}, 														
		Inst { name: "adda",     size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}111001[0-1]{3}").unwrap()}, 														
		Inst { name: "adda",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "addi",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000011000000[0-1]{3}").unwrap()}, 															
		Inst { name: "addi",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000011000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "addi",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000011001000[0-1]{3}").unwrap()}, 															
		Inst { name: "addi",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000011001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "addi",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [16, 14,  2],	bitp: Regex::new(r"0000011010000[0-1]{3}").unwrap()}, 															
		Inst { name: "addi",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [20, 20,  4],	bitp: Regex::new(r"0000011010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "addq",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0101[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "addq",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0101[0-1]{3}000(001|010|011|100|101|110|111)[0-1]{3}").unwrap()}, 
		Inst { name: "addq",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0101[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "addq",     size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0101[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "addq",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0101[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "addq",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0101[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "addq",     size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0101[0-1]{3}010001[0-1]{3}").unwrap()},														
		Inst { name: "addq",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0101[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "addx",     size:  8, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1101(?:111[0-1]{3})100001[0-1]{3}").unwrap()},												
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::ax7,  	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1101111100001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::ay7,  	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1101(?:111[0-1]{3})100001111").unwrap()}, 													
		Inst { name: "addx",     size:  8, sp: SP::mm,  se: SE::axy7, 	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1101111100001111").unwrap()}, 																
		Inst { name: "addx",     size: 16, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1101[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "addx",     size: 16, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1101[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "addx",     size: 32, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 8,  6,  2],	bitp: Regex::new(r"1101[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "addx",     size: 32, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [30, 30, 12],	bitp: Regex::new(r"1101[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "and",      size:  8, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1100[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "and",      size:  8, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1100[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "and",      size:  8, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1100[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "and",      size: 16, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1100[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "and",      size: 16, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1100[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "and",      size: 16, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1100[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "and",      size: 32, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1100[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "and",      size: 32, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1100[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "and",      size: 32, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"1100[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "andi",     size: 16, sp: SP::toc, se: SE::n,    	aea: b"..........", cycles: [20, 16, 12],	bitp: Regex::new(r"0000001000111100").unwrap()}, 																
		Inst { name: "andi",     size: 16, sp: SP::tos, se: SE::n,    	aea: b"..........", cycles: [20, 16, 12],	bitp: Regex::new(r"0000001001111100").unwrap()}, 																
		Inst { name: "andi",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000001000000[0-1]{3}").unwrap()}, 															
		Inst { name: "andi",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000001000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "andi",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000001001000[0-1]{3}").unwrap()}, 															
		Inst { name: "andi",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000001001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "andi",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  2],	bitp: Regex::new(r"0000001010000[0-1]{3}").unwrap()}, 															
		Inst { name: "andi",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [20, 20,  4],	bitp: Regex::new(r"0000001010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "asr",      size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "asr",      size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}000100[0-1]{3}").unwrap()}, 														
		Inst { name: "asr",      size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "asr",      size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}001100[0-1]{3}").unwrap()}, 														
		Inst { name: "asr",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  5],	bitp: Regex::new(r"1110000011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "asr",      size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  6],	bitp: Regex::new(r"1110[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "asr",      size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  6],	bitp: Regex::new(r"1110[0-1]{3}010100[0-1]{3}").unwrap()}, 														
		Inst { name: "asl",      size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "asl",      size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}100100[0-1]{3}").unwrap()}, 														
		Inst { name: "asl",      size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "asl",      size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}101100[0-1]{3}").unwrap()}, 														
		Inst { name: "asl",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  6],	bitp: Regex::new(r"1110000111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "asl",      size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  8],	bitp: Regex::new(r"1110[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "asl",      size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  8],	bitp: Regex::new(r"1110[0-1]{3}110100[0-1]{3}").unwrap()}, 														
		Inst { name: "bcc",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  6],	bitp: Regex::new(r"0110(?:(0000|0001)[0-1]{4})[0-1]{8}").unwrap()}, 												
		Inst { name: "bcc",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10,  6],	bitp: Regex::new(r"0110(0010|0011|0100|0101|0110|0111|1000|1001|1010|1011|1100|1101|1110|1111)00000000").unwrap()},
		Inst { name: "bcc",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10,  6],	bitp: Regex::new(r"0110(0010|0011|0100|0101|0110|0111|1000|1001|1010|1011|1100|1101|1110|1111)11111111").unwrap()},
		Inst { name: "bchg",     size:  8, sp: SP::r,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0000[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "bchg",     size:  8, sp: SP::s,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000100001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "bchg",     size: 32, sp: SP::r,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0000[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "bchg",     size: 32, sp: SP::s,   se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0000100001000[0-1]{3}").unwrap()}, 															
		Inst { name: "bclr",     size:  8, sp: SP::r,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8, 10,  4],	bitp: Regex::new(r"0000[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "bclr",     size:  8, sp: SP::s,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000100010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "bclr",     size: 32, sp: SP::r,   se: SE::d,    	aea: b"..........", cycles: [10, 10,  4],	bitp: Regex::new(r"0000[0-1]{3}110000[0-1]{3}").unwrap()},														
		Inst { name: "bclr",     size: 32, sp: SP::s,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"0000100010000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfchg",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110101011000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfchg",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [14, 14,  4],	bitp: Regex::new(r"1110101011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bfclr",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110110011000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfclr",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [14, 14,  4],	bitp: Regex::new(r"1110110011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bfexts",   size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110101111000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfexts",   size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [14, 14,  4],	bitp: Regex::new(r"1110101111(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bfextu",   size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110100111000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfextu",   size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [14, 14,  4],	bitp: Regex::new(r"1110100111(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bfffo",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110110111000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfffo",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [14, 14,  4],	bitp: Regex::new(r"1110110111(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bfins",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110111111000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfins",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [14, 14,  4],	bitp: Regex::new(r"1110111111(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bfset",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110111011000[0-1]{3}").unwrap()}, 															
		Inst { name: "bfset",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [14, 14,  4],	bitp: Regex::new(r"1110111011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bftst",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"1110100011000[0-1]{3}").unwrap()}, 															
		Inst { name: "bftst",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [14, 14,  4],	bitp: Regex::new(r"1110100011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bkpt",     size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [14, 14,  4],	bitp: Regex::new(r"0100100001001[0-1]{3}").unwrap()}, 															
		Inst { name: "bra",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10, 10],	bitp: Regex::new(r"01100000(?:(00000000|11111111)[0-1]{8})").unwrap()},											
		Inst { name: "bra",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10, 10],	bitp: Regex::new(r"0110000000000000").unwrap()}, 																
		Inst { name: "bra",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10, 10],	bitp: Regex::new(r"0110000011111111").unwrap()}, 																
		Inst { name: "bset",     size:  8, sp: SP::r,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0000[0-1]{3}111(010|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "bset",     size:  8, sp: SP::s,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000100011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "bset",     size: 32, sp: SP::r,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0000[0-1]{3}111000[0-1]{3}").unwrap()}, 														
		Inst { name: "bset",     size: 32, sp: SP::s,   se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0000100011000[0-1]{3}").unwrap()}, 															
		Inst { name: "bsr",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [18, 18,  7],	bitp: Regex::new(r"01100001(?:(00000000|11111111)[0-1]{8})").unwrap()}, 											
		Inst { name: "bsr",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [18, 18,  7],	bitp: Regex::new(r"0110000100000000").unwrap()}, 																
		Inst { name: "bsr",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [18, 18,  7],	bitp: Regex::new(r"0110000111111111").unwrap()}, 																
		Inst { name: "btst",     size:  8, sp: SP::r,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0000[0-1]{3}100(010|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "btst",     size:  8, sp: SP::s,   se: SE::n,    	aea: b"A+-DXWLdx.", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0000100000(000|010|101|110|111)[0-1]{3}").unwrap()}, 											
		Inst { name: "btst",     size: 32, sp: SP::r,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"0000[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "btst",     size: 32, sp: SP::s,   se: SE::d,    	aea: b"..........", cycles: [10, 10,  4],	bitp: Regex::new(r"0000100000000[0-1]{3}").unwrap()}, 															
		Inst { name: "callm",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [10, 10,  4],	bitp: Regex::new(r"0000011011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "cas",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [10, 10,  4],	bitp: Regex::new(r"0000101011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cas",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [10, 10,  4],	bitp: Regex::new(r"0000110011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cas",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [10, 10,  4],	bitp: Regex::new(r"0000111011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cas2",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10,  4],	bitp: Regex::new(r"0000110011111100").unwrap()}, 																
		Inst { name: "cas2",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [10, 10,  4],	bitp: Regex::new(r"0000111011111100").unwrap()}, 																
		Inst { name: "chk",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0100[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "chk",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [10,  8,  8],	bitp: Regex::new(r"0100[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "chk",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0100[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "chk",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [10,  8,  8],	bitp: Regex::new(r"0100[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "chk2cmp2", size:  8, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0000000011111010").unwrap()}, 																
		Inst { name: "chk2cmp2", size:  8, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0000000011111011").unwrap()}, 																
		Inst { name: "chk2cmp2", size:  8, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [10,  8,  8],	bitp: Regex::new(r"0000000011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "chk2cmp2", size:  6, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0000001011111010").unwrap()}, 																
		Inst { name: "chk2cmp2", size:  6, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0000001011111011").unwrap()}, 																
		Inst { name: "chk2cmp2", size:  6, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [10,  8,  8],	bitp: Regex::new(r"0000001011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "chk2cmp2", size:  2, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0000010011111010").unwrap()}, 																
		Inst { name: "chk2cmp2", size:  2, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [10,  8,  8],	bitp: Regex::new(r"0000010011111011").unwrap()}, 																
		Inst { name: "chk2cmp2", size:  2, sp: SP::n,   se: SE::n,    	aea: b"A..DXWL...", cycles: [10,  8,  8],	bitp: Regex::new(r"0000010011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "clr",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100001000000[0-1]{3}").unwrap()}, 															
		Inst { name: "clr",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  4,  4],	bitp: Regex::new(r"0100001000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "clr",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100001001000[0-1]{3}").unwrap()}, 															
		Inst { name: "clr",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  4,  4],	bitp: Regex::new(r"0100001001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "clr",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"0100001010000[0-1]{3}").unwrap()}, 															
		Inst { name: "clr",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12,  6,  4],	bitp: Regex::new(r"0100001010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cmp",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "cmp",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "cmp",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "cmp",      size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "cmp",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "cmp",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1011[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "cmp",      size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1011[0-1]{3}010001[0-1]{3}").unwrap()}, 														
		Inst { name: "cmp",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1011[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "cmpa",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1011[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "cmpa",     size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1011[0-1]{3}011001[0-1]{3}").unwrap()}, 														
		Inst { name: "cmpa",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1011[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "cmpa",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1011[0-1]{3}111000[0-1]{3}").unwrap()}, 														
		Inst { name: "cmpa",     size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1011[0-1]{3}111001[0-1]{3}").unwrap()}, 														
		Inst { name: "cmpa",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1011[0-1]{3}111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110000000[0-1]{3}").unwrap()}, 															
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110000111010").unwrap()}, 																
		Inst { name: "cmpi",     size:  8, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110000111011").unwrap()}, 																
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110001000[0-1]{3}").unwrap()}, 															
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110001111010").unwrap()}, 																
		Inst { name: "cmpi",     size: 16, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000110001111011").unwrap()}, 																
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [14, 12,  2],	bitp: Regex::new(r"0000110010000[0-1]{3}").unwrap()}, 															
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  2],	bitp: Regex::new(r"0000110010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [12, 12,  2],	bitp: Regex::new(r"0000110010111010").unwrap()}, 																
		Inst { name: "cmpi",     size: 32, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [12, 12,  2],	bitp: Regex::new(r"0000110010111011").unwrap()}, 																
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::ax7,  	aea: b"..........", cycles: [12, 12,  9],	bitp: Regex::new(r"1011111100001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::ay7,  	aea: b"..........", cycles: [12, 12,  9],	bitp: Regex::new(r"1011(?:111[0-1]{3})100001111").unwrap()}, 													
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::axy7, 	aea: b"..........", cycles: [12, 12,  9],	bitp: Regex::new(r"1011111100001111").unwrap()}, 																
		Inst { name: "cmpm",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [12, 12,  9],	bitp: Regex::new(r"1011[0-1]{3}100001(?:111[0-1]{3})").unwrap()}, 												
		Inst { name: "cmpm",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [12, 12,  9],	bitp: Regex::new(r"1011[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "cmpm",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"1011[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "cpbcc",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"1111[0-1]{3}01[0-1]{7}").unwrap()}, 															
		Inst { name: "cpdbcc",   size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"1111[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "cpgen",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"1111[0-1]{3}000[0-1]{6}").unwrap()}, 															
		Inst { name: "cpscc",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"1111(?:111[0-1]{3})001(000|010|011|100|101|110|111)[0-1]{3}").unwrap()}, 						
		Inst { name: "cptrapcc", size:  2, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"1111[0-1]{3}001111(010|011|100)").unwrap()}, 													
		Inst { name: "dbt",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"0101000011001[0-1]{3}").unwrap()}, 															
		Inst { name: "dbf",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [14, 14,  6],	bitp: Regex::new(r"0101000111001[0-1]{3}").unwrap()}, 															
		Inst { name: "dbcc",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"0101[0-1]{3}.11001[0-1]{3}").unwrap()}, 														
		Inst { name: "divs",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [158, 122, 56], bitp: Regex::new(r"1000[0-1]{3}111000[0-1]{3}").unwrap()}, 														
		Inst { name: "divs",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [158, 122, 56], bitp: Regex::new(r"1000[0-1]{3}111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "divu",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [140, 108, 44], bitp: Regex::new(r"1000[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "divu",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [140, 108, 44], bitp: Regex::new(r"1000[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "divl",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [140, 108, 44], bitp: Regex::new(r"0100110001000[0-1]{3}").unwrap()}, 															
		Inst { name: "divl",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [140, 108, 44], bitp: Regex::new(r"0100110001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "eor",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "eor",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1011[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "eor",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1011[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "eor",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1011[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "eor",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  6,  2],	bitp: Regex::new(r"1011[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "eor",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"1011[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "eori",     size: 16, sp: SP::toc, se: SE::n,    	aea: b"..........", cycles: [20, 16, 12],	bitp: Regex::new(r"0000101000111100").unwrap()}, 																
		Inst { name: "eori",     size: 16, sp: SP::tos, se: SE::n,    	aea: b"..........", cycles: [20, 16, 12],	bitp: Regex::new(r"0000101001111100").unwrap()}, 																
		Inst { name: "eori",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000101000000[0-1]{3}").unwrap()}, 															
		Inst { name: "eori",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000101000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "eori",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000101001000[0-1]{3}").unwrap()}, 															
		Inst { name: "eori",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000101001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "eori",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [16, 14,  2],	bitp: Regex::new(r"0000101010000[0-1]{3}").unwrap()}, 															
		Inst { name: "eori",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [20, 20,  4],	bitp: Regex::new(r"0000101010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "exg",      size: 32, sp: SP::dd,  se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1100[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "exg",      size: 32, sp: SP::aa,  se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1100[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "exg",      size: 32, sp: SP::da,  se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1100[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "ext",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100100010000[0-1]{3}").unwrap()}, 															
		Inst { name: "ext",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100100011000[0-1]{3}").unwrap()}, 															
		Inst { name: "extb",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100100111000[0-1]{3}").unwrap()}, 															
		Inst { name: "illegal",  size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100101011111100").unwrap()}, 																
		Inst { name: "jmp",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [ 4,  4,  0],	bitp: Regex::new(r"0100111011(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "jsr",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [12, 12,  0],	bitp: Regex::new(r"0100111010(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "lea",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [ 0,  0,  2],	bitp: Regex::new(r"0100[0-1]{3}111(010|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "link",     size: 16, sp: SP::n,   se: SE::a7,   	aea: b"..........", cycles: [16, 16,  5],	bitp: Regex::new(r"0100111001010111").unwrap()}, 																
		Inst { name: "link",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [16, 16,  5],	bitp: Regex::new(r"0100111001010[0-1]{3}").unwrap()}, 															
		Inst { name: "link",     size: 32, sp: SP::n,   se: SE::a7,   	aea: b"..........", cycles: [16, 16,  5],	bitp: Regex::new(r"0100100000001111").unwrap()}, 																
		Inst { name: "link",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [16, 16,  5],	bitp: Regex::new(r"0100100000001[0-1]{3}").unwrap()}, 															
		Inst { name: "lsr",      size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1110[0-1]{3}000001[0-1]{3}").unwrap()}, 														
		Inst { name: "lsr",      size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1110[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "lsr",      size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1110[0-1]{3}010001[0-1]{3}").unwrap()}, 														
		Inst { name: "lsr",      size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}000101[0-1]{3}").unwrap()}, 														
		Inst { name: "lsr",      size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}001101[0-1]{3}").unwrap()}, 														
		Inst { name: "lsr",      size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  6],	bitp: Regex::new(r"1110[0-1]{3}010101[0-1]{3}").unwrap()}, 														
		Inst { name: "lsr",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  5],	bitp: Regex::new(r"1110001011(010|011|100|101|110|111)[0-1]{3}").unwrap()},										
		Inst { name: "lsl",      size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1110[0-1]{3}100001[0-1]{3}").unwrap()}, 														
		Inst { name: "lsl",      size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1110[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "lsl",      size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1110[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "lsl",      size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}100101[0-1]{3}").unwrap()}, 														
		Inst { name: "lsl",      size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"1110[0-1]{3}101101[0-1]{3}").unwrap()}, 														
		Inst { name: "lsl",      size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  6],	bitp: Regex::new(r"1110[0-1]{3}110101[0-1]{3}").unwrap()}, 														
		Inst { name: "lsl",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  5],	bitp: Regex::new(r"1110001111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size:  8, sp: SP::d,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0001[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size:  8, sp: SP::d,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0001[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size:  8, sp: SP::ai,  se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0001[0-1]{3}010000[0-1]{3}").unwrap()},														
		Inst { name: "move",     size:  8, sp: SP::ai,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0001[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size:  8, sp: SP::pi,  se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0001[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size:  8, sp: SP::pi,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0001[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size:  8, sp: SP::pi7, se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0001111011000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size:  8, sp: SP::pi7, se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0001111011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size:  8, sp: SP::pd,  se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0001[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size:  8, sp: SP::pd,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0001[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size:  8, sp: SP::pd7, se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0001111100000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size:  8, sp: SP::pd7, se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0001111100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size:  8, sp: SP::di,  se: SE::d,    	aea: b"..........", cycles: [12, 12,  5],	bitp: Regex::new(r"0001[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size:  8, sp: SP::di,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  5],	bitp: Regex::new(r"0001[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size:  8, sp: SP::ix,  se: SE::d,    	aea: b"..........", cycles: [14, 14,  7],	bitp: Regex::new(r"0001[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size:  8, sp: SP::ix,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [14, 14,  7],	bitp: Regex::new(r"0001[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size:  8, sp: SP::aw,  se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0001000111000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size:  8, sp: SP::aw,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  4],	bitp: Regex::new(r"0001000111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size:  8, sp: SP::al,  se: SE::d,    	aea: b"..........", cycles: [16, 16,  6],	bitp: Regex::new(r"0001001111000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size:  8, sp: SP::al,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [16, 16,  6],	bitp: Regex::new(r"0001001111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 16, sp: SP::d,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0011[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::d,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0011[0-1]{3}000001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::d,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0011[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::ai,  se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0011[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::ai,  se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0011[0-1]{3}010001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::ai,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0011[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::pi,  se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0011[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::pi,  se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0011[0-1]{3}011001[0-1]{3}").unwrap()},														
		Inst { name: "move",     size: 16, sp: SP::pi,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0011[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::pd,  se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0011[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::pd,  se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0011[0-1]{3}100001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::pd,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0011[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::di,  se: SE::d,    	aea: b"..........", cycles: [12, 12,  5],	bitp: Regex::new(r"0011[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::di,  se: SE::a,    	aea: b"..........", cycles: [12, 12,  5],	bitp: Regex::new(r"0011[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::di,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  5],	bitp: Regex::new(r"0011[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::ix,  se: SE::d,    	aea: b"..........", cycles: [14, 14,  7],	bitp: Regex::new(r"0011[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::ix,  se: SE::a,    	aea: b"..........", cycles: [14, 14,  7],	bitp: Regex::new(r"0011[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 16, sp: SP::ix,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [14, 14,  7],	bitp: Regex::new(r"0011[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::aw,  se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0011000111000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::aw,  se: SE::a,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0011000111001[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::aw,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  4],	bitp: Regex::new(r"0011000111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 16, sp: SP::al,  se: SE::d,    	aea: b"..........", cycles: [16, 16,  6],	bitp: Regex::new(r"0011001111000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::al,  se: SE::a,    	aea: b"..........", cycles: [16, 16,  6],	bitp: Regex::new(r"0011001111001[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::al,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [16, 16,  6],	bitp: Regex::new(r"0011001111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 32, sp: SP::d,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0010[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::d,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0010[0-1]{3}000001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::d,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0010[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 32, sp: SP::ai,  se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0010[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::ai,  se: SE::a,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0010[0-1]{3}010001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::ai,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  4],	bitp: Regex::new(r"0010[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 32, sp: SP::pi,  se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0010[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::pi,  se: SE::a,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0010[0-1]{3}011001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::pi,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  4],	bitp: Regex::new(r"0010[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 32, sp: SP::pd,  se: SE::d,    	aea: b"..........", cycles: [12, 14,  5],	bitp: Regex::new(r"0010[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::pd,  se: SE::a,    	aea: b"..........", cycles: [12, 14,  5],	bitp: Regex::new(r"0010[0-1]{3}100001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::pd,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 14,  5],	bitp: Regex::new(r"0010[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 32, sp: SP::di,  se: SE::d,    	aea: b"..........", cycles: [16, 16,  5],	bitp: Regex::new(r"0010[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::di,  se: SE::a,    	aea: b"..........", cycles: [16, 16,  5],	bitp: Regex::new(r"0010[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::di,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [16, 16,  5],	bitp: Regex::new(r"0010[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 32, sp: SP::ix,  se: SE::d,    	aea: b"..........", cycles: [18, 18,  7],	bitp: Regex::new(r"0010[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::ix,  se: SE::a,    	aea: b"..........", cycles: [18, 18,  7],	bitp: Regex::new(r"0010[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "move",     size: 32, sp: SP::ix,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [18, 18,  7],	bitp: Regex::new(r"0010[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 32, sp: SP::aw,  se: SE::d,    	aea: b"..........", cycles: [16, 16,  4],	bitp: Regex::new(r"0010000111000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 32, sp: SP::aw,  se: SE::a,    	aea: b"..........", cycles: [16, 16,  4],	bitp: Regex::new(r"0010000111001[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 32, sp: SP::aw,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [16, 16,  4],	bitp: Regex::new(r"0010000111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 32, sp: SP::al,  se: SE::d,    	aea: b"..........", cycles: [20, 20,  6],	bitp: Regex::new(r"0010001111000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 32, sp: SP::al,  se: SE::a,    	aea: b"..........", cycles: [20, 20,  6],	bitp: Regex::new(r"0010001111001[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 32, sp: SP::al,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [20, 20,  6],	bitp: Regex::new(r"0010001111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "movea",    size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0011[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "movea",    size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0011[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "movea",    size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0011[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "movea",    size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0010[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "movea",    size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0010[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "movea",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0010[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "move",     size: 16, sp: SP::frc, se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100001011000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::frc, se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100001011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 16, sp: SP::toc, se: SE::d,    	aea: b"..........", cycles: [12, 12,  4],	bitp: Regex::new(r"0100010011000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::toc, se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  4],	bitp: Regex::new(r"0100010011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 16, sp: SP::frs, se: SE::d,    	aea: b"..........", cycles: [ 6,  4,  8],	bitp: Regex::new(r"0100000011000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::frs, se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  8],	bitp: Regex::new(r"0100000011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "move",     size: 16, sp: SP::tos, se: SE::d,    	aea: b"..........", cycles: [12, 12,  8],	bitp: Regex::new(r"0100011011000[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 16, sp: SP::tos, se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [12, 12,  8],	bitp: Regex::new(r"0100011011(010|011|100|101|110|111)[0-1]{3}").unwrap()},										
		Inst { name: "move",     size: 32, sp: SP::fru, se: SE::n,    	aea: b"..........", cycles: [ 4,  6,  2],	bitp: Regex::new(r"0100111001101[0-1]{3}").unwrap()}, 															
		Inst { name: "move",     size: 32, sp: SP::tou, se: SE::n,    	aea: b"..........", cycles: [ 4,  6,  2],	bitp: Regex::new(r"0100111001100[0-1]{3}").unwrap()}, 															
		Inst { name: "movec",    size: 32, sp: SP::cr,  se: SE::n,    	aea: b"..........", cycles: [ 4,  6,  2],	bitp: Regex::new(r"0100111001111010").unwrap()}, 																
		Inst { name: "movec",    size: 32, sp: SP::rc,  se: SE::n,    	aea: b"..........", cycles: [ 4,  6,  2],	bitp: Regex::new(r"0100111001111011").unwrap()}, 																
		Inst { name: "movem",    size: 16, sp: SP::re,  se: SE::pd,   	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100100010100[0-1]{3}").unwrap()}, 															
		Inst { name: "movem",    size: 16, sp: SP::re,  se: SE::n,    	aea: b"A..DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100100010(010|100|101|110|111)[0-1]{3}").unwrap()}, 											
		Inst { name: "movem",    size: 32, sp: SP::re,  se: SE::pd,   	aea: b"..........", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100100011100[0-1]{3}").unwrap()}, 															
		Inst { name: "movem",    size: 32, sp: SP::re,  se: SE::n,    	aea: b"A..DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100100011(010|100|101|110|111)[0-1]{3}").unwrap()}, 											
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::pi,   	aea: b"..........", cycles: [12, 12,  8],	bitp: Regex::new(r"0100110010011[0-1]{3}").unwrap()}, 															
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::pcdi, 	aea: b"..........", cycles: [16, 16,  9],	bitp: Regex::new(r"0100110010111010").unwrap()}, 																
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::pcix, 	aea: b"..........", cycles: [18, 18, 11],	bitp: Regex::new(r"0100110010111011").unwrap()}, 																
		Inst { name: "movem",    size: 16, sp: SP::er,  se: SE::n,    	aea: b"A..DXWL...", cycles: [12, 12,  8],	bitp: Regex::new(r"0100110010(010|100|101|110|111)[0-1]{3}").unwrap()}, 											
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::pi,   	aea: b"..........", cycles: [12, 12,  8],	bitp: Regex::new(r"0100110011011[0-1]{3}").unwrap()}, 															
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::pcdi, 	aea: b"..........", cycles: [20, 20,  9],	bitp: Regex::new(r"0100110011111010").unwrap()}, 																
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::pcix, 	aea: b"..........", cycles: [22, 22, 11],	bitp: Regex::new(r"0100110011111011").unwrap()}, 																
		Inst { name: "movem",    size: 32, sp: SP::er,  se: SE::n,    	aea: b"A..DXWL...", cycles: [12, 12,  8],	bitp: Regex::new(r"0100110011(010|100|101|110|111)[0-1]{3}").unwrap()}, 											
		Inst { name: "movep",    size: 16, sp: SP::er,  se: SE::n,    	aea: b"..........", cycles: [16, 16, 12],	bitp: Regex::new(r"0000[0-1]{3}100001[0-1]{3}").unwrap()}, 														
		Inst { name: "movep",    size: 32, sp: SP::er,  se: SE::n,    	aea: b"..........", cycles: [24, 24, 18],	bitp: Regex::new(r"0000[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "movep",    size: 16, sp: SP::re,  se: SE::n,    	aea: b"..........", cycles: [16, 16, 11],	bitp: Regex::new(r"0000[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "movep",    size: 32, sp: SP::re,  se: SE::n,    	aea: b"..........", cycles: [24, 24, 17],	bitp: Regex::new(r"0000[0-1]{3}111001[0-1]{3}").unwrap()}, 														
		Inst { name: "moveq",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0111[0-1]{3}0[0-1]{8}").unwrap()}, 															
		Inst { name: "moves",    size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0000111000[0-1]{6}").unwrap()}, 																
		Inst { name: "moves",    size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0000111001[0-1]{6}").unwrap()}, 																
		Inst { name: "moves",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0000111010[0-1]{6}").unwrap()}, 																
		Inst { name: "muls",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [54, 32, 27],	bitp: Regex::new(r"1100[0-1]{3}111000[0-1]{3}").unwrap()}, 														
		Inst { name: "muls",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [54, 32, 27],	bitp: Regex::new(r"1100[0-1]{3}111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "mulu",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [54, 30, 27],	bitp: Regex::new(r"1100[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "mulu",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [54, 30, 27],	bitp: Regex::new(r"1100[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "mull",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [54, 30, 27],	bitp: Regex::new(r"0100110000000[0-1]{3}").unwrap()}, 															
		Inst { name: "mull",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [54, 30, 27],	bitp: Regex::new(r"0100110000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "nbcd",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  6],	bitp: Regex::new(r"0100100000000[0-1]{3}").unwrap()}, 															
		Inst { name: "nbcd",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  6],	bitp: Regex::new(r"0100100000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "neg",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100010000000[0-1]{3}").unwrap()}, 															
		Inst { name: "neg",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100010000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "neg",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100010001000[0-1]{3}").unwrap()}, 															
		Inst { name: "neg",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100010001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "neg",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"0100010010000[0-1]{3}").unwrap()}, 															
		Inst { name: "neg",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0100010010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "negx",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100000000000[0-1]{3}").unwrap()}, 															
		Inst { name: "negx",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100000000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "negx",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100000001000[0-1]{3}").unwrap()}, 															
		Inst { name: "negx",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100000001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "negx",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"0100000010000[0-1]{3}").unwrap()}, 															
		Inst { name: "negx",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0100000010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "nop",      size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100111001110001").unwrap()}, 																
		Inst { name: "not",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100011000000[0-1]{3}").unwrap()}, 															
		Inst { name: "not",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100011000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "not",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100011001000[0-1]{3}").unwrap()}, 															
		Inst { name: "not",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0100011001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "not",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"0100011010000[0-1]{3}").unwrap()}, 															
		Inst { name: "not",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0100011010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "or",       size:  8, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1000[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "or",       size:  8, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1000[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "or",       size: 16, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1000[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "or",       size: 16, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1000[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "or",       size: 32, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1000[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "or",       size: 32, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1000[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "or",       size:  8, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1000[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "or",       size: 16, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1000[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "or",       size: 32, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"1000[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()},									
		Inst { name: "ori",      size: 16, sp: SP::toc, se: SE::n,    	aea: b"..........", cycles: [20, 16, 12],	bitp: Regex::new(r"0000000000111100").unwrap()}, 																
		Inst { name: "ori",      size: 16, sp: SP::tos, se: SE::n,    	aea: b"..........", cycles: [20, 16, 12],	bitp: Regex::new(r"0000000001111100").unwrap()}, 																
		Inst { name: "ori",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000000000000[0-1]{3}").unwrap()}, 															
		Inst { name: "ori",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000000000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "ori",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000000001000[0-1]{3}").unwrap()}, 															
		Inst { name: "ori",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000000001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "ori",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [16, 14,  2],	bitp: Regex::new(r"0000000010000[0-1]{3}").unwrap()}, 															
		Inst { name: "ori",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [20, 20,  4],	bitp: Regex::new(r"0000000010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "pack",     size: 16, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [20, 20,  4],	bitp: Regex::new(r"1000[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::ax7,  	aea: b"..........", cycles: [20, 20,  4],	bitp: Regex::new(r"1000111101001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::ay7,  	aea: b"..........", cycles: [20, 20,  4],	bitp: Regex::new(r"1000(?:111[0-1]{3})101001111").unwrap()}, 													
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::axy7, 	aea: b"..........", cycles: [20, 20,  4],	bitp: Regex::new(r"1000111101001111").unwrap()}, 																
		Inst { name: "pack",     size: 16, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [20, 20,  4],	bitp: Regex::new(r"1000[0-1]{3}101001(?:111[0-1]{3})").unwrap()}, 												
		Inst { name: "pea",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A..DXWLdx.", cycles: [ 6,  6,  5],	bitp: Regex::new(r"0100100001(010|101|110|111)[0-1]{3}").unwrap()}, 												
		Inst { name: "reset",    size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 0,  0,  0],	bitp: Regex::new(r"0100111001110000").unwrap()}, 																
		Inst { name: "ror",      size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}000011[0-1]{3}").unwrap()}, 														
		Inst { name: "ror",      size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}001011[0-1]{3}").unwrap()}, 														
		Inst { name: "ror",      size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  8],	bitp: Regex::new(r"1110[0-1]{3}010011[0-1]{3}").unwrap()}, 														
		Inst { name: "ror",      size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}000111[0-1]{3}").unwrap()}, 														
		Inst { name: "ror",      size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}001111[0-1]{3}").unwrap()}, 														
		Inst { name: "ror",      size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  8],	bitp: Regex::new(r"1110[0-1]{3}010111[0-1]{3}").unwrap()}, 														
		Inst { name: "ror",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  7],	bitp: Regex::new(r"1110011011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "rol",      size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}100011[0-1]{3}").unwrap()}, 														
		Inst { name: "rol",      size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}101011[0-1]{3}").unwrap()}, 														
		Inst { name: "rol",      size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  8],	bitp: Regex::new(r"1110[0-1]{3}110011[0-1]{3}").unwrap()}, 														
		Inst { name: "rol",      size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}100111[0-1]{3}").unwrap()}, 														
		Inst { name: "rol",      size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  8],	bitp: Regex::new(r"1110[0-1]{3}101111[0-1]{3}").unwrap()}, 														
		Inst { name: "rol",      size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  8],	bitp: Regex::new(r"1110[0-1]{3}110111[0-1]{3}").unwrap()}, 														
		Inst { name: "rol",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  7],	bitp: Regex::new(r"1110011111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "roxr",     size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}000010[0-1]{3}").unwrap()}, 														
		Inst { name: "roxr",     size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}001010[0-1]{3}").unwrap()}, 														
		Inst { name: "roxr",     size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8, 12],	bitp: Regex::new(r"1110[0-1]{3}010010[0-1]{3}").unwrap()}, 														
		Inst { name: "roxr",     size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}000110[0-1]{3}").unwrap()}, 														
		Inst { name: "roxr",     size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}001110[0-1]{3}").unwrap()}, 														
		Inst { name: "roxr",     size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8, 12],	bitp: Regex::new(r"1110[0-1]{3}010110[0-1]{3}").unwrap()}, 														
		Inst { name: "roxr",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  5],	bitp: Regex::new(r"1110010011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "roxl",     size:  8, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}100010[0-1]{3}").unwrap()}, 														
		Inst { name: "roxl",     size: 16, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}101010[0-1]{3}").unwrap()}, 														
		Inst { name: "roxl",     size: 32, sp: SP::s,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8, 12],	bitp: Regex::new(r"1110[0-1]{3}110010[0-1]{3}").unwrap()}, 														
		Inst { name: "roxl",     size:  8, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}100110[0-1]{3}").unwrap()}, 														
		Inst { name: "roxl",     size: 16, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 6,  6, 12],	bitp: Regex::new(r"1110[0-1]{3}101110[0-1]{3}").unwrap()}, 														
		Inst { name: "roxl",     size: 32, sp: SP::r,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8, 12],	bitp: Regex::new(r"1110[0-1]{3}110110[0-1]{3}").unwrap()}, 														
		Inst { name: "roxl",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  5],	bitp: Regex::new(r"1110010111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "rtd",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 8,  8,  5],	bitp: Regex::new(r"0100111001110100").unwrap()}, 																
		Inst { name: "rte",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 24, 20],	bitp: Regex::new(r"0100111001110011").unwrap()}, 																
		Inst { name: "rtm",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 24, 20],	bitp: Regex::new(r"000001101100[0-1]{4}").unwrap()}, 															
		Inst { name: "rtr",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [20, 20, 14],	bitp: Regex::new(r"0100111001110111").unwrap()}, 																
		Inst { name: "rts",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [16, 16, 10],	bitp: Regex::new(r"0100111001110101").unwrap()}, 																
		Inst { name: "sbcd",     size:  8, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 6,  6,  4],	bitp: Regex::new(r"1000[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::ax7,  	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1000111100001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::ay7,  	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1000(?:111[0-1]{3})100001111").unwrap()}, 													
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::axy7, 	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1000111100001111").unwrap()}, 																
		Inst { name: "sbcd",     size:  8, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [18, 18, 16],	bitp: Regex::new(r"1000[0-1]{3}100001(?:111[0-1]{3})").unwrap()}, 												
		Inst { name: "st",       size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  4,  4],	bitp: Regex::new(r"0101000011000[0-1]{3}").unwrap()}, 															
		Inst { name: "st",       size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  6],	bitp: Regex::new(r"0101000011[0-1]{6}").unwrap()}, 																
		Inst { name: "sf",       size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000111000[0-1]{3}").unwrap()}, 															
		Inst { name: "sf",       size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  6],	bitp: Regex::new(r"0101000111[0-1]{6}").unwrap()}, 																
		Inst { name: "scc",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101[0-1]{4}11000[0-1]{3}").unwrap()}, 														
		Inst { name: "scc",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  6],	bitp: Regex::new(r"0101[0-1]{4}11(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 									
		Inst { name: "stop",     size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  8],	bitp: Regex::new(r"0100111001110010").unwrap()}, 																
		Inst { name: "sub",      size:  8, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}000000[0-1]{3}").unwrap()}, 														
		Inst { name: "sub",      size:  8, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "sub",      size: 16, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}001000[0-1]{3}").unwrap()}, 														
		Inst { name: "sub",      size: 16, sp: SP::er,  se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}001001[0-1]{3}").unwrap()}, 														
		Inst { name: "sub",      size: 16, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "sub",      size: 32, sp: SP::er,  se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}010000[0-1]{3}").unwrap()}, 														
		Inst { name: "sub",      size: 32, sp: SP::er,  se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}010001[0-1]{3}").unwrap()}, 														
		Inst { name: "sub",      size: 32, sp: SP::er,  se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "sub",      size:  8, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1001[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "sub",      size: 16, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"1001[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()},									
		Inst { name: "sub",      size: 32, sp: SP::re,  se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"1001[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "suba",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"1001[0-1]{3}011000[0-1]{3}").unwrap()}, 														
		Inst { name: "suba",     size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"1001[0-1]{3}011001[0-1]{3}").unwrap()}, 														
		Inst { name: "suba",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 8,  8,  2],	bitp: Regex::new(r"1001[0-1]{3}011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "suba",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}111000[0-1]{3}").unwrap()}, 														
		Inst { name: "suba",     size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}111001[0-1]{3}").unwrap()}, 														
		Inst { name: "suba",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWLdxI", cycles: [ 6,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}111(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "subi",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000010000000[0-1]{3}").unwrap()}, 															
		Inst { name: "subi",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000010000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "subi",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0000010001000[0-1]{3}").unwrap()}, 															
		Inst { name: "subi",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0000010001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "subi",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [16, 14,  2],	bitp: Regex::new(r"0000010010000[0-1]{3}").unwrap()}, 															
		Inst { name: "subi",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [20, 20,  4],	bitp: Regex::new(r"0000010010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "subq",     size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0101[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "subq",     size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0101[0-1]{3}100(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "subq",     size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0101[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "subq",     size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 8,  4,  2],	bitp: Regex::new(r"0101[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "subq",     size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 8,  8,  4],	bitp: Regex::new(r"0101[0-1]{3}101(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "subq",     size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0101[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "subq",     size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 8,  8,  2],	bitp: Regex::new(r"0101[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "subq",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [12, 12,  4],	bitp: Regex::new(r"0101[0-1]{3}110(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 								
		Inst { name: "subx",     size:  8, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}100000[0-1]{3}").unwrap()}, 														
		Inst { name: "subx",     size: 16, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"1001[0-1]{3}101000[0-1]{3}").unwrap()}, 														
		Inst { name: "subx",     size: 32, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [ 8,  6,  2],	bitp: Regex::new(r"1001[0-1]{3}110000[0-1]{3}").unwrap()}, 														
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::ax7,  	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1001111100001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::ay7,  	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1001(?:111[0-1]{3})100001111").unwrap()}, 													
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::axy7, 	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1001111100001111").unwrap()}, 																
		Inst { name: "subx",     size:  8, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1001[0-1]{3}100001(?:111[0-1]{3})").unwrap()}, 												
		Inst { name: "subx",     size: 16, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [18, 18, 12],	bitp: Regex::new(r"1001[0-1]{3}101001[0-1]{3}").unwrap()}, 														
		Inst { name: "subx",     size: 32, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [30, 30, 12],	bitp: Regex::new(r"1001[0-1]{3}110001[0-1]{3}").unwrap()}, 														
		Inst { name: "swap",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100100001000[0-1]{3}").unwrap()}, 															
		Inst { name: "tas",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100101011000[0-1]{3}").unwrap()}, 															
		Inst { name: "tas",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [14, 14, 12],	bitp: Regex::new(r"0100101011(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "trap",     size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"010011100100[0-1]{4}").unwrap()}, 															
		Inst { name: "trapt",    size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000011111100").unwrap()}, 																
		Inst { name: "trapt",    size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000011111010").unwrap()}, 																
		Inst { name: "trapt",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000011111011").unwrap()}, 																
		Inst { name: "trapf",    size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000111111100").unwrap()}, 																
		Inst { name: "trapf",    size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000111111010").unwrap()}, 																
		Inst { name: "trapf",    size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101000111111011").unwrap()}, 																
		Inst { name: "trapcc",   size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101[0-1]{4}11111100").unwrap()}, 															
		Inst { name: "trapcc",   size: 16, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101[0-1]{4}11111010").unwrap()}, 															
		Inst { name: "trapcc",   size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0101[0-1]{4}11111011").unwrap()}, 															
		Inst { name: "trapv",    size:  0, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [ 4,  4,  4],	bitp: Regex::new(r"0100111001110110").unwrap()}, 																
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101000000[0-1]{3}").unwrap()}, 															
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101000(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101000111010").unwrap()}, 																
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101000111011").unwrap()}, 																
		Inst { name: "tst",      size:  8, sp: SP::n,   se: SE::i,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101000111100").unwrap()}, 																
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101001000[0-1]{3}").unwrap()}, 															
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101001001[0-1]{3}").unwrap()}, 															
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101001(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101001111010").unwrap()}, 																
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101001111011").unwrap()}, 																
		Inst { name: "tst",      size: 16, sp: SP::n,   se: SE::i,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101001111100").unwrap()}, 																
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::d,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101010000[0-1]{3}").unwrap()}, 															
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::a,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101010001[0-1]{3}").unwrap()}, 															
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::n,    	aea: b"A+-DXWL...", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101010(010|011|100|101|110|111)[0-1]{3}").unwrap()}, 										
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::pcdi, 	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101010111010").unwrap()}, 																
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::pcix, 	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101010111011").unwrap()}, 																
		Inst { name: "tst",      size: 32, sp: SP::n,   se: SE::i,    	aea: b"..........", cycles: [ 4,  4,  2],	bitp: Regex::new(r"0100101010111100").unwrap()}, 																
		Inst { name: "unlk",     size: 32, sp: SP::n,   se: SE::a7,   	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"0100111001011111").unwrap()}, 																
		Inst { name: "unlk",     size: 32, sp: SP::n,   se: SE::n,    	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"0100111001011[0-1]{3}").unwrap()}, 															
		Inst { name: "unpk",     size: 16, sp: SP::rr,  se: SE::n,    	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"1000[0-1]{3}110000...").unwrap()}, 															
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::ax7,  	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"1000111110001(?:111[0-1]{3})").unwrap()}, 													
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::ay7,  	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"1000(?:111[0-1]{3})110001111").unwrap()}, 													
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::axy7, 	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"1000111110001111").unwrap()}, 																
		Inst { name: "unpk",     size: 16, sp: SP::mm,  se: SE::n,    	aea: b"..........", cycles: [12, 12,  6],	bitp: Regex::new(r"1000[0-1]{3}110001(?:111[0-1]{3})").unwrap()}, 												
	];

	// finds doubles - currently 0
	let mut dm = HashMap::new();

    for x in 0..0xffff {
        for i in 0..test.len() {
            let ref t = test[i];
            if t.bitp.is_match(format!("{:b}", x).as_str()) {
                //println!("{:b}, {:?}, {:?}, {:?}, {:?}", x, t.name, t.size, t.sp, t.se, );
				if !dm.contains_key(&x) {
					// add new key and opcode
					dm.insert(x, vec![t]);
				} else {
					// just add opcode to list
					dm.get_mut(&x).unwrap().push(t);
				}
            }
        }
    }

	println!("Total matches: {}", dm.len());

	let mut count = 0;

	for (k, v) in dm.iter() {
		if v.len() > 1 {
			println!("0x{:X}:", k);
			for op in v {
				println!("\t{}: \t{} \t{:?} \t{:?}", op.name, op.size, op.sp, op.se);
			}
			count+=1;
		}
	}

	println!("Total double matches: {}", count);
}