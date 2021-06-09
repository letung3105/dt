use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

/// A basic hash map.
///
/// It is required that the keys implement the [`Eq`] and [`Hash`] traits, although this can
/// frequently be achieved by using `#[derive(PartialEq, Eq, Hash)]`. If you implement these
/// yourself, it is important that the following property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two keys are equal, their hashes must be equal.
///
/// # Attributions
///
/// This `LinkedHashMap` implementation is based off [Jon Gjengset's livestream] on the concept and
/// implementation of the data structure itself. The [source code] of the project from the
/// livestream can be found on Github.
///
/// [Jon Gjengset's livestream]: https://www.youtube.com/watch?v=k6xR2kf9hlA
/// [source code]: https://github.com/jonhoo/rust-basic-hashmap
///
/// # Examples
///
/// ```
/// use dt::containers::LinkedHashMap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `LinkedHashMap<String, String>` in this example).
/// let mut book_reviews = LinkedHashMap::new();
///
/// // Review some books.
/// book_reviews.insert(
///     "Adventures of Huckleberry Finn".to_string(),
///     "My favorite book.".to_string(),
/// );
/// book_reviews.insert(
///     "Grimms' Fairy Tales".to_string(),
///     "Masterpiece.".to_string(),
/// );
/// book_reviews.insert(
///     "Pride and Prejudice".to_string(),
///     "Very enjoyable.".to_string(),
/// );
/// book_reviews.insert(
///     "The Adventures of Sherlock Holmes".to_string(),
///     "Eye lyked it alot.".to_string(),
/// );
///
/// // Check for a specific one.
/// // When containers store owned values (String), they can still be
/// // queried using references (&str).
/// if !book_reviews.contains_key("Les Misérables") {
///     println!("We've got {} reviews, but Les Misérables ain't one.",
///              book_reviews.len());
/// }
///
/// // oops, this review has a lot of spelling mistakes, let's delete it.
/// book_reviews.remove("The Adventures of Sherlock Holmes");
///
/// // Look up the values associated with some keys.
/// let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
/// for &book in &to_find {
///     match book_reviews.get(book) {
///         Some(review) => println!("{}: {}", book, review),
///         None => println!("{} is unreviewed.", book)
///     }
/// }
///
/// // Look up the value for a key (will panic if the key is not found).
/// println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
///
/// // Iterate over everything.
/// for (book, review) in &book_reviews {
///     println!("{}: \"{}\"", book, review);
/// }
/// ```
/// ```
/// use dt::containers::LinkedHashMap;
///
/// // type inference lets us omit an explicit type signature (which
/// // would be `LinkedHashMap<&str, u8>` in this example).
/// let mut player_stats = LinkedHashMap::new();
///
/// fn random_stat_buff() -> u8 {
///     // could actually return some random value here - let's just return
///     // some fixed value for now
///     42
/// }
///
/// // insert a key only if it doesn't already exist
/// player_stats.entry("health").or_insert(100);
///
/// // insert a key using a function that provides a new value only if it
/// // doesn't already exist
/// player_stats.entry("defence").or_insert_with(random_stat_buff);
///
/// // update a key, guarding against the key possibly not being set
/// let stat = player_stats.entry("attack").or_insert(100);
/// *stat += random_stat_buff();
/// ```
///
/// The easiest way to use `LinkedHashMap` with a custom key type is to derive [`Eq`] and [`Hash`].
/// We must also derive [`PartialEq`].
///
/// ```
/// use dt::containers::LinkedHashMap;
///
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     country: String,
/// }
///
/// impl Viking {
///     /// Creates a new Viking.
///     fn new(name: &str, country: &str) -> Viking {
///         Viking { name: name.to_string(), country: country.to_string() }
///     }
/// }
///
/// // Use a LinkedHashMap to store the vikings' health points.
/// let mut vikings = LinkedHashMap::new();
///
/// vikings.insert(Viking::new("Einar", "Norway"), 25);
/// vikings.insert(Viking::new("Olaf", "Denmark"), 24);
/// vikings.insert(Viking::new("Harald", "Iceland"), 12);
///
/// // Use derived implementation to print the status of the vikings.
/// for (viking, health) in &vikings {
///     println!("{:?} has {} hp", viking, health);
/// }
/// ```
///
/// ```
/// use dt::containers::LinkedHashMap;
///
/// let timber_resources: LinkedHashMap<&str, i32> =
///     [("Norway", 100), ("Denmark", 50), ("Iceland", 10)].iter().cloned().collect();
/// // use the values stored in map
/// ```
#[derive(Debug)]
pub struct LinkedHashMap<K, V, S = RandomState> {
    // This hash map implementation relies on an array of buckets that is indexed by the hash of an
    // entry's key. If 2 different keys are hashed to the same value, the entries are put into the
    // same bucket. These entries can later be retrieved by comparing both the hashed key and the
    // actual key.
    buckets: Vec<Bucket<K, V>>,
    build_hasher: S,
    entries_count: usize,
}

impl<K, V> Default for LinkedHashMap<K, V, RandomState> {
    fn default() -> Self {
        Self {
            buckets: Vec::new(),
            build_hasher: RandomState::new(),
            entries_count: 0,
        }
    }
}

impl<K, V> LinkedHashMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    /// Creates an empty `LinkedHashMap`.
    ///
    /// The hash map is initially created with an empty list of buckets, so it will not allocate
    /// until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use dt::containers::LinkedHashMap;
    /// let mut map: LinkedHashMap<&str, i32> = LinkedHashMap::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old value is returned.
    /// The key is not updated, though; this matters for types that can be `==` without being
    /// identical.
    ///
    /// # Examples
    ///
    /// ```
    /// use dt::containers::LinkedHashMap;
    ///
    /// let mut map = LinkedHashMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.entries_count > 3 * self.buckets.len() / 4 {
            self.grow();
        }

        let idx = self.index(&key);
        let bucket = &mut self.buckets[idx];

        for &mut (ref k, ref mut v) in bucket.items.iter_mut() {
            if *k == key {
                return Some(std::mem::replace(v, value));
            }
        }
        bucket.items.push((key, value));
        self.entries_count += 1;
        None
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// TODO: make the below statement true for our map
    /// The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed
    /// form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use dt::containers::LinkedHashMap;
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.index(key);
        self.buckets[idx]
            .items
            .iter()
            .find(|&(ref k, _)| k == key)
            .map(|&(_, ref v)| v)
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the
    /// map.
    ///
    /// TODO: make the below statement true for our map
    /// The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed
    /// form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        // self.buckets.remove
        let idx = self.index(&key);
        let bucket = &mut self.buckets[idx];

        let entry_idx = bucket.items.iter().position(|&(ref k, _)| k == key)?;
        self.entries_count -= 1;
        Some(bucket.items.swap_remove(entry_idx).1)
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed
    /// form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        let idx = self.index(key);
        self.buckets[idx]
            .items
            .iter()
            .find(|&(ref k, _)| k == key)
            .is_some()
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut a = HashMap::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a");
    /// assert_eq!(a.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.entries_count
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut a = HashMap::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a");
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entries_count == 0
    }

    /// Increase the size of the array of buckets. If there is no bucket, extend the array by one,
    /// otherwise, double the array's size and reindex all existing entries.
    fn grow(&mut self) {
        let target_size = match self.buckets.len() {
            0 => 1,
            n => 2 * n,
        };
        let mut buckets = Vec::with_capacity(target_size);
        buckets.extend((0..target_size).map(|_| Bucket::default()));
        for (key, value) in self
            .buckets
            .iter_mut()
            .flat_map(|bucket| bucket.items.drain(..))
        {
            let idx = Self::key_to_idx(self.build_hasher.build_hasher(), &key, target_size);
            buckets[idx].items.push((key, value));
        }
        self.buckets = buckets;
    }

    /// Get the index of the bucket for `key`
    fn index(&self, key: &K) -> usize {
        Self::key_to_idx(self.build_hasher.build_hasher(), key, self.buckets.len())
    }

    /// Hash the `hashable` value with the `hasher`, then modulo the hash value with `divisor`.
    fn key_to_idx<H>(mut hasher: H, key: &K, n_buckets: usize) -> usize
    where
        H: Hasher,
    {
        key.hash(&mut hasher);
        (hasher.finish() % n_buckets as u64) as usize
    }
}

/// A data item that holds entries in [`LinkedHashMap`] whose key is hashed to the same value.
///
/// [`LinkedHashMap`]: crate::containers::LinkedHashMap
#[derive(Debug)]
struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

impl<K, V> Default for Bucket<K, V> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_crud_opeartions() {
        let mut map = LinkedHashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        // create
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());

        // update
        map.insert("foo", 43);
        assert_eq!(map.get(&"foo"), Some(&43));
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());

        // remove
        assert_eq!(map.remove(&"foo"), Some(43));
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        // non-existent key
        assert_eq!(map.get(&"foo"), None);
        assert_eq!(map.remove(&"foo"), None);
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }
}
