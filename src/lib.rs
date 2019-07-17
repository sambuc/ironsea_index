use ironsea_table::Table;

pub trait Record<K> {
    fn key(&self) -> K; // Extract key from record.
}

pub trait RecordFields<F> {
    fn fields(&self) -> F;
}

pub trait RecordBuild<K, F, R> {
    fn build(key: &K, fields: &F) -> R;
}

pub trait Indexed<T: Table<R>, R: Record<K>, K> {
    fn find(&self, key: &K) -> Vec<&R>;

    fn find_range(&self, start: &K, end: &K) -> Vec<&R>;
}

pub trait IndexedOwned<T: Table<R>, R: Record<K>, K> {
    fn find(&self, key: &K) -> Vec<R>;

    fn find_range(&self, start: &K, end: &K) -> Vec<R>;
}
