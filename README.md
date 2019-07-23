# rust-async-kv-benchmarks

- kv_actix_async: actix-web. to_async with 1 worker.
- kv_actix_rwlock: actix-web. Uses RwLock for Context.
- kv_actix_cmap: actix-web. Uses RwLock for Context.
- kv_rwlock_multi: hyper multithreaded. Share via RwLock.
- kv_refcell_single: hyper singlethreaded.
- kv_cmap_multi: hyper multi. CHashMap for cache.
- kv_cmap_single: hyper single. CHashMap for cache.

### scaleway C2M:
CPU: Intel(R) Atom(TM) CPU  C2750. 8 cores

```bash
wrk -t4 -c256 -d10s -R200000 http://127.0.0.1:9999 -s test.lua
```

| test | wrk2 Requests/sec |
| -----| -----: |
| actix_async | 18442.46 |
| actix_cmap_async | 64495.68 |
| actix_cmap | 72625.96 |
| actix_rwlock | 66062.04 |
| cmap_multi | 59914.52 |
| cmap_single | 17561.04 |
| refcell_single | 17581.77 |
| rwlock_multi | 57112.52 |

### pc
CPU: i5-7500. 4 cores

```bash
wrk -t2 -c8 -d5s -R100000 http://127.0.0.1:9999 -s test.lua
```

| test           | wrk2 Requests/sec | 
| -------------- | ----------------: |
| actix_async    | 30069.73          |
| actix_rwlock   | 31464.54          |
| actix_cmap     | 32064.03          |
| actix_msg      | 25091.83          |
| rwlock_multi   | 39976.96          |
| refcell_single | 45594.26          |
| cmap_multi     | 39814.56          |
| cmap_single    | 45549.61          |

TODO:
- [x] remove RwLock from single
- [x] separate threads for PUT/GET
- [x] do not PUT-GET the same number (for cmap)
- [x] actix with Message
- [ ] thread-pool variant
- [ ] hyper with channels

