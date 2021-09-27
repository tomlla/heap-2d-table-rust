use std::{collections::HashMap, hash::Hash};

trait TableCell<D1, D2>
where
    D1: std::fmt::Debug,
    D2: std::fmt::Debug,
{
    fn d1(&self) -> &D1;
    fn d2(&self) -> &D2;
}

pub struct MultiCellsTable<'a, T, D1, D2> {
    cells: Vec<T>,
    d1_cells_map: HashMap<D1, Vec<&'a T>>,
    d2_cells_map: HashMap<D2, Vec<&'a T>>,
    d1_d2_cells_map: HashMap<(D1, D2), Vec<&'a T>>,
}
impl<'a, T, D1, D2> MultiCellsTable<'a, T, D1, D2>
where
    T: TableCell<D1, D2>,
    D1: std::fmt::Debug,
    D2: std::fmt::Debug,
{
    fn new(cells: Vec<T>, d1_capacity: usize, d2_capacity: usize) -> Self {
        let d1_cells_map: HashMap<D1, Vec<&'a T>> = HashMap::with_capacity(d1_capacity);
        let d2_cells_map: HashMap<D2, Vec<&'a T>> = HashMap::with_capacity(d2_capacity);
        let d1_d2_cells_map: HashMap<(D1, D2), Vec<&'a T>> =
            HashMap::with_capacity(d1_capacity * d2_capacity);
        Self {
            cells,
            d1_cells_map,
            d2_cells_map,
            d1_d2_cells_map,
        }
    }

    fn insert(&self, cell: TableCell) {
        let d1 = cell.d1();
        let d2 = cell.d1();
        let d1d2 = (d1, d2);
    }
    fn delete(&self, d1: &D1, d2: &d2) {}

    fn get_by_d1(&self) -> Vec<&TableCell> {}
}

#[cfg(test)]
mod tests {
    type Date = u32;
    type Uid = u64;

    struct SampleShift {
        uid: Uid,
        date: Date,
        start: i32,
        end: i32,
    }
    impl TableCell<Uid, Date> for SampleShift {
        fn d1(&self) -> &Uid {
            &self.uid
        }

        fn d2(&self) -> &Date {
            &self.date
        }
    }

    #[test]
    fn test_sample_shifts_table() {
        let sc1 = SampleShift {
            uid: 1,
            date: 20210930,
            start: 1,
            end: 2,
        };
    }
}
