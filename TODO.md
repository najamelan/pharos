# TODO

- make Events clone? means we can only work with broadcast channels

- switch to more performant channels (crossbeam). Will be easier once they provide an async api.

- allow other channel types, like a ringchannel which drops messages on outpacing? To prevent DDOS and OOM attacks?

- scaling? For now we have an ever growing vector of observers

  - other data structure than vec? smallvec?
  - type that allows concurrent access to &mut for each observer, so we can mutate in place rather than have join_all allocate a new vector on easch notify. Maybe partitions crate? -> has 19 lines of unsafe code, needs review.
  - let users set capacity on creation?
