pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        // pointer is 8 bytes
        // note that the source mentions: It follows that their size is the same as the size of a pointer, a usize.
        // 8 bytes = 8*8 bits = 64 bits, so that's the 64 bit that follows the system
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        // whether it is mutable reference or not, a pointer (reference) is 8 bytes
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        // also a reference, so takes a pointer, i.e., 8 bytes
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
