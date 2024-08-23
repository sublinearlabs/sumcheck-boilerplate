use ark_ff::PrimeField;
use std::marker::PhantomData;

/// Structure for point iteration over boolean hypercube
/// e.g. BooleanHyperCube 2 variables
/// Some(00), Some(01), Some(10), Some(11), None
pub struct BooleanHyperCube<F: PrimeField> {
    bit_size: usize,
    total_points: usize,
    current_point: usize,
    _marker: PhantomData<F>,
}

impl<F: PrimeField> BooleanHyperCube<F> {
    pub fn new(bit_size: usize) -> Self {
        Self {
            bit_size,
            total_points: 2_usize.pow(bit_size as u32),
            current_point: 0,
            _marker: PhantomData,
        }
    }
}

impl<F: PrimeField> Iterator for BooleanHyperCube<F> {
    type Item = Vec<F>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point == self.total_points || self.bit_size == 0 {
            return None;
        }

        // convert the current index to binary value of the given length
        let index_as_binary = binary_string(self.current_point, self.bit_size);
        let point = index_as_binary
            .chars()
            .map(|a| if a == '1' { F::one() } else { F::zero() })
            .collect::<Vec<F>>();

        self.current_point += 1;

        Some(point)
    }
}

pub fn binary_string(index: usize, bit_count: usize) -> String {
    let binary = format!("{:b}", index);
    "0".repeat(bit_count.saturating_sub(binary.len())) + &binary
}