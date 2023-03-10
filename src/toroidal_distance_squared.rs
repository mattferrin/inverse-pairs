fn toroidal_distance_squared(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let size = std::u64::MAX;
    let dx = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let dy = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    let dx = dx.min(size - dx);
    let dy = dy.min(size - dy);
    dx * dx + dy * dy
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
            toroidal_distance_squared(0, 0, std::u64::MAX, std::u64::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_2() {
        assert_eq!(
            toroidal_distance_squared(std::u64::MAX, std::u64::MAX, 0, 0),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_3() {
        assert_eq!(
            toroidal_distance_squared(std::u64::MAX, std::u64::MAX, 1, 1),
            2
        );
    }

    #[test]
    fn test_toroidal_distance_edge_4() {
        assert_eq!(toroidal_distance_squared(0, 0, std::u64::MAX, 0), 0);
    }

    #[test]
    fn test_toroidal_distance_edge_5() {
        assert_eq!(toroidal_distance_squared(std::u64::MAX, 0, 0, 0), 0);
    }

    #[test]
    fn test_toroidal_distance_edge_6() {
        assert_eq!(
            toroidal_distance_squared(std::u64::MAX, 0, 0, std::u64::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_7() {
        assert_eq!(
            toroidal_distance_squared(std::u64::MAX, std::u64::MAX, 0, std::u64::MAX),
            0
        );
    }

    #[test]
    fn test_toroidal_distance_edge_8() {
        assert_eq!(
            toroidal_distance_squared(0, std::u64::MAX, std::u64::MAX, std::u64::MAX),
            0
        );
    }
}
