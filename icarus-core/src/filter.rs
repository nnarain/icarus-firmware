//
// filter.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 22 2022
//

use heapless::HistoryBuffer;

/// Simple windowed average filter
pub struct Filter<const N: usize> {
    buf: HistoryBuffer<f32, N>,
}

impl<const N: usize> Default for Filter<N> {
    fn default() -> Self {
        Self {
            buf: HistoryBuffer::new(),
        }
    }
}

impl<const N: usize> Filter<N> {
    pub fn update(&mut self, value: f32) {
        self.buf.write(value)
    }

    pub fn value(&self) -> f32 {
        self.buf.as_slice().iter().sum::<f32>() / (self.buf.len() as f32)
    }
}

/// Filter for tri-axial sensor
pub struct TriAxialFilter<const N: usize> {
    x_filter: Filter<N>,
    y_filter: Filter<N>,
    z_filter: Filter<N>,
}

impl<const N: usize> Default for TriAxialFilter<N> {
    fn default() -> Self {
        Self {
            x_filter: Filter::default(),
            y_filter: Filter::default(),
            z_filter: Filter::default(),
        }
    }
}

impl<const N: usize> TriAxialFilter<N> {
    pub fn update<T: Into<(f32, f32, f32)>>(&mut self, data: T) {
        let (x, y, z) = data.into();

        self.x_filter.update(x);
        self.y_filter.update(y);
        self.z_filter.update(z);
    }

    pub fn value(&self) -> (f32, f32, f32) {
        (
            self.get_x(),
            self.get_y(),
            self.get_z(),
        )
    }

    pub fn get_x(&self) -> f32 {
        self.x_filter.value()
    }

    pub fn get_y(&self) -> f32 {
        self.x_filter.value()
    }

    pub fn get_z(&self) -> f32 {
        self.z_filter.value()
    }
}
