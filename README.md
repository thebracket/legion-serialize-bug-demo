# Legion serialization issue demo

This is based on a problem I was having in Nox Futura. I didn't want to ask anyone to debug 13k lines of code, so this repo boils the issue down to something manageable.

Basically, on a `World` restored from a serialized copy deleting an entity results in crash.

Files:

* `serialize.rs` is based on the serialization example in the Legion Github repo. It's pretty much the same, but I wrapped things in some functions to make it easier to work with. In NF, it serializes/de-serializes 30k entities and a whole lot of components very nicely. Lots of queries and systems work.
* `main.rs` aims to reproduce the issue I'm encountering. `works_fine()` shows that the code works on a world you just made. `crashes()` fails on the same task, using a de-serialized world.

Crash information:

Running with `cargo run` gives me the following. First, the good run:

```
RUN 1: WORKS FINE
-----------------
Insert a bunch of fake trees, similar to those in my game
Searching for tree 1
Found the entity: Entity { index: 1, version: 1 }, world position: Ref { borrow: Shared { state: 2 }, value: Position { idx: 2 } } 
We want to kill tree #1
Found the entity to delete: Entity { index: 1, version: 1 }, world position: Ref { borrow: Shared { state: 2 }, value: Position { idx: 2 } }
Running the delete buffer
Searching for tree 1
You shouldn't have seen a tree there!
```

Then the failed run:

```
RUN 2: CRASHES
------------------
Insert a bunch of fake trees, similar to those in my game
Searching for tree 1
Found the entity: Entity { index: 1, version: 1 }, world position: Ref { borrow: Shared { state: 2 }, value: Position { idx: 2 } } 
Save the world and load it again
Searching for tree 1
Found the entity: Entity { index: 1025, version: 1 }, world position: Ref { borrow: Shared { state: 2 }, value: Position { idx: 2 } }
We want to kill tree #1
Found the entity to delete: Entity { index: 1025, version: 1 }, world position: Ref { borrow: Shared { state: 2 }, value: Position 
{ idx: 2 } }
Running the delete buffer
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\world.rs:369:28
```

The full stack trace is a little intimidating, but here it is:

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\world.rs:369:28
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized
             at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.44\src\backtrace\mod.rs:66 
   1: std::sys_common::backtrace::_print_fmt
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:78
   2: std::sys_common::backtrace::_print::{{impl}}::fmt
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:59
   3: core::fmt::write
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libcore\fmt\mod.rs:1063
   4: std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\io\mod.rs:1426
   5: std::sys_common::backtrace::_print
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:62
   6: std::sys_common::backtrace::print
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:49
   7: std::panicking::default_hook::{{closure}}
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:204
   8: std::panicking::default_hook
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:224
   9: std::panicking::rust_panic_with_hook
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:470
  10: std::panicking::begin_panic_handler
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:378
  11: core::panicking::panic_fmt
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libcore\panicking.rs:85
  12: core::panicking::panic
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libcore\panicking.rs:52
  13: core::option::Option<legion_core::entity::EntityLocation>::unwrap<legion_core::entity::EntityLocation>
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\src\libcore\macros\mod.rs:10
  14: legion_core::world::World::delete
             at C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\world.rs:369
  15: legion_core::command::{{impl}}::write
             at C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\command.rs:98
  16: legion_core::command::CommandBuffer::write
             at C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\command.rs:409
  17: legionbug::crashes
             at .\src\main.rs:107
  18: legionbug::main
             at .\src\main.rs:120
  19: std::rt::lang_start::{{closure}}<()>
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\src\libstd\rt.rs:67
  20: std::rt::lang_start_internal::{{closure}}
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\rt.rs:52
  21: std::panicking::try::do_call<closure-0,i32>
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:303
  22: panic_unwind::__rust_maybe_catch_panic
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libpanic_unwind\lib.rs:86
  23: std::panicking::try
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:281
  24: std::panic::catch_unwind
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panic.rs:394
  25: std::rt::lang_start_internal
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\rt.rs:51
  26: std::rt::lang_start<()>
             at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\src\libstd\rt.rs:67
  27: main
  28: invoke_main
             at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  29: __scrt_common_main_seh
             at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  30: BaseThreadInitThunk
  31: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\legionbug.exe` (exit code: 101)
```

If it helps, with full back-tracing:

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\world.rs:369:28
stack backtrace:
   0:     0x7ff6d0535a9f - backtrace::backtrace::trace_unsynchronized
                               at C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.44\src\backtrace\mod.rs:66
   1:     0x7ff6d0535a9f - std::sys_common::backtrace::_print_fmt
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:78
   2:     0x7ff6d0535a9f - std::sys_common::backtrace::_print::{{impl}}::fmt
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:59
   3:     0x7ff6d054a98b - core::fmt::write
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libcore\fmt\mod.rs:1063
   4:     0x7ff6d053353c - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\io\mod.rs:1426
   5:     0x7ff6d0538a2c - std::sys_common::backtrace::_print
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:62
   6:     0x7ff6d0538a2c - std::sys_common::backtrace::print
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\sys_common\backtrace.rs:49
   7:     0x7ff6d0538a2c - std::panicking::default_hook::{{closure}}
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:204
   8:     0x7ff6d053867f - std::panicking::default_hook
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:224
   9:     0x7ff6d0539187 - std::panicking::rust_panic_with_hook
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:470
  10:     0x7ff6d0538d0f - std::panicking::begin_panic_handler
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:378
  11:     0x7ff6d05488f0 - core::panicking::panic_fmt
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libcore\panicking.rs:85
  12:     0x7ff6d054883c - core::panicking::panic
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libcore\panicking.rs:52
  13:     0x7ff6d04acb16 - core::option::Option<legion_core::entity::EntityLocation>::unwrap<legion_core::entity::EntityLocation>  
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\src\libcore\macros\mod.rs:10
  14:     0x7ff6d049e404 - legion_core::world::World::delete
                               at C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\world.rs:369 
  15:     0x7ff6d04b1574 - legion_core::command::{{impl}}::write
                               at C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\command.rs:98  16:     0x7ff6d04b1fe7 - legion_core::command::CommandBuffer::write
                               at C:\Users\herbe\.cargo\git\checkouts\legion-6fbc02e8da0bdce7\80a3d15\legion_core\src\command.rs:409
  17:     0x7ff6d040d352 - legionbug::crashes
                               at C:\Users\herbe\Documents\LearnRust\legionbug\src\main.rs:107
  18:     0x7ff6d040d5b1 - legionbug::main
                               at C:\Users\herbe\Documents\LearnRust\legionbug\src\main.rs:120
  19:     0x7ff6d0398cfb - std::rt::lang_start::{{closure}}<()>
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\src\libstd\rt.rs:67
  20:     0x7ff6d0538ba7 - std::rt::lang_start_internal::{{closure}}
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\rt.rs:52
  21:     0x7ff6d0538ba7 - std::panicking::try::do_call<closure-0,i32>
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:303
  22:     0x7ff6d053b9c2 - panic_unwind::__rust_maybe_catch_panic
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libpanic_unwind\lib.rs:86
  23:     0x7ff6d05393c8 - std::panicking::try
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panicking.rs:281
  24:     0x7ff6d05393c8 - std::panic::catch_unwind
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\panic.rs:394
  25:     0x7ff6d05393c8 - std::rt::lang_start_internal
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\/src\libstd\rt.rs:51
  26:     0x7ff6d0398cd3 - std::rt::lang_start<()>
                               at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550\src\libstd\rt.rs:67
  27:     0x7ff6d040d830 - main
  28:     0x7ff6d0551654 - invoke_main
                               at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  29:     0x7ff6d0551654 - __scrt_common_main_seh
                               at d:\agent\_work\3\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  30:     0x7ffc22f67bd4 - BaseThreadInitThunk
  31:     0x7ffc24b8ce51 - RtlUserThreadStart
error: process didn't exit successfully: `target\debug\legionbug.exe` (exit code: 101)
```
