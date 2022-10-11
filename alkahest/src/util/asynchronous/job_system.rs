use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use crate::{trace, error};

static mut CURRENT_LABEL: u64 = 0;
static FINISHED_LABEL: AtomicU64 = AtomicU64::new(0);

pub(crate) trait Job: Send + Sync + JobClone {
    fn run(&self, index: u32) -> ();
}

pub(crate) trait JobClone {
    fn clone_box(&self) -> Box<dyn Job>;
}

impl<T> JobClone for T where T: 'static + Job + Clone {
    fn clone_box(&self) -> Box<dyn Job> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Job> {
    fn clone(&self) -> Box<dyn Job> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct GroupJob {
    job_count: u32,
    group_size: u32,
    group_index: u32,
    inner_job: Box<dyn Job>,
}

impl Job for GroupJob {
    fn run(&self, _index: u32) {
        let group_job_offset = self.group_index * self.group_size;
        let group_job_end = std::cmp::min(group_job_offset + self.group_size, self.job_count);

        for job_index in group_job_offset..group_job_end {
            self.inner_job.run(job_index);
        }
    }
}

pub(crate) struct AsyncContext {
    sender: flume::Sender<Box<dyn Job>>,
    should_stop: Arc<AtomicBool>,
    cv_pair: Arc<(Mutex<bool>, Condvar)>,
}

pub(crate) fn init() -> AsyncContext {
    let (tx, rx) = flume::unbounded();

    let thread_count = match std::thread::available_parallelism() {
        Ok(count) => usize::from(count),
        Err(_) => 1usize,
    };

    let should_stop = Arc::new(AtomicBool::new(false));
    let cv_pair = Arc::new((Mutex::new(false), Condvar::new()));

    for _ in 0..thread_count {
        let rx_clone: flume::Receiver<Box<dyn Job>> = rx.clone();
        let should_stop_clone = should_stop.clone();
        let cv_pair_clone = cv_pair.clone();
        std::thread::spawn(move ||{
            trace!("Starting worker thread: {:?}", std::thread::current().id());

            while !should_stop_clone.load(Ordering::SeqCst) {
                match rx_clone.try_recv() {
                    Ok(job) => {
                        job.run(0);    
                        FINISHED_LABEL.fetch_add(1, Ordering::SeqCst);
                    },
                    Err(_) => {
                        let (lock, cvar) = &*cv_pair_clone;
                        let mx = lock.lock().unwrap();
                        let _unused = cvar.wait(mx).unwrap();
                    }
                }
            }
        });
    }

    return AsyncContext { sender: tx, should_stop, cv_pair };
}

pub(crate) fn cleanup(context: &AsyncContext) {
    context.should_stop.store(true, Ordering::SeqCst);
    context.cv_pair.1.notify_all();
}

impl AsyncContext {
    pub(super) fn poll(&self) {
        // We wake one worker and yield our timeslice back to the OS
        self.cv_pair.1.notify_one();
        std::thread::yield_now();
    }

    pub(crate) fn dispatch(&self, job: Box<dyn Job>) -> Result<(), flume::SendError<Box<dyn Job>>> {
        unsafe {
            CURRENT_LABEL = CURRENT_LABEL + 1;
            // The below call blocks! Need to set up a non-blocking method!
            self.sender.send(job)
        }
    }

    pub(crate) fn dispatch_many(&self, job_count: u32, group_size: u32, job: Box<dyn Job>) -> Result<(), flume::SendError<Box<dyn Job>>> {
        if job_count + group_size == 0 {
            error!("AsyncContext.dispatch_many() called with invalid arguments!");
            return Ok(());
        }
        
        let group_count = (job_count + group_size - 1) / group_size;
        for group_index in 0..group_count {
            let group_job = GroupJob { group_index, group_size, job_count, inner_job: job.clone() };
            // The below call blocks! Need to set up a non-blocking method!
            self.sender.send(Box::new(group_job) as Box<dyn Job>)?;
        }

        Ok(())
    }

    pub(crate) fn is_busy(&self) -> bool {
        unsafe {
            FINISHED_LABEL.load(Ordering::SeqCst) < CURRENT_LABEL
        }
    }

    pub(crate) fn wait(&self) {
        while self.is_busy() {
            self.poll();
        }
    }
}
