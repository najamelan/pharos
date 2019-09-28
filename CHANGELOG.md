# Pharos Changelog

## 0.4.0 - 2019-09-28

- **BREAKING CHANGE**: The notify function had a sub optimal implemetation and did not allow notifying observers
  from within `poll_*` functions. It has been replaced with an implementation of Sink on Pharos.
- got rid of dependency on pin_project.
- as Error::kind returns a reference to the error kind, you can now compare `ErrorKind::SomeVariant == err.kind()` without having to write: `&ErrorKind::SomeVariant == err.kind()`.
- updated to futures-preview 0.3.0-alpha.19

## 0.3.2 - 2019-09-23

- check spelling

## 0.3.1 - 2019-09-23

- **BREAKING CHANGE**: Last minute change of heart. I removed two API methods whose only "merit" was
to hide a Box::new from the user.
- fix docs.rs showing readme

## 0.3.0 - 2019-09-23 - yanked

**BREAKING CHANGE**: This is an almost complete rewrite with a much improved API, documentation, ...

- Only have one Observable trait that takes options rather than UboundedObservable.
- Allow filtering events with a predicate.
- Many small improvements.

Please have a look at the readme and the API docs for more.

## 0.2.2 - 2019-08-26

- update to futures 0.3.0-alpha.18
- remove async_await feature (from 1.39 this crate should compile on stable)

## 0.2.1 - 2019-08-02

- remove `await_macro` feature use and convert to new await syntax.
- implement `Default` for `Pharos`.


## 0.2.0 - 2019-07-18

- **BREAKING CHANGE**:  Update dependencies. The Futures library has some changes, which introduce a breaking change. Bounded channels are no longer queue_size + 1. They now are queue_size. This means that `Observable::observe( queue_size: usize )` will now yield a `futures::channel::Receiver` of the exact `queue_size`.
