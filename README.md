# BTC Wallet Finder

Yeah, this is one of those things that try to "brute force" a bitcoin wallet. Will it be effective? No.

## Why it's not effective

### The amount of private keys

To start off, we need to look at the amount of Bitcoin addresses there will ever be in the universe. Given that a bitcoin address is derived from a 160 bit hash (private key), approx this amount:

\[
2^{160} \approx 1.46 \times 10^{48} \, \text{(total possible Bitcoin addresses)}.
\]

Still, you'll also need to look at the amount of private keys that can generate the same bitcoin address. Around **28 million private keys** can generate the same bitcoin address. So the effective possible options of private keys becomes:

\[
\text{Effective search space} = \frac{2^{160}}{28 \times 10^6} \approx 5.2 \times 10^{41}.
\]

So, there are a total of $5.2 \times 10^{41}$ private keys that generate a unique bitcoin address. Typing this number out, you will have a one in 52,000,000,000,000,000,000,000,000,000,000,000,000,000,000 chance to hit a specific bitcoin address.

### The probability of hitting an address of interest

As you might've seen in the repo, there's a `data` folder. This folder contains files (splitted in chunks of 100MB) containing all bitcoin addresses with at least some balance (>=1 satoshi).

I honestly don't remember how many there are, but let's say there are 25 million of these. Assuming a uniform distribution of addresses, the probability of any random Bitcoin address being one of your target addresses is:

\[
P_{\text{hit}} = \frac{\text{Addresses of Interest}}{\text{Effective Address Space}} = \frac{25 \times 10^6}{5.2 \times 10^{41}} \approx 4.81 \times 10^{-35}.
\]

Thus, by generating one bitcoin address, your chance of hitting a Bitcoin address with any kind of balance is 0.0000000000000000000000000000000881%. 

Now don't tell me "hey, there's a chance". I'll write down some examples below.

#### The jackpot

The odds of winning a Powerball jackpot are approx. 1 in 292 million, or \( 3.4 \times 10^{-7} \).

\[
\frac{8.81 \times 10^{-35}}{3.4 \times 10^{-7}} \approx 2.6 \times 10^{-28}.
\]

Adding in the chance of hitting an address of interest, and you're looking at a \( 2.6 \times 10^{-28} \) times higher chance of winning the Powerball jackpot over hitting one of the 25 million addresses. In fact, this number is so big, you could win the jackpot every single week for billions of years, and still not hit one of the addresses you're looking for.

#### Being hit by an asteroid

Here's another fun one. NASA estimates the probability of Earth being struck by a ***catastrophic*** asteroid in any given year is 1 in 74 million, or \( 1.35 \times 10^{-8} \).

Again, the probability of hitting one of the 25 million addresses is much smaller:

\[
\frac{8.81 \times 10^{-35}}{1.35 \times 10^{-8}} \approx 6.5 \times 10^{-27}.
\]

This number is like Earth being hit by an asteroid *every single second* for longer than the universe's age. And ***STILL*** not hitting one of the target addresses.

#### Coin flipping

I think this one is a pretty funny example too. The odds of flipping a fair coin and getting **heads 115 times in a row** are:

\[
\frac{1}{2^{115}} \approx 3.26 \times 10^{-35}.
\]

Which are similar to the odds of hitting one of the addresses. Now I want to see you flip heads 115 times in a row (you won't).

## Time

Yeah, I can hear you saying "but these are the probabilities of generating one bitcoin addresses, programs can generate them much faster!". Here's the thing: no.

The program that I've made generates & checks about 100.000 addresses per second (on my Macbook M2). That must be a huge help right?! Well, no. For simplicity, let's calculate the chance of a hit over various timeframes.

**Chance of Collision (of a singular check)**

I can generate and check **100,000 addresses per second**. Over a period of \( T \) seconds:

\[
\text{Addresses Checked} = 100,000 \times T.
\]

The probability of hitting at least one of our target addresses after \( N \) checks is:

\[
P_{\text{collision}} = 1 - (1 - P_{\text{hit}})^N.
\]

### One second of generation

\[
N = 100,000, \quad P_{\text{collision}} \approx 100,000 \cdot 4.81 \times 10^{-35} = 4.81 \times 10^{-30}.
\]

In one second, hitting one of the 25 million addresses has a chance of one in 481,000,000,000,000,000,000,000,000,000,000,000,000 (that's a huge number).

### One year of generation

One year has 31,536,000 seconds.

\[
N = 100,000 \cdot 31,536,000 \approx 3.15 \times 10^{12}.
\]

The probability of a collision is (chance of it hitting an address):

\[
P_{\text{collision}} \approx 3.15 \times 10^{12} \cdot 4.81 \times 10^{-35} \approx 1.52 \times 10^{-22}.
\]

This is one in 45,200,000,000,000,000,000,000.

### Age of the universe of generation

If I were to somehow wind back time, start from when the birth of the universe happened, and *have electricity* with my macbook, we'll be able to do this many checks:

\[
N = 100,000 \cdot 4.35 \times 10^{17} = 4.35 \times 10^{22}.
\]

Now we do the probability of collision equation again:

\[
P_{\text{collision}} \approx 4.35 \times 10^{22} \cdot 4.81 \times 10^{-35} \approx 2.09 \times 10^{-12}.
\]

And boom! Still a chance of one in 2,090,000,000,000.

**Me with my macbook**:
- After **1 second**, have a chance of practically 0
- After **1 year**, have a chance of practically 0
- After **the age of the universe**, the probability of hitting one of the addresses of interest remains negligible, about one in a trillion.

## But what about a supercomputer?

Always this question, right? A supercomputer. A top-tier supercomputer can (theoretically) perform 1 billion hash calculations per second.

Why theoretically? Well, hash computations aren't as parallelizable as basic operations. That's why I conservatively estimate 1 billion hash calculations per second.

Okayu! Let's crunch the numbers again! Since this computer has a speed of \( 10^{9} \) checks per second. Here's the probability of having a hit after one century:

\[
N = 3.15 \times 10^{24}.
\]
\[
P_{\text{collision}} \approx 3.15 \times 10^{24} \cdot 4.81 \times 10^{-35} \approx 1.52 \times 10^{-10}.
\]

So, after a century of calculating bitcoin addresses with a supercomputer, you'd still have about 1 in a billion chance of hitting one of the addresses of interest. Just not worth it.

## What about a quantum computer?

We'd be using [Grover's algorithm](https://en.wikipedia.org/wiki/Grover%27s_algorithm) here.

Quick note though! Right now (even though there's so much news about ooo quantum here, quantum there), quantum is still at it's baby shoes. 

- Quantum computers capable of running Grover's algorithm on \( 2^{160} \)-bit problems require MILLIONS of qubits with extremely low error rates. The current "best" quantum computer by IBM has "just" 1,121 qubits.
- Quantum computers are highly error-prone. The level of error correction required for such large-scale computations are currently unachievable.
- The energy and cooling requirements for sustaining a quantum computer powerful enough for Grover's algorithm would be... astronomical?
- **Bitcoin could adopt quantum-resistant algorithms (like lattice-based cryptography or hash-based schemes) before practical quantum computers become a threat.**

Now that that's out of the way, let's calculate!

Let's assume a very advanced quantum computer capable of performing \( 10^{12} \) (1 trillion) address checks per second. 

**One year of calculations**

So, you finally get your one million qubits super quantum computer to work in your backyard, and get the chance to bruteforce some bitcoin addresses. How long will that take? With this example I'll just assume you have the computer, that there are no errors at all, and that you're able to power and cool that thing. Anyhow, here's after one year (31,536,000 seconds):

\[
N = 10^{12} \cdot 31,536,000 = 3.15 \times 10^{19}.
\]
\[
P_{\text{collision}} \approx 3.15 \times 10^{19} \cdot 2.08 \times 10^{-17} \approx 0.656.
\]

Crazy isn't it? Now you finally have a chance. You have a 65.6% chance of hitting *one* of the addresses ***after a year***. I can only imagine the look on your face when you find out that address holds 0.00000001 BTC hahaha..

## Conclusion

Yeah, what I'm trying to say is that this program won't suddenly make you rich overnight. I mean, maybe? It's *still a chance*.
