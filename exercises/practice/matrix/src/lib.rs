pub struct Matrix {
    elements: Vec<u32>,
    nrow: usize,
    ncol: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let lines = input.split("\n");
        let nrow = lines.clone().count();
        let elements: Vec<u32> = lines
            .flat_map(|line| line.split(' ').map(|el| el.parse::<u32>().unwrap()))
            .collect();
        let ncol = elements.clone().len() / nrow;

        println!("nrow {nrow}, elements {elements:?}, ncol {ncol}");
        Self {
            nrow,
            elements,
            ncol,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.nrow {
            return None;
        }

        self.elements
            .get(((row_no - 1) * self.ncol)..(row_no * self.ncol))
            .map(|s| s.to_vec())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > self.ncol {
            return None;
        }

        ((col_no - 1)..(self.nrow * self.ncol))
            .step_by(self.ncol)
            .map(|i| self.elements.get(i).copied())
            .collect()
    }
}
