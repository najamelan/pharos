# pharos

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/pharos.svg?branch=master)](https://travis-ci.org/najamelan/pharos)
[![Docs](https://docs.rs/pharos/badge.svg)](https://docs.rs/pharos)
![crates.io](https://img.shields.io/crates/v/pharos.svg)

> An introduction to pharos is available in many formats: [video](https://youtu.be/BAzsxW-nxh8), [wikipedia](https://en.wikipedia.org/wiki/Lighthouse_of_Alexandria) and it was even honored by many artists like [this painting by Micheal Turner](http://omeka.wustl.edu/omeka/files/original/2694d12580166e77d40afd37b492a78e.jpg).

More seriously, pharos is a small [observer](https://en.wikipedia.org/wiki/Observer_pattern) library that let's you create futures 0.3 streams that observers can listen to.

I created it to leverage interoperability we can create by using async Streams and Sinks from the futures library. You can now use all stream combinators, forward it into Sinks and so on. I also want it to be future-proof, so it's aimed directly at async-await. This means it will **not compile on stable rust until async_await has been stabilized**.

## Table of Contents

- [Security](#security)
- [Install](#install)
  - [Dependencies](#dependencies)
- [Usage](#usage)
- [API](#api)
- [Contributing](#contributing)
  - [Code of Conduct](#code-of-conduct)
- [License](#license)

## Security

The main issue with this crate right now is the possibility for the observable to outpace the observer. When using bounded form, there is back pressure, which might allow DDOS attacks if using the pattern on arriving network packets. When using the unbounded form, it might lead to excessive memory consumption if observers are outpaced.

To mitigate these problems effectively, I would like to implement an unbounded drop channel where the stream will only buffer a certain amount events and will overwrite the oldest event instead of blocking the sender when the buffer is full.

This crate has: `#![ forbid( unsafe_code ) ]`


## Install

With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add pharos`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

  pharos: ^0.1
```

With raw Cargo.toml
```toml
[dependencies]

   pharos = "^0.1"
```

### Dependencies

This crate only has one dependiency. Cargo will automatically handle it's dependencies for you.

```yaml
dependencies:

  futures-preview: { version: ^0.3.0-alpha.15 }
```

## Usage

pharos only works for async code, as the notify method is asynchronous. Observers must consume the messages
fast enough, otherwise they will slow down the observable (bounded form) or cause memory leak (unbounded form).

Whenever observers want to unsubscribe, they can just drop the stream or call `close` on it. If you are an observable and you want to notify observers that no more messages will follow, just drop the pharos object. Failing that, create an event type that signifies EOF and send that to observers.

Your event type will be cloned once for each observer, so you might want to put it in an Arc if it's bigger than a pointer size. Eg, there's no point putting an enum without associated data in an Arc.

Examples can be found in the [examples](https://github.com/najamelan/pharos/tree/master/examples) directory. Here is a summary of the most basic one:

```rust
#![ feature( async_await, await_macro )]

use
{
   pharos :: { * } ,

   futures ::
   {
      channel::mpsc :: Receiver      ,
      executor      :: LocalPool     ,
      task          :: LocalSpawnExt ,
      stream        :: StreamExt     ,
   },
};


// here we put a pharos object on our struct
//
struct Godess { pharos: Pharos<GodessEvent> }


impl Godess
{
   fn new() -> Self
   {
      Self { pharos: Pharos::new() }
   }

   // Send Godess sailing so she can tweet about it!
   //
   pub async fn sail( &mut self )
   {
      await!( self.pharos.notify( &GodessEvent::Sailing ) );
   }
}



// Event types need to implement clone, but you can wrap them in Arc if not. Also they will be
// cloned, so if you will have several observers and big event data, putting them in an Arc is
// definitely best. It has no benefit to put a simple dataless enum in an Arc though.
//
#[ derive( Clone, Debug, PartialEq, Copy ) ]
//
enum GodessEvent
{
   Sailing
}


// This is the needed implementation of Observable. We might one day have a derive for this,
// but it's not so interesting, since you always have to point it to your pharos object,
// and when you want to be observable over several types of events, you might want to keep
// pharos in a hashmap over type_id, and a derive would quickly become a mess.
//
impl Observable<GodessEvent> for Godess
{
   fn observe( &mut self, queue_size: usize ) -> Receiver<GodessEvent>
   {
      self.pharos.observe( queue_size )
   }
}


fn main()
{
   let mut pool  = LocalPool::new();
   let mut exec  = pool.spawner();

   let program = async move
   {
      let mut isis = Godess::new();

      // subscribe: bounded channel with 3 + 1 slots
      //
      let mut events = isis.observe( 3 );

      // trigger an event
      //
      await!( isis.sail() );

      // read from stream
      //
      let from_stream = await!( events.next() ).unwrap();

      dbg!( from_stream );
      assert_eq!( GodessEvent::Sailing, from_stream );
   };

   exec.spawn_local( program ).expect( "Spawn program" );

   pool.run();
}
```


## API

Api documentation can be found on [docs.rs](https://docs.rs/pharos).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through github issues.

Pull Requests are welcome on github. By commiting pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)
