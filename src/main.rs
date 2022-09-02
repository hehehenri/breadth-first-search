use std::collections::HashMap;

use breadth_first::Search;

pub fn main() {
    let mut graph = HashMap::new();

    graph.insert("you", Vec::from(["bob", "claire", "alice"]));
    graph.insert("alice", Vec::from(["peggy"]));
    graph.insert("peggy", Vec::<&str>::new());
    graph.insert("bob", Vec::from(["anuj"]));
    graph.insert("claire", Vec::from(["thom", "jonny"]));
    graph.insert("anuj", Vec::<&str>::new());

    let result = Search::from(&graph, &"you", |person| {
        person.ends_with('m')
    });

    match result {
        Some(search_result) => println!("Explored path: {:#?}", search_result.explored_path),
        None => println!("Person not found :(")
    }
}
