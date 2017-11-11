use float_eq::feq;
use mbr::MBR;

//do two lines intersect line segments a && b with
//vertices sa, sb, oa, ob
pub fn intersects(sa: &[f64], sb: &[f64], oa: &[f64], ob: &[f64]) -> bool {
    inters(sa, sb, oa, ob, false)
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
    (ua_0_1 && ub_0_1) || extln
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
    if feq(v, 0.0) { 0.0 } else { v }
}

//clamp to zero or one
#[inline]
fn snap_to_zero_or_one(v: f64) -> f64 {
    if feq(v, 0.0) { 0.0 } else if feq(v, 1.0) { 1.0 } else { v }
}

//updates coords that are in bounds let mut books = HashSet::new();
//fn update_coords_inbounds(bounds:&mbr::MBR, x1:f64, y1:f64, x2:f64, y2:f64 , set:&HashSet) {
//	if bounds.ContainsXY(x1, y1) {
//		set.Add([]float64{x1, y1})
//	}
//
//	if bounds.ContainsXY(x2, y2) {
//		set.Add([]float64{x2, y2})
//	}
//}
