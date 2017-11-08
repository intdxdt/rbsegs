struct BrutForce {
    intervals: Vec<f64>,
    index: Vec<usize>,
    count: usize
}

impl BrutForce {
    fn insert( mut self, lo: f64, hi: f64, index: usize) {
        let count = self.count;
        self.index[count] = index;
        self.intervals[2 * count] = lo;
        self.intervals[2 * count + 1] = hi;
        self.count += 1;
    }

    fn remove( mut self, index:usize){
        let count  = self.count;
        for i  in (0..count).rev() {
            if self.index[i] == index {
                self.index[i] = self.index[count-1];
                self.intervals[2*i] = self.intervals[2*(count-1)];
                self.intervals[2*i+1] = self.intervals[2*count-1];
                self.count =  (self.count as i32 -1) as usize;
                return
            }
        }
    }
}

fn new_brut_force_list (capacity: usize) -> BrutForce {
    BrutForce {
        intervals: Vec::with_capacity(capacity),
        index:     Vec::with_capacity(capacity),
        count: 0
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
