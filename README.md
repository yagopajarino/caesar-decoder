# Caesar Decoder

This is a simple Caesar decoder to prove that the caesar algorithm is quite unsafe, for those sceptic.

It is written in Rust and can be executed through this bash command

`cargo run 'Aopz pz h zljyla tlzzhnl, wslhzl rllw pa zhml iyv'`

## ¿How it works?

[Caesar Cipher](https://en.wikipedia.org/wiki/Caesar_cipher) is one of the simplest cipher algorithms, apparently used by Julius Caesar to send messages to their generals on the battle fields.

It simply changes every letter of the message for the one $$n$$ times after in the alphabet.

It's clear that the decoding of a encrypted message if knowing the key is quite simple, just shift left the alphabet $$key$$ times and replace each letter you got.

If you don't know the key, you need to crute force the every possible key (the number of items in the alphabet) and try with each one of them.

So this algorithm does just that, it looks for every shifting value ultil it gets an english message.
