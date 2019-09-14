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


type GameEntry = (usize, bool); // id, forFinished

fn main() {
    let n = read::<usize>();
    let m = n * (n - 1); // Num of Games

    let mut init_games: HashSet<usize> = HashSet::new();
    let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(m); m];
    let mut input: Vec<i32> = vec![0;m];
    for i in 0..n {
        let row = read_vec::<usize>();
        if row.len() <= 0 {
            continue;
        }
        let mut g_prev = get_game_idx(n, i, row[0]);
        init_games.insert(g_prev);
        for j in 1..row.len() {
            let g = get_game_idx(n, i, row[j]);
            graph[g_prev].push(g);
            input[g] += 1;
            g_prev = g;
        }
    }

    //println!("IIIIIII {:?}", input);

    let d = solve(m, &graph, &init_games, &input);
 
    println!("{}", d);
}


fn get_game_idx(n: usize, i: usize, j: usize) -> usize {
    let mut p1 = i;
    let mut p2 = j - 1;
    if p1 > p2 {
        let tmp = p1;
        p1 = p2;
        p2 = tmp;
    }
    return p1 * n + p2;
}


fn solve(m: usize, graph2: &[Vec<usize>], init_games: &HashSet<usize>, input: &[i32],) -> i32 {
    //println!("XXX 000 {}, {}", n, m);
    // Only games without input can be start points.
    let no_input = init_games.iter().cloned().filter(|v|input[*v] == 0).collect::<Vec<_>>();
    let lpd = dfs(m, graph2, &no_input);

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
fn dfs(m: usize, graph: &[Vec<usize>], init_games: &[usize]) -> i32 {
    // println!("dfs000 init={:?}", init_games);
    let mut visited: Vec<bool> = vec![false; m];
    let mut finished: Vec<bool> = vec![false; m];

    let mut lpd: Vec<i32> = vec![0; m];
    let mut stack: Vec<GameEntry> = Vec::with_capacity(m);

    for g in init_games {
        stack.push((*g, false));
    }
    
    while !stack.is_empty() {
        let entry = stack.pop().unwrap();

        // println!("L000 entry={:?}", entry);
        
        // A. Fin entry
        if entry.1 {
            finished[entry.0] = true;

            if graph[entry.0].len() > 0 {
                lpd[entry.0] = graph[entry.0].iter().map(|v|lpd[*v]).max().unwrap() + 1;
            }

            continue;
        }

        //println!("\nYYYY iat = {}, d={}", iat.0, d[iat.0]);

        // B. Normal entry
        stack.push((entry.0, true));

        if visited[entry.0] {
            if finished[entry.0] {
                // Already visited
                continue;
            } else {
                // Cycle
                return -1;
            }
        }

        visited[entry.0] = true;
        
        // B-3 Some children
        for next in &graph[entry.0] {
            stack.push((*next, false));
        }
    }
    return init_games.iter().map(|g| lpd[*g]).max().unwrap_or(-1);
}
