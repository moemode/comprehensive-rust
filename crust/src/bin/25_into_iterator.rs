struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        GridIter {grid: self, index: 0}
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    index: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.grid.x_coords.len() {
            None 
        } else {
            let next = (self.grid.x_coords[self.index], self.grid.y_coords[self.index]);
            self.index += 1;
            Some(next)
        }
    }
}

fn main() {
    let g = Grid {x_coords: vec![1, 2], y_coords: vec![3, 4]};
    for c in &g {
        println!("{c:?}");
    }
    for c in &g {
        println!("{c:?}");
    }
}