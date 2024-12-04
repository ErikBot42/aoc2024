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
    let mut s = String::new();
    while let Some(line) = readllq() {
        s.push_str(line.as_str())
    }
    let s = s.as_bytes();
    let n = s.len();

    let mut sum = 0;
    let mut i = 0;
    while i < n {
        if s[i] != b'm' {
            i += 1;
            continue;
        }
        i += 1;
        if i == n {
            break;
        }
        if s[i] != b'u' {
            i += 1;
            continue;
        }
        i += 1;
        if i == n {
            break;
        }
        if s[i] != b'l' {
            i += 1;
            continue;
        }
        i += 1;
        if i == n {
            break;
        }
        if s[i] != b'(' {
            i += 1;
            continue;
        }
        i += 1;
        if i == n {
            break;
        }
        let mut n1 = 0;
        while i != n && matches!(s[i], b'0'..=b'9') {
            n1 *= 10;
            n1 += (s[i] - b'0') as i64;
            i += 1;
        }
        if i == n {
            break;
        }
        if s[i] != b',' {
            i += 1;
            continue;
        }
        i += 1;
        if i == n {
            break;
        }
        let mut n2 = 0;
        while i != n && matches!(s[i], b'0'..=b'9') {
            n2 *= 10;
            n2 += (s[i] - b'0') as i64;
            i += 1;
        }
        if i == n {
            break;
        }
        if s[i] != b')' {
            i += 1;
            continue;
        }
        i += 1;
        sum += n1 * n2
    }
    outln!("{sum}");

    // for c in s.chars() {
    //     match c {
    //         'm' => state = M,
    //         'u' if state == M => state = U,
    //         'u' => state = Start,
    //         'l' if state == U => state = L,
    //         'l' => state = Start,
    //         '(' if state == L => {
    //             state = Open;
    //             n1.clear()
    //         }
    //         '(' => state = Start,
    //         '0'..'9' if state == N1 => {
    //             n1.push(c);
    //             n2.clear()
    //         }
    //         _ => todo!(),
    //     }
    // }

    // (.*mul(<number>,<number>).*)*
}