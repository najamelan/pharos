# TODO

- make Events clone? means we can only work with broadcast channels

- switch to more performant channels (crossbeam). Will be easier once they provide an async api.

- allow other channel types, like a ringchannel which drops messages on outpacing? To prevent DDOS and OOM attacks?

- scaling? For now we have an ever growing vector of observers

  - other data structure than vec? smallvec?
  - keep a vector of free indexes, pop one on observe, push on removal, to reuse
  - let users set capacity on creation?
