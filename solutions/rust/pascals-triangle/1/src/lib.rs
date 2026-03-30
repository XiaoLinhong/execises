pub struct PascalsTriangle{
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut values: Vec<Vec<u32>>  = Vec::new();
        for _ in 0..self.row_count {
            if values.is_empty() {
                values.push(cal_row(&[]));
            } else {
                values.push(cal_row(values.last().unwrap()));
            }
            
        }
        values
    }

}

fn cal_row(previous: &[u32]) -> Vec<u32>{
    if previous.is_empty() {
        return vec![1];
    }
    std::iter::once(0).chain(previous.iter().cloned())
                      .chain(std::iter::once(0))
                      .collect::<Vec<u32>>()
                      .windows(2) // 为什么报错
                      .map(|w| w[0]+w[1])
                      .collect::<Vec<u32>>()
}
