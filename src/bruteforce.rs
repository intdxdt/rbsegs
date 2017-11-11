
use seg::intersects;

pub fn brute_force(red: &[Vec<Vec<f64>>], blue: &[Vec<Vec<f64>>]) -> Vec<Vec<usize>>{
	let nr = red.len();
	let nb = blue.len();
	let mut crossings:Vec<Vec<usize>> = Vec::new();

	for i in  0..nr {
		let rseg = &red[i];
		//let a    = rseg[0]
		//let b    = rseg[1]
		for j in  0.. nb {
			let bseg = &blue[j];
			//c := bseg[0]
			//d := bseg[1]
			if intersects(&rseg[0], &rseg[1], &bseg[0], &bseg[1]) {
				crossings.push(vec![i, j])
			}
		}
	}
	return crossings
}
