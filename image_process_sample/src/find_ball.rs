use opencv::{core::Point2i, prelude::*};

struct Ball {
    position: Point2i,
    radius: i32,
}

impl Ball {
    pub fn new(position: Point2i, radius: i32) -> Self {
        Self { position, radius }
    }
    pub fn position(&self) -> Point2i {
        self.position
    }
    pub fn radius(&self) -> i32 {
        self.radius
    }

    pub fn find(&self, frame: &Mat) -> bool {
        let mut gray = Mat::default();
        opencv::imgproc::cvt_color(&frame, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0).unwrap();
        let mut circles = opencv::types::VectorOfVec3f::new();
        opencv::imgproc::hough_circles(
            &gray,
            &mut circles,
            opencv::imgproc::HOUGH_GRADIENT,
            1.0,
            20.0,
            100.0,
            30.0,
            0,
            0,
        )
        .unwrap();
        for circle in circles {
            let center = Point2i::new(circle[0] as i32, circle[1] as i32);
            let radius = circle[2] as i32;
        }
        false
    }
}
