use std::{fmt::Debug, mem::MaybeUninit};

const L1_TABLE_BYTES: usize = 4096;
const L2_MAX_TABLE_BYTES: usize = 4096;
const L3_MAX_TABLE_BYTES: usize = 4096;

const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

const BYTES_PER_USIZE: usize = std::mem::size_of::<usize>();
const BITS_PER_USIZE: usize = BYTES_PER_USIZE * 8;

const L1_BIT_WIDTH: usize = (L1_TABLE_BYTES * BYTES_PER_USIZE).trailing_zeros() as usize;
const L2_BIT_WIDTH: usize = min(
    32 - L1_BIT_WIDTH,
    (L2_MAX_TABLE_BYTES / BYTES_PER_USIZE).trailing_zeros() as usize,
);
const L3_BIT_WIDTH: usize = min(
    32 - L1_BIT_WIDTH - L2_BIT_WIDTH,
    (L3_MAX_TABLE_BYTES / BYTES_PER_USIZE).trailing_zeros() as usize,
);
const L4_BIT_WIDTH: usize = 32 - L1_BIT_WIDTH - L2_BIT_WIDTH - L3_BIT_WIDTH;

const fn width_to_entries(bit_width: usize) -> usize {
    ((bit_width > 0) as usize) << bit_width
}

type L1 = Chunks<{ (1 << L1_BIT_WIDTH) / BITS_PER_USIZE }>;
type L2 = Table<{ width_to_entries(L2_BIT_WIDTH) }, L1>;
type L3 = Table<{ width_to_entries(L3_BIT_WIDTH) }, L2>;
type L4 = Table<{ width_to_entries(L4_BIT_WIDTH) }, L3>;

#[derive(Debug)]
enum State {
    Init,
    OneLevel { start_idx: u32, chunks: Box<L1> },
    TwoLevel { start_idx: u32, tables: Box<L2> },
    ThreeLevel { start_idx: u32, tables: Box<L3> },
    FourLevel { tables: Box<L4> },
}

#[derive(Debug)]
pub struct RadixBitSet {
    state: State,
    len: u32, // number of bits set
}
impl Default for RadixBitSet {
    fn default() -> Self {
        RadixBitSet::new()
    }
}
impl RadixBitSet {
    pub fn new() -> Self {
        RadixBitSet {
            state: State::Init,
            len: 0,
        }
    }

    fn expand_if_necessary(state: State, bit_idx: u32) -> State {
        match state {
            State::Init => {
                let start_idx = bit_idx & !(L1::MASK | L1::CHILD_MASK);
                let chunks = Box::new(Chunks::new());
                State::OneLevel { start_idx, chunks }
            }
            State::OneLevel { start_idx, chunks }
                if bit_idx & !(L1::MASK | L1::CHILD_MASK) == start_idx =>
            {
                // no expansion necessary
                State::OneLevel { start_idx, chunks }
            }
            State::OneLevel { start_idx, chunks } => {
                // expand
                let mut tables = Box::new(Table::new());
                *tables.get_entry_mut(start_idx) = Some(chunks);
                Self::expand_if_necessary(
                    State::TwoLevel {
                        start_idx: start_idx & !(L2::MASK | L2::CHILD_MASK),
                        tables,
                    },
                    bit_idx,
                )
            }
            State::TwoLevel { start_idx, tables }
                if bit_idx & !(L2::MASK | L2::CHILD_MASK) == start_idx =>
            {
                // no expansion necessary
                State::TwoLevel { start_idx, tables }
            }
            State::TwoLevel {
                start_idx,
                tables: l2_table,
            } => {
                // expand
                let mut tables = Box::new(Table::new());
                *tables.get_entry_mut(start_idx) = Some(l2_table);
                Self::expand_if_necessary(
                    State::ThreeLevel {
                        start_idx: start_idx & !(L3::MASK | L3::CHILD_MASK),
                        tables,
                    },
                    bit_idx,
                )
            }
            State::ThreeLevel { start_idx, tables }
                if bit_idx & !(L3::MASK | L3::CHILD_MASK) == start_idx =>
            {
                // no expansion necessary
                State::ThreeLevel { start_idx, tables }
            }
            State::ThreeLevel {
                start_idx,
                tables: l3_table,
            } => {
                // expand
                let mut tables = Box::new(Table::new());
                *tables.get_entry_mut(start_idx) = Some(l3_table);
                State::FourLevel { tables }
            }
            State::FourLevel { tables } => {
                // expansion never neceesary
                State::FourLevel { tables }
            }
        }
    }

    pub fn set_bit(&mut self, bit_idx: u32) -> bool {
        let state = std::mem::replace(&mut self.state, State::Init);
        self.state = Self::expand_if_necessary(state, bit_idx);
        let inserted = match &mut self.state {
            State::Init => panic!("expand_if_necessary should have prevented this"),
            State::OneLevel { chunks, .. } => {
                let chunk = chunks.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk |= 1 << (bit_idx % BITS_PER_USIZE as u32);
                prev ^ *chunk != 0
            }
            State::TwoLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk |= 1 << (bit_idx % BITS_PER_USIZE as u32);
                prev ^ *chunk != 0
            }
            State::ThreeLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk |= 1 << (bit_idx % BITS_PER_USIZE as u32);
                prev ^ *chunk != 0
            }
            State::FourLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk |= 1 << (bit_idx % BITS_PER_USIZE as u32);
                prev ^ *chunk != 0
            }
        };
        self.len += inserted as u32;
        inserted
    }

    pub fn clear_bit(&mut self, bit_idx: u32) -> bool {
        let removed = match &mut self.state {
            State::Init => false,
            State::OneLevel { chunks, .. } => {
                let chunk = chunks.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk &= !(1 << (bit_idx % BITS_PER_USIZE as u32));
                prev ^ *chunk != 0
            }
            State::TwoLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk &= !(1 << (bit_idx % BITS_PER_USIZE as u32));
                prev ^ *chunk != 0
            }
            State::ThreeLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk &= !(1 << (bit_idx % BITS_PER_USIZE as u32));
                prev ^ *chunk != 0
            }
            State::FourLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                let prev = *chunk;
                *chunk &= !(1 << (bit_idx % BITS_PER_USIZE as u32));
                prev ^ *chunk != 0
            }
        };
        self.len -= removed as u32;
        removed
    }

    pub fn test_bit(&self, bit_idx: u32) -> bool {
        match &self.state {
            State::Init => false,
            State::OneLevel { start_idx, .. }
                if bit_idx & !(L1::MASK | L1::CHILD_MASK) != *start_idx =>
            {
                false
            }
            State::TwoLevel { start_idx, .. }
                if bit_idx & !(L2::MASK | L2::CHILD_MASK) != *start_idx =>
            {
                false
            }
            State::ThreeLevel { start_idx, .. }
                if bit_idx & !(L3::MASK | L3::CHILD_MASK) != *start_idx =>
            {
                false
            }
            State::OneLevel { chunks, .. } => chunks
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % BITS_PER_USIZE as u32)) != 0)
                .unwrap_or(false),
            State::TwoLevel { tables, .. } => tables
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % BITS_PER_USIZE as u32)) != 0)
                .unwrap_or(false),
            State::ThreeLevel { tables, .. } => tables
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % BITS_PER_USIZE as u32)) != 0)
                .unwrap_or(false),
            State::FourLevel { tables, .. } => tables
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % BITS_PER_USIZE as u32)) != 0)
                .unwrap_or(false),
        }
    }

    pub fn iter(&self) -> RadixBitSetIter {
        RadixBitSetIter(match &self.state {
            State::Init => RadixBitSetIterState::Init,
            State::OneLevel { start_idx, chunks } => RadixBitSetIterState::OneLevel {
                start_idx: *start_idx,
                iter: chunks.iter(),
            },
            State::TwoLevel { start_idx, tables } => RadixBitSetIterState::TwoLevel {
                start_idx: *start_idx,
                iter: tables.iter(),
            },
            State::ThreeLevel { start_idx, tables } => RadixBitSetIterState::ThreeLevel {
                start_idx: *start_idx,
                iter: tables.iter(),
            },
            State::FourLevel { tables } => RadixBitSetIterState::FourLevel {
                iter: tables.iter(),
            },
        })
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn space_used(&self) -> usize {
        std::mem::size_of::<Self>()
            + match &self.state {
                State::Init => 0,
                State::OneLevel {
                    start_idx: _,
                    chunks,
                } => chunks.space_used(),
                State::TwoLevel {
                    start_idx: _,
                    tables,
                } => tables.space_used(),
                State::ThreeLevel {
                    start_idx: _,
                    tables,
                } => tables.space_used(),
                State::FourLevel { tables } => tables.space_used(),
            }
    }
}

pub struct RadixBitSetIter<'a>(RadixBitSetIterState<'a>);
impl<'a> Iterator for RadixBitSetIter<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        match &mut self.0 {
            RadixBitSetIterState::Init => None,
            RadixBitSetIterState::OneLevel { start_idx, iter } => {
                iter.next().map(|idx| idx | *start_idx)
            }
            RadixBitSetIterState::TwoLevel { start_idx, iter } => {
                iter.next().map(|idx| idx | *start_idx)
            }
            RadixBitSetIterState::ThreeLevel { start_idx, iter } => {
                iter.next().map(|idx| idx | *start_idx)
            }
            RadixBitSetIterState::FourLevel { iter } => iter.next(),
        }
    }
}

enum RadixBitSetIterState<'a> {
    Init,
    OneLevel {
        start_idx: u32,
        iter: ChunksIter<'a>,
    },
    TwoLevel {
        start_idx: u32,
        iter: <L2 as Node>::Iter<'a>,
    },
    ThreeLevel {
        start_idx: u32,
        iter: <L3 as Node>::Iter<'a>,
    },
    FourLevel {
        iter: <L4 as Node>::Iter<'a>,
    },
}

#[derive(Debug)]
struct Chunks<const N: usize>([usize; N]);
impl<const N: usize> Chunks<N> {
    fn iter(&self) -> ChunksIter {
        let mut chunk_iter = self.0.iter().copied().enumerate();
        let (chunk_idx, chunk) = chunk_iter.next().unwrap();
        let bit_iter = IterBits(chunk);
        ChunksIter {
            chunk_iter,
            chunk_idx: chunk_idx as u32,
            bit_iter,
        }
    }
}
impl<const N: usize> Node for Chunks<N> {
    const CHILD_MASK: u32 = 0;
    const MASK: u32 = std::mem::size_of::<Self>() as u32 * 8 - 1;

    type Iter<'a> = ChunksIter<'a>;
    fn new() -> Self {
        Chunks([0; N])
    }
    fn walk(&self, bit_idx: u32) -> Option<&usize> {
        let chunk_idx = (bit_idx & Self::MASK) / BITS_PER_USIZE as u32;
        Some(&self.0[chunk_idx as usize])
    }
    fn walk_or_create(&mut self, bit_idx: u32) -> &mut usize {
        let chunk_idx = (bit_idx & Self::MASK) / BITS_PER_USIZE as u32;
        &mut self.0[chunk_idx as usize]
    }
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
    fn space_used(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

#[derive(Debug)]
struct ChunksIter<'a> {
    chunk_iter: std::iter::Enumerate<std::iter::Copied<std::slice::Iter<'a, usize>>>,
    chunk_idx: u32,
    bit_iter: IterBits,
}
impl<'a> Iterator for ChunksIter<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.bit_iter.next() {
                Some(bitpos) => return Some(bitpos + (BITS_PER_USIZE as u32 * self.chunk_idx)),
                None => {
                    let (chunk_idx, chunk) = self.chunk_iter.next()?;
                    self.chunk_idx = chunk_idx as u32;
                    self.bit_iter = IterBits(chunk);
                }
            }
        }
    }
}
#[derive(Debug)]
struct IterBits(usize);
impl Iterator for IterBits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let lsb = self.0.trailing_zeros();
        // x & x.wrapping_neg() results in x with only the least significant bit set
        // x ^ (x & x.wrapping_neg()) results in clearing only the least significant bit
        self.0 ^= self.0 & self.0.wrapping_neg();
        Some(lsb)
    }
}

trait Node {
    const CHILD_MASK: u32;
    const MASK: u32;
    type Iter<'a>: Iterator<Item = u32>
    where
        Self: 'a;

    fn new() -> Self;
    fn walk(&self, bit_idx: u32) -> Option<&usize>;
    fn walk_or_create(&mut self, bit_idx: u32) -> &mut usize;
    fn iter(&self) -> Self::Iter<'_>;
    fn space_used(&self) -> usize;
}
trait InternalNode: Node {
    type ChildNode;
}

#[derive(Debug)]
struct Table<const NUM_ENTRIES: usize, ChildNode>([Option<Box<ChildNode>>; NUM_ENTRIES])
where
    ChildNode: Debug;

impl<const NUM_ENTRIES: usize, ChildNode> InternalNode for Table<NUM_ENTRIES, ChildNode>
where
    ChildNode: Debug + Node,
{
    type ChildNode = ChildNode;
}

impl<const NUM_ENTRIES: usize, ChildNode> Table<NUM_ENTRIES, ChildNode>
where
    Self: Debug + Node,
    ChildNode: Debug + Node,
{
    const fn mask(num_entries: usize, child_mask: u32) -> u32 {
        if num_entries == 0 {
            0
        } else {
            (num_entries as u32 - 1) << child_mask.trailing_ones()
        }
    }

    fn get_entry(&self, bit_idx: u32) -> Option<&ChildNode> {
        self.0[((bit_idx & Self::MASK) >> Self::CHILD_MASK.trailing_ones()) as usize]
            .as_ref()
            .map(|x| &**x)
    }

    fn get_entry_mut(&mut self, bit_idx: u32) -> &mut Option<Box<ChildNode>> {
        &mut self.0[((bit_idx & Self::MASK) >> Self::CHILD_MASK.trailing_ones()) as usize]
    }
}

impl<const NUM_ENTRIES: usize, ChildNode> Node for Table<NUM_ENTRIES, ChildNode>
where
    ChildNode: Debug + Node,
{
    const CHILD_MASK: u32 = ChildNode::MASK | ChildNode::CHILD_MASK;
    const MASK: u32 = Self::mask(NUM_ENTRIES, Self::CHILD_MASK);
    type Iter<'a> = TableIter<'a, NUM_ENTRIES, Self> where Self: 'a;

    fn new() -> Self {
        // safety: Option<Box<T>> is guaranteed for ffi interop to be a nullable
        // pointer. zeroed() is therefore a valid initialization.
        Table(unsafe {
            MaybeUninit::<[Option<Box<ChildNode>>; NUM_ENTRIES]>::zeroed().assume_init()
        })
    }
    fn walk(&self, bit_idx: u32) -> Option<&usize> {
        self.get_entry(bit_idx)
            .and_then(|child_table| child_table.walk(bit_idx))
    }

    fn walk_or_create(&mut self, bit_idx: u32) -> &mut usize {
        self.get_entry_mut(bit_idx)
            .get_or_insert_with(|| Box::new(ChildNode::new()))
            .walk_or_create(bit_idx)
    }

    fn iter(&self) -> Self::Iter<'_> {
        let mut table_iter = self.0.iter().enumerate();
        let (child_offset, child) = table_iter
            .find_map(|(child_offset, child)| {
                let child_offset = (child_offset as u32) << Self::CHILD_MASK.trailing_ones();
                Some((child_offset, child.as_ref()?))
            })
            .expect("unexpected table with no children");
        TableIter {
            table_iter,
            child_offset,
            child_iter: child.iter(),
        }
    }
    fn space_used(&self) -> usize {
        std::mem::size_of::<Self>()
            + self
                .0
                .iter()
                .filter_map(|child| child.as_ref())
                .map(|child| child.space_used())
                .sum::<usize>()
    }
}

struct TableIter<'a, const NUM_ENTRIES: usize, Table>
where
    Table: InternalNode + 'a,
    Table::ChildNode: Node + 'a,
{
    table_iter: std::iter::Enumerate<std::slice::Iter<'a, Option<Box<Table::ChildNode>>>>,
    child_offset: u32,
    child_iter: <<Table as InternalNode>::ChildNode as Node>::Iter<'a>,
}
impl<'a, const NUM_ENTRIES: usize, Table> Iterator for TableIter<'a, NUM_ENTRIES, Table>
where
    Table: Debug + InternalNode + 'a,
    Table::ChildNode: Debug + Node + 'a,
{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(x) = self.child_iter.next() {
                return Some(self.child_offset | x);
            }
            loop {
                match self.table_iter.next() {
                    Some((child_idx, Some(next_child))) => {
                        self.child_offset = (child_idx as u32) << Table::CHILD_MASK.trailing_ones();
                        self.child_iter = next_child.iter();
                        break;
                    }
                    Some((_child_idx, None)) => {}
                    None => {
                        return None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn sanity() {
        dbg!(L1_BIT_WIDTH);
        dbg!(L2_BIT_WIDTH);
        dbg!(L3_BIT_WIDTH);
        dbg!(L4_BIT_WIDTH);
        assert_eq!((L1::MASK | L2::MASK | L3::MASK | L4::MASK).count_zeros(), 0);

        // no overlap.
        assert_eq!((L1::MASK & L2::MASK), 0);
        assert_eq!((L1::MASK & L3::MASK), 0);
        assert_eq!((L1::MASK & L4::MASK), 0);
        assert_eq!((L2::MASK & L1::MASK), 0);
        assert_eq!((L2::MASK & L3::MASK), 0);
        assert_eq!((L2::MASK & L4::MASK), 0);
        assert_eq!((L3::MASK & L1::MASK), 0);
        assert_eq!((L3::MASK & L2::MASK), 0);
        assert_eq!((L3::MASK & L4::MASK), 0);
        assert_eq!((L4::MASK & L1::MASK), 0);
        assert_eq!((L4::MASK & L2::MASK), 0);
        assert_eq!((L4::MASK & L3::MASK), 0);
    }

    #[test]
    fn test_iter_bits() {
        assert_eq!(Vec::<u32>::new(), IterBits(0).collect::<Vec<_>>());
        assert_eq!(vec![0], IterBits(1).collect::<Vec<_>>());
        assert_eq!(vec![0, 1], IterBits(3).collect::<Vec<_>>());
        assert_eq!(
            vec![0, 2, 3, 5, 6],
            IterBits(0b0110_1101).collect::<Vec<_>>()
        );
        assert_eq!(
            (0..64).collect::<Vec<_>>(),
            IterBits(u64::MAX as usize).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_chunks_iter() {
        let mut chunks = [0; 512];
        chunks[0] = 0x8000_0000_0000_0001;
        chunks[1] = 0x8000_0000_0000_0001;
        chunks[2] = 0x8000_0000_0000_0001;
        chunks[3] = 0x8000_0000_0000_0001;
        assert_eq!(
            vec![0, 63, 64, 127, 128, 191, 192, 255],
            Chunks(chunks).iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_sparse_bit_set_iter() {
        let mut bs = RadixBitSet::new();
        let mut bits = vec![1 << 8, u32::MAX, 1 << 0, 1 << 16];
        for bit in bits.iter().copied() {
            bs.set_bit(bit);
        }
        bits.sort();
        assert_eq!(bits, bs.iter().collect::<Vec<_>>())
    }

    #[test]
    fn test_sparse_bit_set() {
        let mut bs = RadixBitSet::new();
        assert!(!bs.test_bit(0));
        bs.set_bit(0);
        assert!(bs.test_bit(0));
        bs.set_bit(256);
        bs.set_bit(511);
        assert!(bs.test_bit(256));
        assert!(bs.test_bit(511));
        assert!(!bs.test_bit(257));
        assert!(!bs.test_bit(510));

        bs.set_bit(1024);
        assert!(bs.test_bit(1024));
        bs.set_bit(1 << 15);
        assert!(bs.test_bit(1 << 15));
        bs.set_bit(1 << 16);
        assert!(bs.test_bit(1 << 16));
        bs.set_bit(1 << 24);
        assert!(bs.test_bit(1 << 24));
        bs.set_bit(u32::MAX);
        assert!(bs.test_bit(u32::MAX));
    }

    #[test]
    fn worst_case_space_used() {
        let mut bs = RadixBitSet::new();
        let end = (!0) >> L1_BIT_WIDTH;
        dbg!(end);
        for i in 0..=end {
            bs.set_bit(i << L1_BIT_WIDTH);
        }
        dbg!(bs.space_used());
    }

    #[test]
    fn test_space_used() {
        const BASE_SIZE: usize = std::mem::size_of::<RadixBitSet>();
        const L1_TABLE_SIZE: usize = std::mem::size_of::<L1>();
        const L2_TABLE_SIZE: usize = std::mem::size_of::<L2>();
        const L3_TABLE_SIZE: usize = std::mem::size_of::<L3>();
        const L4_TABLE_SIZE: usize = std::mem::size_of::<L4>();

        const L1_MIN: u32 = 0;
        const L1_MAX: u32 = L1::MASK | L1::CHILD_MASK;
        const L2_MIN: u32 = L1_MAX + 1;
        const L2_MAX: u32 = L2::MASK | L2::CHILD_MASK;

        let mut bs = RadixBitSet::new();
        assert_eq!(BASE_SIZE, bs.space_used());

        bs.set_bit(L1_MIN);
        assert_eq!(BASE_SIZE + L1_TABLE_SIZE, bs.space_used());

        bs.set_bit(L1_MAX);
        assert_eq!(BASE_SIZE + L1_TABLE_SIZE, bs.space_used());

        bs.set_bit(L2_MIN);
        assert_eq!(
            BASE_SIZE + (2 * L1_TABLE_SIZE) + L2_TABLE_SIZE,
            bs.space_used()
        );

        bs.set_bit(L2_MAX);
        assert_eq!(
            BASE_SIZE + (3 * L1_TABLE_SIZE) + L2_TABLE_SIZE,
            bs.space_used()
        );

        if L2_MAX == u32::MAX {
            return;
        }
        let l3_min: u32 = L2_MAX.saturating_add(1);
        let l3_max: u32 = L3::MASK | L3::CHILD_MASK;

        bs.set_bit(l3_min);
        assert_eq!(
            BASE_SIZE + (4 * L1_TABLE_SIZE) + (2 * L2_TABLE_SIZE) + L3_TABLE_SIZE,
            bs.space_used()
        );

        bs.set_bit(l3_max);
        assert_eq!(
            BASE_SIZE + (5 * L1_TABLE_SIZE) + (3 * L2_TABLE_SIZE) + L3_TABLE_SIZE,
            bs.space_used()
        );

        if l3_max == u32::MAX {
            return;
        }
        let l4_min: u32 = l3_max.saturating_add(1);
        let l4_max: u32 = L4::MASK | L4::CHILD_MASK;
        bs.set_bit(l4_min);
        assert_eq!(
            BASE_SIZE
                + (6 * L1_TABLE_SIZE)
                + (4 * L2_TABLE_SIZE)
                + (2 * L3_TABLE_SIZE)
                + L4_TABLE_SIZE,
            bs.space_used()
        );
        bs.set_bit(l4_max);
        assert_eq!(
            BASE_SIZE
                + (7 * L1_TABLE_SIZE)
                + (5 * L2_TABLE_SIZE)
                + (3 * L3_TABLE_SIZE)
                + L4_TABLE_SIZE,
            bs.space_used()
        );
    }

    proptest! {
      #[test]
      fn test_properties(mut values: Vec<u32>) {
        let mut bs = RadixBitSet::new();
        for v in values.iter().copied() {
            assert!(bs.set_bit(v));
        }
        values.sort();
        values.dedup();

        assert_eq!(values.len() as u32, bs.len());
        assert_eq!(&values, &bs.iter().collect::<Vec<_>>());
        for v in values.iter().copied() {
            assert!(bs.test_bit(v));
        }

        for v in values.iter().copied() {
            assert!(bs.clear_bit(v));
        }
        assert_eq!(0, bs.len());
        assert_eq!(Vec::<u32>::new(), bs.iter().collect::<Vec<_>>());
        for v in values.iter().copied() {
            assert!(!bs.test_bit(v));
        }
      }
    }
}
