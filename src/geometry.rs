use raylib::math::{Rectangle, Vector2};

/// Check if a rectangle contains a point.
///
/// This function returns true if the given closed rectangle contains the given point and false
/// otherwise. Closed here means that boundary points are treated as part of the rectangle.
///
/// # Examples
///
/// ```
/// use sudoku_solver::geometry::rect_contains_point;
///
/// assert!(rect_contains_point(
///     Rectangle { x: 10.0, y: 20.0, width: 50.0, height: 60.0 },
///     Vector2 { x: 20.0, y: 30.0 },
/// ));
///
/// assert!(!rect_contains_point(
///     Rectangle { x: 10.0, y: 20.0, width: 50.0, height: 60.0 },
///     Vector2 { x: 90.0, 80.0 },
/// ));
/// ```
pub fn rect_contains_point(rect: Rectangle, point: Vector2) -> bool {
    rect.x <= point.x
        && point.x <= rect.x + rect.width
        && rect.y <= point.y
        && point.y <= rect.y + rect.height
}
