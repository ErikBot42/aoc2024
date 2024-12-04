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

fn main() {
    init();
    let mut board: Vec<Vec<u8>> = Vec::new();
    while let Some(line) = readllq() {
        board.push(line.into_bytes());
    }
    board.push((0..board[0].len()).map(|_| b'.').collect());
    board.push((0..board[0].len()).map(|_| b'.').collect());
    for i in 0..board.len() {
        board[i].push(b'.');
        board[i].push(b'.');
    }

    // ---> x
    // |
    // |
    // V
    // y    true

    let mut xl: Vec<Vec<_>> = board
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    let mut xr: Vec<Vec<_>> = board
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    let mut draw: Vec<Vec<_>> = board
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    let word = b"MAS";

    for y0 in 0..board.len() {
        for x0 in 0..board[y0].len() {
            for (dx, dy, f) in [
                (1, 1, true),
                (usize::MAX, usize::MAX, true),
                (usize::MAX, 1, false),
                (1, usize::MAX, false),
            ] {
                let mut x = x0;
                let mut y = y0;
                let mut ok = true;
                for i in 0..word.len() {
                    let Some(&e) = board.get(y).and_then(|row| row.get(x)) else {
                        ok = false;
                        break;
                    };
                    if e != word[i] {
                        ok = false;
                        break;
                    }
                    x = x.wrapping_add(dx);
                    y = y.wrapping_add(dy);
                }
                if ok {
                    let xx = if f { &mut xl } else { &mut xr };
                    let mut x = x0;
                    let mut y = y0;
                    for i in 0..word.len() {
                        draw[y][x] = true;
                        x = x.wrapping_add(dx);
                        y = y.wrapping_add(dy);
                    }

                    let x = x0.wrapping_add(dx);
                    let y = y0.wrapping_add(dy);
                    xx[y][x] = true;
                }
            }
        }
    }
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if xl[y][x]&&xr[y][x] {
                out!("a");

            } else if draw[y][x] {
                out!("{}", board[y][x] as char);
            } else {
                out!(".");
            }
        }
        outln!();
    }
    let mut total = 0;
    for y in 0..xl.len() {
        for x in 0..xl[0].len() {
            total += (xl[y][x] && xr[y][x]) as usize;
        }
    }
    outln!("{total}");
}
