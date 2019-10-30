pub trait Record<K> {
    fn key(&self) -> K; // Extract key from record.
}

pub trait RecordFields<F> {
    fn fields(&self) -> F;
}

// Generic types are not sorted alphabetically, to match next trait semantic order
pub trait Indexed<R, K> {
    fn find(&self, key: &K) -> Vec<&R>;

    fn find_range(&self, start: &K, end: &K) -> Vec<&R>;
}

pub trait IndexedDestructured<F, K> {
    fn find(&self, key: &K) -> Vec<&F>;

    fn find_range(&self, start: &K, end: &K) -> Vec<(K, &F)>;
}
