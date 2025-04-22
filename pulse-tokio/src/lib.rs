use libc::timeval;
use libpulse_binding::context::{self, Context};
use libpulse_binding::def::{Retval, RetvalActual};
use libpulse_binding::mainloop::api::DeferEventCb;
use libpulse_binding::mainloop::api::DeferEventDestroyCb;
use libpulse_binding::mainloop::api::IoEventCb;
use libpulse_binding::mainloop::api::IoEventDestroyCb;
use libpulse_binding::mainloop::api::Mainloop as MainloopTrait;
use libpulse_binding::mainloop::api::MainloopApi;
use libpulse_binding::mainloop::api::TimeEventCb;
use libpulse_binding::mainloop::api::TimeEventDestroyCb;
use libpulse_binding::mainloop::api::{MainloopInnerType, MainloopInternalType};
use libpulse_binding::mainloop::events::deferred::DeferEventInternal;
use libpulse_binding::mainloop::events::io::FlagSet as IoEventFlagSet;
use libpulse_binding::mainloop::events::io::IoEventInternal;
use libpulse_binding::mainloop::events::timer::TimeEventInternal;
use std::cell::{Cell, UnsafeCell};
use std::fmt;
use std::future::{Future,poll_fn};
use std::os::raw::c_void;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::io::unix::AsyncFd;

struct Fd(RawFd);

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

enum Item {
    Defer {
        main: Weak<MainInner>,
        dead: bool,
        enabled: bool,
        cb: Option<DeferEventCb>,
        userdata: *mut c_void,
        free: Option<DeferEventDestroyCb>,
    },
    Timer {
        main: Weak<MainInner>,
        dead: bool,
        ts: Cell<Option<Duration>>,
        cb: Option<TimeEventCb>,
        userdata: *mut c_void,
        free: Option<TimeEventDestroyCb>,
    },
    Event {
        main: Weak<MainInner>,
        dead: Cell<bool>,
        fd: i32,
        afd: Cell<Option<AsyncFd<Fd>>>,
        cb: Option<IoEventCb>,
        events: Cell<IoEventFlagSet>,
        userdata: *mut c_void,
        free: Option<IoEventDestroyCb>,
    },
}

impl Item {
    fn is_dead(&self) -> bool {
        match self {
            Item::Defer { dead, .. } | Item::Timer { dead, .. } => *dead,
            Item::Event { dead, .. } => dead.get(),
        }
    }

    fn kill(&mut self) {
        match self {
            Item::Defer { dead, .. } | Item::Timer { dead, .. } => {
                *dead = true;
            }
            Item::Event { .. } => unreachable!(),
        }
    }
}

/// An implementation of the [pulse](libpulse_binding) [Mainloop](MainloopTrait) trait that
/// dispatches through tokio.
#[derive(Debug)]
pub struct TokioMain {
    mi: Rc<MainInner>,
}

/// The state structure passed to pulse.
pub struct MainInner {
    api: MainloopApi,
    /// Note: items are stored as raw pointers because the actual items are also available to C and
    /// via iter_get_item.  Otherwise, they are Box pointers owned by this vector.
    items: UnsafeCell<Vec<*mut Item>>,
    /// Note: only allow access following the rules for Pin
    sleep: UnsafeCell<Option<tokio::time::Sleep>>,
    waker: Cell<Option<task::Waker>>,
    quit: Cell<Option<RetvalActual>>,
}

impl fmt::Debug for MainInner {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "MainInner")
    }
}

impl MainloopTrait for TokioMain {
    type MI = MainInner;
    fn inner(&self) -> Rc<MainInner> {
        self.mi.clone()
    }
}

impl MainloopInternalType for MainInner {}

impl MainloopInnerType for MainInner {
    type I = Self;
    fn get_ptr(&self) -> *mut Self {
        panic!("This function is not well-defined and is never called")
    }

    fn get_api_ptr(&self) -> *const MainloopApi {
        &self.api
    }

    fn get_api(&self) -> &MainloopApi {
        &self.api
    }

    fn supports_rtclock(&self) -> bool {
        false
    }
}

impl Drop for MainInner {
    fn drop(&mut self) {
        unsafe {
            // drop the cyclic weak reference set up by TokioMain::new
            Weak::from_raw(self.api.userdata as *mut MainInner);
            // drop any remaining items (they should all be dead anyway)
            for item in self.items.get_mut().drain(..) {
                drop(Box::from_raw(item));
            }
        }
    }
}

impl TokioMain {
    pub fn new() -> Self {
        let mut mi = Rc::new(MainInner {
            api: MainloopApi {
                userdata: 0 as *mut _,
                io_new: Some(MainInner::io_new),
                io_enable: Some(MainInner::io_enable),
                io_free: Some(MainInner::io_free),
                io_set_destroy: Some(MainInner::io_set_destroy),
                time_new: Some(MainInner::time_new),
                time_restart: Some(MainInner::time_restart),
                time_free: Some(MainInner::time_free),
                time_set_destroy: Some(MainInner::time_set_destroy),
                defer_new: Some(MainInner::defer_new),
                defer_enable: Some(MainInner::defer_enable),
                defer_free: Some(MainInner::defer_free),
                defer_set_destroy: Some(MainInner::defer_set_destroy),
                quit: Some(MainInner::quit),
            },
            items: UnsafeCell::new(Vec::new()),
            sleep: UnsafeCell::new(None),
            waker: Cell::new(None),
            quit: Cell::new(None),
        });
        let v = Rc::get_mut(&mut mi).unwrap();
        v.api.userdata = v as *mut MainInner as *mut _;
        let _cyclic = Rc::downgrade(&mi).into_raw();
        TokioMain { mi }
    }

    fn iter_get_item(&mut self, i: usize) -> Option<(&MainloopApi, &Item)> {
        let api = &self.mi.api;
        let items = unsafe { &mut *self.mi.items.get() };
        loop {
            if i >= items.len() {
                return None;
            }
            if unsafe { (*items[i]).is_dead() } {
                let mut dead = unsafe { Box::from_raw(items.swap_remove(i)) };
                match &*dead {
                    &Item::Defer {
                        free: Some(cb),
                        userdata,
                        ..
                    } => {
                        let raw_item = &mut *dead as *mut Item;
                        cb(api, raw_item as *mut _, userdata);
                    }
                    &Item::Timer {
                        free: Some(cb),
                        userdata,
                        ..
                    } => {
                        let raw_item = &mut *dead as *mut Item;
                        cb(api, raw_item as *mut _, userdata);
                    }
                    &Item::Event {
                        free: Some(cb),
                        userdata,
                        ..
                    } => {
                        let raw_item = &mut *dead as *mut Item;
                        cb(api, raw_item as *mut _, userdata);
                    }
                    _ => {}
                }
                drop(dead);
                continue;
            }
            let item = unsafe { &*items[i] };
            return Some((api, item));
        }
    }

    /// Run callbacks and register wakers for the pulse mainloop.
    ///
    /// This returns Ready if a callback was invoked, or Pending if everything is waiting on timers
    /// or I/O.  The async run() or wait_for_ready() functions call this function internally.
    pub fn tick(&mut self, ctx: &mut task::Context) -> task::Poll<Option<Retval>> {
        let inow = tokio::time::Instant::now();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut wake = None::<Duration>;
        let mut rv = task::Poll::Pending;
        let mut i = 0;
        self.mi.waker.set(Some(ctx.waker().clone()));
        while let Some((api, item)) = self.iter_get_item(i) {
            let raw_item = item as *const Item;
            i += 1;
            match &*item {
                &Item::Defer {
                    enabled: true,
                    cb: Some(cb),
                    userdata,
                    ..
                } => {
                    cb(api, raw_item as *mut _, userdata);
                }
                &Item::Defer { .. } => continue,
                &Item::Timer { cb: None, .. } => continue,
                &Item::Timer {
                    cb: Some(cb),
                    userdata,
                    ref ts,
                    ..
                } => {
                    match ts.replace(None) {
                        Some(ts) if ts < now => {
                            rv = task::Poll::Ready(None);
                            let tv = timeval {
                                tv_sec: ts.as_secs() as i64,
                                tv_usec: ts.subsec_micros() as i64,
                            };
                            cb(api, raw_item as *mut _, &tv, userdata);
                        }
                        later => ts.set(later),
                    }

                    if let Some(ts) = ts.get() {
                        if wake.is_some() {
                            wake = std::cmp::min(wake, Some(ts));
                        } else {
                            wake = Some(ts);
                        }
                    }
                }
                &Item::Event { cb: None, .. } => continue,
                &Item::Event {
                    cb: Some(cb),
                    userdata,
                    fd,
                    ref afd,
                    ref events,
                    ref dead,
                    ..
                } => {
                    // Operate on a local, mutable version of the AsyncFd and restore it later
                    let mut local_fd = afd.take();

                    // this creation of AsyncFd is a bit lazy to allow deletions to happen
                    // first, since tokio may reject attempts to add duplicate FDs
                    let async_fd = local_fd
                        .get_or_insert_with(|| AsyncFd::new(Fd(fd)).expect("Pulse fed a bad FD"));
                    let mut ready = IoEventFlagSet::NULL;
                    let mut rg = None;
                    let mut wg = None;
                    if events.get().contains(IoEventFlagSet::INPUT) {
                        match async_fd.poll_read_ready(ctx) {
                            task::Poll::Ready(Ok(g)) => {
                                ready |= IoEventFlagSet::INPUT;
                                rg = Some(g);
                            }
                            task::Poll::Ready(Err(_)) => ready |= IoEventFlagSet::ERROR,
                            task::Poll::Pending => {}
                        }
                    }
                    if events.get().contains(IoEventFlagSet::OUTPUT) {
                        match async_fd.poll_write_ready(ctx) {
                            task::Poll::Ready(Ok(g)) => {
                                ready |= IoEventFlagSet::OUTPUT;
                                wg = Some(g);
                            }
                            task::Poll::Ready(Err(_)) => ready |= IoEventFlagSet::ERROR,
                            task::Poll::Pending => {}
                        }
                    }
                    if ready == IoEventFlagSet::NULL {
                        afd.set(local_fd);
                        continue;
                    }

                    rv = task::Poll::Ready(None);
                    cb(api, raw_item as *mut _, fd, ready, userdata);
                    if dead.get() {
                        // don't restore afd if it was supposed to be dead
                        continue;
                    }
                    let wants = events.get();
                    if wants.intersects(ready) {
                        // pulse still wants an event that was reported as ready.  We might
                        // need to inform tokio that the FD is not ready
                        let mut pfd = libc::pollfd {
                            fd: fd,
                            events: 0,
                            revents: 0,
                        };
                        if wants.contains(IoEventFlagSet::INPUT) && rg.is_some() {
                            pfd.events |= libc::POLLIN;
                        }
                        if wants.contains(IoEventFlagSet::OUTPUT) && wg.is_some() {
                            pfd.events |= libc::POLLOUT;
                        }
                        unsafe {
                            libc::poll(&mut pfd, 1, 0);
                        }
                        if let Some(mut g) = rg {
                            if (pfd.revents & libc::POLLIN) != 0 {
                                g.retain_ready();
                            } else {
                                g.clear_ready();
                            }
                        }
                        if let Some(mut g) = wg {
                            if (pfd.revents & libc::POLLOUT) != 0 {
                                g.retain_ready();
                            } else {
                                g.clear_ready();
                            }
                        }
                    }
                    afd.set(local_fd);
                }
            }
        }
        if let Some(ret) = self.mi.quit.replace(None) {
            return task::Poll::Ready(Some(Retval(ret)));
        }
        if rv.is_pending() {
            // Note: we can't pin the Rc normally because the Mainloop trait inner() returns an
            // unpinned Rc.  However, the value is in fact pinned because the TokioMain::inner Rc
            // is not directly exposed (preventing Rc::try_unwrap from succeeding), and the Drop
            // impl for that struct overwrites the field with None.
            let mut sleep = unsafe { Pin::new_unchecked(&mut *self.mi.sleep.get()) };
            if let Some(d) = wake {
                sleep.set(Some(tokio::time::sleep_until(inow + d)));
                match sleep.as_mut().as_pin_mut().map(|f| f.poll(ctx)) {
                    Some(task::Poll::Ready(())) => {
                        sleep.set(None);
                        rv = task::Poll::Ready(None);
                    }
                    _ => {}
                }
            } else {
                sleep.set(None);
            }
        }
        rv
    }

    /// Run the mainloop until the given context is either Ready or Failed/Terminated.
    ///
    /// When initializing a single Context object, this can be simpler use than registering a state
    /// callback.  You will need to call run on another task to actually use the context.
    pub async fn wait_for_ready(&mut self, ctx: &Context) -> Result<context::State, Retval> {
        loop {
            match poll_fn(|ctx| self.tick(ctx)).await {
                Some(rv) => return Err(rv),
                None => {}
            }
            let s = ctx.get_state();
            match s {
                context::State::Ready | context::State::Failed | context::State::Terminated => {
                    return Ok(s);
                }
                _ => {}
            }
        }
    }

    /// Run the mainloop until a quit is requested through the pulse API
    pub async fn run(&mut self) -> Retval {
        loop {
            match poll_fn(|ctx| self.tick(ctx)).await {
                Some(rv) => return rv,
                None => (),
            }
        }
    }
}

impl Drop for TokioMain {
    fn drop(&mut self) {
        let mut sleep = unsafe { Pin::new_unchecked(&mut *self.mi.sleep.get()) };
        sleep.set(None);
        // It is now safe to move MainInner
    }
}

impl MainInner {
    unsafe fn from_api(api: *const MainloopApi) -> Rc<Self> {
        let ptr = Weak::from_raw((*api).userdata as *const Self);
        let rv = ptr.upgrade();
        let _ = ptr.into_raw(); // we only want to borrow the Weak, not own it...
        rv.expect("Called from_api on a dropped MainloopApi")
    }

    fn push(&self, item: Box<Item>) {
        let items = unsafe { &mut *self.items.get() };
        items.push(Box::into_raw(item));
    }

    fn wake(main: &Weak<MainInner>) {
        main.upgrade().map(|inner| inner.wake_real());
    }

    fn wake_real(&self) {
        self.waker.take().map(|waker| waker.wake());
    }

    extern "C" fn io_new(
        a: *const MainloopApi,
        fd: i32,
        events: IoEventFlagSet,
        cb: Option<IoEventCb>,
        userdata: *mut c_void,
    ) -> *mut IoEventInternal {
        unsafe {
            let inner = MainInner::from_api(a);
            let events = Cell::new(events);
            let mut item = Box::new(Item::Event {
                fd,
                cb,
                events,
                userdata,
                free: None,
                afd: Cell::new(None),
                dead: Cell::new(false),
                main: Rc::downgrade(&inner),
            });
            let rv = &mut *item as *mut Item as *mut _;
            inner.push(item);
            inner.wake_real();
            rv
        }
    }
    extern "C" fn io_enable(e: *mut IoEventInternal, new: IoEventFlagSet) {
        unsafe {
            let item: *mut Item = e.cast();
            match &*item {
                Item::Event { main, events, .. } => {
                    events.set(new);
                    MainInner::wake(main);
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn io_free(e: *mut IoEventInternal) {
        unsafe {
            let item: *mut Item = e.cast();
            match &*item {
                Item::Event { dead, afd, .. } => {
                    dead.set(true);
                    afd.set(None);
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn io_set_destroy(e: *mut IoEventInternal, cb: Option<IoEventDestroyCb>) {
        unsafe {
            let item: *mut Item = e.cast();
            match &mut *item {
                Item::Event { free, .. } => {
                    *free = cb;
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn time_new(
        a: *const MainloopApi,
        tv: *const timeval,
        cb: Option<TimeEventCb>,
        userdata: *mut c_void,
    ) -> *mut TimeEventInternal {
        unsafe {
            let inner = MainInner::from_api(a);
            let tv = tv.read();
            let ts = Cell::new(Some(
                Duration::from_secs(tv.tv_sec as u64) + Duration::from_micros(tv.tv_usec as u64),
            ));
            let mut item = Box::new(Item::Timer {
                main: Rc::downgrade(&inner),
                ts,
                cb,
                userdata,
                free: None,
                dead: false,
            });
            let rv = &mut *item as *mut Item as *mut _;
            inner.push(item);
            inner.wake_real();
            rv
        }
    }
    extern "C" fn time_restart(e: *mut TimeEventInternal, tv: *const timeval) {
        unsafe {
            let item: *mut Item = e.cast();
            match &*item {
                Item::Timer { main, ts, .. } => {
                    let tv = tv.read();
                    ts.set(Some(
                        Duration::from_secs(tv.tv_sec as u64)
                            + Duration::from_micros(tv.tv_usec as u64),
                    ));
                    MainInner::wake(main);
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn time_free(e: *mut TimeEventInternal) {
        unsafe {
            let item: *mut Item = e.cast();
            (*item).kill();
        }
    }
    extern "C" fn time_set_destroy(e: *mut TimeEventInternal, cb: Option<TimeEventDestroyCb>) {
        unsafe {
            let item: *mut Item = e.cast();
            match &mut *item {
                Item::Timer { free, .. } => {
                    *free = cb;
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn defer_new(
        a: *const MainloopApi,
        cb: Option<DeferEventCb>,
        userdata: *mut c_void,
    ) -> *mut DeferEventInternal {
        unsafe {
            let inner = MainInner::from_api(a);
            let mut item = Box::new(Item::Defer {
                main: Rc::downgrade(&inner),
                cb,
                userdata,
                free: None,
                dead: false,
                enabled: true,
            });
            let rv = &mut *item as *mut Item as *mut _;
            inner.push(item);
            inner.wake_real();
            rv
        }
    }
    extern "C" fn defer_enable(e: *mut DeferEventInternal, b: i32) {
        unsafe {
            let item: *mut Item = e.cast();
            match &mut *item {
                Item::Defer { main, enabled, .. } => {
                    *enabled = b != 0;
                    if b != 0 {
                        MainInner::wake(main);
                    }
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn defer_free(e: *mut DeferEventInternal) {
        unsafe {
            let item: *mut Item = e.cast();
            (*item).kill();
        }
    }
    extern "C" fn defer_set_destroy(e: *mut DeferEventInternal, cb: Option<DeferEventDestroyCb>) {
        unsafe {
            let item: *mut Item = e.cast();
            match &mut *item {
                Item::Defer { free, .. } => {
                    *free = cb;
                }
                _ => panic!(),
            }
        }
    }
    extern "C" fn quit(a: *const MainloopApi, retval: RetvalActual) {
        unsafe {
            let inner = MainInner::from_api(a);
            inner.quit.set(Some(retval));
            inner.wake_real();
        }
    }
}
