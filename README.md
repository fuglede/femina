# Femina horoscope parser

Parser for [Femina's](https://www.femina.dk/) Danish daily horoscopes. The application reads the horoscopes and exposes them through a [Rocket](https://rocket.rs/) server. From there, you can use the horoscopes as part of your favorite system, e.g. a login script, an IRC chatbot, or something equally useful.


## Example usage

Running the server locally:

```
$ rustup override set nightly && cargo run
$ curl localhost:8000/krebs
Dagen bliver alt andet end kedelig, især hvis du vælger at slutte dig til fællesskabet med gode venner. Husk at bryde op mens legen er god.
```

Alternatively, you can grab it from Docker Hub:

```
$ docker run -d -p 8000:8000 fuglede/femina
$ curl localhost:8000/krebs
Dagen bliver alt andet end kedelig, især hvis du vælger at slutte dig til fællesskabet med gode venner. Husk at bryde op mens legen er god.
```