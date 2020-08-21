mod executor;

use futures;
use std::{future::Future, pin::Pin, sync::{Arc, Mutex},
          task::{Context, Poll, Waker}, thread, time::Duration};
use crate::executor::run_executor;

fn main() {
    // 我们现在还没有实现调度器，所以要用一下futues库里的一个调度器。
    // futures::executor::block_on(TimerFuture::new(Duration::new(10, 0)));
    run_executor();
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

// 我们想要实现一个定时器Future
pub struct TimerFuture {
    share_state: Arc<Mutex<SharedState>>,
}

// impl Future trait for TimerFuture.
impl Future for TimerFuture {
    type Output = String;
    // executor will run this poll ,and Context is to tell future how to wakeup the task.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut share_state = self.share_state.lock().unwrap();
        if share_state.completed {
            println!("future ready. execute poll to return.");
            Poll::Ready(String::from("timer done."))
        } else {
            println!("future not ready, tell the future task how to wakeup to executor");
            // 你要告诉future，当事件就绪后怎么唤醒任务去调度执行，而这个waker根具体的调度器有关
            // 调度器执行的时候会将上下文信息传进来，里面最重要的一项就是Waker
            share_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let share_state = Arc::new(Mutex::new(SharedState{completed:false, waker:None}));
        let thread_shared_state = share_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut share_state = thread_shared_state.lock().unwrap();
            share_state.completed = true;
            if let Some(waker) = share_state.waker.take() {
                println!("detect future is ready, wakeup the future task to executor.");
                waker.wake()    // wakeup the future task to executor.
            }
        });

        TimerFuture {share_state}
    }
}
