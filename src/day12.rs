use common::read_input;
use common::Part;
use std::collections::HashMap;
use petgraph::{Graph, Undirected};
use petgraph::data::FromElements;
use petgraph::algo::{has_path_connecting, min_spanning_tree, condensation, toposort};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node{
    name:i32,
    neighbors:Vec<i32>,
}
pub fn solve(part:Part){
    let input = read_input(12);
    let mut graph= Graph::<i32, (), Undirected>::new_undirected();
    let mut node_map = HashMap::new();
    let mut nodes = Vec::new();
    for line in input.lines(){
        let split:Vec<&str> = line.split("<->").collect();
        let name:i32= split[0].trim().parse().unwrap();
        let neighbors:Vec<i32> = split[1].split(",")
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();
        let index = graph.add_node(name);
        node_map.insert(name, index);
        let node = Node{name, neighbors};
        nodes.push(node);
    }
   // println!("{:?}", nodes);
    for node in nodes.iter(){
        for neighbor in node.neighbors.iter(){
            graph.add_edge(node_map[&node.name], node_map[neighbor], () );
        }
    }
    match part{
        Part::PartOne=>{
            let mut count = 0;
            let target = node_map[&0];
            for node in nodes.iter(){
                if has_path_connecting(&graph, target, node_map[&node.name], None){
                    count +=1;
                }
            }
            println!("Number of nodes reachable from 0 {}", count);
        },
        Part::PartTwo=>{
            let condense = condensation(graph, true);
            let count = condense.node_indices().count();
            //let sorted = toposort(&condense, None).unwrap();
            println!("{}",count);
        }
    }

}