use crate::{fixed_circular_buffer::FixedCircularBuffer, process_event::EventInfo};
use std::collections::HashMap;
use uuid::Uuid;

// TODO: make toroidal wrapping work
pub fn toroidal_rolling_flee_average(
    event_map: &HashMap<Uuid, EventInfo>,
    event_buffer: &FixedCircularBuffer<Uuid>,
    previous_average: &Option<(u32, u32)>,
) -> Option<(u32, u32)> {
    let buffer_len = event_buffer.len() as u32;
    let buffer_capacity = event_buffer.capacity as u32;

    if buffer_len == 0 {
        return None;
    }

    let buffer_previous_length = buffer_len - 1;

    if let Some(latest_uuid) = event_buffer.front() {
        let latest_info = event_map.get(&latest_uuid);

        if let Some(latest) = latest_info {
            if let Some(prev) = previous_average {
                if buffer_capacity == buffer_len {
                    if let Some(oldest_uuid) = event_buffer.back() {
                        let oldest_info = event_map.get(&oldest_uuid);

                        if let Some(info) = oldest_info {
                            let x = ((prev.0 * buffer_previous_length) - info.flee_x
                                + latest.flee_x)
                                / buffer_previous_length;
                            let y = ((prev.1 * buffer_previous_length) - info.flee_y
                                + latest.flee_y)
                                / buffer_previous_length;

                            return Some((x, y));
                        } else {
                            return None;
                        }
                    }
                } else {
                    let x = ((prev.0 * buffer_previous_length) + latest.flee_x) / buffer_len;
                    let y = ((prev.1 * buffer_previous_length) + latest.flee_y) / buffer_len;

                    return Some((x, y));
                }
            } else {
                return Some((latest.flee_x, latest.flee_y));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toroidal_rolling_average_with_empty_input() {
        let empty_map = HashMap::new();
        let empty_buffer = FixedCircularBuffer::new(0);
        let previous_rolling_average = None;
        assert_eq!(
            toroidal_rolling_flee_average(&empty_map, &empty_buffer, &previous_rolling_average),
            None
        );
    }

    #[test]
    fn test_toroidal_rolling_average_with_one_item_in_buffer() {
        let mut map = HashMap::new();
        let item_uuid = Uuid::new_v4();
        let event_info = EventInfo {
            flee_x: 3,
            flee_y: 7,
            follow_x: u32::MAX,
            follow_y: u32::MAX,
        };
        map.insert(item_uuid, event_info);

        let mut buffer = FixedCircularBuffer::new(1);
        buffer.push_front(item_uuid);

        let previous_rolling_average = None;

        assert_eq!(
            toroidal_rolling_flee_average(&map, &buffer, &previous_rolling_average),
            Some((3, 7))
        );
    }

    #[test]
    fn test_toroidal_rolling_average_with_previous_rolling_average_and_one_item_in_buffer() {
        let mut map = HashMap::new();
        let item_uuid = Uuid::new_v4();
        let event_info = EventInfo {
            flee_x: 1,
            flee_y: 2,
            follow_x: 0,
            follow_y: 0,
        };
        map.insert(item_uuid, event_info);

        let mut buffer = FixedCircularBuffer::new(3);
        buffer.push_front(item_uuid);

        // this previous rolling average is ignored because it should be None in reality
        let previous_rolling_average = Some((3, 4));

        assert_eq!(
            toroidal_rolling_flee_average(&map, &buffer, &previous_rolling_average),
            Some((1, 2))
        );
    }

    #[test]
    fn test_toroidal_rolling_average_with_previous_rolling_average_and_three_items_in_buffer() {
        let mut map = HashMap::new();
        let item_uuid_1 = Uuid::new_v4();
        let item_uuid_2 = Uuid::new_v4();
        let item_uuid_3 = Uuid::new_v4();
        let event_info_3 = EventInfo {
            flee_x: 9,
            flee_y: 12,
            follow_x: 0,
            follow_y: 0,
        };
        map.insert(item_uuid_3, event_info_3);

        let mut buffer = FixedCircularBuffer::new(4);
        buffer.push_front(item_uuid_1);
        buffer.push_front(item_uuid_2);
        buffer.push_front(item_uuid_3);

        let previous_rolling_average = Some((3, 6));

        assert_eq!(
            toroidal_rolling_flee_average(&map, &buffer, &previous_rolling_average),
            Some((5, 8))
        );
    }

    #[test]
    fn test_toroidal_rolling_average_x_with_capacity_full() {
        let mut map = HashMap::new();
        let item_uuid_1 = Uuid::parse_str("849761d6-e58f-423d-82fb-69ac2889408e").unwrap();
        let event_info_1 = EventInfo {
            flee_x: 4,
            flee_y: 0,
            follow_x: 0,
            follow_y: 0,
        };
        map.insert(item_uuid_1, event_info_1);
        let item_uuid_2 = Uuid::parse_str("7a2d65a7-b338-4f7f-891b-3c612ea36d73").unwrap();
        let item_uuid_3 = Uuid::parse_str("249e486c-e7f3-40c9-a33d-9159fcd1e5ca").unwrap();
        let event_info_3 = EventInfo {
            flee_x: 12,
            flee_y: 0,
            follow_x: 0,
            follow_y: 0,
        };
        map.insert(item_uuid_3, event_info_3);

        let mut buffer = FixedCircularBuffer::new(3);
        buffer.push_front(item_uuid_1);
        buffer.push_front(item_uuid_2);
        buffer.push_front(item_uuid_3);

        let previous_rolling_average = Some((3, 0));

        assert_eq!(
            toroidal_rolling_flee_average(&map, &buffer, &previous_rolling_average),
            Some((7, 0))
        );
    }
}
