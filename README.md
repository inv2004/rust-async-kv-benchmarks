# rust-async-kv-benchmarks

- kv_actix: actix-web. Uses RwLock for Context.
- kv_rwlock_multi: hyper multithreaded. Share via RwLock.
- kv_rwlock_single: hyper multithreaded (was not able to run server in current_thread only). RwLock to share.
- kv_cmap_single: hyper multithreaded. CHashMap for cache.

CPU: i5-7500. 4 cores.

```bash
wrk -t2 -c8 -d5s -R100000 http://127.0.0.1:9999 -s test.lua
```

| test          | wrk2 Requests/sec | 
| ------------- | ----------------: |
| rwlock_multi  | 58713.57          |
| rwlock_single |   (why?) 59572.84 |
| cmap_single   | 64341.83          |
| actix-web     | 58190.41          |

TODO:
- [ ] remove RwLock from single
- [ ] thread-pool variant
- [ ] actix with Message
- [ ] hyper with channels

