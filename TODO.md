# TODO

- instead of providing channels, just take a Sink in observe? That would keep the lib much simpler and allow
  users more flexibility.

- ObserveOnce = observable without events being clone? Handy for types that want to provide a detached
  stream over types that don't implement clone or for arbitrary user types. Came up during the dev of
  async_nursery.
- be abstract over channels like thespis?
- try_recv, which allows just checking if there is an event without blocking the task until there is,
  handy for cooperative cancellation.
- use matches! in docs and examples for filters.
- test on wasm
- should Pharos be Sync?
- use NonZeroUsize as parameter in bounded channel
- make Events clone, means we can only work with broadcast channels
- switch to more performant channels (flume?). Will be easier once they provide an async api.
- allow other channel types, like a ringchannel which drops messages on outpacing? To prevent DDOS and OOM attacks?


# Rewrite goals:

- possibility to write to the same pharos object from different async tasks. Right now it requires mut and mutexes and refcells shouldn't be held accross await points.

- Events to be clone so something that is observing can clone it's stream and give it to some other object/task. Or make Events Observable itself?

- be channel agnostic?

Sending out to observers is async and provides back pressure. Other sends and adding observers requires that we are not in the middle of sending out.

