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

    let mut initial_games: Vec<Game> = Vec::with_capacity(n);
    let mut initial_games2: HashSet<usize> = HashSet::new();
    let mut graph: Vec<Vec<Game>> = vec![Vec::with_capacity(m); m];
    let mut input: Vec<i32> = vec![0;m];
    for i in 0..n {
        let row = read_vec::<usize>();
        if row.len() <= 0 {
            continue;
        }
        let mut g_prev = get_game(n, i, row[0]);
        initial_games.push(g_prev);
        initial_games2.insert(g_prev.2);
        for j in 1..row.len() {
            let g = get_game(n, i, row[j]);
            graph[g_prev.2].push(g);
            input[g.2] += 1;
            g_prev = g;
        }
    }

    //println!("IIIIIII {:?}", input);

    let d = solve2(n, m, &graph, &input, &initial_games2);
 
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


fn solve2(n: usize, m: usize, graph: &[Vec<Game>], input: &[i32], ig2: &HashSet<usize>) -> i32 {
    //println!("XXX 000 {}, {}", n, m);
    let mut lp:Vec<(bool, i32)> = vec![(false, 0); m];
    let mut max: i32 = -1;

    let ig3 = ig2.iter().cloned().filter(|v|input[*v] == 0).collect::<Vec<_>>();
    let lpd = dfs4(n, m, graph, &ig3);

    // Num of days = lpd + 1
    let days = if lpd >= 0 {
        lpd + 1
    } else {
        lpd
    };
    return days;
}


fn solve(n: usize, m: usize, graph: &[Vec<Game>], initial_games: &[Game], input: &[i32]) -> i32 {
    //println!("XXX 000 {}, {}", n, m);
    let mut lp:Vec<(bool, i32)> = vec![(false, 0); m];
    let mut max: i32 = -1;
    for g in initial_games {
        //println!("XXXXXX g = {:?}", g);
        if input[g.2] > 0 {
            //println!("XXXXXXXX input > 0!");
            // このcontinueを有効にするとWAになる。
            // 全員 input > 0 のときに注意！
            continue;
        }
        
        if lp[g.2].0 {
            // Already tried.
            continue;
        }
        let d = dfs3(n, m, graph, g);
        //println!("XXXXXX d = {} --> {}", d, g.2);
        
        if d < 0 {
            return d;
        }
        lp[g.2] = (true, d);
        if d > max {
            max = d;
        }
    }
    // Num of nodes = distance + 1
    if max >= 0 {
        max += 1;
    }
    return max;
}



/**
 * Put the longest path distance at each node.
 * Calculate the length of the longest path by DFS.
 * Returns -1 if it contains the cycle.
 */
fn dfs4(n: usize, m: usize, graph: &[Vec<Game>], initial_games: &[usize]) -> i32 {
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

            if graph[iat.0].len() > 0 {
                lpd[iat.0] = graph[iat.0].iter().map(|v|lpd[v.2]).max().unwrap() + 1;
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
        for next in &graph[iat.0] {
            stack.push((next.2, false));
        }
    }
    return initial_games.iter().map(|g| lpd[*g]).max().unwrap();
}






/**
 * Put the longest path distance at each node.
 * Calculate the length of the longest path by DFS.
 * Returns -1 if it contains the cycle.
 */
fn dfs3(n: usize, m: usize, graph: &[Vec<Game>], initial_game: &Game) -> i32 {
    let mut visited: Vec<bool> = vec![false; m];
    let mut finished: Vec<bool> = vec![false; m];

    let mut lpd: Vec<i32> = vec![0; m];
    let mut stack: Vec<IdandTrace> = Vec::with_capacity(m);

    stack.push((initial_game.2, false));
    while !stack.is_empty() {
        let iat = stack.pop().unwrap();
        
        // A. Fin entry
        if iat.1 {
            finished[iat.0] = true;

            if graph[iat.0].len() > 0 {
                lpd[iat.0] = graph[iat.0].iter().map(|v|lpd[v.2]).max().unwrap() + 1;
            }

            continue;
        }

        //println!("\nYYYY iat = {}, d={}", iat.0, d[iat.0]);

        // B. Normal entry
        stack.push((iat.0, true));

        if visited[iat.0] {
            if finished[iat.0] {
                continue;
            } else {
                return -1;
            }
        }

        visited[iat.0] = true;
        
        // B-3 Some children
        for next in &graph[iat.0] {
            stack.push((next.2, false));
        }
    }

    return lpd[initial_game.2];
}









/**
 * Put the longest path distance at each node.
 * Calculate the length of the longest path by DFS.
 * Returns -1 if it contains the cycle.
 */
fn dfs2(n: usize, m: usize, graph: &[Vec<Game>], initial_game: &Game) -> i32 {
    let mut visited: Vec<bool> = vec![false; m];
    let mut finished: Vec<bool> = vec![false; m];

    let mut d: Vec<i32> = vec![0; m];
    let mut lpd: Vec<i32> = vec![0; m];
    let mut stack: Vec<IdandTrace> = Vec::with_capacity(m);

    stack.push((initial_game.2, false));
    let mut max = 0;
    while !stack.is_empty() {
        let iat = stack.pop().unwrap();
        

        // A. Fin entry
        if iat.1 {
            finished[iat.0] = true;

            if graph[iat.0].len() > 0 {
                lpd[iat.0] = graph[iat.0].iter().map(|v|lpd[v.2]).max().unwrap() + 1;
            }

            continue;
        }

        //println!("\nYYYY iat = {}, d={}", iat.0, d[iat.0]);

        // B. Normal entry
        stack.push((iat.0, true));

        if finished[iat.0] {
            continue;
        }

        visited[iat.0] = true;
        

        // B-1 No child
        if graph[iat.0].len() == 0 {
            continue;
        }

        // B-3 Some children
        let new_d = d[iat.0] + 1;
        for next in &graph[iat.0] {
            //println!("YYY {:?}", next);
            if visited[next.2] && !finished[next.2] {
                // Cycle!
                //println!("YYY Cycle!");
                return -1;
            }

            if d[next.2] < new_d {
                //println!("YYY Update to {}", new_d);
                // Update
                d[next.2] = new_d;
                stack.push((next.2, false));
                if new_d > max {
                    max = new_d;
                }
            }
        }
    }
    //return max;
    return lpd[initial_game.2];
}






/**
 * Calculate the length of the longest path by DFS.
 * Returns -1 if it contains the cycle.
 */
fn dfs(n: usize, m: usize, graph: &[Vec<Game>], initial_game: &Game) -> i32 {
    let mut visited: Vec<bool> = vec![false; m];
    let mut finished: Vec<bool> = vec![false; m];

    let mut d: Vec<i32> = vec![0; m];
    let mut stack: Vec<IdandTrace> = Vec::with_capacity(m);

    stack.push((initial_game.2, false));
    let mut max = 0;
    while !stack.is_empty() {
        let iat = stack.pop().unwrap();
        

        // A. Fin entry
        if iat.1 {
            finished[iat.0] = true;
            continue;
        }

        //println!("\nYYYY iat = {}, d={}", iat.0, d[iat.0]);

        // B. Normal entry
        visited[iat.0] = true;
        stack.push((iat.0, true));

        // B-1 No child
        if graph[iat.0].len() == 0 {
            continue;
        }

        // B-3 Some children
        let new_d = d[iat.0] + 1;
        for next in &graph[iat.0] {
            //println!("YYY {:?}", next);
            if visited[next.2] && !finished[next.2] {
                // Cycle!
                //println!("YYY Cycle!");
                return -1;
            }

            if d[next.2] < new_d {
                //println!("YYY Update to {}", new_d);
                // Update
                d[next.2] = new_d;
                stack.push((next.2, false));
                if new_d > max {
                    max = new_d;
                }
            }
        }
    }
    return max;
}
