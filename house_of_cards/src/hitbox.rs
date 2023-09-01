use macroquad::prelude as mq;

// Math from: https://chat.openai.com/share/f0826594-c5e9-4ea6-a1cf-0a010295fbfa

pub trait Circle {
    fn center(&self) -> mq::Vec2;
    fn radius(&self) -> f32;
}

pub trait Rectangle {
    fn position(&self) -> mq::Vec2;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn rotation(&self) -> f32;
}

pub fn circles_collide(circle1: &dyn Circle, circle2: &dyn Circle) -> bool {
    let delta = circle2.center() - circle1.center();
    let distance_squared = delta.x * delta.x + delta.y * delta.y;
    let combined_radius = circle1.radius() + circle2.radius();
    distance_squared <= combined_radius * combined_radius
}

pub fn rectangles_collide(rect1: &dyn Rectangle, rect2: &dyn Rectangle) -> bool {
    // Implement rectangle-rectangle collision logic considering rotation

    // Convert rotation angles to sine and cosine values
    let cos1 = rect1.rotation().cos();
    let sin1 = rect1.rotation().sin();
    let cos2 = rect2.rotation().cos();
    let sin2 = rect2.rotation().sin();

    // Calculate the corners of the first rectangle
    let corners1 = [
        rect1.position(),
        rect1.position()
            + mq::Vec2 {
                x: cos1 * rect1.width(),
                y: sin1 * rect1.width(),
            },
        rect1.position()
            - mq::Vec2 {
                x: sin1 * rect1.height(),
                y: cos1 * rect1.height(),
            },
        rect1.position()
            + mq::Vec2 {
                x: cos1 * rect1.width() - sin1 * rect1.height(),
                y: sin1 * rect1.width() + cos1 * rect1.height(),
            },
    ];

    // Calculate the corners of the second rectangle
    let corners2 = [
        rect2.position(),
        rect2.position()
            + mq::Vec2 {
                x: cos2 * rect2.width(),
                y: sin2 * rect2.width(),
            },
        rect2.position()
            - mq::Vec2 {
                x: sin2 * rect2.height(),
                y: cos2 * rect2.height(),
            },
        rect2.position()
            + mq::Vec2 {
                x: cos2 * rect2.width() - sin2 * rect2.height(),
                y: sin2 * rect2.width() + cos2 * rect2.height(),
            },
    ];

    // Check for overlapping in each dimension
    let overlapping_x = corners1.iter().any(|corner1| {
        corners2
            .iter()
            .any(|corner2| corner1.x <= corner2.x && corner2.x <= corner1.x + rect1.width())
    });
    let overlapping_y = corners1.iter().any(|corner1| {
        corners2
            .iter()
            .any(|corner2| corner1.y <= corner2.y && corner2.y <= corner1.y + rect1.height())
    });

    overlapping_x && overlapping_y
}

pub fn rectangle_circle_collide(rect: &dyn Rectangle, circle: &dyn Circle) -> bool {
    // Convert rotation angle to sine and cosine values
    let cos = rect.rotation().cos();
    let sin = rect.rotation().sin();

    // Transform circle position into the rotated rectangle's coordinate system
    let delta = circle.center() - rect.position();
    let transformed_x = delta.x * cos + delta.y * sin;
    let transformed_y = -delta.x * sin + delta.y * cos;

    // Check if the transformed circle position is within the rectangle
    let half_width = rect.width() / 2.0;
    let half_height = rect.height() / 2.0;
    let within_x = -half_width <= transformed_x && transformed_x <= half_width;
    let within_y = -half_height <= transformed_y && transformed_y <= half_height;

    // If the transformed position is within both dimensions, there's a collision
    within_x && within_y
}
