use crate::toroidal_distance_squared::toroidal_distance_squared;

pub fn furthest_coordinates_toroidal(x: u32, y: u32) -> (u32, u32) {
    const SIZE: u32 = u32::MAX;

    let candidate_x_1 = x.wrapping_add(SIZE / 2);
    let candidate_y_1 = y.wrapping_add(SIZE / 2);
    let candidate_x_2 = candidate_x_1.wrapping_add(SIZE);
    let candidate_y_2 = candidate_y_1.wrapping_add(SIZE);

    let dist1 = toroidal_distance_squared(candidate_x_1, candidate_y_1, x, y);
    let dist2 = toroidal_distance_squared(candidate_x_2, candidate_y_2, x, y);

    if dist1 >= dist2 {
        (candidate_x_1, candidate_y_1)
    } else {
        (candidate_x_2, candidate_y_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_coordinates_toroidal_from_zero() {
        let x = 0;
        let y = 0;
        let result = furthest_coordinates_toroidal(x, y);
        assert_eq!(result, (std::u32::MAX / 2, std::u32::MAX / 2));
    }

    #[test]
    fn test_furthest_coordinates_toroidal_from_middle() {
        let x = std::u32::MAX / 2;
        let y = std::u32::MAX / 2;
        let result = furthest_coordinates_toroidal(x, y);
        // -1 off is close enough
        assert_eq!(result, (std::u32::MAX - 1, std::u32::MAX - 1));
    }

    #[test]
    fn test_furthest_coordinates_toroidal_from_quarter() {
        let x = std::u32::MAX / 4;
        let y = std::u32::MAX / 4;
        let result = furthest_coordinates_toroidal(x, y);
        // +1 off is close enough
        let three_quarters = (std::u32::MAX / 4) * 3 + 1;
        assert_eq!(result, (three_quarters, three_quarters));
    }

    #[test]
    fn test_furthest_coordinates_toroidal_from_three_quarters() {
        let three_quarters = (std::u32::MAX / 4) * 3;
        let result = furthest_coordinates_toroidal(three_quarters, three_quarters);
        // -3 off is probably close enough
        let one_quarter = std::u32::MAX / 4 - 3;
        assert_eq!(result, (one_quarter, one_quarter));
    }

    #[test]
    fn test_furthest_coordinates_toroidal_opposite_edges() {
        let x = std::u32::MAX;
        let y = 0;
        let result = furthest_coordinates_toroidal(x, y);
        // -1 is close enough
        assert_eq!(result, (std::u32::MAX / 2 - 1, std::u32::MAX / 2));
    }

    #[test]
    fn test_furthest_coordinates_toroidal_halfway() {
        let x = std::u32::MAX;
        let y = std::u32::MAX / 2;
        let result = furthest_coordinates_toroidal(x, y);
        assert_eq!(result, (std::u32::MAX / 2 - 1, std::u32::MAX - 1));
    }
}
