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
    let word = b"XMAS";
    let mut total = 0;
    let mut map = HashMap::new();
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            total += solve(x, y, &board, word.as_slice(), &mut map);
        }
    }
    // for y in 0..board.len() {
    //     for x in 0..board[y].len() {
    //         if let Some(&c) = map.get(&(x, y)) {
    //             assert_eq!(board[y][x], b'X');
    //             out!("{c}");
    //         } else {
    //             out!(".");
    //         }
    //     }
    //     outln!();
    // }
    // outln!();

    // for y in 0..board.len() {
    //     for x in 0..board[y].len() {
    //         out!("{}", board[y][x] as char);
    //     }
    //     outln!();
    // }

    outln!("{total}");
}

fn solve(
    x0: usize,
    y0: usize,
    board: &[Vec<u8>],
    word: &[u8],
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let mut total = 0;
    for dx in [usize::MAX, 0, 1] {
        for dy in [usize::MAX, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut ok = true;
            let mut x = x0;
            let mut y = y0;
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
                *map.entry((x0, y0)).or_default() += 1;
                total += 1;
            }
        }
    }
    total
}
