collections

Expose Collection

SDK collection	std equivalent	Description
WeilVec<T>	Vec<T>	A growable array type. The values are sharded in memory and can be used for iterable and indexable values that are dynamically sized
WeilMap<K, V>	HashMap<K, V>	This structure behaves as a thin wrapper around the key-value storage available to contracts. This structure does not contain any metadata about the elements in the map, so it is not iterable.
WeilSet<T>	HashSet<T>	equals WeilMap<T, ()>
WeilTrieMap<T>	NA	A string-keyed map like collection which supports efficient prefix queries along with standard get and set queries of WeilMap<K, V>
WeilMemory	Vec<u8>	A Vec<u8> like contiguous array which internally divides into 64KB chunks. This is more optimized than WeilVec<T> for arbitrary offset reads and writes. In WeilVec<T> reads and writes happen per element basis whereas WeilMemory reads and writes happened per chunk (64KB) basis thus balancing the performance and in-memory data for better extended offset reads and writes.
Collection APIs

WeilVec<T>:

fn push(&mut self, item: T);

fn get(&self, index: usize) -> Option<T>;

fn set(&mut self, index: usize, item: T) -> Result<(), IndexOutOfBoundsError>;

fn pop(&mut self) -> Option<T>;

fn iter(&self) -> WeilVecIter<T>;

WeilMap<K, V>

fn insert(&mut self, key: K, val: V);

fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Serialize;

fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Serialize;

WeilSet<T>

fn insert(&mut self, value: T);

fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + Serialize;

WeilTrieMap<T>

fn insert(&mut self, key: String, val: T);

fn get(&self, key: &str) -> Option<T>;

fn remove(&self, key: &str) -> Option<T>;

fn get_with_prefix(&self, prefix: &str) -> Option<WeilTriePrefixMap<T>>;

WeilMemory

fn read(&self, offset: usize, dst: &mut [u8]);

fn write(&mut self, offset: usize, src: &[u8]);

fn grow(&mut self, num_chunks: u32);

fn size(&self) -> usize;