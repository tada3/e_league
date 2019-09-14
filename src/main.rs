use std::collections::HashSet;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

type Game = (usize, usize, usize); // p1, p2, id
type IdandTrace = (usize, bool); // id, forFinished

fn main() {
    let n = read::<usize>();
    let m = n * (n - 1); // Num of Games

    let mut init_games: HashSet<usize> = HashSet::new();
    let mut graph: Vec<Vec<Game>> = vec![Vec::with_capacity(m); m];
    let mut graph2: Vec<Vec<usize>> = vec![Vec::with_capacity(m); m];
    let mut input: Vec<i32> = vec![0;m];
    for i in 0..n {
        let row = read_vec::<usize>();
        if row.len() <= 0 {
            continue;
        }
        let mut g_prev = get_game(n, i, row[0]);
        init_games.insert(g_prev.2);
        for j in 1..row.len() {
            let g = get_game(n, i, row[j]);
            graph[g_prev.2].push(g);
            graph2[g_prev.2].push(g.2);
            input[g.2] += 1;
            g_prev = g;
        }
    }

    //println!("IIIIIII {:?}", input);

    let d = solve(m, &graph2, &graph,&init_games, &input);
 
    println!("{}", d);
}

fn get_game(n: usize, i: usize, j: usize) -> Game {
    let mut p1 = i;
    let mut p2 = j - 1;
    if p1 > p2 {
        let tmp = p1;
        p1 = p2;
        p2 = tmp;
    }
    return (p1, p2, p1 * n + p2);
}


fn solve(m: usize, graph2: &[Vec<usize>], graph: &[Vec<Game>], init_games: &HashSet<usize>, input: &[i32],) -> i32 {
    //println!("XXX 000 {}, {}", n, m);
    // Only games without input can be start points.
    let ig3 = init_games.iter().cloned().filter(|v|input[*v] == 0).collect::<Vec<_>>();
    let lpd = dfs(m, graph2, graph, &ig3);

    // Num of days = lpd + 1
    let days = if lpd >= 0 {
        lpd + 1
    } else {
        lpd
    };
    return days;
}

/**
 * Put the longest path distance at each node.
 * Calculate the length of the longest path by DFS.
 * Returns -1 if it contains the cycle.
 */
fn dfs(m: usize, graph2: &[Vec<usize>], graph: &[Vec<Game>], initial_games: &[usize]) -> i32 {
    let mut visited: Vec<bool> = vec![false; m];
    let mut finished: Vec<bool> = vec![false; m];

    let mut lpd: Vec<i32> = vec![0; m];
    let mut stack: Vec<IdandTrace> = Vec::with_capacity(m);

    initial_games.iter().for_each(|g| stack.push((*g, false)));
    while !stack.is_empty() {
        let iat = stack.pop().unwrap();
        
        // A. Fin entry
        if iat.1 {
            finished[iat.0] = true;

            if graph2[iat.0].len() > 0 {
                lpd[iat.0] = graph2[iat.0].iter().map(|v|lpd[*v]).max().unwrap() + 1;
            }

            continue;
        }

        //println!("\nYYYY iat = {}, d={}", iat.0, d[iat.0]);

        // B. Normal entry
        stack.push((iat.0, true));

        if visited[iat.0] {
            if finished[iat.0] {
                // Already visited
                continue;
            } else {
                // Cycle
                return -1;
            }
        }

        visited[iat.0] = true;
        
        // B-3 Some children
        for next in &graph2[iat.0] {
            stack.push((*next, false));
        }
    }
    return initial_games.iter().map(|g| lpd[*g]).max().unwrap();
}
