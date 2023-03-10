use std::collections::HashMap;

use super::process_event::EventInfo;
use uuid::Uuid;

use super::fixed_circular_buffer::FixedCircularBuffer;

pub fn sum_event_info_flees<'a>(
    buffer: &'a FixedCircularBuffer<Uuid>,
    map: &HashMap<Uuid, EventInfo>,
) -> (u64, u64) {
    let (sum_flee_x, sum_flee_y) = buffer
        .into_iter()
        .filter_map(|uuid| map.get(&uuid))
        .fold((0, 0), |(acc_x, acc_y), event_info| {
            (acc_x + event_info.flee_x, acc_y + event_info.flee_y)
        });
    (sum_flee_x, sum_flee_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::RandomState;
    use std::iter::FromIterator;

    #[test]
    fn test_sum_event_info_flees_normal() {
        let mut buffer = FixedCircularBuffer::new(3);

        let mut map = HashMap::new();
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();
        map.insert(
            uuid1,
            EventInfo {
                flee_x: 1,
                flee_y: 2,
                follow_x: 0,
                follow_y: 0,
            },
        );
        map.insert(
            uuid2,
            EventInfo {
                flee_x: 3,
                flee_y: 4,
                follow_x: 0,
                follow_y: 0,
            },
        );
        map.insert(
            uuid3,
            EventInfo {
                flee_x: 5,
                flee_y: 6,
                follow_x: 0,
                follow_y: 0,
            },
        );

        buffer.push_front(uuid1);
        buffer.push_front(uuid2);
        buffer.push_front(uuid3);

        let (sum_flee_x, sum_flee_y) = sum_event_info_flees(&buffer, &map);
        assert_eq!((sum_flee_x, sum_flee_y), (9, 12));
    }

    #[test]
    fn test_sum_event_info_flees_buffer_overflow() {
        let mut buffer = FixedCircularBuffer::new(3);

        let mut map = HashMap::new();
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();
        let uuid4 = Uuid::new_v4();
        map.insert(
            uuid1,
            EventInfo {
                flee_x: 1,
                flee_y: 2,
                follow_x: 0,
                follow_y: 0,
            },
        );
        map.insert(
            uuid2,
            EventInfo {
                flee_x: 3,
                flee_y: 4,
                follow_x: 0,
                follow_y: 0,
            },
        );
        map.insert(
            uuid3,
            EventInfo {
                flee_x: 5,
                flee_y: 6,
                follow_x: 0,
                follow_y: 0,
            },
        );
        map.insert(
            uuid4,
            EventInfo {
                flee_x: 7,
                flee_y: 8,
                follow_x: 0,
                follow_y: 0,
            },
        );

        buffer.push_front(uuid1);
        buffer.push_front(uuid2);
        buffer.push_front(uuid3);
        buffer.push_front(uuid4);

        let (sum_flee_x, sum_flee_y) = sum_event_info_flees(&buffer, &map);
        assert_eq!((sum_flee_x, sum_flee_y), (15, 18));
    }

    #[test]
    fn test_sum_event_info_flees_empty_buffer() {
        let buffer = FixedCircularBuffer::new(3);
        let map = HashMap::new();

        let (sum_flee_x, sum_flee_y) = sum_event_info_flees(&buffer, &map);
        assert_eq!((sum_flee_x, sum_flee_y), (0, 0));
    }

    #[test]
    fn test_sum_event_info_flees_empty_map() {
        let mut buffer = FixedCircularBuffer::new(3);

        let map = HashMap::new();

        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();

        buffer.push_front(uuid1);
        buffer.push_front(uuid2);
        buffer.push_front(uuid3);

        let (sum_flee_x, sum_flee_y) = sum_event_info_flees(&buffer, &map);
        assert_eq!((sum_flee_x, sum_flee_y), (0, 0));
    }

    #[test]
    fn test_sum_event_info_flees_single_event() {
        let mut buffer = FixedCircularBuffer::new(1);

        let mut map = HashMap::new();
        let uuid1 = Uuid::new_v4();
        map.insert(
            uuid1,
            EventInfo {
                flee_x: 1,
                flee_y: 2,
                follow_x: 0,
                follow_y: 0,
            },
        );

        buffer.push_front(uuid1);

        let (sum_flee_x, sum_flee_y) = sum_event_info_flees(&buffer, &map);
        assert_eq!((sum_flee_x, sum_flee_y), (1, 2));
    }
}
