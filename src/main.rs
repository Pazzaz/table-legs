use std::collections::HashMap;

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
        print!("{}:", ind);
        for (n, count) in sorted {
            print!("({},{}),", n, count);
        }
        println!();

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
