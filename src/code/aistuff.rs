use std::collections::{HashMap, HashSet, VecDeque};


use crate::code::board;
use crate::code::enums::{Guess, Intelligence, ShipType, Direction, Mode};
use crate::code::utils::base_26;

use super::board::GameData;

#[derive(Clone, Debug)]
pub struct Placement {
    pub ship: ShipType,
    pub cells: Vec<(usize, usize)>,
    pub direction: Direction,
}

pub struct AIdata {
    pub rows: usize,
    pub cols: usize,
    pub intelligence: Intelligence,
    pub mode: Mode,

    pub board_status: Vec<Vec<Guess>>,
    pub heatmap: Vec<Vec<usize>>,

    pub valid_places: HashMap<ShipType, Vec<Placement>>,
    pub alive_ships: HashSet<ShipType>,

    // Targeting sandbox: only exists while in targeting mode
    pub targeting_places: Option<HashMap<ShipType, Vec<Placement>>>,

    // hits not yet assigned to a sunk ship
    pub unresolved_hits: Vec<(usize, usize)>,
}

impl AIdata {
    // Response Entry
    /// response is "M", "H", "S<size>"
    pub fn process_feedback(&mut self, row: usize, col: usize, response: &str) {
        // normalize
        let resp = response.trim();

        match resp.chars().next() {
            Some('M') => {
                // Miss
                self.board_status[row][col] = Guess::Miss;
                self.prune_on_miss((row, col));
                // If in targeting mode, pruning may eliminate possibilities -> if none left, revert to hunt
                if let Some(tp) = &self.targeting_places {
                    if tp.values().all(|v| v.is_empty()) {
                        self.targeting_places = None;
                        self.mode = Mode::Hunt;
                    }
                }
            }
            Some('H') => {
                // Hit: mark and create or prune targeting sandbox
                self.board_status[row][col] = Guess::Hit;
                // add to unresolved hits
                if !self.unresolved_hits.contains(&(row, col)) {
                    self.unresolved_hits.push((row, col));
                }
                self.init_or_prune_targeting_for_hit((row, col));
                self.mode = Mode::Targeting;
            }
            Some('S') => {
                // Sunk: format is "S<size>"
                // The S also functions as a hit on (row,col)
                self.board_status[row][col] = Guess::Hit;
                if !self.unresolved_hits.contains(&(row, col)) {
                    self.unresolved_hits.push((row, col));
                }

                // parse size
                let size_str = &resp[1..];
                if let Ok(size) = size_str.parse::<usize>() {
                    self.process_sink((row, col), size);
                } else {
                    // malformed; ignore for now
                }
            }
            _ => {
                // Q, E, unknown — we ignore here
            }
        }

        // After handling feedback, rebuild the heatmap (Hunt uses global, Target uses targeting)
        self.build_heatmap();
    }

    // -------------- Pruning and targeting management --------------

    /// Remove placements (global & targeting) which include the given miss cell
    fn prune_on_miss(&mut self, miss: (usize, usize)) {
        // global pruning
        for (_ship, placements) in self.valid_places.iter_mut() {
            placements.retain(|p| !p.cells.contains(&miss));
        }
        // targeting pruning if present
        if let Some(tp) = &mut self.targeting_places {
            for (_ship, placements) in tp.iter_mut() {
                placements.retain(|p| !p.cells.contains(&miss));
            }
        }
    }

    /// Initialize or prune the targeting_places after a Hit
    /// Option A2: only include placements for unsunk ships that contain the hit cell
    /// and that are consistent with the current unresolved hit cluster sizes and orientation constraints.
    fn init_or_prune_targeting_for_hit(&mut self, hit: (usize, usize)) {
        // Build cluster information including the new hit
        let mut cluster_map = self.cluster_unresolved_hits(); // Vec<Vec<(r,c)>>
        // Find which cluster contains 'hit'
        let mut target_cluster: Option<Vec<(usize, usize)>> = None;
        for cluster in cluster_map.iter() {
            if cluster.contains(&hit) {
                target_cluster = Some(cluster.clone());
                break;
            }
        }

        // if no cluster contains it (rare), treat the single hit as a cluster
        if target_cluster.is_none() {
            target_cluster = Some(vec![hit]);
        }
        let cluster = target_cluster.unwrap();
        let cluster_len = cluster.len();

        // Build new targeting_places: only placements for *alive ships* whose size >= cluster_min
        // Aggressive pruning rule A2: require ship.size >= cluster_len
        let mut tp: HashMap<ShipType, Vec<Placement>> = HashMap::new();
        for ship in self.alive_ships.iter() {
            if ship.size < cluster_len {
                // cannot be this ship
                continue;
            }

            // For each global placement of this ship, include only those that:
            // 1) contain ALL coordinates in `cluster` (so assume cluster belongs to this ship)
            // 2) contain the hit cell (redundant since cluster contains hit)
            if let Some(global_list) = self.valid_places.get(ship) {
                let mut candidates: Vec<Placement> = Vec::new();
                'placement_loop: for p in global_list.iter() {
                    // must contain all hits in cluster
                    for &coord in cluster.iter() {
                        if !p.cells.contains(&coord) {
                            continue 'placement_loop;
                        }
                    }
                    // also ensure placement doesn't conflict with known misses
                    let mut ok = true;
                    for &(r, c) in p.cells.iter() {
                        if self.board_status[r][c] == Guess::Miss {
                            ok = false;
                            break;
                        }
                        // Also, do not include placements that use coordinates already assigned to a *confirmed* sunk ship
                        // (we don't have confirmed sunk storage separate now — but since we remove placements on sink, it's ok)
                    }
                    if ok {
                        candidates.push(p.clone());
                    }
                }
                if !candidates.is_empty() {
                    tp.insert(*ship, candidates);
                }
            }
        }

        // If tp is empty (no placements found under strict A2), fall back to looser rule:
        if tp.is_empty() {
            // fallback: include any placement (for alive ships) that contains the hit cell
            for ship in self.alive_ships.iter() {
                if let Some(global_list) = self.valid_places.get(ship) {
                    let candidates: Vec<Placement> = global_list
                        .iter()
                        .filter(|p| p.cells.contains(&hit) && !p.cells.iter().any(|&(r,c)| self.board_status[r][c]==Guess::Miss))
                        .cloned()
                        .collect();
                    if !candidates.is_empty() {
                        tp.insert(*ship, candidates);
                    }
                }
            }
        }

        // set targeting_places
        self.targeting_places = Some(tp);
    }

    /// Process sink: identify the sunk cluster, remove placements accordingly
    fn process_sink(&mut self, sink_coord: (usize, usize), size: usize) {
        // Step 1: build clusters (including the sink coord which was added earlier)
        let clusters = self.cluster_unresolved_hits();
        // Candidate clusters must be exactly size `size`
        let mut candidates: Vec<Vec<(usize, usize)>> = clusters.into_iter().filter(|c| c.len()==size).collect();

        // If multiple candidates, prefer the one containing sink_coord
        let chosen: Option<Vec<(usize, usize)>> = if candidates.is_empty() {
            // No exact cluster found — fallback: try to find placements in targeting_places that contain sink_coord and have length == size
            if let Some(tp) = &self.targeting_places {
                for (_ship, placements) in tp.iter() {
                    for p in placements.iter() {
                        if p.ship.size == size && p.cells.contains(&sink_coord) {
                            // select the contiguous run within p.cells that includes sink_coord of length=size
                            // since p.cells is contiguous by construction (horizontal/vertical),
                            // we can take p.cells and choose the run of length size containing sink_coord.
                            if let Some(run) = Self::contiguous_run_containing(&p.cells, sink_coord, size) {
                                return self.finalize_sunk(size, run);
                            }
                        }
                    }
                }
            }
            None
        } else {
            // prefer candidate containing sink_coord
            let mut chosen_idx: Option<usize> = None;
            for (i, c) in candidates.iter().enumerate() {
                if c.contains(&sink_coord) {
                    chosen_idx = Some(i);
                    break;
                }
            }
            if chosen_idx.is_none() {
                // choose the candidate with sink_coord contained in it if possible (rare)
                // Otherwise choose the one that contains the most recently guessed hits — but we will choose first for now
                chosen_idx = Some(0);
            }
            chosen_idx.map(|i| candidates.remove(i))
        };

        if let Some(cluster_cells) = chosen {
            // finalize
            self.finalize_sunk(size, cluster_cells);
        } else {
            // As a last fallback, we will attempt to deduce from global placements: find any global placement of size that includes sink_coord
            if let Some(global_list) = self.valid_places.get(&ShipType { size }) {
                for p in global_list.iter() {
                    if p.cells.contains(&sink_coord) {
                        if let Some(run) = Self::contiguous_run_containing(&p.cells, sink_coord, size) {
                            self.finalize_sunk(size, run);
                            return;
                        }
                    }
                }
            }
            // If we still can't find anything, be conservative: remove ship from alive_ships and clear any placements of that size.
            self.alive_ships.retain(|s| s.size != size);
            self.valid_places.remove(&ShipType { size });
            self.targeting_places = None;
            // remove resolved hits that are impossible? For safety, keep unresolved hits as-is
        }
    }

    /// Helper which given a contiguous placement (p.cells) finds the contiguous run of length `size`
    /// that contains `anchor` (sink coordinate). Returns Some(Vec<coords>) or None.
    fn contiguous_run_containing(cells: &Vec<(usize, usize)>, anchor: (usize, usize), size: usize) -> Option<Vec<(usize, usize)>> {
        // cells is contiguous in placement order; find index of anchor
        if let Some(idx) = cells.iter().position(|&c| c == anchor) {
            let n = cells.len();
            // we need to pick a window of length size that includes idx
            if size > n { return None; }
            let start_min = if idx + 1 >= size { idx + 1 - size } else { 0 };
            let start_max = if idx <= n - size { idx } else { n - size };
            for start in start_min..=start_max {
                let run = cells[start..start+size].to_vec();
                if run.contains(&anchor) { return Some(run); }
            }
        }
        None
    }

    /// Finalize a sunk ship given its determined coordinates
    fn finalize_sunk(&mut self, size: usize, coords: Vec<(usize, usize)>) {
        // Remove these coords from unresolved_hits
        let coords_set: HashSet<(usize, usize)> = coords.iter().copied().collect();
        self.unresolved_hits.retain(|c| !coords_set.contains(c));

        // Mark those cells (they are hits already) - could mark specially if you want
        for &(r, c) in coords.iter() {
            self.board_status[r][c] = Guess::Hit; // we already set it, but reaffirm
        }

        // Remove ship from alive_ships (we assume ship id == size for current challenge)
        self.alive_ships.retain(|s| s.size != size);
        // Remove all global placements for this ship size that do NOT exactly match coords
        self.valid_places.remove(&ShipType { size }); // remove all, then re-add only matching ones? Simpler: remove all
        // Additionally remove placements of other ships that include any of these coords
        for (_ship, placements) in self.valid_places.iter_mut() {
            placements.retain(|p| !p.cells.iter().any(|c| coords_set.contains(c)));
        }

        // Clear targeting sandbox (we finished with that ship)
        self.targeting_places = None;

        // If unresolved_hits empty -> return to Hunt
        if self.unresolved_hits.is_empty() {
            self.mode = Mode::Hunt;
        } else {
            self.mode = Mode::Targeting;
            // If there are remaining unresolved hits, rebuild targeting_places for them conservatively
            // We'll initialize a targeting sandbox around the first unresolved hit
            if let Some(&first) = self.unresolved_hits.first() {
                self.init_or_prune_targeting_for_hit(first);
            }
        }
    }

    // -------------- Hit clustering ----------------
    /// Groups unresolved_hits into orthogonally connected clusters.
    fn cluster_unresolved_hits(&self) -> Vec<Vec<(usize, usize)>> {
        let mut clusters: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let hitset: HashSet<(usize, usize)> = self.unresolved_hits.iter().copied().collect();

        for &pt in self.unresolved_hits.iter() {
            if seen.contains(&pt) { continue; }
            // BFS/DFS to collect connected component
            let mut q: VecDeque<(usize, usize)> = VecDeque::new();
            let mut comp: Vec<(usize, usize)> = Vec::new();
            q.push_back(pt);
            seen.insert(pt);
            while let Some((r, c)) = q.pop_front() {
                comp.push((r, c));
                // neighbors up/down/left/right
                let neigh = [
                    (r.wrapping_sub(1), c),
                    (r + 1, c),
                    (r, c.wrapping_sub(1)),
                    (r, c + 1),
                ];
                for &(nr, nc) in neigh.iter() {
                    if nr < self.rows && nc < self.cols && hitset.contains(&(nr, nc)) && !seen.contains(&(nr, nc)) {
                        seen.insert((nr, nc));
                        q.push_back((nr, nc));
                    }
                }
            }
            clusters.push(comp);
        }
        clusters
    }

    // -------------- Heatmap building --------------
    /// Rebuild heatmap using either global valid_places (Hunt) or targeting_places (Target)
    pub fn build_heatmap(&mut self) {
        // Zero
        for row in &mut self.heatmap {
            for cell in row.iter_mut() {
                *cell = 0;
            }
        }

        // Which placements to iterate?
        let iter_places: Vec<Vec<Placement>> = match &self.mode {
            Mode::Hunt => {
                // flatten global valid_places
                self.valid_places.values().cloned().collect()
            }
            Mode::Targeting => {
                if let Some(tp) = &self.targeting_places {
                    tp.values().cloned().collect()
                } else {
                    self.valid_places.values().cloned().collect()
                }
            }
        };

        // count
        for plist in iter_places.iter() {
            for p in plist.iter() {
                // If any cell in p conflicts with a known Miss, skip (shouldn't happen if prunes were correct)
                if p.cells.iter().any(|&(r,c)| self.board_status[r][c] == Guess::Miss) {
                    continue;
                }
                for &(r, c) in p.cells.iter() {
                    self.heatmap[r][c] += 1;
                }
            }
        }

        // adjacency bonus around unresolved hits (finishing ships)
        for &(hr, hc) in &self.unresolved_hits {
            let dirs = [(-1isize,0),(1,0),(0,-1),(0,1)];
            for (dr,dc) in dirs.iter() {
                let rr = hr as isize + dr;
                let cc = hc as isize + dc;
                if rr >= 0 && cc >= 0 && (rr as usize) < self.rows && (cc as usize) < self.cols {
                    // only boost unknown cells
                    let (rru, ccu) = (rr as usize, cc as usize);
                    if self.board_status[rru][ccu] == Guess::Unknown {
                        self.heatmap[rru][ccu] += 20; // tunable
                    }
                }
            }
        }
    }

    // -------------- Guess selection (simple) --------------
    /// Choose best guess based on current mode & intelligence:
    /// - Hunting: use heatmap with parity (for medium) or pure heatmap (for hard)
    /// - Targeting: prefer adjacent unknown cells to unresolved hits, else highest heat
    pub fn choose_best_guess(&self) -> Option<(usize, usize)> {
        // First: if Targeting, try adjacency to unresolved hits
        if self.mode == Mode::Targeting && !self.unresolved_hits.is_empty() {
            let dirs = [(-1isize,0),(1,0),(0,-1),(0,1)];
            for &(hr, hc) in &self.unresolved_hits {
                // prefer unknown neighbors
                let mut candidates = Vec::new();
                for (dr,dc) in dirs.iter() {
                    let rr = hr as isize + dr;
                    let cc = hc as isize + dc;
                    if rr >= 0 && cc >= 0 && (rr as usize) < self.rows && (cc as usize) < self.cols {
                        let (rru, ccu) = (rr as usize, cc as usize);
                        if self.board_status[rru][ccu] == Guess::Unknown {
                            candidates.push((rru, ccu));
                        }
                    }
                }
                if !candidates.is_empty() {
                    // pick the candidate with max heat (tie broken by first)
                    let mut best = candidates[0];
                    let mut bestv = self.heatmap[best.0][best.1];
                    for &cand in candidates.iter().skip(1) {
                        let v = self.heatmap[cand.0][cand.1];
                        if v > bestv {
                            best = cand;
                            bestv = v;
                        }
                    }
                    return Some(best);
                }
            }
        }

        // Hunting: use parity heuristic when Intelligence::Medium, else use pure heat
        let mut best_list: Vec<(usize,usize)> = Vec::new();
        let mut maxv: usize = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.board_status[r][c] != Guess::Unknown { continue; }

                // parity pruning for Medium: prefer (r+c) % 2 based on smallest alive ship (or 2)
                if self.intelligence == Intelligence::Medium {
                    let min_ship = self.alive_ships.iter().map(|s| s.size).min().unwrap_or(2);
                    if (r + c) % min_ship != 0 { continue; }
                }

                let v = self.heatmap[r][c];
                if v > maxv {
                    maxv = v;
                    best_list.clear();
                    best_list.push((r,c));
                } else if v == maxv {
                    best_list.push((r,c));
                }
            }
        }

        if best_list.is_empty() {
            // fallback to any unknown cell
            for r in 0..self.rows {
                for c in 0..self.cols {
                    if self.board_status[r][c] == Guess::Unknown {
                        best_list.push((r,c));
                    }
                }
            }
        }

        // choose first / randomize if you like
        best_list.get(0).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShipType {
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct Placement {
    pub ship: ShipType,
    pub cells: Vec<(usize, usize)>,       // Expanded list of all coordinates
    pub direction: Direction,
}

pub struct AIdata {
    pub rows: usize,
    pub cols: usize,
    pub intelligence: Intelligence,             // Level of AI

    pub mode: Mode,

    pub board_status: Vec<Vec<Guess>>,           // Board with Unknown/miss/hit
    pub heatmap: Vec<Vec<usize>>,           // 
    
    pub valid_places: HashMap<ShipType, Vec<Placement>>,        // All possible placements
    pub alive_ships: HashSet<ShipType>,

    pub unresolved_hits: Vec<(usize, usize)>,           // Hits not attached to a sunken ship
}

impl AIdata {
    pub fn new(rows:usize, cols: usize, ships: Vec<ShipType>, 
        intelligence: Intelligence) -> Self {

        let board_status = vec![vec![Guess::Unknown; cols]; rows];
        let heatmap = vec![vec![0;cols];rows];
        let mut valid_places = HashMap::new();
        let alive_ships = ships.into_iter().copied().collect();

        for &ship in ships {
            let placements = AIdata::generate_placements(rows, cols, ship);
            valid_places.insert(ship, placements);
        }
        
        Self {
           rows,
           cols,
           intelligence,
           mode: Mode::Hunt,
           board_status,
           heatmap,
           valid_places,
           alive_ships,
           unresolved_hits: Vec::new(),
        }
    }

    fn generate_placements(rows:usize, cols:usize, ship: ShipType) -> Vec<Placement> {
        let mut placements = Vec::new();

        // Horizontal first
        for r in 0..rows {
            for c in 0..=cols - ship.size {
                let cells: Vec<(usize, usize)> = (0..ship.size).map(|i| (r, c+i)).collect();
                placements.push(Placement {
                    ship, cells, direction: Direction::Horizontal,});
            }
        }
        
        // Vertical next
        for c in 0..cols {
            for r in 0..=rows - ship.size {
                let cells: Vec<(usize, usize)> = (0..ship.size).map(|i| (r+i, c)).collect();
                placements.push(Placement {
                    ship, cells, direction: Direction::Vertical,});
            }
        }
        placements
    
    }

    pub fn reset_heatmap(&mut self) {
        for row in &mut self.heatmap {
            for cell in row.iter_mut() {
                *cell = 0;
            }
        }
    }

}