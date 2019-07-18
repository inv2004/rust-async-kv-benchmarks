# rust-async-kv-benchmarks

- kv_actix: actix-web. Uses RwLock for Context.
- kv_rwlock_multi: hyper multithreaded. Share via RwLock.
- kv_refcell_single: hyper singlethreaded.
- kv_cmap_multi: hyper multi. CHashMap for cache.
- kv_cmap_single: hyper single. CHashMap for cache.

CPU: i5-7500. 4 cores.

```bash
wrk -t2 -c8 -d5s -R100000 http://127.0.0.1:9999 -s test.lua
```

| test           | wrk2 Requests/sec | 
| -------------- | ----------------: |
| actix-web      | 31464.54          |
| rwlock_multi   | 39976.96          |
| refcell_single | 45594.26          |
| cmap_multi     | 39814.56          |
| cmap_single    | 45549.61          |

TODO:
- [x] remove RwLock from single
- [x] separate threads for PUT/GET
- [x] do not PUT-GET the same number (for cmap)
- [ ] thread-pool variant
- [ ] actix with Message
- [ ] hyper with channels

