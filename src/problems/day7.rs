use std::collections::HashMap;

use crate::DayContext;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::{Dfs, EdgeRef, Walker},
};

type Input = (NodeIndex, DiGraph<(), u64>);

pub fn part_1((shiny_gold, graph): &Input) -> color_eyre::Result<String> {
    let mut graph = graph.clone();
    graph.reverse();
    let containing_count = Dfs::new(&graph, *shiny_gold).iter(&graph).count() - 1;
    Ok(format!(
        "Number of bags that can contain shiny gold: {:?}",
        containing_count
    ))
}

pub fn part_2((shiny_gold, graph): &Input) -> color_eyre::Result<String> {
    let mut amount = HashMap::new();
    let shiny_gold = recurse_amount(&mut amount, *shiny_gold, graph);
    Ok(format!("Shiny gold will contain: {} bags", shiny_gold))
}

fn recurse_amount(
    amounts: &mut HashMap<NodeIndex, u64>,
    node: NodeIndex,
    graph: &DiGraph<(), u64>,
) -> u64 {
    graph.edges(node).fold(0, |total, edge| {
        let amount = match amounts.get(&edge.target()) {
            Some(a) => *a,
            None => {
                let amount = recurse_amount(amounts, edge.target(), graph);
                amounts.insert(edge.target(), amount);
                amount
            }
        };
        total +  (amount + 1) * edge.weight()
    })
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();

    context.accumulate_str_lines(|_, s| {
        let (container, containing) = crate::large_split_str_sep(s, "bags contain")
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not split line {}", s))?;
        if containing != " no other bags." {
            let container = container.trim_end_matches("bags").trim();

            let container_node = match nodes.get(container) {
                Some(id) => *id,
                None => {
                    let node = graph.add_node(());
                    nodes.insert(container.to_owned(), node);
                    node
                }
            };

            //print!("In {} we can have", container);
            for bag in containing.split(",").map(|s| {
                s.trim_end_matches('.')
                    .trim_end_matches("bags")
                    .trim_end_matches("bag")
                    .trim()
            }) {
                let (number, color) = crate::split_string_separator(bag, ' ')
                    .ok_or_else(|| color_eyre::eyre::eyre!("Could not parse bag: {}", bag))?;
                let color = color.trim();
                //print!("{},", color);

                let bag_node = match nodes.get(color) {
                    Some(id) => *id,
                    None => {
                        let node = graph.add_node(());
                        nodes.insert(color.to_owned(), node);
                        node
                    }
                };
                graph.add_edge(container_node, bag_node, number.parse()?);
            }
        }
        //println!();

        Ok(())
    })?;

    let shiny_gold = *nodes
        .get("shiny gold")
        .ok_or_else(|| color_eyre::eyre::eyre!("No shiny gold bag was found"))?;

    Ok((shiny_gold, graph))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(&input, part_1, part_2)
}
