
use seg::intersects;

pub struct BrutForceList {
    pub intervals: Vec<f64>,
    pub index: Vec<usize>,
    pub count: usize
}

impl BrutForceList {
    pub fn insert(&mut self, lo: f64, hi: f64, index: usize) {
        let count = self.count;
        self.index[count] = index;
        self.intervals[2 * count] = lo;
        self.intervals[2 * count + 1] = hi;
        self.count += 1;
    }

   pub fn remove(&mut self, index: usize) {
        let count = self.count;
        for i in (0..count).rev() {
            if self.index[i] == index {
                self.index[i] = self.index[count - 1];
                self.intervals[2 * i] = self.intervals[2 * (count - 1)];
                self.intervals[2 * i + 1] = self.intervals[2 * count - 1];
                self.count = (self.count as i32 - 1) as usize;
                return;
            }
        }
    }
}

pub fn new_brute_force_list(capacity: usize) -> BrutForceList {
    BrutForceList {
        intervals: vec![0.0; 2 * capacity],
        index: vec![0; capacity],
        count: 0
    }
}


//add_segment(index, red, red_list, blue, blue_list, visit, false)
//red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]
pub fn add_segment(index: usize, red: &[Vec<Vec<f64>>],
               red_list: &mut BrutForceList, blue: &[Vec<Vec<f64>>], blue_list: &mut BrutForceList,
               visit: &mut FnMut(usize, usize) -> bool, flip: bool) -> bool {
    //Look up segment
    let seg = &red[index];

    //Get segment end points
    let x0 = &seg[0];
    let x1 = &seg[1];

    //Read out components
    let a0 = x0[1];
    let a1 = x1[1];
    let l0 = a0.min(a1);
    let h0 = a0.max(a1);

    //Scan over blue intervals for point
    let intervals = &blue_list.intervals;
    let blue_index = &blue_list.index;
    let count = blue_list.count;
    let mut ptr = 2 * count;

    for i in (0..count).rev() {
        ptr = (ptr as i32 - 1) as usize;
        let h1 = intervals[ptr];
        ptr = (ptr as i32 - 1) as usize;
        let l1 = intervals[ptr];

        //Test if intervals overlap
        if l0 <= h1 && l1 <= h0 {
            let bindex = blue_index[i];
            let bseg = &blue[bindex];

            //Test if segments intersect
            if intersects(&seg[0], &seg[1], &bseg[0], &bseg[1]) {
                let ret: bool;
                if flip {
                    ret = visit(bindex, index);
                } else {
                    ret = visit(index, bindex);
                }
                if ret {
                    return ret;
                }
            }
        }
    }

    red_list.insert(l0, h0, index);
    false
}


