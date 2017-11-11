use std;
use std::cmp::Ordering;
use float_eq::feq;

#[derive(Clone)]
pub struct Event {
    pub val: f64,
    pub ev: i32,
    pub idx: usize,
}

pub const CREATE_RED: i32 = 0;
pub const CREATE_BLUE: i32 = 1;
pub const REMOVE_RED: i32 = 2;
pub const REMOVE_BLUE: i32 = 3;


pub fn prepare_events(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]) -> Vec<Event> {
    let nr = red.len();
    let nb = blue.len();
    let n = nr + nb;
    let mut ptr: usize = 0;
    let mut data: Vec<Event> = vec![Event { val: 0.0, ev: 0, idx: 0 }; 2 * n];

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
    data
}



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
    }
    if id < 0 { Ordering::Less } else { Ordering::Greater }
}
