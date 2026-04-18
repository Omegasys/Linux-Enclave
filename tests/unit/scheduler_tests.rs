use crate::core::scheduler::Scheduler;

#[test]
fn test_scheduler_queue() {
    let mut scheduler = Scheduler::new();

    scheduler.submit(vec![1,2,3]);
    assert_eq!(scheduler.len(), 1);

    let task = scheduler.next().unwrap();
    assert_eq!(task, vec![1,2,3]);
}
