use std::error::Error;
// src/bitcoin/cross_chain/routing.rs

use std::collections::{HashMap, HashSet, VecDeque};
use crate::bitcoin::error::BitcoinError;
use crate::AnyaResult;

pub struct PathFinder {
    // Graph representation: chain -> [(connected_chain, fee_basis_points)]
    chain_graph: HashMap<String, Vec<(String, u32)>>,
}

impl PathFinder {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        let mut chain_graph = HashMap::new();
        
        // Bitcoin connections
        chain_graph.insert("bitcoin".to_string(), vec![
            ("lightning".to_string(), 100),  // 1% fee
            ("liquid".to_string(), 150),     // 1.5% fee
            ("rsk".to_string(), 200),        // 2% fee
            ("stacks".to_string(), 125),     // 1.25% fee
        ]);
        
        // Lightning connections
        chain_graph.insert("lightning".to_string(), vec![
            ("bitcoin".to_string(), 100),    // 1% fee
        ]);
        
        // Liquid connections
        chain_graph.insert("liquid".to_string(), vec![
            ("bitcoin".to_string(), 150),    // 1.5% fee
        ]);
        
        // RSK connections
        chain_graph.insert("rsk".to_string(), vec![
            ("bitcoin".to_string(), 200),    // 2% fee
        ]);
        
        // Stacks connections
        chain_graph.insert("stacks".to_string(), vec![
            ("bitcoin".to_string(), 125),    // 1.25% fee
        ]);
        
        Self { chain_graph }
    }
    
    pub fn find_best_path(&self, from: &str, to: &str) -> AnyaResult<Vec<String>>  -> Result<(), Box<dyn Error>> {
        if from == to {
            return Ok(vec![from.to_string()]);
        }
        
        // Simple BFS to find shortest path
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut prev = HashMap::new();
        
        queue.push_back(from.to_string());
        visited.insert(from.to_string());
        
        while let Some(current) = queue.pop_front() {
            if current == to {
                // Path found, reconstruct it
                let mut path = Vec::new();
                let mut current_node = current;
                
                path.push(current_node.clone());
                while let Some(previous) = prev.get(&current_node) {
                    path.push(previous.clone());
                    current_node = previous.clone();
                }
                
                path.reverse();
                return Ok(path);
            }
            
            if let Some(neighbors) = self.chain_graph.get(&current) {
                for (neighbor, _) in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back(neighbor.clone());
                        visited.insert(neighbor.clone());
                        prev.insert(neighbor.clone(), current.clone());
                    }
                }
            }
        }
        
        // No path found
        Err(BitcoinError::CrossChain(format!(
            "No path found from {} to {}", from, to
        )).into())
    }
    
    pub fn calculate_total_fee(&self, path: &[String]) -> AnyaResult<u32>  -> Result<(), Box<dyn Error>> {
        if path.len() < 2 {
            return Ok(0);
        }
        
        let mut total_fee = 0;
        
        for i in 0..path.len() - 1 {
            let from = &path[i];
            let to = &path[i + 1];
            
            if let Some(neighbors) = self.chain_graph.get(from) {
                if let Some((_, fee)) = neighbors.iter().find(|(chain, _)| chain == to) {
                    total_fee += fee;
                } else {
                    return Err(BitcoinError::CrossChain(format!(
                        "No direct connection from {} to {}", from, to
                    )).into());
                }
            } else {
                return Err(BitcoinError::CrossChain(format!(
                    "Chain {} not found in the graph", from
                )).into());
            }
        }
        
        Ok(total_fee)
    }
} 
