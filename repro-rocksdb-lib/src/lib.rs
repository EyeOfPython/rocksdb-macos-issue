#[no_mangle]
pub extern "C" fn repro_rocksdb_rust() {
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);
    let db = rocksdb::DB::open(&opts, "/tmp/repro-rocksdb").unwrap();
    db.put(b"key", b"value").unwrap();
}
