use std::collections::VecDeque;
use tokio::net::TcpStream;
use std::time::{Instant, Duration};
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::thread;
use futures::task::{self, ArcWake};
use crossbeam::channel;
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::thread::sleep;

async fn delay(dur: Duration) {

    struct Delay {
        when: Instant,
        waker: Option<Arc<Mutex<Waker>>>,
    }

    impl Future for Delay {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
                -> Poll<()>
        {

            if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();

                if !waker.will_wake(cx.waker()) {
                    *waker = cx.waker().clone();
                }
            } else {
                let when = self.when;
                let waker = Arc::new(Mutex::new(cx.waker().clone()));
                self.waker = Some(waker.clone());

                thread::spawn(move || {
                    let now = Instant::now();
                    if now < when {
                        thread::sleep(when - now);
                    }

                    let waker = waker.lock().unwrap();
                    waker.wake_by_ref();
                });
            }
            if Instant::now() >= self.when {
                Poll::Ready(())
            } else {
                // // Get a handle to the waker for the current task
                // let waker = cx.waker().clone();
                // let when = self.when;
                //
                // // Spawn a timer thread.
                // thread::spawn(move || {
                //     let now = Instant::now();
                //
                //     if now < when {
                //         thread::sleep(when - now);
                //     }
                //
                //     waker.wake();
                // });

                Poll::Pending
            }
        }
    }

    // Create an instance of our `Delay` future.
    let future = Delay {
        when: Instant::now() + dur,
        waker: None,
    };

    // Wait for the duration to complete.
    future.await;
}

async fn my_async_fn() {
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async TCP operation complete");
}

#[tokio::main]
async fn main() {
    // let when = Instant::now() + Duration::from_millis(10);
    // let future = Delay { when };
    //
    // let out = future.await;
    // assert_eq!(out, "done");
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {

        spawn(async {
            let mut index = 1;
            while index < 30 {
                println!("{}", index);
                index += 1;
                delay(Duration::from_secs(1)).await;
            }
        });

        spawn(async {
            delay(Duration::from_secs(10)).await;
            println!("world");
        });
        // let when = Instant::now() + Duration::from_millis(10);
        // let future = Delay { when };

        // Spawn a second task
        spawn(async {
            delay(Duration::from_secs(25)).await;
            println!("hello");
        });

        delay(Duration::from_secs(60)).await;
        std::process::exit(0);
        // let out = future.await;
        // assert_eq!(out, "done");
    });

    mini_tokio.run();
}

// Used to track the current mini-tokio instance so that the `spawn` function is
// able to schedule spawned tasks.
thread_local! {
    static CURRENT: RefCell<Option<channel::Sender<Arc<Task>>>> =
        RefCell::new(None);
}

pub fn spawn<F>(future: F)
    where
        F: Future<Output = ()> + Send + 'static,
{
    CURRENT.with(|cell| {
        let borrow = cell.borrow();
        let sender = borrow.as_ref().unwrap();
        Task::spawn(future, sender);
    });
}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    // The `Mutex` is to make `Task` implement `Sync`. Only
    // one thread accesses `future` at any given time. The
    // `Mutex` is not required for correctness. Real Tokio
    // does not use a mutex here, but real Tokio has
    // more lines of code than can fit in a single tutorial
    // page.
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { sender, scheduled }
    }

    /// Spawn a future onto the mini-tokio instance.
    fn spawn<F>(&mut self, future: F)
        where
            F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {

        CURRENT.with(|cell| {
            *cell.borrow_mut() = Some(self.sender.clone());
        });

        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
        // let waker = task::noop_waker();
        // let mut cx = Context::from_waker(&waker);
        //
        // while let Some(mut task) = self.tasks.pop_front() {
        //     if task.as_mut().poll(&mut cx).is_pending() {
        //         self.tasks.push_back(task);
        //     }
        // }
    }
}

impl Task {
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
        where
            F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

    fn poll(self: Arc<Self>) {
        // Create a waker from the `Task` instance. This
        // uses the `ArcWake` impl from above.
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // No other thread ever tries to lock the future
        let mut future = self.future.try_lock().unwrap();

        // Poll the future
        let _ = future.as_mut().poll(&mut cx);
    }
}
