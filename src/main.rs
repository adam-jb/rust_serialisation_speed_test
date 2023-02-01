use std::time::Instant;
use rand::{thread_rng, seq::SliceRandom};
use smallvec::SmallVec;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct NodeID(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Cost(u16);

#[derive(Serialize, Deserialize)]
struct EdgeOriginal {
    to: NodeID,
    cost: Cost,
}

#[derive(Serialize, Deserialize)]
struct Graph {
    edges_per_node: Vec<SmallVec<[EdgeOriginal; 4]>>,
}


fn main() {

    /// 1st test: big loop
    let now = Instant::now();
    let mut i = 0;

    while i < 100_000_000 {
        i = i + 1;
    }
    println!("Loop took {:?}", now.elapsed());


    //// 2nd test: serialisation/deserialisation
    // create big vector of vectors
    let mut rng = thread_rng();
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let random_numbers = (0..1_000_000)
        .map(|_| vec![*nums.choose(&mut rng).unwrap(), *nums.choose(&mut rng).unwrap()])
        .collect::<Vec<_>>();

    let now = Instant::now();
    let serialized_numbers = bincode::serialize(&random_numbers).unwrap();
    println!("Vector Serialisation took {:?}", now.elapsed());

    let now = Instant::now();
    let deserialised_numbers: Vec<Vec<i32>> = bincode::deserialize(&serialized_numbers).unwrap();
    println!("Vector Deserialisation took {:?}", now.elapsed());

    
    // 3rd test: graph serialisation
    // make random graph
    let now = Instant::now();
    let mut graph = Graph {
        edges_per_node: std::iter::repeat_with(SmallVec::new)
            .take(1_000_000)
            .collect(),
    };
     for from in 1..1_000_000 {
        let mut edges = SmallVec::new();
        for n in 1..3 {
            edges.push(EdgeOriginal {
                    to: NodeID(*nums.choose(&mut rng).unwrap() as u32),
                    cost: Cost(*nums.choose(&mut rng).unwrap() as u16),
                });
        }
        graph.edges_per_node[from] = edges;
    }
    println!("Graph population took {:?}", now.elapsed());

    let now = Instant::now();
    let serialized_graph = bincode::serialize(&graph).unwrap();
    println!("Graph Serialisation took {:?}", now.elapsed());

    let now = Instant::now();
    let deserialised_graph: Graph = bincode::deserialize(&serialized_graph).unwrap();
    println!("Graph Deserialisation took {:?}", now.elapsed());

    



}

/*
Local:
Loop took 374.942667ms
Vector Serialisation took 437.238125ms
Vector Deserialisation took 299.177417ms
Graph population took 2.550919625s
Graph Serialisation took 588.363ms
Graph Deserialisation took 470.972417ms


CE:
Loop took 235.249075ms
Vector Serialisation took 572.806317ms
Vector Deserialisation took 355.906657ms
Graph population took 2.178187848s
Graph Serialisation took 797.817181ms
Graph Deserialisation took 679.524387ms
*/


