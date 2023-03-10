use crate::fixed_circular_buffer::FixedCircularBuffer;
use crate::sum_event_info_flees::sum_event_info_flees;
use std::collections::HashMap;
use uuid::Uuid;

pub const BUFFER_LENGHT: usize = 16;

struct Event {
    id: Uuid,
}

#[derive(PartialEq, Debug)]
pub struct EventInfo {
    pub follow_x: u64,
    pub follow_y: u64,
    pub flee_x: u64,
    pub flee_y: u64,
}

fn process_event(
    event: &Event,
    buffer: &mut FixedCircularBuffer<Uuid>,
    map: &mut HashMap<Uuid, EventInfo>,
) -> Result<(), Uuid> {
    // TODO: shift incoming event follow towards flees
    // TODO: shift incoming event flee away from follows
    let sum_flee_coordinates = sum_event_info_flees(buffer, map);
    let event_info = EventInfo {
        follow_x: 0,
        follow_y: 0,
        flee_x: 0,
        flee_y: 0,
    };

    buffer.push_front(event.id);
    // prod.push(event.id);
    map.insert(event.id, event_info);
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
            follow_x: 0,
            follow_y: 0,
            flee_x: 0,
            flee_y: 0,
        };
        process_event(&event, &mut buffer, &mut map);

        // assert_eq!(buffer.into().iter()[BUFFER_LENGHT - 2], None);
        assert_eq!(buffer.front(), Some(&event.id));
        assert_eq!(buffer.len(), 1);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&event.id), Some(&event_info));
    }

    #[test]
    fn test_process_event_inserts_second_item() {
        let mut buffer = FixedCircularBuffer::new(64);
        let mut map = HashMap::new();

        let event1 = Event { id: Uuid::new_v4() };
        let event2 = Event { id: Uuid::new_v4() };
        let event1_info = EventInfo {
            follow_x: 0,
            follow_y: 0,
            flee_x: 0,
            flee_y: 0,
        };
        let event2_info = EventInfo {
            follow_x: 0,
            follow_y: 0,
            flee_x: 0,
            flee_y: 0,
        };
        process_event(&event1, &mut buffer, &mut map);
        process_event(&event2, &mut buffer, &mut map);

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.back(), Some(&event1.id));
        assert_eq!(buffer.front(), Some(&event2.id));

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&event1.id), Some(&event1_info));
        assert_eq!(map.get(&event2.id), Some(&event2_info));
    }
}
