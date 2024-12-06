#![allow(unused_imports)]
use std::{
    array::from_fn,
    cmp::{Ordering, Reverse},
    collections::{
        hash_map::DefaultHasher, BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
    },
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    io::{BufRead, StdinLock, StdoutLock, Write},
    iter::FromIterator,
    mem::{replace, swap, take, MaybeUninit},
    num::ParseIntError,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
        DerefMut, Div, DivAssign, Drop, Fn, FnMut, FnOnce, Index, IndexMut, Mul, MulAssign, Neg,
        Not, RangeBounds, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::{FromStr, SplitWhitespace},
};

static mut GLOBAL_STATE: MaybeUninit<GlobalState> = MaybeUninit::uninit();
static mut LINE_ITER: MaybeUninit<SplitWhitespace<'static>> = MaybeUninit::uninit();

struct GlobalState {
    stdout: StdoutLock<'static>,
    stdin: StdinLock<'static>,
    line_buffer: String,
}

#[allow(unused)]
fn default<T: Default>() -> T {
    T::default()
}

fn read<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    loop {
        match unsafe { LINE_ITER.assume_init_mut() }.next() {
            Some(s) => {
                break T::from_str(s).unwrap();
            }
            None => {
                let state = unsafe { GLOBAL_STATE.assume_init_mut() };
                state.line_buffer.clear();
                let _ = state.stdin.read_line(&mut state.line_buffer);
                unsafe { LINE_ITER.write(state.line_buffer.split_whitespace()) };
            }
        }
    }
}

#[allow(unused)]
fn reada<T: FromStr, const LEN: usize>() -> [T; LEN]
where
    <T as FromStr>::Err: Debug,
{
    from_fn(|_| read())
}

#[allow(unused)]
fn readllq() -> Option<String> {
    let stdin = unsafe { &mut GLOBAL_STATE.assume_init_mut().stdin };
    let mut tmp = String::new();
    stdin.read_line(&mut tmp).ok().and_then(|_| {
        while tmp.chars().last().map(|c| c.is_whitespace()) == Some(true) {
            tmp.pop();
        }
        (!tmp.is_empty()).then_some(tmp)
    })
}

#[allow(unused)]
fn readll() -> String {
    readllq().unwrap()
}

#[allow(unused)]
fn readl<T: FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let stdin = unsafe { &mut GLOBAL_STATE.assume_init_mut().stdin };
    let mut tmp = String::new();
    stdin.read_line(&mut tmp);
    let mut out = Vec::new();
    for s in tmp.split_whitespace() {
        out.push(T::from_str(s).unwrap());
    }
    out
}

#[allow(unused)]
macro_rules! out {
    ($($arg:tt)*) => {{
        let stdout = unsafe { &mut GLOBAL_STATE.assume_init_mut().stdout };
        let _ = write!(stdout, $($arg)*);
    }};
}
#[allow(unused)]
macro_rules! outln {
    ($($arg:tt)*) => {{
        let stdout = unsafe { &mut GLOBAL_STATE.assume_init_mut().stdout };
        let _ = writeln!(stdout, $($arg)*);
    }};
}
#[allow(unused)]
macro_rules! flush {
    () => {{
        let stdout = unsafe { &mut GLOBAL_STATE.assume_init_mut().stdout };
        stdout.flush();
    }};
}
#[allow(unused)]
macro_rules! outi {
    ($iterable:tt) => {{
        for item in $iterable {
            out!("{item} ");
        }
        outln!();
    }};
}
#[allow(unused)]
macro_rules! outiln {
    ($iterable:tt) => {{
        for item in $iterable {
            outln!("{item}");
        }
    }};
}

fn init() {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    let line_buffer = String::new();
    let global_state = GlobalState {
        stdin,
        stdout,
        line_buffer,
    };
    unsafe { GLOBAL_STATE.write(global_state) };

    let state = unsafe { GLOBAL_STATE.assume_init_mut() };

    unsafe { LINE_ITER.write(state.line_buffer.split_whitespace()) };
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
use Dir::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Element {
    Empty,
    Visited,
    Occupied,
}

fn main() {
    init();
    let mut board = Vec::new();
    while let Some(line) = readllq() {
        board.push(line.into_bytes());
    }
    let n = board.len();
    let m = board[0].len();
    let mut startx = 0;
    let mut starty = 0;
    for y in 0..n {
        for x in 0..m {
            if board[y][x] == b'^' {
                startx = x;
                starty = y;
            }
        }
    }
    let total = solve(startx, starty, &mut board);

    outln!("{total}");
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    dir: Dir,
}
impl State {
    fn step(self, board: &[Vec<u8>]) -> Option<Self> {
        let n = board.len();
        let m = board[0].len();
        let State {
            mut x,
            mut y,
            mut dir,
        } = self;
        #[rustfmt::skip]
        let (dx, dy, next) = match dir {
            Up =>    (               0, (-1i32) as usize, Right),
            Right => (               1,                0, Down),
            Down =>  (               0,                1, Left),
            Left =>  ((-1i32) as usize,                0, Up),
        };

        let xn = x.wrapping_add(dx);
        let yn = y.wrapping_add(dy);
        if let Some(&e) = board.get(yn).and_then(|row| row.get(xn)) {
            if e == b'#' {
                dir = next;
            } else {
                (x, y) = (xn, yn);
            }
        } else {
            (x, y) = (xn, yn);
        }

        if y < n && x < m {
            Some(State { x, y, dir })
        } else {
            None
        }
    }
}

fn check_cycle(state: State, board: &[Vec<u8>]) -> bool {
    let mut state1 = state;
    let mut state2 = state;
    loop {
        state1 = match state1.step(board) {
            Some(x) => x,
            None => return false,
        };
        state2 = match state2.step(board) {
            Some(x) => x,
            None => return false,
        };
        state2 = match state2.step(board) {
            Some(x) => x,
            None => return false,
        };
        if state1 == state2 {
            return true;
        }
    }
}

fn solve(startx: usize, starty: usize, board: &mut [Vec<u8>]) -> usize {
    let n = board.len();
    let m = board[0].len();
    let mut visited = vec![vec![false; m]; n];

    // --> x
    // |
    // v
    //
    // y
    let mut state = State {
        x: startx,
        y: starty,
        dir: Up,
    };

    visited[starty][startx] = true;

    let mut total = 0;
    loop {
        visited[state.y][state.x] = true;
        let Some(sn) = state.step(board) else {
            break;
        };
        if board[sn.y][sn.x] == b'.' && !visited[sn.y][sn.x] {
            board[sn.y][sn.x] = b'#';
            total += check_cycle(state, &*board) as usize;
            board[sn.y][sn.x] = b'.';
        } else {
        }

        state = sn
    }
    total
}
