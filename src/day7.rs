use common::read_input;
use common::Part;

use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;

use petgraph::Graph;
use petgraph::algo::toposort;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node{
    weight:u32,
    total:u32,
    name:String,
    children:Vec<String>,
}

pub fn solve(part:Part){
    let input = read_input(7);
    let rows:Vec<String> = input.lines()
        .map(|s| s.replace("->", ""))
        .map(|s| s.replace(",", ""))
        .map(|s| s.replace("(", ""))
        .map(|s| s.replace(")",""))
        .collect();
    let mut nodes:Vec<Node> = Vec::new();
    let mut node_map = HashMap::new();
    let mut graph = Graph::<Node, ()>::new();
    for row in rows{
        let mut element_it = row.split_whitespace();
        let name = element_it.next().unwrap().to_string();
        let weight = element_it.next().unwrap().parse().unwrap();
        let mut children:Vec<String> = Vec::new();
        for token in element_it{
            children.push(token.to_string());
        }
        let node = Node{weight, name:name.clone(), children, total:weight};
        let index = graph.add_node(node.clone());
        node_map.insert(name.clone(), index);
        nodes.push(node.clone());
    }
    for node in nodes.iter(){
        for child in node.children.iter(){
            graph.add_edge(node_map[&node.name], node_map[child], ());
        }
    }
    let sorted = toposort(&graph, None).unwrap();


    match part{
        Part::PartOne=>{
            println!("root {}", graph[sorted[0]].name);
        },
        Part::PartTwo=>{
            for &node in sorted.iter().rev() {
                if !graph.neighbors(node).map(|n| graph[n].total).all_equal() {
                    let (min, max) = graph.neighbors(node)
                        .map(|n| graph[n].total)
                        .minmax().into_option().unwrap();
                    let (left, right): (Vec<_>, Vec<_>) = graph.neighbors(node)
                        .partition(|&n| graph[n].total == min);
                    let unbalanced = if left.len() == 1 {
                        &graph[left[0]]
                    } else {
                        &graph[right[0]]
                    };
                    println!("New weight (for \"{}\") is: {}", unbalanced.name, unbalanced.weight + min - max);
                    break;
                }

                // Otherwise, update this node's total weight
                graph[node].total += graph.neighbors(node).map(|n| graph[n].total).sum::<u32>();
            }
        }
    }






    //println!("{}", nodes.len());
}
