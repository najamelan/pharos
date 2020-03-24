# TODO

- use matches! in docs and examples for filters.
- test on wasm
- should Pharos be Sync?
- use NonZeroUsize as parameter in bounded channel
- make Events clone? means we can only work with broadcast channels
- switch to more performant channels (flume?). Will be easier once they provide an async api.
- allow other channel types, like a ringchannel which drops messages on outpacing? To prevent DDOS and OOM attacks?


