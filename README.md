# box_dest_receiver

A Postgres [DestReceiver](dr) implementation that renders query results in 
a box format, inspired by DuckDB's box renderer.  It is meant to be used under
the single-user mode as it writes to stdout.


```sh
$ ./install/bin/postgres --single -D data  postgres

PostgreSQL stand-alone backend 19devel
backend> SELECT * FROM buzz;
┌─────────────────────┬────────────┐
│ receive_func        │ is_it_good │
├─────────────────────┼────────────┤
│ debugtup            │ f          │
├─────────────────────┼────────────┤
│ box_dr_receive_slot │ t          │
├─────────────────────┼────────────┤
│ ???                 │ NULL       │
└─────────────────────┴────────────┘
```

## How to use it

1. Build the project, we need to know the location of Postgres include directory.  It is
   intended to not invoke `pg_config`, in case you have multiple Postgres installed.
   The build procedure only works on macOS.

   ```sh
   $ PG_INCLUDE_DIR=<your Postgres include dir> cargo b -r

   # If you want to use pg_config
   $ PG_INCLUDE_DIR=$(pg_config --includedir);
   ```

   Then you will get 2 archive files under `target/release`:

   ```sh
   $ find target/release -name '*.a'
   target/release/libstatic_inline_c_funcs.a
   target/release/libbox_dest_receiver.a 
   ```

2. Let Postgres link them:

   ```makefile
   LIBS += box_dest_receiver/target/release/libbox_dest_receiver.a
   LIBS += box_dest_receiver/target/release/libstatic_inline_c_funcs.a
   ```

   I believe there are better approaches to do this, but I currently don't know.

3. Use this DestReceiver in Postgres

   > src/backend/tcop/dest.c

   ```diff
    #include "libpq/libpq.h"
    #include "libpq/pqformat.h"
    
   +#include "box_dest_receiver.h"
   +
    
   /* ----------------
    *             dummy DestReceiver functions
   @@ -130,7 +132,7 @@ CreateDestReceiver(CommandDest dest)
                           return unconstify(DestReceiver *, &donothingDR);
    
                    case DestDebug:
   -                       return unconstify(DestReceiver *, &debugtupDR);
   +                       return box_dr_create(dest);
    
                    case DestSPI:
                           return unconstify(DestReceiver *, &spi_printtupDR); 
   ```


[dr]: https://github.com/postgres/postgres/blob/4fbfdde58e4cd091f88737dffa241b08c23d8829/src/include/tcop/dest.h#L102-L130