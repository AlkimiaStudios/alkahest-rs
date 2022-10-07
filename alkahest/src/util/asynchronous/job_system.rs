use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use crate::{trace, error};

static mut CURRENT_LABEL: u64 = 0;
static FINISHED_LABEL: AtomicU64 = AtomicU64::new(0);

pub(crate) struct AsyncContext {
    sender: flume::Sender<&'static Job>,
    should_stop: Arc<AtomicBool>,
    cv_pair: Arc<(Mutex<bool>, Condvar)>,
}

pub(crate) struct Job {
    task: fn() -> (),
    id: u32,
}

pub(crate) fn init() -> AsyncContext {
    let (tx, rx) = flume::bounded(256);

    let thread_count = match std::thread::available_parallelism() {
        Ok(count) => usize::from(count),
        Err(_) => 1usize,
    };

    let should_stop = Arc::new(AtomicBool::new(false));
    let cv_pair = Arc::new((Mutex::new(false), Condvar::new()));

    for _ in 0..thread_count {
        let rx_clone: flume::Receiver<&Job> = rx.clone();
        let should_stop_clone = should_stop.clone();
        let cv_pair_clone = cv_pair.clone();
        std::thread::spawn(move ||{
            trace!("Starting worker thread: {:?}", std::thread::current().id());

            while !should_stop_clone.load(Ordering::Relaxed) {
                match rx_clone.try_recv() {
                    Ok(job) => {
                        (job.task)();    
                        FINISHED_LABEL.fetch_add(1, Ordering::Relaxed);
                    },
                    Err(_) => {
                        let (lock, cvar) = &*cv_pair_clone;
                        let mx = lock.lock().unwrap();
                        let _ = cvar.wait(mx).unwrap();
                    }
                }
            }
        });
    }

    return AsyncContext { sender: tx, should_stop, cv_pair };
}

pub(crate) fn cleanup(context: &AsyncContext) {
    context.should_stop.store(true, Ordering::Relaxed);
    context.cv_pair.1.notify_all();
}

impl AsyncContext {
    pub(super) fn poll(&self) {
        self.cv_pair.1.notify_one();
    }

    pub(crate) fn dispatch(&self, job: &'static Job) {
        unsafe {
            CURRENT_LABEL = CURRENT_LABEL + 1;
            let mut submitted = false;
            while !submitted {
                match self.sender.try_send(job) {
                    Ok(()) => { submitted = true; },
                    Err(_) => { self.poll(); },
                }
            }
        }
    }

    pub(crate) fn dispatch_many(&self, job_count: u32, group_size: u32, job: &'static Job) {
        if job_count + group_size == 0 {
            return;
        }

        let group_count = (job_count + group_size - 1) / group_size;
        for group_index in 0..group_count {
            let job_task = || {
                let group_job_offset = group_index * group_size;
                let group_job_end = std::cmp::min(group_job_offset + group_size, job_count);

                for _ in group_job_offset..group_job_end {
                    (job.task)();
                }
            };

            let group_job = Job { task: job_task, id: group_index };
            self.dispatch(&group_job);
        }
    }

    pub(crate) fn is_busy(&self) -> bool {
        unsafe {
            FINISHED_LABEL.load(Ordering::Relaxed) < CURRENT_LABEL
        }
    }

    pub(crate) fn wait(&self) {
        while self.is_busy() {
            self.poll();
        }
    }
}
