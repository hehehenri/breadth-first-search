use std::{collections::{HashMap, LinkedList}, hash::Hash, fmt::Debug};

pub struct Search<'a, T: Hash + Eq + Debug> {
    pub explored_path: HashMap<&'a T, &'a T>,
    pub target_vertex: &'a T,
}

impl<'a, T: Hash + Eq + Debug> Search<'a, T> {
    pub fn from<F>(graph: &'a HashMap<T, Vec<T>>, initial: &'a T, condition: F) -> Option<Self>
        where F: Fn(&'a T) -> bool,
    {
        let mut queue = LinkedList::from([initial]);
        let mut explored = HashMap::new();

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();

            for adj_vertex in graph.get(vertex).unwrap() {
                if explored.contains_key(adj_vertex) {
                    continue;
                }

                explored.insert(adj_vertex, vertex);

                if condition(adj_vertex) {
                    return Some(Self {
                        explored_path: explored,
                        target_vertex: adj_vertex,
                    });
                }

                queue.push_back(adj_vertex);
            }
        }

        None
    }
}
