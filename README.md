# rust-rocksdb linker issue on MacOS Ventura

Building this project fails on MacOS Ventura (13.5) with the following linker error:

```
[4/4] Linking CXX executable repro
FAILED: repro 
: && /opt/homebrew/opt/ccache/libexec/c++ -arch arm64 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX13.3.sdk -Wl,-search_paths_first -Wl,-headerpad_max_install_names -L/opt/homebrew/opt/openssl@3/lib -L/opt/homebrew/opt/llvm/lib CMakeFiles/repro.dir/repro.cpp.o -o repro -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib -Wl,-rpath,/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib  librepro_rocksdb_lib.a  -lSystem  -lc  -lm && :
Undefined symbols for architecture arm64:
  "_rocksdb_cache_create_hyper_clock", referenced from:
      rocksdb::db_options::Cache::new_hyper_clock_cache::h7d14defec8d9ff91 in librepro_rocksdb_lib.a(rocksdb-dbbfe0cff2ccc4a0.rocksdb.4d0d22d9589b67d2-cgu.14.rcgu.o)
  "_rocksdb_readoptions_set_async_io", referenced from:
      rocksdb::db_options::ReadOptions::set_async_io::hf801c96d8347bdbf in librepro_rocksdb_lib.a(rocksdb-dbbfe0cff2ccc4a0.rocksdb.4d0d22d9589b67d2-cgu.14.rcgu.o)
ld: symbol(s) not found for architecture arm64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
ninja: build stopped: subcommand failed.
```

As you can see, I am using ccache, but I cleared the cache before the compilation (just to be sure), so it's certainly not a ccache issue.

It started after I upgraded to MacOS Ventura; but this doesn't mean that the issue is strictly related to the MacOS upgrade.

# Reproduction steps
1. `brew install cmake`
2. `mkdir build && cd build`
3. `cmake -GNinja ..`
4. `ninja`
5. -> Linking fails

# My setup
1. `clang -v`: Homebrew clang version 16.0.6 (Target: arm64-apple-darwin22.6.0, Thread model: posix, InstalledDir: /opt/homebrew/opt/llvm/bin)
2. `cmake --version`: cmake version 3.27.1
3. `ninja --version`: 1.10.2
