use std::{collections::{HashMap, LinkedList}, hash::Hash, fmt::Debug};

pub struct Search<'a, T: Hash + Eq + Debug> {
    pub explored_path: HashMap<&'a T, &'a T>,
    pub target_vertex: &'a T,
    initial_vertex: &'a T,
}

impl<'a, T: Hash + Eq + Debug> Search<'a, T> {
    pub fn from<F>(graph: &'a HashMap<T, Vec<T>>, initial_vertex: &'a T, condition: F) -> Option<Self>
        where F: Fn(&'a T) -> bool,
    {
        let mut queue = LinkedList::from([initial_vertex]);
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
                        initial_vertex
                    });
                }

                queue.push_back(adj_vertex);
            }
        }

        None
    }

    pub fn path(&self) -> LinkedList<&T> {
        let mut path = LinkedList::new();

        let mut next_key = self.target_vertex;

        while next_key != self.initial_vertex {
            path.push_front(next_key);

            next_key = self.explored_path.get(next_key).unwrap();
        }

        path.push_front(self.initial_vertex);

        path
    }
}
