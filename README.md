# news

A conscious social news website focusing on parenting, spirituality, science and psychology.

## Rust versions supported

[v1.58.1](https://blog.rust-lang.org/2022/01/20/Rust-1.58.1.html) and above,
language edition `2021`.

## Translations

Missing strings in locale files or unsupported locale identifiers can be logged during runtime on the _STDERR_
by running `cargo build -vv` instead of `cargo build`. This will give you outputs such as the following:

```
[plabayo-news-web 0.0.1] plabayo-news_builder: missing string for locale nl, resolved by using fallback: STRINGS_DEFAULT.site.nav.header.logout; Please add the translated string to nl.yml!
```

All logs originating from the plabayo news builder will be prefixed by `plabayo-news_builder:`.
There is no desire for this to fail a build, as translation is considered best-effort
and is on a strict voluntary basis.
