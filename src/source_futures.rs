// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use futures_channel::{mpsc, oneshot};
use futures_core::stream::Stream;
use futures_core::task::Context;
use futures_core::{Async, Future, Never};

use MainContext;
use Source;
use Continue;
use Priority;

pub struct SourceFuture<F, T> {
    create_source: Option<F>,
    source: Option<(Source, oneshot::Receiver<T>)>,
}

impl<F, T: 'static> SourceFuture<F, T>
where
    F: FnOnce(oneshot::Sender<T>) -> Source + Send + 'static,
{
    pub fn new(create_source: F) -> Box<Future<Item = T, Error = Never>> {
        Box::new(SourceFuture {
            create_source: Some(create_source),
            source: None,
        })
    }
}

impl<F, T> Future for SourceFuture<F, T>
where
    F: FnOnce(oneshot::Sender<T>) -> Source + Send + 'static,
{
    type Item = T;
    type Error = Never;

    fn poll(&mut self, ctx: &mut Context) -> Result<Async<T>, Never> {
        let SourceFuture {
            ref mut create_source,
            ref mut source,
            ..
        } = *self;

        if let Some(create_source) = create_source.take() {
            let main_context = MainContext::ref_thread_default();
            match main_context {
                None => unreachable!(),
                Some(ref main_context) => {
                    assert!(main_context.is_owner());

                    // Channel for sending back the Source result to our future here.
                    //
                    // In theory we could directly continue polling the
                    // corresponding task from the Source callback,
                    // however this would break at the very least
                    // the g_main_current_source() API.
                    let (send, recv) = oneshot::channel();

                    let s = create_source(send);

                    s.attach(Some(main_context));
                    *source = Some((s, recv));
                }
            }
        }

        // At this point we must have a receiver
        let res = {
            let &mut (_, ref mut receiver) = source.as_mut().unwrap();
            receiver.poll(ctx)
        };
        match res {
            Err(_) => unreachable!(),
            Ok(Async::Ready(v)) => {
                // Get rid of the reference to the source, it triggered
                let _ = source.take();
                Ok(Async::Ready(v))
            }
            Ok(Async::Pending) => Ok(Async::Pending),
        }
    }
}

impl<T, F> Drop for SourceFuture<T, F> {
    fn drop(&mut self) {
        // Get rid of the source, we don't care anymore if it still triggers
        if let Some((source, _)) = self.source.take() {
            source.destroy();
        }
    }
}

pub fn timeout_future(value: u32) -> Box<Future<Item = (), Error = Never>> {
    timeout_future_with_priority(::PRIORITY_DEFAULT, value)
}

pub fn timeout_future_with_priority(priority: Priority, value: u32) -> Box<Future<Item = (), Error = Never>> {
    SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::timeout_source_new(value, None, priority, move || {
            let _ = send.take().unwrap().send(());
            Continue(false)
        })
    })
}

pub fn timeout_future_seconds(value: u32) -> Box<Future<Item = (), Error = Never>> {
    timeout_future_seconds_with_priority(::PRIORITY_DEFAULT, value)
}

pub fn timeout_future_seconds_with_priority(priority: Priority, value: u32) -> Box<Future<Item = (), Error = Never>> {
    SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::timeout_source_new_seconds(value, None, priority, move || {
            let _ = send.take().unwrap().send(());
            Continue(false)
        })
    })
}

pub fn child_watch_future(pid: ::Pid) -> Box<Future<Item = (::Pid, i32), Error = Never>> {
    child_watch_future_with_priority(::PRIORITY_DEFAULT, pid)
}

pub fn child_watch_future_with_priority(priority: Priority, pid: ::Pid) -> Box<Future<Item = (::Pid, i32), Error = Never>> {
    SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::child_watch_source_new(pid, None, priority, move |pid, code| {
            let _ = send.take().unwrap().send((pid, code));
        })
    })
}

#[cfg(any(unix, feature = "dox"))]
pub fn unix_signal_future(signum: i32) -> Box<Future<Item = (), Error = Never>> {
    unix_signal_future_with_priority(::PRIORITY_DEFAULT, signum)
}

#[cfg(any(unix, feature = "dox"))]
pub fn unix_signal_future_with_priority(priority: Priority, signum: i32) -> Box<Future<Item = (), Error = Never>> {
    SourceFuture::new(move |send| {
        let mut send = Some(send);
        ::unix_signal_source_new(signum, None, priority, move || {
            let _ = send.take().unwrap().send(());
            Continue(false)
        })
    })
}

pub struct SourceStream<F, T> {
    create_source: Option<F>,
    source: Option<(Source, mpsc::UnboundedReceiver<T>)>,
}

impl<F, T: 'static> SourceStream<F, T>
where
    F: FnOnce(mpsc::UnboundedSender<T>) -> Source + Send + 'static,
{
    pub fn new(create_source: F) -> Box<Stream<Item = T, Error = Never>> {
        Box::new(SourceStream {
            create_source: Some(create_source),
            source: None,
        })
    }
}

impl<F, T> Stream for SourceStream<F, T>
where
    F: FnOnce(mpsc::UnboundedSender<T>) -> Source + Send + 'static,
{
    type Item = T;
    type Error = Never;

    fn poll_next(&mut self, ctx: &mut Context) -> Result<Async<Option<T>>, Never> {
        let SourceStream {
            ref mut create_source,
            ref mut source,
            ..
        } = *self;

        if let Some(create_source) = create_source.take() {
            let main_context = MainContext::ref_thread_default();
            match main_context {
                None => unreachable!(),
                Some(ref main_context) => {
                    assert!(main_context.is_owner());

                    // Channel for sending back the Source result to our future here.
                    //
                    // In theory we could directly continue polling the
                    // corresponding task from the Source callback,
                    // however this would break at the very least
                    // the g_main_current_source() API.
                    let (send, recv) = mpsc::unbounded();

                    let s = create_source(send);

                    s.attach(Some(main_context));
                    *source = Some((s, recv));
                }
            }
        }

        // At this point we must have a receiver
        let res = {
            let &mut (_, ref mut receiver) = source.as_mut().unwrap();
            receiver.poll_next(ctx)
        };
        match res {
            Err(_) => unreachable!(),
            Ok(Async::Ready(v)) => {
                if v.is_none() {
                    // Get rid of the reference to the source, it triggered
                    let _ = source.take();
                }
                Ok(Async::Ready(v))
            }
            Ok(Async::Pending) => Ok(Async::Pending),
        }
    }
}

impl<T, F> Drop for SourceStream<T, F> {
    fn drop(&mut self) {
        // Get rid of the source, we don't care anymore if it still triggers
        if let Some((source, _)) = self.source.take() {
            source.destroy();
        }
    }
}

pub fn interval_stream(value: u32) -> Box<Stream<Item = (), Error = Never>> {
    interval_stream_with_priority(::PRIORITY_DEFAULT, value)
}

pub fn interval_stream_with_priority(priority: Priority, value: u32) -> Box<Stream<Item = (), Error = Never>> {
    SourceStream::new(move |send| {
        ::timeout_source_new(value, None, priority, move || {
            if send.unbounded_send(()).is_err() {
                Continue(false)
            } else {
                Continue(true)
            }
        })
    })
}


pub fn interval_stream_seconds(value: u32) -> Box<Stream<Item = (), Error = Never>> {
    interval_stream_seconds_with_priority(::PRIORITY_DEFAULT, value)
}

pub fn interval_stream_seconds_with_priority(priority: Priority, value: u32) -> Box<Stream<Item = (), Error = Never>> {
    SourceStream::new(move |send| {
        ::timeout_source_new_seconds(value, None, priority, move || {
            if send.unbounded_send(()).is_err() {
                Continue(false)
            } else {
                Continue(true)
            }
        })
    })
}
