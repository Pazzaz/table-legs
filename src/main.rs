use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use sprs::CsMat;
use std::process::Command;
use rug::Integer;
use sprs::MulAcc;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;
use ndarray::Array;
use ndarray::linalg::Dot;
use num_traits::identities::Zero;
use std::ops::Add;
use ndarray::Ix1;

type Table = [([bool; 6], usize); 6];


const ZERO_TABLE: Table = [
    ([false, false, false, false, false, false], 0),
    ([false, false, false, false, false, false], 0),
    ([false, false, false, false, false, false], 0),
    ([false, false, false, false, false, false], 0),
    ([false, false, false, false, false, false], 0),
    ([false, false, false, false, false, false], 0),
];


fn main() {
    repeat_mul();
}

fn sort_result() {
    Command::new("sort")
        .args(&["-n", "-o", "sorted_final", "full_final"])
        .status()
        .expect("failed to execute process");
}

#[derive(Debug, Clone)]
struct IntegerE(Integer);

impl Deref for IntegerE {
    type Target = Integer;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for IntegerE { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

impl MulAcc for IntegerE { fn mul_acc(&mut self, a: &Self, b: &Self) { self.0 += &a.0 * &b.0; } }

impl Zero for IntegerE {
    #[inline]
    fn zero() -> IntegerE { IntegerE(Integer::new()) }
    #[inline]
    fn is_zero(&self) -> bool { self.0 == Integer::new() }
}

impl Add for IntegerE {
    type Output = Self;
    fn add(self, other: Self) -> Self { IntegerE(self.0 + other.0) }
}



fn repeat_mul() {
    println!("LOADING DATA");
    let rows    = get_numbers_usize(&"final/row_index");
    let columns = get_numbers_usize(&"final/column_index");
    let data    = get_numbers_integer(&"final/values");

    println!("LOADED!");
    
    let m = CsMat::new((20109024, 20109024), rows, columns, data);
    m.transpose_mut();
    println!("Matrix created!");
    
    let vec1 = get_numbers_integer(&"vectors/0");
    let mut vec = Array::from(vec1);
    for i in 1..1000000 {
        println!("STARTING {}", i);
        vec = Dot::dot(&m, &vec);
        println!("GOTTEM {} : {}", i, vec[448].0);
        let path = format!("vectors/{}", i);
        print_integers(&path, &vec);
    }
}

fn print_integers(path: &str, list: &Array<IntegerE, Ix1>) {
    let v = File::create(path).unwrap();
    let mut f = BufWriter::new(v);
    for n in list {
        writeln!(f, "{}", n.0).unwrap();
    }
}


fn get_numbers_usize(path: &str) -> Vec<usize> {
    let fr = File::open(path).unwrap();
    let br = BufReader::new(fr);
    br.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

fn get_numbers_integer(path: &str) -> Vec<IntegerE> {
    let fr = File::open(path).unwrap();
    let br = BufReader::new(fr);
    br.lines().map(|line| IntegerE(Integer::from_str(&line.unwrap()).unwrap()) ).collect()
}

fn create_start_vector() {
    fs::create_dir_all("./vectors/").unwrap();
    let v = File::create("./vectors/0").unwrap();
    let mut vector = BufWriter::new(v);
    for i in 0usize..20109024 {
        if i == 0 {
            writeln!(vector, "1").unwrap();
        } else {
            writeln!(vector, "0").unwrap();
        }
    }
}

fn convert() {
    if let Ok(lines) = read_lines("./sorted_final") {
        fs::create_dir_all("./final/").unwrap();

        let v = File::create("./final/values").unwrap();
        let mut values = BufWriter::new(v);
        let c = File::create("./final/column_index").unwrap();
        let mut column_index = BufWriter::new(c);
        let r = File::create("./final/row_index").unwrap();
        let mut row_index = BufWriter::new(r);

        let mut got: usize = 0;
        
        for line in lines {
            writeln!(row_index, "{}", got).unwrap();
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split(':').collect();
                let mut things: Vec<(usize, usize)> = parts[1].split("),").filter(|&s| s != "").map(|part| {
                    let coords: Vec<&str> = part.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .collect();
                    let column: usize = coords[0].parse().unwrap();
                    let value: usize = coords[1].parse().unwrap();
                    (column, value)
                }).collect();

                things.sort();

                for (column, value) in things {
                    writeln!(values,       "{}", value).unwrap();
                    writeln!(column_index, "{}", column).unwrap();
                    got += 1;
                }
            }
        }
        writeln!(row_index, "{}", got).unwrap();
    }
}

fn running() {
    let mut tables : HashMap<Table, usize> = HashMap::new();
    let table: Table = [
        ([true,  false, false, false, false, false], 0),
        ([false, true,  false, false, false, false], 1),
        ([false, false, true,  false, false, false], 2),
        ([false, false, false, true,  false, false], 3),
        ([false, false, false, false, true,  false], 4),
        ([false, false, false, false, false, true ], 5),
    ];
    traverse(table, &mut tables);
    eprintln!("DONE!");
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn counting(mut orig: Vec<usize>) -> Vec<(usize, usize)> {
    orig.sort();
    match orig.split_first() {
        Some((head, tail)) => {
            let mut curr: usize = *head;
            let mut curr_count: usize = 1;
            let mut acc = Vec::new();
            for &n in tail {
                if n == curr {
                    curr_count += 1;
                } else {
                    acc.push((curr, curr_count));
                    curr = n;
                    curr_count = 1;
                }
            }
            acc.push((curr, curr_count));
            acc
        },
        None => Vec::new()
    }
}

fn traverse(current: Table, seen: &mut HashMap<Table, usize>) {
    let f = File::create("full_final").unwrap();
    let mut file = BufWriter::new(f);
    seen.insert(current, 0);
    let mut unchecked = vec![current];
    let mut last = 0;
    loop {
        let n = match unchecked.pop() {
            Some(k) => k,
            None    => return,
        };
        let mut around_n: Vec<usize> = Vec::new();
        let bors = neighbours(n);
        'outer: for mut b in bors {
            b.sort();
            for mut symm in symmetries(b) {
                symm.sort();
                if let Some(ind) = seen.get(&symm) {
                    around_n.push(*ind);
                    continue 'outer;
                }
            }
            around_n.push(seen.len());
            seen.insert(b, seen.len());
            unchecked.push(b);
        }

        let ind = seen.get(&n).unwrap();
        let sorted = counting(around_n);
        write!(file, "{}:", ind).unwrap();
        for (n, count) in sorted {
            write!(file, "({},{}),", n, count).unwrap();
        }
        writeln!(file).unwrap();

        let le = seen.len();
        if le != last {
            eprintln!("seen: {}; unchecked: {}", le, unchecked.len());
            last = le;
        }
    }
}

fn neighbours(table: Table) -> [Table; 144] {
    let mut tables = [ZERO_TABLE; 144];
    let mut p = [0; 6];

    // p represents which leg is at that position right now.
    for i in 0..6 {
        p[table[i].1] = i;
    }
    // The top middle one go to the left
    let l1m1  = [p[0], p[3], p[1]];
    let l1m2  = [p[1], p[3], p[0]];
    let l1m3  = [p[0], p[1], p[3]];
    let l1m4  = [p[1], p[0], p[3]];
    let l1m5  = [p[3], p[0], p[1]];
    let l1m6  = [p[3], p[1], p[0]];


    let r1m1  = [p[2], p[5], p[4]];
    let r1m2  = [p[4], p[5], p[2]];
    let r1m3  = [p[2], p[4], p[5]];
    let r1m4  = [p[4], p[2], p[5]];
    let r1m5  = [p[5], p[2], p[4]];
    let r1m6  = [p[5], p[4], p[2]];

    let mut i = 0;
    for l in [l1m1, l1m2, l1m3, l1m4, l1m5, l1m6] {
        for r in [r1m1, r1m2, r1m3, r1m4, r1m5, r1m6] {
            let table1 = [
                l[0],l[1],r[0],
                l[2],r[1],r[2],
            ];
            let table2 = [
                l[0],r[1],r[0],
                l[2],l[1],r[2],
            ];
            let t1 = transform(table.clone(), table1);
            tables[i] = t1;
            i += 1;
            let t2 = transform(table.clone(), table2);
            tables[i] = t2;
            i += 1;
        }
    }
    
    // The top middle one go to the right
    let l2m1  = [p[0], p[3], p[4]];
    let l2m2  = [p[4], p[3], p[0]];
    let l2m3  = [p[0], p[4], p[3]];
    let l2m4  = [p[4], p[0], p[3]];
    let l2m5  = [p[3], p[0], p[4]];
    let l2m6  = [p[3], p[4], p[0]];


    let r2m1  = [p[2], p[5], p[1]];
    let r2m2  = [p[1], p[5], p[2]];
    let r2m3  = [p[2], p[1], p[5]];
    let r2m4  = [p[1], p[2], p[5]];
    let r2m5  = [p[5], p[2], p[1]];
    let r2m6  = [p[5], p[1], p[2]];

    for l in [l2m1, l2m2, l2m3, l2m4, l2m5, l2m6] {
        for r in [r2m1, r2m2, r2m3, r2m4, r2m5, r2m6] {
            let table1 = [
                l[0],l[1],r[0],
                l[2],r[1],r[2],
            ];
            let table2 = [
                l[0],r[1],r[0],
                l[2],l[1],r[2],
            ];
            let t1 = transform(table.clone(), table1);
            tables[i] = t1;
            i += 1;
            let t2 = transform(table.clone(), table2);
            tables[i] = t2;
            i += 1;
        }
    }
    assert!(i == 144);
    return tables;
}

fn transform(mut table: Table, new_pos: [usize; 6]) ->  Table {
    for i in 0..6 {
        table[new_pos[i]].1    = i;
        table[new_pos[i]].0[i] = true;
    }
    return table
}

// The Automorphism group of the (2,3) King Graph is `PermutationGroup[{Cycles[{{2,5}}],Cycles[{{3,6}}],Cycles[{{1,3},{4,6}}]}]`
// The order is 16
// This includes the identity
fn symmetries(table: Table) ->  [Table; 16] {
    let permutations:  [[usize; 6]; 16] = [
        [ 0,1,2,
          3,4,5 ], // 1
        [ 3,1,2,
          0,4,5 ], // 2
        [ 0,4,2,
          3,1,5 ], // 3
        [ 0,1,5,
          3,4,2 ], // 4
        [ 3,4,2,
          0,1,5 ], // 5
        [ 3,1,5,
          0,4,2 ], // 6
        [ 3,4,5,
          0,1,2 ], // 7
        [ 2,1,0,
          5,4,3 ], // 8
        [ 5,1,0,
          2,4,3 ], // 9
        [ 2,4,0,
          5,1,3 ], // 10
        [ 2,1,3,
          5,4,0 ], // 11
        [ 5,4,0,
          2,1,3 ], // 12
        [ 5,1,3,
          2,4,0 ], // 13
        [ 5,4,3,
          2,1,0 ], // 14
        [ 0,4,5,
          3,1,2 ], // 15
        [ 2,4,3,
          5,1,0 ], // 16
    ];
    let mut symmetries = [ZERO_TABLE; 16];
    for i in 0..16 {
        let perm = permutations[i];
        let mut tab = table.clone();
        for i in 0..6 {
            tab[i].0 = [table[i].0[perm[0]], table[i].0[perm[1]], table[i].0[perm[2]], table[i].0[perm[3]], table[i].0[perm[4]], table[i].0[perm[5]]];
            if perm[0] == tab[i].1 { tab[i].1 = 0; } else
            if perm[1] == tab[i].1 { tab[i].1 = 1; } else
            if perm[2] == tab[i].1 { tab[i].1 = 2; } else
            if perm[3] == tab[i].1 { tab[i].1 = 3; } else
            if perm[4] == tab[i].1 { tab[i].1 = 4; } else
            if perm[5] == tab[i].1 { tab[i].1 = 5; } else { panic!("WHAT") }
        }
        symmetries[i] = tab;
    }
    return symmetries;
}
