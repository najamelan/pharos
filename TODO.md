# TODO

- Create a filter possibility. Let users provide a closure with a predicate to filter which events they want to receive.
  The reasons for doing this in pharos are:

  - It's unwieldy to do this in the client because of the unwieldy type that you need to annotate if you need to store
    the stream (FilterMap will include a closure in it's type)
  - performance. If filtering happens on the client side, we will clone and send out events they are not interested in.
    By doing it in pharos, we can avoid that.

- switch to more performant channels (crossbeam). Will be easier once they provide an async api.

- allow other channel types, like a ringchannel which drops messages on outpacing? To prevent DDOS and OOM attacks?

- scaling? For now we have an ever growing vector of observers
