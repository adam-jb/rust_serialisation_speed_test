use std::time::Instant;
use rand::{thread_rng, seq::SliceRandom};
use smallvec::SmallVec;
use serde::{Deserialize, Serialize};

use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use nanorand::{Rng, WyRand};
use std::fmt;

use self::priority_queue::PriorityQueueItem;

mod priority_queue;


#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct NodeID(u32);

// implement display options for printing during debug
impl fmt::Display for NodeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Cost(u16);

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Edge {
    to: NodeID,
    cost: Cost,
}


#[derive(Serialize, Deserialize, Clone)]
struct Graph {
    edges_per_node: HashMap<usize, SmallVec<[Edge; 4]>>,
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    //convert_input();
    //create_mini_graph();

    test_mini_graph();


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
        edges_per_node: HashMap::new(),
    };
    for from in 1..1_000_000 {
        let mut edges = SmallVec::new();
        for n in 1..3 {
            edges.push(Edge {
                to: NodeID(*nums.choose(&mut rng).unwrap() as u32),
                cost: Cost(*nums.choose(&mut rng).unwrap() as u16),
            });
        }
        let from: usize = from;
        graph.edges_per_node.insert(from, edges);
    }
    println!("Graph population took {:?}", now.elapsed());

    let now = Instant::now();
    let serialized_graph = bincode::serialize(&graph).unwrap();
    println!("Graph Serialisation took {:?}", now.elapsed());

    let now = Instant::now();
    let deserialised_graph: Graph = bincode::deserialize(&serialized_graph).unwrap();
    println!("Graph Deserialisation took {:?}", now.elapsed());
    

}


fn convert_input() {
    println!("Read file");
    let contents = std::fs::read_to_string("walk.txt").unwrap();
    println!("Parse");
    let input: HashMap<String, Vec<[usize; 5]>> = serde_json::from_str(&contents).unwrap();

    println!("Converting into graph");
    let mut graph = Graph {
        edges_per_node: HashMap::new(),
    };
    for (from, input_edges) in input {
        let mut edges = SmallVec::new();
        for array in input_edges {
            edges.push(Edge {
                to: NodeID(array[1] as u32),
                cost: Cost(array[0] as u16),
            });
        }

        let from: usize = from.parse().unwrap();
        graph.edges_per_node.insert(from, edges);
    }

    println!("Saving the graph");
    let file = BufWriter::new(File::create("graph.bin").unwrap());
    bincode::serialize_into(file, &graph).unwrap();
}


fn test_mini_graph() {

    let file = BufReader::new(File::open("sample_graph.bin").unwrap());
    let graph: Graph = bincode::deserialize_from(file).unwrap();

    let file = BufReader::new(File::open("start_node.bin").unwrap());
    let start_node: NodeID = bincode::deserialize_from(file).unwrap();

    let now = Instant::now();
    for _ in 1..1000 {
        let results = floodfill(&graph, start_node);
    }

    println!("Mini network Djikstra benchmark: {:?}", now.elapsed());
    let results = floodfill(&graph, start_node);
    println!("Reached {} nodes", results.len());
}



fn create_mini_graph() {
    println!("Loading graph");
    let now = Instant::now();
    let file = BufReader::new(File::open("graph.bin").unwrap());
    let graph: Graph = bincode::deserialize_from(file).unwrap();
    println!("Loading full graph took {:?}", now.elapsed());

    // Run djikstra once
    let mut rng = WyRand::new();
    let now = Instant::now();
    let start = NodeID(rng.generate_range(0..graph.edges_per_node.len() as u32));
    let results = floodfill(&graph, start);


    let mut sample_graph = Graph {
        edges_per_node: HashMap::new(),
    };

    for (key, value) in results.into_iter() {

        let input_edges = &graph.edges_per_node[&(key.0 as usize)];
        let mut edges = SmallVec::new();

        for edge in input_edges {
            edges.push(*edge);
        }
        
        sample_graph.edges_per_node.insert(key.0 as usize, edges);

    }

    // Serialise and save smaller graph
    let file = BufWriter::new(File::create("sample_graph.bin").unwrap());
    bincode::serialize_into(file, &sample_graph).unwrap();
    println!("Saved mini network of len {}", sample_graph.edges_per_node.len());

    let file = BufWriter::new(File::create("start_node.bin").unwrap());
    bincode::serialize_into(file, &start).unwrap();

}

fn floodfill(graph: &Graph, start: NodeID) -> HashMap<NodeID, Cost> {

    let time_limit = Cost(3600);

    let mut queue: BinaryHeap<PriorityQueueItem<Cost, NodeID>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: Cost(0),
        value: start,
    });

    let mut cost_per_node = HashMap::new();

    while let Some(current) = queue.pop() {
        if cost_per_node.contains_key(&current.value) {
            continue;
        }
        if current.cost > time_limit {
            continue;
        }
        cost_per_node.insert(current.value, current.cost);

        /// got some casting here: could any of it be hurting performance?
        for edge in &graph.edges_per_node[&(current.value.0 as usize)] {
            queue.push(PriorityQueueItem {
                cost: Cost(current.cost.0 + edge.cost.0),
                value: edge.to,
            });
        }
    }

    cost_per_node
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


