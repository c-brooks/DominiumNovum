use crate::map::ProvinceMap;
use bevy::prelude::Resource;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq)]
pub enum QueuedAction {
    Travel { to_province: u32, arriving: bool },
    Idle,
}

#[derive(Resource)]
pub struct WeekQueue {
    pub days: [QueuedAction; 7],
    pub current_day: usize, // which day Step is executing
    next_slot: usize,       // where the next push_action writes
}

impl Default for WeekQueue {
    fn default() -> Self {
        Self {
            days: std::array::from_fn(|_| QueuedAction::Idle),
            current_day: 0,
            next_slot: 0,
        }
    }
}

impl WeekQueue {
    pub fn push_action(&mut self, action: QueuedAction) {
        if self.next_slot >= 7 {
            return;
        }
        self.days[self.next_slot] = action;
        self.next_slot += 1;
    }

    pub fn queue_travel(&mut self, from: u32, to: u32, province_map: &ProvinceMap) {
        let path = dijkstra(from, to, province_map);
        for (i, (province_id, cost)) in path.iter().enumerate() {
            let is_last_hop = i == path.len() - 1;
            for day in 0..(*cost as usize) {
                let arriving = is_last_hop && day == *cost as usize - 1;
                self.push_action(QueuedAction::Travel {
                    to_province: *province_id,
                    arriving,
                });
            }
        }
    }
}

// Returns the cheapest path from `from` to `to` as a list of (province_id, travel_days) hops,
// excluding the starting province. Returns an empty vec if no path exists or from == to.
fn dijkstra(from: u32, to: u32, province_map: &ProvinceMap) -> Vec<(u32, u8)> {
    if from == to {
        return vec![];
    }
    println!("Calculating path from {} to {}...", from, to);

    // (accumulated_cost, province_id)
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = BinaryHeap::new();
    let mut dist: HashMap<u32, u32> = HashMap::new();
    let mut prev: HashMap<u32, u32> = HashMap::new();

    heap.push(Reverse((0, from)));
    dist.insert(from, 0);

    while let Some(Reverse((cost, current))) = heap.pop() {
        if current == to {
            break;
        }
        if cost > *dist.get(&current).unwrap_or(&u32::MAX) {
            continue; // stale entry
        }
        let Some(province) = province_map.get(current) else {
            continue;
        };
        for &neighbor_id in &province.neighbors {
            let Some(neighbor) = province_map.get(neighbor_id) else {
                continue;
            };
            let new_cost = cost + neighbor.travel_days as u32;
            if new_cost < *dist.get(&neighbor_id).unwrap_or(&u32::MAX) {
                dist.insert(neighbor_id, new_cost);
                prev.insert(neighbor_id, current);
                heap.push(Reverse((new_cost, neighbor_id)));
            }
        }
    }

    // Reconstruct path from `to` back to `from`
    if !prev.contains_key(&to) {
        println!("No path found from {} to {}", from, to);
        return vec![]; // no path found
    }
    let mut path = vec![];
    let mut current = to;
    while current != from {
        let Some(&parent) = prev.get(&current) else {
            break;
        };
        let days = province_map
            .get(current)
            .map(|p| p.travel_days)
            .unwrap_or(1);
        path.push((current, days));
        current = parent;
    }
    path.reverse();
    println!("Path found: {:?}", path);
    path
}
