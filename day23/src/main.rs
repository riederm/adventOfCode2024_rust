use std::collections::HashSet;

use multimap::MultiMap;

fn main() {
    let edges = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut graph = MultiMap::new();

    for (from, to) in &edges {
        graph.insert(*from, *to);
        graph.insert(*to, *from);
    }

    let edges: HashSet<(&str, &str)> = HashSet::from_iter(edges.into_iter());

    let mut groups = HashSet::new();
    let mut current_group = Vec::new();
    for &start in graph.keys() {
        current_group.push(start);
        find_circles_of_size(3, &graph, &edges, &mut groups, &mut current_group);
        current_group.pop();
    }

    let filtered_groups = groups
        .iter()
        .filter(|g| g.iter().any(|n| n.starts_with("t")))
        .collect::<Vec<_>>();
    println!("Groups: {}", filtered_groups.len());

    // part 2
    let mut biggest_group = Vec::new();
    let mut visited = HashSet::new();
    for &start in graph.keys() {
        current_group.push(start);
        find_biggest_possible_circle(
            &graph,
            &edges,
            &mut current_group,
            &mut biggest_group,
            &mut visited,
        );
        current_group.pop();
    }

    println!("Biggest group: {:?}", biggest_group.len());
    biggest_group.sort();
    println!("{:}", biggest_group.join(","));
}

fn find_circles_of_size<'a>(
    desired_len: usize,
    graph: &MultiMap<&'a str, &'a str>,
    edges: &HashSet<(&str, &str)>,
    groups: &mut HashSet<Vec<&'a str>>,
    current_group: &mut Vec<&'a str>,
) {
    if current_group.len() == desired_len {
        let mut g = current_group.clone();
        g.sort();
        groups.insert(g);
        return;
    }

    let &last_node = current_group.last().unwrap();
    if let Some(e) = graph.get_vec(last_node) {
        for edge in e {
            if current_group
                .iter()
                .all(|it| edges.contains(&(*it, edge)) || edges.contains(&(edge, *it)))
            {
                current_group.push(edge);
                find_circles_of_size(desired_len, graph, edges, groups, current_group);
                current_group.pop();
            }
        }
    }
}

fn find_biggest_possible_circle<'a>(
    graph: &MultiMap<&'a str, &'a str>,
    edges: &HashSet<(&str, &str)>,
    current_group: &mut Vec<&'a str>,
    biggest_group: &mut Vec<&'a str>,
    visited: &mut HashSet<Vec<&'a str>>,
) {
    if current_group.len() > biggest_group.len() {
        biggest_group.clear();
        biggest_group.extend_from_slice(&current_group);
    }

    let &last_node = current_group.last().unwrap();
    if let Some(e) = graph.get_vec(last_node) {
        for &edge in e {
            if !current_group.contains(&edge) {
                // check if new node is connected to every node in the group
                if current_group
                    .iter()
                    .all(|&n| edges.contains(&(n, edge)) || edges.contains(&(edge, n)))
                {
                    current_group.push(edge);

                    let mut key = current_group.clone();
                    key.sort();
                    if visited.insert(key) {
                        find_biggest_possible_circle(
                            graph,
                            edges,
                            current_group,
                            biggest_group,
                            visited,
                        );
                    }
                    current_group.pop();
                }
            }
        }
    }
}
