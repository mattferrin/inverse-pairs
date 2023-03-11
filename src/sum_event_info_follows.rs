use std::collections::HashMap;

use super::process_event::EventInfo;
use uuid::Uuid;

use super::fixed_circular_buffer::FixedCircularBuffer;

pub fn sum_event_info_follows<'a>(
    buffer: &'a FixedCircularBuffer<Uuid>,
    map: &HashMap<Uuid, EventInfo>,
) -> (u64, u64) {
    let (sum_follow_x, sum_follow_y) = buffer.into_iter().filter_map(|uuid| map.get(&uuid)).fold(
        (0, 0),
        |(acc_x, acc_y), event_info| {
            (
                acc_x + event_info.follow_x as u64,
                acc_y + event_info.follow_y as u64,
            )
        },
    );
    (sum_follow_x, sum_follow_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_event_info_follows_normal() {
        let mut buffer = FixedCircularBuffer::new(3);

        let mut map = HashMap::new();
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();
        map.insert(
            uuid1,
            EventInfo {
                follow_x: 1,
                follow_y: 2,
                flee_x: 0,
                flee_y: 0,
            },
        );
        map.insert(
            uuid2,
            EventInfo {
                follow_x: 3,
                follow_y: 4,
                flee_x: 0,
                flee_y: 0,
            },
        );
        map.insert(
            uuid3,
            EventInfo {
                follow_x: 5,
                follow_y: 6,
                flee_x: 0,
                flee_y: 0,
            },
        );

        buffer.push_front(uuid1);
        buffer.push_front(uuid2);
        buffer.push_front(uuid3);

        let (sum_follow_x, sum_follow_y) = sum_event_info_follows(&buffer, &map);
        assert_eq!((sum_follow_x, sum_follow_y), (9, 12));
    }

    #[test]
    fn test_sum_event_info_follows_buffer_overflow() {
        let mut buffer = FixedCircularBuffer::new(3);

        let mut map = HashMap::new();
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();
        let uuid4 = Uuid::new_v4();
        map.insert(
            uuid1,
            EventInfo {
                follow_x: 1,
                follow_y: 2,
                flee_x: 0,
                flee_y: 0,
            },
        );
        map.insert(
            uuid2,
            EventInfo {
                follow_x: 3,
                follow_y: 4,
                flee_x: 0,
                flee_y: 0,
            },
        );
        map.insert(
            uuid3,
            EventInfo {
                follow_x: 5,
                follow_y: 6,
                flee_x: 0,
                flee_y: 0,
            },
        );
        map.insert(
            uuid4,
            EventInfo {
                follow_x: 7,
                follow_y: 8,
                flee_x: 0,
                flee_y: 0,
            },
        );

        buffer.push_front(uuid1);
        buffer.push_front(uuid2);
        buffer.push_front(uuid3);
        buffer.push_front(uuid4);

        let (sum_follow_x, sum_follow_y) = sum_event_info_follows(&buffer, &map);
        assert_eq!((sum_follow_x, sum_follow_y), (15, 18));
    }

    #[test]
    fn test_sum_event_info_follows_empty_buffer() {
        let buffer = FixedCircularBuffer::new(3);
        let map = HashMap::new();

        let (sum_follow_x, sum_follow_y) = sum_event_info_follows(&buffer, &map);
        assert_eq!((sum_follow_x, sum_follow_y), (0, 0));
    }

    #[test]
    fn test_sum_event_info_follows_empty_map() {
        let mut buffer = FixedCircularBuffer::new(3);

        let map = HashMap::new();

        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();

        buffer.push_front(uuid1);
        buffer.push_front(uuid2);
        buffer.push_front(uuid3);

        let (sum_follow_x, sum_follow_y) = sum_event_info_follows(&buffer, &map);
        assert_eq!((sum_follow_x, sum_follow_y), (0, 0));
    }

    #[test]
    fn test_sum_event_info_follows_single_event() {
        let mut buffer = FixedCircularBuffer::new(1);

        let mut map = HashMap::new();
        let uuid1 = Uuid::new_v4();
        map.insert(
            uuid1,
            EventInfo {
                follow_x: 1,
                follow_y: 2,
                flee_x: 0,
                flee_y: 0,
            },
        );

        buffer.push_front(uuid1);

        let (sum_follow_x, sum_follow_y) = sum_event_info_follows(&buffer, &map);
        assert_eq!((sum_follow_x, sum_follow_y), (1, 2));
    }
}
