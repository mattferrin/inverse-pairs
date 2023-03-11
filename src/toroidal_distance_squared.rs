pub fn toroidal_distance_squared(x1: u32, y1: u32, x2: u32, y2: u32) -> u64 {
    const SIZE: u32 = u32::MAX;

    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    let wrapped_dx = if dx > SIZE / 2 { SIZE - dx } else { dx };
    let wrapped_dy = if dy > SIZE / 2 { SIZE - dy } else { dy };

    (wrapped_dx as u64).pow(2) + (wrapped_dy as u64).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toroidal_distance_normal_1() {
        assert_eq!(toroidal_distance_squared(0, 0, 3, 4), 25);
    }

    #[test]
    fn test_toroidal_distance_normal_2() {
        assert_eq!(toroidal_distance_squared(5, 5, 10, 10), 50);
    }

    #[test]
    fn test_toroidal_distance_edge_1() {
        assert_eq!(
            toroidal_distance_squared(0, 0, std::u32::MAX, std::u32::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_2() {
        assert_eq!(
            toroidal_distance_squared(std::u32::MAX, std::u32::MAX, 0, 0),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_3() {
        assert_eq!(
            toroidal_distance_squared(std::u32::MAX, std::u32::MAX, 1, 1),
            2
        );
    }

    #[test]
    fn test_toroidal_distance_edge_4() {
        assert_eq!(toroidal_distance_squared(0, 0, std::u32::MAX, 0), 0);
    }

    #[test]
    fn test_toroidal_distance_edge_5() {
        assert_eq!(toroidal_distance_squared(std::u32::MAX, 0, 0, 0), 0);
    }

    #[test]
    fn test_toroidal_distance_edge_6() {
        assert_eq!(
            toroidal_distance_squared(std::u32::MAX, 0, 0, std::u32::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_7() {
        assert_eq!(
            toroidal_distance_squared(std::u32::MAX, std::u32::MAX, 0, std::u32::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_8() {
        assert_eq!(
            toroidal_distance_squared(0, std::u32::MAX, std::u32::MAX, std::u32::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_squared_small_values() {
        // Test case 1: Small input values
        assert_eq!(toroidal_distance_squared(10, 20, 30, 40), 800);
    }

    #[test]
    fn test_toroidal_distance_squared_inputs_close_to_size() {
        // Test case 2: Inputs close to SIZE
        assert_eq!(
            toroidal_distance_squared(std::u32::MAX - 20, std::u32::MAX - 30, 10, 20),
            3400
        );
    }

    #[test]
    fn test_toroidal_distance_squared_inputs_close_to_size_with_wrapping() {
        // Test case 3: Inputs close to SIZE with wrapping
        assert_eq!(
            toroidal_distance_squared(std::u32::MAX - 20, 30, 10, std::u32::MAX - 20),
            3400
        );
    }

    #[test]
    fn test_toroidal_distance_squared_inputs_wrapping_around_several_times() {
        // Test case 4: Inputs wrapping around several times
        assert_eq!(
            toroidal_distance_squared(10, 20, std::u32::MAX - 30, std::u32::MAX - 40),
            5200
        );
    }

    #[test]
    fn test_toroidal_distance_squared_inputs_wrapping_around_several_times_with_larger_distance() {
        // Test case 5: Inputs wrapping around several times with larger distance
        assert_eq!(
            toroidal_distance_squared(10, 20, std::u32::MAX - 10, std::u32::MAX - 20),
            2000
        );
    }
}
