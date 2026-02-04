use std::collections::BTreeMap;

pub struct TopSmallestSum {
    capacity: usize,
    smaller: BTreeMap<i32, usize>,
    larger: BTreeMap<i32, usize>,
    smaller_sum: i64,
    smaller_count: usize,
}

impl TopSmallestSum {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            smaller: BTreeMap::new(),
            larger: BTreeMap::new(),
            smaller_sum: 0,
            smaller_count: 0,
        }
    }

    pub fn add(&mut self, val: i32) {
        self.insert_smaller(val);

        if self.smaller_count > self.capacity {
            if let Some(max_val_from_smaller) = self.pop_max_smaller() {
                self.insert_larger(max_val_from_smaller);
            }
        }
    }

    pub fn remove(&mut self, val: i32) {
        if self.smaller.contains_key(&val) {
            self.remove_from_smaller(val);

            if self.smaller_count < self.capacity {
                if let Some(min_val_from_larger) = self.pop_min_larger() {
                    self.insert_smaller(min_val_from_larger);
                }
            }
        } else {
            self.remove_from_larger(val);
        }
    }

    pub fn get_sum(&self) -> i64 {
        self.smaller_sum
    }

    fn insert_smaller(&mut self, val: i32) {
        *self.smaller.entry(val).or_insert(0) += 1;
        self.smaller_sum += val as i64;
        self.smaller_count += 1;
    }

    fn insert_larger(&mut self, val: i32) {
        *self.larger.entry(val).or_insert(0) += 1;
    }

    fn remove_from_smaller(&mut self, val: i32) {
        if let Some(count) = self.smaller.get_mut(&val) {
            *count -= 1;
            if *count == 0 {
                self.smaller.remove(&val);
            }
            self.smaller_sum -= val as i64;
            self.smaller_count -= 1;
        }
    }

    fn remove_from_larger(&mut self, val: i32) {
        if let Some(count) = self.larger.get_mut(&val) {
            *count -= 1;
            if *count == 0 {
                self.larger.remove(&val);
            }
        }
    }

    fn pop_max_smaller(&mut self) -> Option<i32> {
        let mut entry = self.smaller.last_entry()?;
        let val = *entry.key();
        *entry.get_mut() -= 1;
        if *entry.get() == 0 {
            entry.remove();
        }
        self.smaller_sum -= val as i64;
        self.smaller_count -= 1;
        Some(val)
    }

    fn pop_min_larger(&mut self) -> Option<i32> {
        let mut entry = self.larger.first_entry()?;
        let val = *entry.key();
        *entry.get_mut() -= 1;
        if *entry.get() == 0 {
            entry.remove();
        }
        Some(val)
    }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k_size = (k - 2) as usize;
        let dist = dist as usize;

        let mut top_k = TopSmallestSum::new(k_size);

        for i in 1..=dist {
            top_k.add(nums[i]);
        }
        let mut min_total_cost = i64::MAX;

        for i in 1..(n - k_size) {
            top_k.remove(nums[i]);
            if i + dist < n {
                top_k.add(nums[i + dist]);
            }
            let current_cost = nums[0] as i64 + nums[i] as i64 + top_k.get_sum();
            min_total_cost = min_total_cost.min(current_cost);
        }

        min_total_cost
    }
}
