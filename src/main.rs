//
// impl World{
//     fn new(grid: [[[i32; 10]; 10]; 10]) -> Self {
//  0   Self { grid }
//     }

//     /// Returns a mutable reference to the grid of this [`World`].
//     fn grid_mut(&mut self) -> &mut [[[i32; 10]; 10]; 10] {
//         &mut self.grid
//     }

// }
#[derive(Debug)]

struct World([[[i32; 10]; 10]; 10]);

impl World {
    fn set_vox(&mut self, pos_xyz: (usize, usize, usize), val: i32) {
        self.0[pos_xyz.0][pos_xyz.1][pos_xyz.2] = val;
    }
}

fn main() {
    let mut world = World([[[0; 10]; 10]; 10]);
    let mut i = 0;

    for d1 in 0..world.0.len() {
        for d2 in 0..world.0[0].len() {
            for d3 in 0..world.0[0][0].len() {
                println!("{}{}{}", d1, d2, d3);
                world.set_vox((d3, d2, d1), i);
                i = i + 1
            }
        }
    }

    println!("{:#?}", world.0);
    // let mut sum: usize = 0;
    // for n in 0..world.0[0][0].len() {
    //     println!("{n}",);
    // }
}
