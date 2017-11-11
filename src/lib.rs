extern crate mbr;
extern crate float_eq;

use std::cmp::Ordering;
use float_eq::feq;
use mbr::MBR;


const CREATE_RED: i32 = 0;
const CREATE_BLUE: i32 = 1;
const REMOVE_RED: i32 = 2;
const REMOVE_BLUE: i32 = 3;


struct Event {
    val: f64,
    ev: i32,
    idx: usize,
}
//add_segment(index, red, red_list, blue, blue_list, visit, false)
//red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]
fn add_segment(index: usize, red: &[Vec<Vec<f64>>],
               red_list: &mut BrutForce, blue: &[Vec<Vec<f64>>], blue_list: &mut BrutForce,
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
    return false;
}


fn prepare_events(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]) -> Vec<Event> {
    let nr = red.len();
    let nb = blue.len();
    let n = nr + nb;
    let mut ptr: usize = 0;
    let mut data: Vec<Event> = Vec::with_capacity(2 * n);

    for i in 0..nr {
        let (x, y) = (red[i][0][0], red[i][1][0]);
        data[ptr] = Event { val: x.min(y), ev: CREATE_RED, idx: i };
        ptr += 1;

        data[ptr] = Event { val: x.max(y), ev: REMOVE_RED, idx: i };
        ptr += 1
    }
    for i in 0..nb {
        let (x, y) = (blue[i][0][0], blue[i][1][0]);
        data[ptr] = Event { val: x.min(y), ev: CREATE_BLUE, idx: i };
        ptr += 1;

        data[ptr] = Event { val: x.max(y), ev: REMOVE_BLUE, idx: i };
        ptr += 1;
    }
    lexsort(&mut data);
    return data;
}


fn rb_intersection(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]) -> Vec<Vec<usize>> {
    let mut crossings: Vec<Vec<usize>> = vec![Vec::new()];
    {
        let mut visit = |i: usize, j: usize| -> bool {
            crossings.push(vec![i, j]);
            false
        };
        red_blue_line_segment_intersection(red, blue, &mut visit);
    }
    return crossings;
}


fn red_blue_line_segment_intersection(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>],
                                      visit: &mut FnMut(usize, usize) -> bool) -> bool {
    let nr = (red).len();
    let nb = (blue).len();
    let n = nr + nb;
    let ne = 2 * n;
    let mut ret: bool = false;

    let events = prepare_events(red, blue);
    //console.log(unpack(events))

    let mut red_list = new_brute_force_list(nr);
    let mut blue_list = new_brute_force_list(nb);

    for i in 0..ne {
        let (ev, index) = (events[i].ev, events[i].idx);
        if ev == CREATE_RED {
            ret = add_segment(
                index, red, &mut red_list, blue, &mut blue_list,
                visit, false
            );
        } else if ev == CREATE_BLUE {
            ret = add_segment(
                index, blue, &mut blue_list, red, &mut red_list,
                visit, true
            );
        } else if ev == REMOVE_RED {
            red_list.remove(index);
        } else if ev == REMOVE_BLUE {
            blue_list.remove(index);
        }

        if ret {
            break
        }
    }

    return ret;
}


struct BrutForce {
    intervals: Vec<f64>,
    index: Vec<usize>,
    count: usize
}

impl BrutForce {
    fn insert(&mut self, lo: f64, hi: f64, index: usize) {
        let count = self.count;
        self.index[count] = index;
        self.intervals[2 * count] = lo;
        self.intervals[2 * count + 1] = hi;
        self.count += 1;
    }

    fn remove(&mut self, index: usize) {
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

fn new_brute_force_list(capacity: usize) -> BrutForce {
    BrutForce {
        intervals: Vec::with_capacity(capacity),
        index: Vec::with_capacity(capacity),
        count: 0
    }
}

//do two lines intersect line segments a && b with
//vertices sa, sb, oa, ob
fn intersects(sa: &[f64], sb: &[f64], oa: &[f64], ob: &[f64]) -> bool {
    return inters(sa, sb, oa, ob, false);
}

fn inters(sa: &[f64], sb: &[f64], oa: &[f64], ob: &[f64], extln: bool) -> bool {
    let mut bln = false;
    let (mut a, mut b, mut d,
        x1, y1, x2, y2,
        x3, y3, x4, y4) = segseg_intersect_abdxy(sa, sb, oa, ob);

    //snap to zero if near -0 or 0
    a = snap_to_zero(a);
    b = snap_to_zero(b);
    d = snap_to_zero(d);

    if d == 0.0 {
        if a == 0.0 && b == 0.0 {
            let abox = MBR::new(x1, y1, x2, y2);
            let bbox = MBR::new(x3, y3, x4, y4);
            bln = abox.intersects(&bbox);
        }
        return bln;
    }
    //intersection along the the seg or extended seg
    let ua = snap_to_zero_or_one(a / d);
    let ub = snap_to_zero_or_one(b / d);

    let ua_0_1 = 0.0 <= ua && ua <= 1.0;
    let ub_0_1 = 0.0 <= ub && ub <= 1.0;
    bln = (ua_0_1 && ub_0_1) || extln;
    return bln;
}

fn segseg_intersect_abdxy(sa: &[f64], sb: &[f64], oa: &[f64], ob: &[f64]) -> (
    f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let x: usize = 0;
    let y: usize = 1;
    //	let (x1, y1, x2, y2, x3, y3, x4, y4, d, a, b):f64;

    let (x1, y1) = (sa[x], sa[y]);
    let (x2, y2) = (sb[x], sb[y]);

    let (x3, y3) = (oa[x], oa[y]);
    let (x4, y4) = (ob[x], ob[y]);

    let d = ((y4 - y3) * (x2 - x1)) - ((x4 - x3) * (y2 - y1));
    let a = ((x4 - x3) * (y1 - y3)) - ((y4 - y3) * (x1 - x3));
    let b = ((x2 - x1) * (y1 - y3)) - ((y2 - y1) * (x1 - x3));
    (a, b, d, x1, y1, x2, y2, x3, y3, x4, y4)
}


//clamp to zero if float is near zero
#[inline]
fn snap_to_zero(v: f64) -> f64 {
    if feq(v, 0.0) {
        0.0
    } else {
        v
    }
}

//clamp to zero or one
#[inline]
fn snap_to_zero_or_one(v: f64) -> f64 {
    if feq(v, 0.0) {
        0.0
    } else if feq(v, 1.0) {
        1.0
    } else {
        v
    }
}
//updates coords that are in bounds
//let mut books = HashSet::new();
//fn update_coords_inbounds(bounds:&mbr::MBR, x1:f64, y1:f64, x2:f64, y2:f64 , set:&HashSet) {
//	if bounds.ContainsXY(x1, y1) {
//		set.Add([]float64{x1, y1})
//	}
//
//	if bounds.ContainsXY(x2, y2) {
//		set.Add([]float64{x2, y2})
//	}
//}


fn lexsort(data: &mut [Event]) {
    data.sort_by(|a, b| lex3d(a, b));
}

//sort lexicographically
#[inline]
fn lex3d(a: &Event, b: &Event) -> std::cmp::Ordering {
    let d = a.val - b.val;
    let mut id: i32;
    if feq(d, 0f64) {
        id = a.ev - b.ev;
    } else {
        return if d < 0f64 { Ordering::Less } else { Ordering::Greater };
    }

    if id == 0 {
        id = a.idx as i32 - b.idx as i32;
    } else {
        return if id < 0 { Ordering::Less } else { Ordering::Greater };
    }
    return if id < 0 { Ordering::Less } else { Ordering::Greater };
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
