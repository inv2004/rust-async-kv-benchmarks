# rust-async-kv-benchmarks

- kv_actix_async. actix-web, to_async.
- kv_actix_cmap_async. actix-web with CHashMap, to_async.
- kv_actix_cmap. actix-web with CHashMap.
- kv_actix_msg. actix-web with Message into sync actor.
- kv_actix_rwlock. actix-web with RwLock on HashMap
- kv_cmap_multi. hyper with CHashMap.
- kv_cmap_single. hyper on current_thread with CHashMap.
- kv_refcell_single. hyper on current_thread with HashMap.
- kv_rwlock_multi. hyper with RwLock on HashMap.

### scaleway:
CPU: AMD EPYC 7401P 24-Core Processor. 4 Cores.

```bash
wrk -t4 -c256 -d10s -R200000 http://127.0.0.1:9999 -s test.lua
```

| test             | wrk2 Requests/sec |
| ---------------- | -------: |
| actix_async      | 66899.86 |
| actix_cmap_async | 68179.46 |
| actix_cmap       | 84830.59 |
| actix_msg        | 45251.34 |
| actix_rwlock     | 66798.22 |
| cmap_multi       | 72856.85 |
| cmap_single      | 40023.77 |
| refcell_single   | 43050.81 |
| rwlock_multi     | 64817.87 |

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

