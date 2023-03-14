use crate::fixed_circular_buffer::FixedCircularBuffer;
use crate::furthest_coordinates_toroidal::furthest_coordinates_toroidal;
use crate::toroidal_rolling_flee_average::toroidal_rolling_flee_average;
use std::collections::HashMap;
use uuid::Uuid;

struct Event {
    id: Uuid,
}

#[derive(PartialEq, Debug)]
pub struct EventInfo {
    pub follow_x: u32,
    pub follow_y: u32,
    pub flee_x: u32,
    pub flee_y: u32,
}

// TODO: fix tests since in the middle of swapping out average with rolling toroidal average still being written
fn process_event(
    event: &Event,
    buffer: &mut FixedCircularBuffer<Uuid>,
    map: &mut HashMap<Uuid, EventInfo>,
    previous_flee_average: &Option<(u32, u32)>,
) -> Result<(), Uuid> {
    let sum_flee_coordinates =
        toroidal_rolling_flee_average(map, buffer, previous_flee_average).unwrap();

    if map.contains_key(&event.id) {
        // TODO: shift incoming event follow towards flees
        // TODO: shift incoming event flee away from follows
        println!("Event is in hashmap!");
    } else {
        let (flee_anti_x, flee_anti_y) =
            furthest_coordinates_toroidal(sum_flee_coordinates.0, sum_flee_coordinates.1);

        // TODO: fix broken test because you can't just sum and then take the average in toroidal space
        println!(
            "sums {:?} {:?}",
            sum_flee_coordinates.0, sum_flee_coordinates.1,
        );
        println!("opposites {:?} {:?}", flee_anti_x, flee_anti_y);

        // The idea is to place initial points far from each other and continue some consistent rule.
        let event_info = EventInfo {
            follow_x: flee_anti_x,
            follow_y: flee_anti_y,
            flee_x: 0,
            flee_y: 0,
        };
        map.insert(event.id, event_info);
    }

    buffer.push_front(event.id);
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_event_inserts_first_item() {
        let mut buffer = FixedCircularBuffer::<Uuid>::new(64);
        let mut map = HashMap::new();

        let event = Event { id: Uuid::new_v4() };
        let event_info = EventInfo {
            follow_x: u32::MAX / 2,
            follow_y: u32::MAX / 2,
            flee_x: u32::MAX / 2,
            flee_y: u32::MAX / 2,
        };
        process_event(&event, &mut buffer, &mut map, &Some((0, 0)));

        assert_eq!(buffer.front(), Some(&event.id));
        assert_eq!(buffer.len(), 1);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&event.id), Some(&event_info));
    }

    #[test]
    fn test_process_event_inserts_second_item() {
        let mut buffer = FixedCircularBuffer::new(64);
        let mut map = HashMap::new();

        let event1 = Event {
            id: Uuid::parse_str("fa84077a-7a27-48cf-b6f4-0becc82b09ac").unwrap(),
        };
        let event2 = Event {
            id: Uuid::parse_str("96d9a909-87ce-4b94-a877-462fdc56831d").unwrap(),
        };
        let event1_info = EventInfo {
            follow_x: u32::MAX / 2,
            follow_y: u32::MAX / 2,
            flee_x: u32::MAX / 2,
            flee_y: u32::MAX / 2,
        };
        let event2_info = EventInfo {
            follow_x: u32::MAX - 1,
            follow_y: u32::MAX - 1,
            flee_x: u32::MAX - 1,
            flee_y: u32::MAX - 1,
        };
        process_event(&event1, &mut buffer, &mut map, &Some((0, 0)));
        process_event(&event2, &mut buffer, &mut map, &Some((0, 0)));

        assert_eq!(buffer.len(), 2);
        assert_eq!(
            buffer.back().unwrap(),
            &Uuid::parse_str("fa84077a-7a27-48cf-b6f4-0becc82b09ac").unwrap()
        );
        assert_eq!(
            buffer.front().unwrap(),
            &Uuid::parse_str("96d9a909-87ce-4b94-a877-462fdc56831d").unwrap()
        );

        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get(&Uuid::parse_str("fa84077a-7a27-48cf-b6f4-0becc82b09ac").unwrap())
                .unwrap(),
            &event1_info
        );
        assert_eq!(
            map.get(&Uuid::parse_str("96d9a909-87ce-4b94-a877-462fdc56831d").unwrap())
                .unwrap(),
            &event2_info
        );
    }

    #[test]
    fn test_process_event_adds_same_id_to_buffer_twice() {
        let mut buffer = FixedCircularBuffer::new(3); // set buffer length to 2
        let mut map = HashMap::new();

        let event1 = Event {
            id: Uuid::parse_str("fa84077a-7a27-48cf-b6f4-0becc82b09ac").unwrap(),
        };
        process_event(&event1, &mut buffer, &mut map, &Some((0, 0))).unwrap();

        // check if the entry is updated correctly by calling process_event again
        process_event(&event1, &mut buffer, &mut map, &Some((0, 0))).unwrap();

        // check that the buffer still inserts the id
        assert_eq!(
            *buffer.front().unwrap(),
            Uuid::parse_str("fa84077a-7a27-48cf-b6f4-0becc82b09ac").unwrap()
        );
        assert_eq!(
            *buffer.back().unwrap(),
            Uuid::parse_str("fa84077a-7a27-48cf-b6f4-0becc82b09ac").unwrap()
        );

        // check buffer length is 2
        assert_eq!(buffer.len(), 2);

        // check hashmap length is 1
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_process_event_average_toroidal_point() {
        // Create a buffer with two events already in it
        let id_1 = Uuid::parse_str("f4ecfb47-3f6f-4f11-b5bf-67146c3afcfd").unwrap();
        let id_2 = Uuid::parse_str("13b94a73-5a1b-407a-98a4-26d41ddfc9e5").unwrap();
        let mut buffer = FixedCircularBuffer::new(4);
        buffer.push_front(id_1);
        buffer.push_front(id_2);

        // Create a map with the follow and flee points for the two events
        let mut map = HashMap::new();
        let eigth = u32::MAX / 8;
        map.insert(
            id_1,
            EventInfo {
                follow_x: eigth,
                follow_y: eigth * 3,
                flee_x: eigth * 5,
                flee_y: eigth,
            },
        );
        map.insert(
            id_2,
            EventInfo {
                follow_x: u32::MAX - eigth,
                follow_y: eigth * 5,
                flee_x: eigth * 3,
                flee_y: u32::MAX - eigth,
            },
        );
        let id_3 = Uuid::parse_str("95893064-fbf9-41ec-b5d7-632bc76bbe9a").unwrap();
        let event_3 = Event { id: id_3 };

        // Call the process_event function to add the third event
        let result = process_event(&event_3, &mut buffer, &mut map, &Some((0, 0)));
        assert!(result.is_ok());

        // Check that the third event is now in the buffer
        let buffer_contents: Vec<Uuid> = buffer.into_iter().collect();
        assert_eq!(buffer_contents.len(), 3);
        assert_eq!(buffer_contents[0], id_3);
        assert_eq!(buffer_contents[1], id_2);
        assert_eq!(buffer_contents[2], id_1);

        // Check that the follow and flee points for the third event have been averaged
        let event3_info = map.get(&id_3).unwrap();
        assert_eq!(event3_info.follow_x, u32::MAX);
        assert_eq!(event3_info.flee_x, u32::MAX / 2);
        assert_eq!(event3_info.follow_y, u32::MAX / 2);
        assert_eq!(event3_info.flee_y, u32::MAX - 4);
    }
}
