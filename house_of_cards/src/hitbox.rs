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

// struct Circle {
//     center: (f32, f32),
//     radius: f32,
// }

// struct Rectangle {
//     position: (f32, f32),
//     width: f32,
//     height: f32,
//     rotation: f32, // in radians
// }

// fn circles_collide(circle1: &Circle, circle2: &Circle) -> bool {
//     let dx = circle2.center().0 - circle1.center().0;
//     let dy = circle2.center().1 - circle1.center().1;
//     let distance_squared = dx * dx + dy * dy;
//     let combined_radius = circle1.radius() + circle2.radius();
//     distance_squared <= combined_radius * combined_radius
// }

// fn rectangles_collide(rect1: &Rectangle, rect2: &Rectangle) -> bool {
//     // Convert rotation angles to sine and cosine values
//     let cos1 = rect1.rotation().cos();
//     let sin1 = rect1.rotation().sin();
//     let cos2 = rect2.rotation().cos();
//     let sin2 = rect2.rotation().sin();

//     // Calculate the corners of the first rectangle
//     let corners1 = [
//         (rect1.position().0, rect1.position().1),
//         (
//             rect1.position().0 + cos1 * rect1.width(),
//             rect1.position().1 + sin1 * rect1.width(),
//         ),
//         (
//             rect1.position().0 - sin1 * rect1.height(),
//             rect1.position().1 + cos1 * rect1.height(),
//         ),
//         (
//             rect1.position().0 + cos1 * rect1.width() - sin1 * rect1.height(),
//             rect1.position().1 + sin1 * rect1.width() + cos1 * rect1.height(),
//         ),
//     ];

//     // Calculate the corners of the second rectangle
//     let corners2 = [
//         (rect2.position().0, rect2.position().1),
//         (
//             rect2.position().0 + cos2 * rect2.width(),
//             rect2.position().1 + sin2 * rect2.width(),
//         ),
//         (
//             rect2.position().0 - sin2 * rect2.height(),
//             rect2.position().1 + cos2 * rect2.height(),
//         ),
//         (
//             rect2.position().0 + cos2 * rect2.width() - sin2 * rect2.height(),
//             rect2.position().1 + sin2 * rect2.width() + cos2 * rect2.height(),
//         ),
//     ];

//     // Check for overlapping in each dimension
//     let overlapping_x =
//         corners1.iter().any(|&(x1, _)| corners2.iter().any(|&(x2, _)| x1 <= x2 && x2 <= x1 + rect1.width()));
//     let overlapping_y =
//         corners1.iter().any(|&(_, y1)| corners2.iter().any(|&(_, y2)| y1 <= y2 && y2 <= y1 + rect1.height()));

//     overlapping_x && overlapping_y
// }

// fn rectangle_circle_collide(rect: &Rectangle, circle: &Circle) -> bool {
//     // Convert rotation angle to sine and cosine values
//     let cos = rect.rotation().cos();
//     let sin = rect.rotation().sin();

//     // Transform circle position into the rotated rectangle's coordinate system
//     let dx = circle.center().0 - rect.position().0;
//     let dy = circle.center().1 - rect.position().1;
//     let transformed_x = dx * cos + dy * sin;
//     let transformed_y = -dx * sin + dy * cos;

//     // Check if the transformed circle position is within the rectangle
//     let half_width = rect.width() / 2.0;
//     let half_height = rect.height() / 2.0;
//     let within_x = -half_width <= transformed_x && transformed_x <= half_width;
//     let within_y = -half_height <= transformed_y && transformed_y <= half_height;

//     // If the transformed position is within both dimensions, there's a collision
//     within_x && within_y
// }
