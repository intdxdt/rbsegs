extern crate mbr;
extern crate float_eq;

mod seg;
mod brutelist;
mod event;

use seg::intersects;
use brutelist::{new_brute_force_list, add_segment};
use event::{prepare_events, CREATE_RED, CREATE_BLUE, REMOVE_RED, REMOVE_BLUE};

pub fn rb_intersection(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]) -> Vec<Vec<usize>> {
    let mut crossings: Vec<Vec<usize>> = Vec::new();
    {
        let mut visit = |i: usize, j: usize| -> bool {
            crossings.push(vec![i, j]);
            false
        };
        red_blue_line_segment_intersection(red, blue, &mut visit);
    }
    crossings
}


pub fn red_blue_line_segment_intersection(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>],
                                          visit: &mut dyn FnMut(usize, usize) -> bool) -> bool {
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

        if ret { break }
    }

    ret
}


pub fn brute_force(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]) -> Vec<Vec<usize>> {
    let nr = red.len();
    let nb = blue.len();
    let mut crossings: Vec<Vec<usize>> = Vec::new();

    for i in 0..nr {
        let rseg = &red[i];
        //let a    = rseg[0]
        //let b    = rseg[1]
        for j in 0..nb {
            let bseg = &blue[j];
            //c := bseg[0]
            //d := bseg[1]
            if intersects(&rseg[0], &rseg[1], &bseg[0], &bseg[1]) {
                crossings.push(vec![i, j])
            }
        }
    }
    return crossings;
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use std;
    use self::rand::random;
    use std::cmp::Ordering;
    use super::{
        brute_force as brutal,
        rb_intersection as rblsi,
        red_blue_line_segment_intersection,
    };

    fn rnd() -> f64 {
        random::<f64>()
    }

    fn lexcrossings(a: &Vec<usize>, b: &Vec<usize>) -> std::cmp::Ordering {
        let mut d = a[0] as i32 - b[0] as i32;
        if d == 0 {
            d = a[1] as i32 - b[1] as i32;
        }
        if d < 0 { Ordering::Less } else { Ordering::Greater }
    }

    fn cmp(a: &[Vec<usize>], b: &[Vec<usize>]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut bln = false;
        for i in 0usize..a.len() as usize {
            let va = &a[i];
            let vb = &b[i];
            bln = (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| usize_same_val(*a, *b));
            if !bln {
                break
            }
        }
        bln
    }

    fn usize_same_val(a: usize, b: usize) -> bool {
        a == b
    }

    #[test]
    fn edge_case() {
        let red = [vec![vec![224.0, 328.0], vec![224.0, 331.0]]];
        let blue = [vec![vec![224.0, 146.0], vec![224.0, 330.0]]];
        let mut visit = |_: usize, _: usize| -> bool{
            assert!(true);
            false
        };
        red_blue_line_segment_intersection(&red, &blue, &mut visit);
    }


    #[test]
    fn fuzz() {
        for j in 0..20 {
            println!("# fuzz test {} {}", j + 1, " ...");
            let mut red: Vec<Vec<Vec<f64>>> = Vec::new();
            for _ in 0..(10 * (j + 1)) {
                red.push(vec![vec![rnd(), rnd()], vec![rnd(), rnd()]]);
            }

            let mut blue: Vec<Vec<Vec<f64>>> = Vec::new();
            for _ in 0..(10 * (j + 1)) {
                blue.push(vec![vec![rnd(), rnd()], vec![rnd(), rnd()]]);
            }

            let mut expected = brutal(&red, &blue);
            expected.sort_by(|a, b| lexcrossings(a, b));

            let mut actual = rblsi(&red, &blue);
            actual.sort_by(|a, b| lexcrossings(a, b));
            assert_eq!(actual, expected);
            assert!(cmp(&actual, &expected));
        }
    }
}
