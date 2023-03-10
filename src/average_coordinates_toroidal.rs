fn average_coordinates_toroidal(x1: u64, y1: u64, x2: u64, y2: u64) -> (u64, u64) {
    let size = std::u64::MAX;
    let dx = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let dy = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    let avg_dx = (if dx <= size / 2 { dx } else { size - dx }) / 2;
    let avg_dy = (if dy <= size / 2 { dy } else { size - dy }) / 2;
    let avg_x = if x1 < x2 {
        (x1 + avg_dx) % size
    } else {
        (x2 + avg_dx) % size
    };
    let avg_y = if y1 < y2 {
        (y1 + avg_dy) % size
    } else {
        (y2 + avg_dy) % size
    };
    (avg_x, avg_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_coordinates_toroidal_when_x1_less_than_x2_and_y1_less_than_y2() {
        let (x1, y1, x2, y2) = (10, 20, 30, 40);
        let expected_result = (20, 30);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_greater_than_x2_and_y1_greater_than_y2() {
        let (x1, y1, x2, y2) = (30, 40, 10, 20);
        let expected_result = (20, 30);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_less_than_x2_and_y1_greater_than_y2() {
        let (x1, y1, x2, y2) = (10, 40, 30, 20);
        let expected_result = (20, 30);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_greater_than_x2_and_y1_less_than_y2() {
        let (x1, y1, x2, y2) = (30, 20, 10, 40);
        let expected_result = (20, 30);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_less_than_y2() {
        let (x1, y1, x2, y2) = (10, 20, 10, 40);
        let expected_result = (10, 30);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_greater_than_y2() {
        let (x1, y1, x2, y2) = (10, 40, 10, 20);
        let expected_result = (10, 30);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_y1_equals_y2_and_x1_less_than_x2() {
        let (x1, y1, x2, y2) = (10, 20, 30, 20);
        let expected_result = (20, 20);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_y1_equals_y2_and_x1_greater_than_x2() {
        let (x1, y1, x2, y2) = (30, 20, 10, 20);
        let expected_result = (20, 20);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_equals_y2() {
        let (x1, y1, x2, y2) = (10, 20, 10, 20);
        let expected_result = (10, 20);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_equals_y2_equals_max() {
        let (x1, y1, x2, y2) = (std::u64::MAX, std::u64::MAX, std::u64::MAX, std::u64::MAX);
        let expected_result = (0, 0);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_equals_y2_equals_0() {
        let (x1, y1, x2, y2) = (0, 0, 0, 0);
        let expected_result = (0, 0);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_equals_y2_and_x1_equals_max() {
        let (x1, y1, x2, y2) = (std::u64::MAX, 10, std::u64::MAX, 10);
        let expected_result = (0, 10);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_x1_equals_x2_and_y1_equals_y2_and_x1_equals_0() {
        let (x1, y1, x2, y2) = (0, 10, 0, 10);
        let expected_result = (0, 10);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_y1_equals_y2_and_x1_equals_x2_equals_max() {
        let (x1, y1, x2, y2) = (std::u64::MAX, std::u64::MAX, std::u64::MAX, std::u64::MAX);
        let expected_result = (0, 0);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_when_y1_equals_y2_and_x1_equals_x2_equals_0() {
        let (x1, y1, x2, y2) = (0, 0, 0, 0);
        let expected_result = (0, 0);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }

    #[test]
    fn test_average_coordinates_toroidal_normal_operation() {
        let (x1, y1, x2, y2) = (100, 200, 300, 400);
        let expected_result = (200, 300);
        assert_eq!(
            average_coordinates_toroidal(x1, y1, x2, y2),
            expected_result
        );
    }
}
