mod cdr;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::cdr::{CallRecord, RecordRepository};

    use super::*;

    #[test]
    fn record_repository_mocking() {
        let mut record_repository = RecordRepository::mocked(40);
        assert_eq!(record_repository.calls.len(), 40);
        record_repository.add_call(CallRecord::mocked());
        assert_eq!(record_repository.calls.len(), 41);
    }
}
