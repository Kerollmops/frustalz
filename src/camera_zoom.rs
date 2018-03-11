#[derive(Debug)]
pub struct CameraZoom {
    pub dimensions: (f64, f64),
    pub zoom: f64,
    pub to: (f64, f64),
}

impl CameraZoom {

    pub fn zoom(&self, pos: (f64, f64)) -> (f64, f64) {
        let (x, y) = pos;
        let (width, height) = self.dimensions;

        let (x, y) = (x / width - 0.5, y / height - 0.5);

        let (to_x, to_y) = self.to;
        let (focus_x, focus_y) = (to_x / 2.0, to_y / 2.0);

        let (x, y) = (x + focus_x / self.zoom, y + focus_y / self.zoom);

        (x * self.zoom, y * self.zoom)
    }
}
