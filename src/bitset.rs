use std::{fmt::Debug, mem::MaybeUninit};

#[derive(Debug)]
enum State {
    Init,
    OneLevel {
        start_idx: u32,
        chunks: Box<Chunks>,
    },
    TwoLevel {
        start_idx: u32,
        tables: Box<Table<2, Chunks>>,
    },
    ThreeLevel {
        start_idx: u32,
        tables: Box<Table<3, Table<2, Chunks>>>,
    },
    FourLevel {
        tables: Box<Table<4, Table<3, Table<2, Chunks>>>>,
    },
}

const L1_MASK: u32 = 0xffff_ff00;
const L2_MASK: u32 = 0xffff_0000;
const L3_MASK: u32 = 0xff00_0000;
#[derive(Debug)]
pub struct SparseBitSet(State);
impl Default for SparseBitSet {
    fn default() -> Self {
        SparseBitSet::new()
    }
}
impl SparseBitSet {
    pub fn new() -> Self {
        SparseBitSet(State::Init)
    }

    fn expand_if_necessary(state: State, bit_idx: u32) -> State {
        match state {
            State::Init => {
                let start_idx = bit_idx & L1_MASK;
                let chunks = Box::new(Chunks([0; 4]));
                State::OneLevel { start_idx, chunks }
            }
            State::OneLevel { start_idx, chunks } if bit_idx & L1_MASK == start_idx => {
                // no expansion necessary
                State::OneLevel { start_idx, chunks }
            }
            State::OneLevel { start_idx, chunks } => {
                // expand
                let mut tables = Box::new(Table::new());
                tables.0[(start_idx as usize >> 8) & 0xff] = Some(chunks);
                Self::expand_if_necessary(
                    State::TwoLevel {
                        start_idx: start_idx & L2_MASK,
                        tables,
                    },
                    bit_idx,
                )
            }
            State::TwoLevel { start_idx, tables } if bit_idx & L2_MASK == start_idx => {
                // no expansion necessary
                State::TwoLevel { start_idx, tables }
            }
            State::TwoLevel {
                start_idx,
                tables: l2_table,
            } => {
                // expand
                let mut tables = Box::new(Table::new());
                tables.0[(start_idx as usize >> 8) & 0xff] = Some(l2_table);
                Self::expand_if_necessary(State::ThreeLevel { start_idx, tables }, bit_idx)
            }
            State::ThreeLevel { start_idx, tables } if bit_idx & L3_MASK == start_idx => {
                // no expansion necessary
                State::ThreeLevel { start_idx, tables }
            }
            State::ThreeLevel {
                start_idx,
                tables: l3_table,
            } => {
                // expand
                let mut tables = Box::new(Table::new());
                tables.0[(start_idx as usize >> 8) & 0xff] = Some(l3_table);
                State::FourLevel { tables }
            }
            State::FourLevel { tables } => {
                // expansion never neceesary
                State::FourLevel { tables }
            }
        }
    }

    pub fn set_bit(&mut self, bit_idx: u32) {
        let state = std::mem::replace(&mut self.0, State::Init);
        self.0 = Self::expand_if_necessary(state, bit_idx);
        match &mut self.0 {
            State::Init => panic!("expand_if_necessary should have prevented this"),
            State::OneLevel { chunks, .. } => {
                let chunk = chunks.walk_or_create(bit_idx);
                *chunk |= 1 << (bit_idx % 64);
            }
            State::TwoLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                *chunk |= 1 << (bit_idx % 64);
            }
            State::ThreeLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                *chunk |= 1 << (bit_idx % 64);
            }
            State::FourLevel { tables, .. } => {
                let chunk = tables.walk_or_create(bit_idx);
                *chunk |= 1 << (bit_idx % 64);
            }
        }
    }

    pub fn test_bit(&self, bit_idx: u32) -> bool {
        match &self.0 {
            State::Init => false,
            State::OneLevel { start_idx, .. } if bit_idx & L1_MASK != *start_idx => false,
            State::TwoLevel { start_idx, .. } if bit_idx & L2_MASK != *start_idx => false,
            State::ThreeLevel { start_idx, .. } if bit_idx & L3_MASK != *start_idx => false,
            State::OneLevel { chunks, .. } => chunks
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % 64)) != 0)
                .unwrap(),
            State::TwoLevel { tables, .. } => tables
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % 64)) != 0)
                .unwrap(),
            State::ThreeLevel { tables, .. } => tables
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % 64)) != 0)
                .unwrap(),
            State::FourLevel { tables, .. } => tables
                .walk(bit_idx)
                .map(|chunk| chunk & (1 << (bit_idx % 64)) != 0)
                .unwrap(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        match &self.0 {
            State::Init => SparseBitSetIter::Init,
            State::OneLevel { start_idx, chunks } => SparseBitSetIter::OneLevel {
                start_idx: *start_idx,
                iter: chunks.iter(),
            },
            State::TwoLevel { start_idx, tables } => SparseBitSetIter::TwoLevel {
                start_idx: *start_idx,
                iter: tables.iter(),
            },
            State::ThreeLevel { start_idx, tables } => SparseBitSetIter::ThreeLevel {
                start_idx: *start_idx,
                iter: tables.iter(),
            },
            State::FourLevel { tables } => SparseBitSetIter::FourLevel {
                iter: tables.iter(),
            },
        }
    }
}

#[derive(Debug)]
enum SparseBitSetIter<'a> {
    Init,
    OneLevel {
        start_idx: u32,
        iter: ChunksIter<'a>,
    },
    TwoLevel {
        start_idx: u32,
        iter: <Table<2, Chunks> as Walker>::Iter<'a>,
    },
    ThreeLevel {
        start_idx: u32,
        iter: <Table<3, Table<2, Chunks>> as Walker>::Iter<'a>,
    },
    FourLevel {
        iter: <Table<4, Table<3, Table<2, Chunks>>> as Walker>::Iter<'a>,
    },
}
impl<'a> Iterator for SparseBitSetIter<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        match self {
            SparseBitSetIter::Init => None,
            SparseBitSetIter::OneLevel { start_idx, iter } => {
                iter.next().map(|idx| idx | *start_idx)
            }
            SparseBitSetIter::TwoLevel { start_idx, iter } => {
                iter.next().map(|idx| idx | *start_idx)
            }
            SparseBitSetIter::ThreeLevel { start_idx, iter } => {
                iter.next().map(|idx| idx | *start_idx)
            }
            SparseBitSetIter::FourLevel { iter } => {
                iter.next()
            }
        }
    }
}

#[derive(Debug)]
struct Chunks([u64; 4]);
impl Chunks {
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
impl Walker for Chunks {
    type Iter<'a> = ChunksIter<'a>;
    fn new() -> Self {
        Chunks([0; 4])
    }
    fn walk(&self, bit_idx: u32) -> Option<&u64> {
        let chunk_idx = (bit_idx & 0xff) / 64;
        Some(&self.0[chunk_idx as usize])
    }
    fn walk_or_create(&mut self, bit_idx: u32) -> &mut u64 {
        let chunk_idx = (bit_idx & 0xff) / 64;
        &mut self.0[chunk_idx as usize]
    }
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

#[derive(Debug)]
struct ChunksIter<'a> {
    chunk_iter: std::iter::Enumerate<std::iter::Copied<std::slice::Iter<'a, u64>>>,
    chunk_idx: u32,
    bit_iter: IterBits,
}
impl<'a> Iterator for ChunksIter<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.bit_iter.next() {
                Some(bitpos) => return Some(bitpos + (64 * self.chunk_idx)),
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
struct IterBits(u64);
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

trait Walker {
    type Iter<'a>: Iterator<Item = u32>
    where
        Self: 'a;

    fn new() -> Self;
    fn walk(&self, bit_idx: u32) -> Option<&u64>;
    fn walk_or_create(&mut self, bit_idx: u32) -> &mut u64;
    fn iter(&self) -> Self::Iter<'_>;
}

#[derive(Debug)]
struct Table<const LEVEL: u32, ChildTable>([Option<Box<ChildTable>>; 256])
where
    ChildTable: Debug;
impl<const LEVEL: u32, ChildTable> Walker for Table<LEVEL, ChildTable>
where
    ChildTable: Debug + Walker,
{
    type Iter<'a> = TableIter<'a, LEVEL, ChildTable> where ChildTable: 'a;

    fn new() -> Self {
        // safety: Option<Box<T>> is guaranteed for ffi interop to be a nullable
        // pointer. zeroed() is therefore a valid initialization.
        Table(unsafe { MaybeUninit::<[Option<Box<ChildTable>>; 256]>::zeroed().assume_init() })
    }
    fn walk(&self, bit_idx: u32) -> Option<&u64> {
        let offset = bit_idx >> ((LEVEL - 1) * 8) & 0xff;
        self.0[offset as usize]
            .as_ref()
            .and_then(|child_table| child_table.walk(bit_idx))
    }

    fn walk_or_create(&mut self, bit_idx: u32) -> &mut u64 {
        let offset = bit_idx >> ((LEVEL - 1) * 8) & 0xff;
        self.0[offset as usize]
            .get_or_insert_with(|| Box::new(ChildTable::new()))
            .walk_or_create(bit_idx)
    }

    fn iter(&self) -> Self::Iter<'_> {
        let mut table_iter = self.0.iter().enumerate();
        let (child_offset, child) = table_iter
            .find_map(|(child_offset, child)| {
                let child_offset = (child_offset as u32) << ((LEVEL - 1) * 8);
                Some((child_offset, child.as_ref()?))
            })
            .expect("unexpected table with no children");
        TableIter {
            table_iter,
            child_offset,
            child_iter: child.iter(),
        }
    }
}

#[derive(Debug)]
struct TableIter<'a, const LEVEL: u32, ChildTable: Walker + 'a> {
    table_iter: std::iter::Enumerate<std::slice::Iter<'a, Option<Box<ChildTable>>>>,
    child_offset: u32,
    child_iter: ChildTable::Iter<'a>,
}
impl<'a, const LEVEL: u32, ChildTable> Iterator for TableIter<'a, LEVEL, ChildTable>
where
    ChildTable: Walker + 'a,
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
                        self.child_offset = (child_idx as u32) << ((LEVEL - 1) * 8);
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
            IterBits(u64::MAX).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_chunks_iter() {
        assert_eq!(
            vec![0, 63, 64, 127, 128, 191, 192, 255],
            Chunks([
                0x8000_0000_0000_0001,
                0x8000_0000_0000_0001,
                0x8000_0000_0000_0001,
                0x8000_0000_0000_0001
            ])
            .iter()
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_sparse_bit_set_iter() {
        let mut bs = SparseBitSet::new();
        let mut bits = vec![1 << 8, u32::MAX, 1 << 0, 1<<16];
        for bit in bits.iter().copied() {
            bs.set_bit(bit);
        }
        bits.sort();
        assert_eq!(bits, bs.iter().collect::<Vec<_>>())
    }

    #[test]
    fn test_sparse_bit_set() {
        let mut bs = SparseBitSet::new();
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
}
