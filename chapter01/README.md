# Chapter 01

## Thread Safety: Send and Sync
Single thread - Rc, Cell...

1. Send
A type is Send if it can be sent to another thread. 
Arc<i32> is Send, but Rc<i32> is not

2. Sync
A type is Sync if it can be shared with another thread. 
i32 is Sync, but a Cell<i32> is not.

All primitive types such as i32, bool, and str are both Send and Sync

## Locking: Mutexes and RwLocks
mutex("mutual exclusion")
a mutex has only two states: locked and unlocked

### Lock Poisoning
The unwrap() calls in the examples above relate to lock poisoning.
Calling lock() on a poisoned mutex still locks the mutex. 

### Reader-Writer Lock
A mutex is only concerned with exclusive access.
The MutexGuard will provide us an exclusive reference (&mut T) to the protected data

## Waiting: Parking and Condition Variables

### Thread Parking
One way to wait for a notification from another thread is called thread parking. 

### Condition Variables
Condition variables are a more commonly used option for waiting for something to happen to data protected by a mutex. 