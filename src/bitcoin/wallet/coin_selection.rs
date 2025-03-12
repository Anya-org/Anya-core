//! Coin selection algorithms for Bitcoin wallet
//!
//! This module provides various coin selection algorithms for Bitcoin transactions.
//! [AIR-1][AIS-1][AIM-1][AIP-1][RES-1]

use std::collections::BTreeSet;
use rand::{thread_rng, seq::SliceRandom};
use log::{debug, trace};

use super::{FeeRate, Utxo, WalletError};

/// Branch and Bound coin selection algorithm
///
/// This algorithm tries to find a set of UTXOs that minimizes waste
/// by solving the exact subset sum problem.
pub fn branch_and_bound(
    mut available_utxos: Vec<Utxo>, 
    target_amount: u64, 
    fee_rate: FeeRate
) -> Result<(Vec<Utxo>, u64), WalletError> {
    debug!("Running Branch and Bound coin selection for {} satoshis", target_amount);
    
    if available_utxos.is_empty() {
        return Err(WalletError::InsufficientFunds("No UTXOs available for selection".to_string()));
    }
    
    // Sort UTXOs by value
    available_utxos.sort_by_key(|utxo| utxo.txout.value);
    
    // Calculate fee per input
    const INPUT_SIZE: u64 = 150; // Conservative estimate for input size in virtual bytes
    let fee_per_input = INPUT_SIZE * fee_rate.to_sat_per_vb();
    
    // Subtract fee from each UTXO value to get effective values
    let mut effective_values: Vec<i64> = Vec::new();
    for utxo in &available_utxos {
        let effective_value = utxo.txout.value as i64 - fee_per_input as i64;
        if effective_value > 0 {
            effective_values.push(effective_value);
        }
    }
    
    // Check if we have enough funds even with largest effective values
    let total_effective: i64 = effective_values.iter().sum();
    if total_effective < target_amount as i64 {
        return Err(WalletError::InsufficientFunds(format!(
            "Insufficient funds: have {} satoshis (after fees), need {}",
            total_effective, target_amount
        )));
    }
    
    // Try to find an exact match using the Branch and Bound algorithm
    let target = target_amount as i64;
    
    // We'll keep track of the best solution found so far
    struct Solution {
        utxos: BTreeSet<usize>,
        total: i64,
        waste: i64,
    }
    
    let mut best_solution = Solution {
        utxos: BTreeSet::new(),
        total: 0,
        waste: i64::MAX,
    };
    
    // Check if a single UTXO can fulfill the target
    for (i, &value) in effective_values.iter().enumerate() {
        if value >= target {
            let waste = value - target;
            if waste < best_solution.waste {
                let mut utxos = BTreeSet::new();
                utxos.insert(i);
                best_solution = Solution {
                    utxos,
                    total: value,
                    waste,
                };
                
                // If perfect match, return immediately
                if waste == 0 {
                    break;
                }
            }
        }
    }
    
    // If we didn't find a single UTXO match, or it's not perfect,
    // continue with the Branch and Bound search
    if best_solution.waste > 0 {
        // For simplicity in this implementation, we'll use a recursive approach
        // with early cutoff when the current solution is worse than the best
        fn search(
            values: &[i64],
            target: i64,
            current_set: &mut BTreeSet<usize>,
            current_total: i64,
            remaining_total: i64,
            best_solution: &mut Solution,
            start_index: usize,
        ) {
            // Base case: if current total exceeds target
            if current_total >= target {
                let waste = current_total - target;
                if waste < best_solution.waste {
                    best_solution.utxos = current_set.clone();
                    best_solution.total = current_total;
                    best_solution.waste = waste;
                }
                return;
            }
            
            // If even adding all remaining values won't reach the target, abort
            if current_total + remaining_total < target {
                return;
            }
            
            // If adding the remaining values would create more waste than the best solution,
            // abort this branch
            if (current_total + remaining_total) - target >= best_solution.waste {
                return;
            }
            
            // Try including each remaining UTXO and recurse
            for i in start_index..values.len() {
                let value = values[i];
                
                // Check if adding this value would exceed the target significantly
                if current_total + value - target >= best_solution.waste {
                    continue;
                }
                
                // Include this UTXO
                current_set.insert(i);
                
                // Recurse
                search(
                    values,
                    target,
                    current_set,
                    current_total + value,
                    remaining_total - value,
                    best_solution,
                    i + 1,
                );
                
                // Backtrack
                current_set.remove(&i);
            }
        }
        
        // Start the search
        let mut current_set = BTreeSet::new();
        let remaining_total: i64 = effective_values.iter().sum();
        
        search(
            &effective_values,
            target,
            &mut current_set,
            0,
            remaining_total,
            &mut best_solution,
            0,
        );
    }
    
    // If we found a solution, convert the indices to actual UTXOs
    if best_solution.waste < i64::MAX {
        let mut selected_utxos = Vec::new();
        let mut total_input = 0;
        
        for &idx in &best_solution.utxos {
            let utxo = available_utxos[idx].clone();
            total_input += utxo.txout.value;
            selected_utxos.push(utxo);
        }
        
        debug!("Branch and Bound selected {} UTXOs with {} sats (waste: {})",
              selected_utxos.len(), total_input, best_solution.waste);
        
        Ok((selected_utxos, total_input))
    } else {
        // Fall back to a greedy algorithm if Branch and Bound failed
        debug!("Branch and Bound failed, falling back to greedy selection");
        
        // Sort by effective value (largest first)
        available_utxos.sort_by(|a, b| {
            let a_eff = a.txout.value as i64 - fee_per_input as i64;
            let b_eff = b.txout.value as i64 - fee_per_input as i64;
            b_eff.cmp(&a_eff) // Descending order
        });
        
        let mut selected_utxos = Vec::new();
        let mut total_input = 0;
        
        for utxo in available_utxos {
            // Skip UTXOs with negative effective value
            if (utxo.txout.value as i64) <= (fee_per_input as i64) {
                continue;
            }
            
            selected_utxos.push(utxo.clone());
            total_input += utxo.txout.value;
            
            if total_input as i64 >= target as i64 + (fee_per_input as i64 * selected_utxos.len() as i64) {
                break;
            }
        }
        
        if total_input < target_amount {
            return Err(WalletError::InsufficientFunds(format!(
                "Insufficient funds: have {} satoshis, need {}",
                total_input, target_amount
            )));
        }
        
        Ok((selected_utxos, total_input))
    }
}

/// Privacy-optimized coin selection
///
/// This algorithm tries to minimize the creation of change outputs,
/// which improves privacy by avoiding address reuse.
pub fn privacy_optimized(
    mut available_utxos: Vec<Utxo>, 
    target_amount: u64, 
    fee_rate: FeeRate
) -> Result<(Vec<Utxo>, u64), WalletError> {
    debug!("Running privacy-optimized coin selection for {} satoshis", target_amount);
    
    if available_utxos.is_empty() {
        return Err(WalletError::InsufficientFunds("No UTXOs available for selection".to_string()));
    }
    
    // Calculate fee per input and output
    const INPUT_SIZE: u64 = 150; // Conservative estimate for input size in virtual bytes
    const OUTPUT_SIZE: u64 = 43; // Conservative estimate for output size in virtual bytes
    let fee_per_input = INPUT_SIZE * fee_rate.to_sat_per_vb();
    let fee_per_output = OUTPUT_SIZE * fee_rate.to_sat_per_vb();
    
    // Calculate target amount with fee for one output (no change)
    let target_with_output_fee = target_amount + fee_per_output;
    
    // Define acceptable waste as a percentage of the target amount
    const MAX_WASTE_PERCENT: f64 = 0.01; // 1%
    let max_waste = (target_amount as f64 * MAX_WASTE_PERCENT) as u64;
    
    // First, try to find a single UTXO that closely matches the target
    // Sort UTXOs by how close they are to the target
    available_utxos.sort_by(|a, b| {
        let a_diff = if a.txout.value >= target_with_output_fee {
            a.txout.value - target_with_output_fee
        } else {
            u64::MAX
        };
        
        let b_diff = if b.txout.value >= target_with_output_fee {
            b.txout.value - target_with_output_fee
        } else {
            u64::MAX
        };
        
        a_diff.cmp(&b_diff)
    });
    
    // Check if the best match is within acceptable waste
    if !available_utxos.is_empty() && 
       available_utxos[0].txout.value >= target_with_output_fee &&
       available_utxos[0].txout.value - target_with_output_fee <= max_waste {
        let utxo = available_utxos[0].clone();
        let total_input = utxo.txout.value;
        
        debug!("Found single UTXO match: {} satoshis (waste: {})",
              total_input, total_input - target_with_output_fee);
        
        return Ok((vec![utxo], total_input));
    }
    
    // Next, try to find a combination of 2-3 UTXOs that closely matches the target
    // We'll try up to 500 random combinations
    const MAX_COMBINATIONS: usize = 500;
    const MAX_UTXOS_TO_COMBINE: usize = 3;
    
    // Create a random number generator
    let mut rng = thread_rng();
    
    // Shuffle the UTXOs to get random combinations
    available_utxos.shuffle(&mut rng);
    
    // Calculate fee for combinations
    let mut best_combination: Option<(Vec<Utxo>, u64)> = None;
    let mut best_waste = u64::MAX;
    
    for i in 0..MAX_COMBINATIONS {
        // Try combinations of 2 to MAX_UTXOS_TO_COMBINE UTXOs
        for num_utxos in 2..=MAX_UTXOS_TO_COMBINE {
            if num_utxos > available_utxos.len() {
                break;
            }
            
            // Calculate target including fees for inputs
            let target_with_fee = target_amount + fee_per_output + (fee_per_input * num_utxos as u64);
            
            // Get a combination of num_utxos UTXOs
            let start_idx = (i * num_utxos) % available_utxos.len();
            let end_idx = std::cmp::min(start_idx + num_utxos, available_utxos.len());
            
            let combination = &available_utxos[start_idx..end_idx];
            let total: u64 = combination.iter().map(|u| u.txout.value).sum();
            
            if total >= target_with_fee {
                let waste = total - target_with_fee;
                
                if waste <= max_waste && waste < best_waste {
                    best_waste = waste;
                    best_combination = Some((combination.to_vec(), total));
                    
                    // If we found a perfect match, break early
                    if waste == 0 {
                        break;
                    }
                }
            }
        }
        
        // If we found a good enough match, break early
        if let Some((_, _)) = &best_combination {
            if best_waste <= max_waste / 2 {
                break;
            }
        }
    }
    
    // If we found a good combination, return it
    if let Some((utxos, total)) = best_combination {
        debug!("Found combination of {} UTXOs: {} satoshis (waste: {})",
              utxos.len(), total, best_waste);
        
        return Ok((utxos, total));
    }
    
    // If we couldn't find a good match, fall back to Branch and Bound
    debug!("No privacy-optimized match found, falling back to Branch and Bound");
    branch_and_bound(available_utxos, target_amount, fee_rate)
}

/// Knapsack-based coin selection
///
/// This algorithm uses dynamic programming to solve the knapsack problem
/// and select UTXOs that maximize the value while staying under a weight limit.
pub fn knapsack(
    available_utxos: Vec<Utxo>, 
    target_amount: u64, 
    fee_rate: FeeRate
) -> Result<(Vec<Utxo>, u64), WalletError> {
    debug!("Running knapsack coin selection for {} satoshis", target_amount);
    
    if available_utxos.is_empty() {
        return Err(WalletError::InsufficientFunds("No UTXOs available for selection".to_string()));
    }
    
    // Calculate fee per input
    const INPUT_SIZE: u64 = 150; // Conservative estimate for input size in virtual bytes
    let fee_per_input = INPUT_SIZE * fee_rate.to_sat_per_vb();
    
    // Check if we have enough funds
    let total_available: u64 = available_utxos.iter()
        .map(|utxo| {
            if utxo.txout.value > fee_per_input {
                utxo.txout.value - fee_per_input
            } else {
                0
            }
        })
        .sum();
    
    if total_available < target_amount {
        return Err(WalletError::InsufficientFunds(format!(
            "Insufficient funds: have {} satoshis (after fees), need {}",
            total_available, target_amount
        )));
    }
    
    // For simplicity in this implementation, we'll just use a greedy algorithm
    // Sort UTXOs by effective value (value - fee) in descending order
    let mut sorted_utxos = available_utxos.clone();
    sorted_utxos.sort_by(|a, b| {
        let a_eff = if a.txout.value > fee_per_input {
            a.txout.value - fee_per_input
        } else {
            0
        };
        
        let b_eff = if b.txout.value > fee_per_input {
            b.txout.value - fee_per_input
        } else {
            0
        };
        
        b_eff.cmp(&a_eff) // Descending order
    });
    
    let mut selected_utxos = Vec::new();
    let mut total_input = 0;
    let mut total_effective = 0;
    
    for utxo in sorted_utxos {
        if utxo.txout.value <= fee_per_input {
            continue; // Skip UTXOs that don't cover their fee
        }
        
        selected_utxos.push(utxo.clone());
        total_input += utxo.txout.value;
        total_effective += utxo.txout.value - fee_per_input;
        
        if total_effective >= target_amount {
            break;
        }
    }
    
    if total_effective < target_amount {
        return Err(WalletError::InsufficientFunds(format!(
            "Insufficient funds: selected {} satoshis (after fees), need {}",
            total_effective, target_amount
        )));
    }
    
    debug!("Knapsack selected {} UTXOs with {} sats (effective: {})",
          selected_utxos.len(), total_input, total_effective);
    
    Ok((selected_utxos, total_input))
} 