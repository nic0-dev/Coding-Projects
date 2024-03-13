fn main() {
    let num_nodes = 5;
    let edge_list = vec![
        (0, 1),
        (0, 2),
        (1, 2),
        (1, 3),
        (2, 4),
        (3, 4),
        (1, 4),
        (2, 3),
    ];
    
    let is_bidi = true;

    // Build adjlist
    let mut adjlist: Vec <Vec <u64> > = vec![];
    
    for _ in 0..num_nodes {
        adjlist.push(vec![]);
    }

    for (a, b) in edge_list {
        if let Some(x) = adjlist.get_mut(a as usize) {
            x.push(b);
        }
        
        if is_bidi {
            if let Some(x) = adjlist.get_mut(b as usize) {
                x.push(a);
            }
        }
    }

    // Traverse adjlist via bfs
    let orig_node = 0;

    let mut q = vec![(orig_node, 0)];
    let mut visited_dist = vec![-1; num_nodes];
    visited_dist[orig_node] = 0;
    
    while let Some((top_node, top_node_dist)) = q.pop() {
        if let Some(nbrs_vec) = adjlist.get(top_node) {
            for each_nbr in nbrs_vec {
                if visited_dist[(*each_nbr) as usize] >= 0 {
                    continue;
                }
    
                q.push(((*each_nbr) as usize, top_node_dist + 1));
    
                visited_dist[(*each_nbr) as usize] = top_node_dist + 1;
            }
        }
    }

    // Get distance of each node from origin
    for each_node in 0..num_nodes {
        println!("Node {} -> {} has dist {}", orig_node, each_node, visited_dist[each_node]);
    }
}