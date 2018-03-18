#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub screen_size: [f64; 2],
    pub center: [f64; 2],
    pub zoom: f64,
}

impl Camera {
    pub fn new(screen_size: [f64; 2]) -> Camera {
        Camera {
            screen_size,
            center: [0.0, 0.0],
            zoom: 1.0,
        }
    }

    /// Moves the camera center in order to
    /// keep the given `point` to the same position on screen coordinates.
    ///
    /// Useful when you want a user to zoom into a specific area
    /// without forcing him to change the position of his mouse.
    ///
    /// This is the current way Google Map zoom,
    /// https://stackoverflow.com/questions/22427395
    ///
    /// `point` is the point in screen coordinates (`[0, width[`).
    pub fn target_on(&mut self, point: [f64; 2], zoom: f64) {
        let [cx, cy] = self.center;
        let [x, y] = self.screen_to_world(point);

        // https://stackoverflow.com/a/22700814/1941280
        //
        // move the center keeping the screen distance the same between
        // the center and the targeted point:
        //
        // `cx - x` translate the point to the center
        // `* zoom` reduce this to the current zoom
        // `+ x` translate it back to the original position w/o zoom
        let cx = (cx - x) * zoom + x;
        let cy = (cy - y) * zoom + y;

        self.zoom = zoom;
        self.center = [cx, cy];
    }

    /// Transforms the point in screen coordinates in a point in world coordinates,
    /// taking the screen data and the zoom into account.
    ///
    /// `point` is the point in screen coordinates (`[0, width[`).
    pub fn screen_to_world(&self, point: [f64; 2]) -> [f64; 2] {
        let [sx, sy] = self.screen_size;
        let [cx, cy] = self.center;
        let [x, y] = point;

        assert!(x >= 0.0 && x < sx, "x is outside screen domain coordinates");
        assert!(y >= 0.0 && y < sy, "y is outside screen domain coordinates");

        let screen_ratio = sx / sy;

        // reduce the point to world coordinates:
        //
        // `* 2.0` move to `[0, 2 * width[`
        // `x / sx` reduce to `[0, 2[`
        // `- 1.0` move to `[-1, 1[`
        // `* screen_ratio` keep the screen ratio
        // `* self.zoom` reduce to the current zoom
        // `+ cx` translate the point to the center
        let x = (x * 2.0 / sx - 1.0) * screen_ratio * self.zoom + cx;
        let y = (y * 2.0 / sy - 1.0) * self.zoom + cy;

        [x, y]
    }
}
