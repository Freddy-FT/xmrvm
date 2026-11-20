# Cryptocurrencies: From First Principles to Advanced Practice

> A complete guide for the serious newcomer — from foundations to research-level depth, with particular weight on Proof of Work, consensus algorithms, consensus rules, and Monero (XMR).
>
> **Fact labels:** `[Established]` = well-documented and uncontroversial. `[Disputed]` = contested in the literature or community; both sides are stated. `[Uncertain]` = limited evidence; author is not certain. Where figures depend on date, the date checked is stated.
>
> **Date of primary research:** November 2024 (the author re-checked key parameters; some live network figures such as hashrate change daily).

---

## Table of Contents

1. [Part 1: Foundations](#part-1-foundations)
2. [Part 2: Cryptographic Building Blocks](#part-2-cryptographic-building-blocks)
3. [Part 3: Anatomy of a Blockchain](#part-3-anatomy-of-a-blockchain)
4. [Part 4: The Consensus Problem](#part-4-the-consensus-problem)
5. [Part 5: Consensus Algorithms — A Full Survey](#part-5-consensus-algorithms--a-full-survey)
6. [Part 6: Proof of Work Algorithms in Depth](#part-6-proof-of-work-algorithms-in-depth)
7. [Part 7: Consensus Rules in Depth](#part-7-consensus-rules-in-depth)
8. [Part 8: Monero (XMR) in Depth](#part-8-monero-xmr-in-depth)
9. [Part 9: Virtual Machines](#part-9-virtual-machines)
10. [Part 10: What a Chain Needs to Function](#part-10-what-a-chain-needs-to-function)
11. [Part 11: Advanced Topics](#part-11-advanced-topics)
12. [Glossary](#glossary)
13. [Recommended Reading](#recommended-reading)

---

## Part 1: Foundations

### 1.1 What Money Is

Money is not a thing — it is a social technology. It solves a coordination problem: how can people exchange value across time and space without each trade requiring a perfect coincidence of wants?

Economists identify three functions that money must perform:

| Function | Meaning | Example |
|---|---|---|
| **Medium of exchange** | Accepted in trade for goods and services | Paying for bread with banknotes |
| **Store of value** | Preserves purchasing power over time | Saving for future use |
| **Unit of account** | A common yardstick for prices | Expressing all prices in dollars |

Commodity money (gold, silver, grain) satisfied these functions because the commodity itself had intrinsic value and was scarce by nature. Paper money (fiat) works because governments declare it legal tender and restrict its supply. Digital money today — a balance in a bank's database — is entirely abstract: a number that the bank promises to honor.

The critical insight is this: **in every system before cryptocurrency, the integrity of the ledger depended on trusting some institution**. That institution can fail, censor, inflate, or exclude.

### 1.2 The Double-Spending Problem

Suppose you send me a file called `one_bitcoin.txt`. You can also send the same file to Alice. The file is trivially copyable. This is the **double-spending problem**: a purely digital token can be duplicated and spent twice. Physical coins cannot be doubled this way, because handing me a coin removes it from your possession.

**The traditional solution** is a trusted authority — a bank — that maintains a master ledger. When you spend a dollar, the bank deducts it from your account. The bank enforces uniqueness. The cost of this solution is dependency: the bank can freeze your account, charge fees, exclude unbanked populations, or be coerced by governments.

**The Bitcoin solution** (described by Satoshi Nakamoto in a 2008 whitepaper [1]) removes the need for a trusted authority by replacing trust in an institution with trust in cryptographic proof and economic incentives. All participants collectively maintain and verify the ledger. Cheating is made computationally or economically prohibitive. This is the foundational idea of all public blockchains.

**A simplification note:** Saying "you can trust math instead of banks" elides significant complexity. The security of a public blockchain rests on game-theoretic assumptions (participants act rationally to maximize profit) and on the difficulty of certain mathematical problems (SHA-256 preimages, elliptic-curve discrete logarithms). None of these are unconditionally true. The rest of this guide unpacks exactly what those assumptions are.

### 1.3 Why Trusted Third Parties Were Needed — and What Removes Them

The Byzantine Generals Problem (Part 4) formalises why distributed agreement is hard. Before Bitcoin, every serious attempt at digital cash (DigiCash, e-gold, Hashcash) either relied on a central issuer, collapsed when the issuer was shut down, or failed to solve double-spending. `[Established]`

Satoshi's breakthrough was combining four pre-existing ideas:
1. **Cryptographic hash functions** — to link blocks immutably.
2. **Public-key digital signatures** — to prove ownership without a password database.
3. **Proof of Work** — to make Sybil attacks expensive (Part 6).
4. **Peer-to-peer gossip networking** — to distribute the ledger with no single point of failure.

None of these ideas was new. The novelty was in their combination and in the specific incentive structure (block rewards and fees) that makes honest participation the economically rational strategy.

---

### Part 1 Review Questions

**Q1. Name the three functions of money and give one example of a failure of each in fiat systems.**
A: Medium of exchange — hyperinflation destroys it (Weimar Germany, Zimbabwe). Store of value — inflation erodes purchasing power. Unit of account — if prices change too rapidly, the yardstick becomes useless.

**Q2. Why does simply sending a file not work as a digital payment?**
A: A file can be copied. The sender retains the original and can send it again (double-spend). There is no mechanism enforcing scarcity.

**Q3. What are the four pre-existing components Satoshi combined?**
A: Cryptographic hash functions, public-key digital signatures, Proof of Work, and peer-to-peer networking.

**Q4. What does "trust in math instead of banks" actually mean, and what are its limits?**
A: Security shifts from trusting a legal/institutional authority to trusting (a) the hardness of specific mathematical problems and (b) the assumption that rational actors will not attack a profitable system they depend on. It is not unconditional — quantum computers, bugs, or coordination failures could break it.

**Q5. Why did all pre-Bitcoin digital cash systems fail?**
A: They either required a central issuer (single point of failure, censorable) or could not prevent double-spending without centralised enforcement.

### Common Misconceptions

- **"Cryptocurrencies are anonymous."** Bitcoin is *pseudonymous*. All transactions are public. Only if you never link your pseudonym to your real identity are they private.
- **"Cryptocurrencies are backed by nothing."** Fiat currencies are also backed by nothing tangible. Bitcoin is backed by the cost of producing it (energy, hardware) and the collective agreement of its users. Whether that constitutes "backing" is a philosophical question.
- **"Cryptocurrencies will replace all money."** This is a prediction, not a fact.

### After Part 1 You Should Be Able To

Explain what money is and why physical money solves the double-spend problem. Explain why the digital equivalent requires either a trusted third party or a novel cryptographic system. Name the four ingredients Satoshi combined.

### Sources — Part 1

| Source | Date | Type |
|---|---|---|
| Satoshi Nakamoto, "Bitcoin: A Peer-to-Peer Electronic Cash System" — bitcoin.org/bitcoin.pdf | October 2008 | Primary (whitepaper) |
| Adam Back, "Hashcash — A Denial of Service Counter-Measure" — hashcash.org/papers/hashcash.pdf | 2002 | Primary (technical report) |
| Nick Szabo, "Bit Gold" — nakamotoinstitute.org/bit-gold | 2005 | Primary |
| Satoshi Nakamoto Institute, "The Complete Satoshi" — nakamotoinstitute.org | Various | Primary (archive) |

---

## Part 2: Cryptographic Building Blocks

### 2.1 Hash Functions

**Intuition:** A hash function is a blender that reduces any input — a word, a novel, a hard drive image — to a fixed-size fingerprint. The fingerprint appears random, but the same input always produces the same fingerprint. Crucially, you cannot reverse the blender: knowing the fingerprint tells you almost nothing about the input.

**Technical definition:** A cryptographic hash function H: {0,1}* → {0,1}^n satisfies:

- **Pre-image resistance:** Given y = H(x), it is computationally infeasible to find x.
- **Second pre-image resistance:** Given x, it is infeasible to find x' ≠ x such that H(x) = H(x').
- **Collision resistance:** It is infeasible to find any x, x' (x ≠ x') with H(x) = H(x').
- **Avalanche effect:** A single-bit change in input flips roughly half the output bits.

**SHA-256** (Bitcoin's choice) produces a 256-bit output, meaning there are 2^256 ≈ 10^77 possible outputs. Finding a collision by brute force would require more computations than there are atoms in the observable universe. `[Established]`

**Toy example:** The real SHA-256 is not human-readable, but here is a conceptual illustration. If H were a simple 4-bit hash:

```
Input "alice pays bob 1 BTC"  →  H  →  1011
Input "alice pays bob 2 BTC"  →  H  →  0100  (completely different!)
```

Change one character in the input and the entire output changes — that is the avalanche effect.

**Bitcoin uses SHA-256 twice (SHA-256d)** for the Proof of Work hash and transaction identifiers. This double application provides marginal extra security against certain length-extension attacks. `[Established]`

Monero uses a different hash function family (Keccak for some internal operations, and CryptoNight previously, then RandomX) discussed in Parts 6 and 8.

### 2.2 Public-Key Cryptography

**Intuition:** Imagine a special padlock you can manufacture and give to anyone. Anyone with the padlock can lock a box and send it to you. Only you have the matching key to open it. The padlock is your **public key**; the key is your **private key**.

**Technical foundation:** Public-key cryptography is built on **trapdoor functions** — mathematical operations easy to perform in one direction, infeasible to reverse without a secret piece of information.

The two dominant trapdoor families used in cryptocurrencies:
1. **Integer factorisation** (RSA): multiply two large primes p and q easily; factor their product n = p×q back into p and q — infeasible at scale.
2. **Elliptic-curve discrete logarithm** (ECDSA, Ed25519): multiply a point G by a scalar k easily; recover k from the product k·G — infeasible.

Cryptocurrencies overwhelmingly favour elliptic curves because they achieve the same security level as RSA with dramatically smaller keys. A 256-bit elliptic curve key is as secure as a ~3072-bit RSA key. `[Established]`

### 2.3 Elliptic Curves

**Intuition first:** An elliptic curve over the real numbers is a smooth curve satisfying an equation of the form y² = x³ + ax + b. Bitcoin uses the **secp256k1** curve defined by y² = x³ + 7 (mod p), where p is the 256-bit prime 2^256 − 2^32 − 977.

The curve has a special binary operation called **point addition**: draw a line through two points on the curve; it intersects the curve at a third point; reflect that point over the x-axis — that reflection is the "sum." Adding a point to itself (the tangent-line case) gives **point doubling**.

By repeating addition k times we get **scalar multiplication**: k · G, where G is a fixed base point. This operation is fast — it takes only O(log k) steps using the double-and-add method.

The **discrete logarithm problem on the curve** is: given G and Q = k·G, find k. No efficient classical algorithm is known for this. `[Established, with the caveat that a sufficiently large quantum computer running Shor's algorithm could solve it.] [Disputed: timeline for quantum threat varies from "decades away" to "10–15 years" among researchers.]`

**Private key → public key:**
1. Generate a random 256-bit integer `k` (private key). Keep secret.
2. Compute `Q = k · G` (scalar multiplication of the base point). This is the public key.
3. Share Q freely. No one can recover k from Q with current computers.

### 2.4 Digital Signatures (ECDSA and Schnorr)

A digital signature scheme lets you prove, without sharing your private key, that you authorised a specific message.

**ECDSA (Elliptic Curve Digital Signature Algorithm)** — used in Bitcoin before Taproot and in Monero internally:

```
Signing:
  1. Hash the message: z = H(message)
  2. Pick random nonce k (crucial: must be different every time and secret)
  3. Compute point (x, y) = k · G; set r = x mod n
  4. Compute s = k^-1 · (z + r·privKey) mod n
  5. Signature = (r, s)

Verification (anyone with public key Q can do this):
  1. Recompute z = H(message)
  2. Compute u1 = z · s^-1 mod n, u2 = r · s^-1 mod n
  3. Compute point (x, y) = u1·G + u2·Q
  4. Valid if x mod n = r
```

A critical property: if you reuse the nonce k across two different messages, your private key can be computed. The PlayStation 3 was broken this way in 2010. `[Established]`

**Schnorr signatures** (Bitcoin Taproot, BIP 340, activated November 2021) are simpler, linearly aggregatable, and provably secure in the random oracle model. They allow multiple parties to combine their keys and produce a single signature — enabling efficient multi-signature schemes.

**Ed25519** — used in Monero for spend authorisation — is a Schnorr-style signature on the Twisted Edwards curve Ed25519, known for high speed and resilience against poor random number generators.

### 2.5 Merkle Trees

**Intuition:** Suppose a block contains 2,000 transactions. If you want to prove that transaction #847 is in the block, you do not want to download all 2,000 transactions. A Merkle tree solves this by hashing transactions into a binary tree; you need only log₂(2000) ≈ 11 hashes to prove inclusion. This is a Merkle proof.

**Construction:**

```
Transactions:  T1   T2   T3   T4

Leaf hashes:   H1   H2   H3   H4
                 \ /         \ /
Internal nodes: H12          H34
                      \ /
Merkle root:         H1234
```

Each internal node is computed as H(left_child || right_child). The Merkle root is included in the block header. Changing any transaction changes its leaf hash, propagates up, and changes the root — invalidating the header. `[Established]`

**Merkle proofs (SPV):** A lightweight client that knows only block headers can verify "transaction T3 is in block B" by requesting the path [H4, H12] from a full node and recomputing:
- H34 = H(H3 || H4)
- H1234 = H(H12 || H34)
- Check: does H1234 match the root in the header?

This is the basis for **Simplified Payment Verification (SPV)** described in the Bitcoin whitepaper.

---

### Part 2 Review Questions

**Q1. What is the avalanche effect in hash functions and why does it matter for blockchains?**
A: A one-bit change in input changes approximately half the output bits. It means you cannot predict or reverse-engineer anything about the input from the output, and tampering with any part of a block produces a completely different hash.

**Q2. Why do cryptocurrencies prefer elliptic curves over RSA for key pairs?**
A: Equivalent security at much smaller key sizes. A 256-bit elliptic key ≈ a 3072-bit RSA key. Smaller keys mean less storage, faster computation, and smaller transactions.

**Q3. What catastrophic mistake can break ECDSA, and what real-world example illustrates it?**
A: Reusing the nonce k. If k is used for two messages, the private key can be algebraically recovered. The PlayStation 3 security was broken this way.

**Q4. What is a Merkle proof, and what problem does it solve?**
A: A set of intermediate hashes that proves a specific transaction is in a block without downloading all transactions. Enables lightweight clients (SPV) to verify transactions with only block headers.

**Q5. What is the discrete logarithm problem on an elliptic curve and why is it the foundation of key security?**
A: Given G and Q = k·G, find k. No efficient classical algorithm exists; this asymmetry is what makes private keys unrecoverable from public keys.

### Common Misconceptions

- **"Your Bitcoin is stored in a wallet."** A wallet stores private keys. The Bitcoin itself is a record on the blockchain. Lose the key and the coins are gone forever.
- **"Hashing the same input twice is useless."** SHA-256d provides marginal extra security against specific attack types (length-extension), not purely redundancy.
- **"Public keys are just addresses."** A Bitcoin address is typically the hash of a public key (not the key itself), adding a layer of indirection.

### After Part 2 You Should Be Able To

Explain what a hash function does and name its four key properties. Explain why scalar multiplication on an elliptic curve is a trapdoor. Sketch how ECDSA signing and verification work. Explain a Merkle proof and why SPV clients use it.

### Sources — Part 2

| Source | Date | Type |
|---|---|---|
| NIST FIPS 180-4: Secure Hash Standard — csrc.nist.gov/publications/fips/fips180-4 | 2015 | Primary (standard) |
| NIST FIPS 186-5: Digital Signature Standard — csrc.nist.gov/publications/fips/fips186-5 | 2023 | Primary (standard) |
| Bitcoin BIP 340 (Schnorr Signatures) — github.com/bitcoin/bips/blob/master/bip-0340.mediawiki | 2020 | Primary |
| Standards for Efficient Cryptography, SEC 2: secp256k1 — secg.org/SEC2-Ver-1.0.pdf | 2010 | Primary (standard) |
| Satoshi Nakamoto, Bitcoin whitepaper §7 (Reclaiming Disk Space / Merkle Trees) | 2008 | Primary |
| Dan Boneh, "Twenty Years of Attacks on RSA" — Notices of the AMS, 1999 | 1999 | Secondary |


---

## Part 3: Anatomy of a Blockchain

### 3.1 Transactions

A **transaction** is a signed instruction that changes the state of the ledger. Its structure differs between UTXO-based chains (Bitcoin) and account-based chains (Ethereum, Monero).

#### 3.1.1 The UTXO Model (Bitcoin)

**Intuition:** Think of UTXOs as physical banknotes. You do not have a bank balance; you have a pile of notes. When you spend, you hand over notes and get change back. Your "balance" is the sum of all the unspent notes that belong to your keys.

**UTXO = Unspent Transaction Output.** Every transaction:
- **Consumes inputs:** References to UTXOs from previous transactions (by transaction ID + output index). The signature unlocks them.
- **Creates outputs:** New UTXOs assigned to recipients (expressed as a locking script / address).
- **Fee = sum(inputs) − sum(outputs).** The difference goes to the miner.

```
Transaction structure (simplified Bitcoin):

  VERSION (4 bytes)
  ┌────────────────────────────────────────────────────┐
  │ INPUTS                                              │
  │   [0]: prev_txid=ab12..., vout=1, scriptSig=<sig>  │
  │   [1]: prev_txid=cd34..., vout=0, scriptSig=<sig>  │
  ├────────────────────────────────────────────────────┤
  │ OUTPUTS                                             │
  │   [0]: value=0.5 BTC,  scriptPubKey=OP_DUP ...     │
  │   [1]: value=0.49 BTC, scriptPubKey=OP_DUP ... (change) │
  └────────────────────────────────────────────────────┘
  LOCKTIME (4 bytes)
```

A transaction's **TXID** (transaction identifier) is SHA-256d of its serialised bytes. Nodes keep a **UTXO set** — a database of all currently unspent outputs — and check every spending input against it to prevent double-spending. `[Established]`

**Worked example:** Alice has two UTXOs: 0.3 BTC and 0.4 BTC. She wants to pay Bob 0.6 BTC.
1. She creates a transaction spending both UTXOs (total input: 0.7 BTC).
2. She adds two outputs: 0.6 BTC to Bob's address, 0.09 BTC to her own change address.
3. The miner collects 0.7 − 0.6 − 0.09 = 0.01 BTC as the fee.
4. After confirmation, her old UTXOs are destroyed and two new UTXOs exist.

#### 3.1.2 The Account Model (Ethereum)

**Intuition:** Think of a bank account. Every address has a balance stored in global state. Sending ETH directly subtracts from one account and adds to another.

Ethereum has two account types:
- **Externally Owned Account (EOA):** Controlled by a private key. Has a nonce, balance.
- **Contract Account:** Contains code and storage. Triggered by transactions.

An Ethereum transaction includes: `nonce`, `gasPrice` (or `maxFeePerGas` / `maxPriorityFeePerGas` after EIP-1559), `gasLimit`, `to`, `value`, `data`, `v/r/s` (signature).

The **nonce** is a per-sender counter that prevents replay attacks and enforces ordering of transactions from the same address.

**Comparison Table: UTXO vs Account**

| Property | UTXO (Bitcoin) | Account (Ethereum) |
|---|---|---|
| State representation | Set of unspent outputs | Key-value map of balances/storage |
| Parallelism | Easy — independent UTXOs | Harder — shared state conflicts |
| Privacy potential | Better — fresh address per transaction | Worse — all activity linked to address |
| Smart contracts | Difficult — limited script | Native — full Turing-complete VM |
| Double-spend prevention | UTXO set lookup | Nonce check + balance check |
| Replay attack | Naturally prevented by consuming UTXOs | Nonce + chain ID |

**Monero** uses a modified UTXO model (outputs rather than a traditional UTXO set) but with heavy cryptographic obfuscation — amounts are hidden and outputs are addressed to one-time stealth addresses. See Part 8.

### 3.2 Blocks

A **block** is a container that bundles transactions and links them to the chain.

**Bitcoin block structure:**

```
┌─────────────────────────────────────────────────────────┐
│                     BLOCK HEADER (80 bytes)              │
│  version (4B)                                            │
│  prev_block_hash (32B)   ← Links to parent block        │
│  merkle_root (32B)       ← Commits to all transactions  │
│  timestamp (4B)          ← Unix time                    │
│  bits (4B)               ← Encoded difficulty target    │
│  nonce (4B)              ← Miners vary this to find PoW │
├─────────────────────────────────────────────────────────┤
│                     BLOCK BODY                           │
│  tx_count (varint)                                       │
│  Coinbase transaction (creates new BTC)                  │
│  Transaction 1                                           │
│  Transaction 2                                           │
│  ...                                                     │
└─────────────────────────────────────────────────────────┘
```

The **block hash** = SHA-256d(header). It commits to everything in the header, which via the Merkle root commits to all transactions.

The **coinbase transaction** is special: it has no inputs (creates money from nothing), and must pay at most the block subsidy plus fees. It is how new coins enter circulation. Bitcoin's block subsidy is currently 3.125 BTC per block (after the fourth halving in April 2024). `[Established, checked November 2024]`

### 3.3 Chain Structure

Blocks form a **chain** because each block header contains the hash of the previous block. Changing any block changes its hash, which invalidates the hash pointer in the next block, which cascades all the way to the present — requiring the attacker to redo Proof of Work for every subsequent block.

```
  Block N-1          Block N           Block N+1
┌──────────┐       ┌──────────┐      ┌──────────┐
│ Header   │       │ Header   │      │ Header   │
│ prev= ───┼──────→│ prev= ───┼─────→│ prev= ───┼──→ ...
│ merkle   │       │ merkle   │      │ merkle   │
│ nonce    │       │ nonce    │      │ nonce    │
├──────────┤       ├──────────┤      ├──────────┤
│ Txs ...  │       │ Txs ...  │      │ Txs ...  │
└──────────┘       └──────────┘      └──────────┘
```

The **genesis block** (block 0) has no parent; its `prev_block_hash` is all zeros.

### 3.4 Nodes and Participant Types

The term "node" covers many distinct roles in a blockchain network. Here is a comprehensive taxonomy:

| Node Type | What It Does | Storage | Who Runs It |
|---|---|---|---|
| **Full node** | Downloads, validates, and stores all blocks and transactions from genesis. Enforces all consensus rules. | Full chain | Exchanges, individuals, infrastructure |
| **Archive node** | Full node + stores complete historical state at every block height. Supports `eth_getBalance` at arbitrary past blocks. | Full chain + all states | Blockchain explorers, analytics firms |
| **Light / SPV node** | Downloads only headers. Verifies own transactions via Merkle proofs. Trusts that the heaviest chain is valid. | Headers only | Mobile wallets |
| **Miner node** | Performs Proof of Work. Assembles blocks. Collects rewards. | Usually full node | Mining farms, solo miners |
| **Validator node** | In PoS chains: proposes and votes on blocks, subject to slashing. | Full or partial state | Stakers, staking services |
| **Seed / bootstrap node** | Provides a list of peers to new nodes joining the network. Does not validate blocks itself. | Peer address book | Protocol teams, well-known operators |
| **Masternode** (Dash) | Provides privacy mixing (CoinJoin), InstantSend; requires 1,000 DASH collateral. Earns a portion of block rewards. | Full node | Dash investors |
| **Sequencer** (rollups) | In an L2 rollup: orders user transactions and posts batches to L1. Centralised in most current rollups. | L2 state | Rollup operators (Arbitrum, Optimism teams) |
| **Relayer** | Passes messages between chains (IBC, LayerZero). Does not validate; passes signed packets. | Partial | Protocol teams, independent operators |
| **Oracle node** | Brings off-chain data (prices, weather, sports results) on-chain via signed reports or aggregation. | External data sources | Chainlink, Band Protocol, Pyth |
| **Block builder** (PBS) | In Ethereum post-Merge: constructs block bodies to maximise MEV, sells them to proposers. | Full state | Sophisticated MEV bots, builders |
| **Searcher** (MEV) | Monitors mempools for profitable ordering opportunities; bribes builders/validators. | Mempool, state | Algorithmic trading bots |
| **Watchtower** (Lightning) | Monitors Bitcoin for fraudulent channel closings on behalf of offline LN users. | LN channel state | Service providers, nodes |
| **Prover node** (ZK rollups) | Generates zero-knowledge proofs for rollup state transitions. Very compute-intensive. | L2 state + ZK circuit | Rollup operators (zkSync, StarkNet) |

**Note on completeness:** This table is likely not exhaustive. As blockchain architectures specialise (modular chains, restaking, decentralised sequencer networks), new participant roles emerge. When in doubt, check the specific chain's documentation.

### 3.5 The Mempool

The **mempool** (memory pool) is each node's local in-memory set of valid but unconfirmed transactions waiting to be included in a block. It is not a global consensus object; each node has its own mempool and they differ.

Transactions enter the mempool when:
1. The node receives a new transaction via peer-to-peer gossip.
2. The transaction passes basic validity checks (valid signature, unspent inputs, no malformed structure).

Miners/validators select transactions from the mempool using fee-ordering (highest fee-rate first). Bitcoin uses `sat/vbyte` as its fee unit; Ethereum uses `gwei/gas`.

**Replace-By-Fee (RBF):** Bitcoin's opt-in mechanism that allows a sender to replace an unconfirmed transaction with a higher-fee version. Useful when fees spike after broadcast.

**Child-Pays-For-Parent (CPFP):** A child transaction spending an unconfirmed parent can offer a large enough fee that miners include both the parent and child to collect the total fee.

Mempools are bounded in size. When full, low-fee transactions are evicted. During periods of high demand (Bitcoin in 2017, 2021), mempool backlogs reached hundreds of thousands of transactions.

### 3.6 Full Transaction Lifecycle

```
  User creates transaction
         │
         ▼
  Wallet signs with private key
         │
         ▼
  Broadcast to peer-to-peer network (gossip protocol)
         │
         ▼
  Every receiving node validates:
   ─ Signatures correct?
   ─ Inputs unspent? (UTXO set / account nonce)
   ─ Fee ≥ minimum?
   ─ No other rule violations?
         │  (if invalid, drop silently)
         ▼
  Transaction enters each node's mempool
         │
         ▼
  Miner/validator selects transaction for a block
   (prioritised by fee; subject to MEV ordering)
         │
         ▼
  Block is mined/proposed and broadcast
         │
         ▼
  Network validates and accepts block
         │
         ▼
  Transaction confirmed in block height N
         │
         ▼
  Each subsequent block ("confirmation") makes
  reversion exponentially harder
         │
         ▼
  Practical finality:
   Bitcoin: ~6 confirmations (~60 min) for high-value
   Ethereum: BFT finality in ~12.8 minutes
   Monero: 10 confirmations (~20 min) standard;
           outputs unlock after 10 blocks (ratchet rule)
```

**A note on finality:** "Finality" means different things in different systems. In Nakamoto-consensus chains, finality is **probabilistic** — the probability of reversal decreases exponentially with each confirmation but is never strictly zero. In BFT-based systems (Part 5), finality is **deterministic** once a quorum of validators has committed to a block. This distinction matters for exchanges, payment processors, and smart contract calls that depend on irreversible settlement. The specifics of Monero's output unlock rule are covered in Part 8. `[Established]`

---

### Part 3 Review Questions

**Q1. In Bitcoin's UTXO model, where does the transaction fee go, and how is it computed?**
A: The miner collects it. It equals sum(inputs) − sum(outputs). There is no explicit fee field; it is an implicit remainder.

**Q2. What is the difference between a full node and an archive node?**
A: Both validate every block from genesis. An archive node additionally stores the full chain state at every block height, allowing historical state queries. A regular full node only retains the current UTXO set (Bitcoin) or current state (Ethereum).

**Q3. What is the role of the nonce in an Ethereum transaction (not mining)?**
A: A per-sender counter that prevents replay attacks and enforces ordering. Transaction N+1 from the same account cannot be included before transaction N.

**Q4. How does the block's Merkle root protect transaction integrity?**
A: The Merkle root is included in the block header, which is included in the block hash, which is included in the next block's `prev_block_hash`. Changing any transaction changes the Merkle root, which changes the block hash, requiring all subsequent Proof of Work to be redone.

**Q5. What is the difference between a sequencer and a validator in modern blockchain architectures?**
A: A validator participates in consensus and verifies blocks (PoS chains). A sequencer is specific to L2 rollups and orders user transactions off-chain, posting batches to L1. Sequencers do not perform consensus validation; they are centralised ordering services in most current rollup designs.

### Common Misconceptions

- **"The longest chain is always correct."** The correct chain is the one with the most cumulative Proof of Work (the heaviest chain), not necessarily the most blocks. See Part 7.
- **"SPV clients are as secure as full nodes."** SPV clients trust that the heaviest chain is valid; they do not independently verify all consensus rules. A majority of miners could feed them invalid transactions in some attack scenarios.
- **"The mempool is a global, shared data structure."** Each node has its own mempool. There is no global mempool and no guarantee that all nodes have seen the same transactions.

### After Part 3 You Should Be Able To

Explain the UTXO model and account model, and state one advantage of each. Draw the structure of a Bitcoin block header and explain why each field exists. Name and distinguish at least five types of network participant. Trace a transaction from creation to confirmation.

### Sources — Part 3

| Source | Date | Type |
|---|---|---|
| Satoshi Nakamoto, Bitcoin whitepaper §6–7 | 2008 | Primary |
| Bitcoin Core developer guide — developer.bitcoin.org/devguide/transactions | Ongoing | Documentation |
| Ethereum Yellow Paper (Gavid Wood) — ethereum.github.io/yellowpaper/paper.pdf | 2014, updated | Primary |
| docs.getmonero.org/technical-specs/ — Monero Technical Specification | Ongoing | Documentation |
| BIP-125: Opt-in Full Replace-by-Fee Signalling | 2015 | Primary (BIP) |


---

## Part 4: The Consensus Problem

### 4.1 What Nodes Must Agree On

For a blockchain to function, nodes must agree on three things:
1. **Which transactions are valid** — are signatures correct, are inputs unspent, are all rules satisfied?
2. **Which blocks exist** — the network may receive conflicting blocks at the same height.
3. **In what order** — the same set of transactions executed in a different order can produce different final states (especially with smart contracts and balance-dependent operations).

This is the consensus problem: achieving agreement among distributed, possibly unreliable, possibly adversarial participants.

### 4.2 The Byzantine Generals Problem

**Intuition:** Imagine several army divisions surrounding an enemy city. The generals must all attack at the same time to succeed. They communicate only by messenger. Some generals may be traitors who send conflicting messages to different generals to cause disagreement and defeat. How do the loyal generals agree on a plan?

**Formal definition (Lamport, Shostak, Pease, 1982):** Given n generals, f of whom are Byzantine (malicious — they can send arbitrary, contradictory messages), it is possible to reach agreement if and only if n ≥ 3f + 1. With fewer honest generals, no deterministic protocol can guarantee agreement. `[Established]`

**Applied to blockchains:** Nodes are the generals. Byzantine nodes can send conflicting blocks, withhold information, or lie about which transactions they received. Consensus protocols are algorithms that enable the loyal majority to agree.

**Crash faults vs Byzantine faults:** A crash fault is a node that stops responding (simple failure). A Byzantine fault is a node that behaves arbitrarily and maliciously. Byzantine fault tolerance (BFT) is strictly harder than crash fault tolerance (CFT). Many classical distributed databases handle crash faults (CFT) only; blockchain protocols must handle Byzantine faults.

### 4.3 Sybil Attacks

A **Sybil attack** (named after the 1973 book about multiple personality disorder) is an attack where one physical entity creates many fake identities to gain disproportionate influence. If consensus were simply "one node, one vote," an attacker could create millions of fake nodes and control the outcome.

Blockchain protocols defend against Sybil attacks by tying voting power to a scarce resource:
- **Proof of Work:** Voting power ∝ computational power. Creating fake identities costs nothing, but mining requires real hardware and energy.
- **Proof of Stake:** Voting power ∝ staked coins. More Sybil identities do not increase total stake.
- **Proof of Identity/Authority:** Only known, vetted participants may vote (PoA chains). Not Sybil-resistant in a permissionless sense.

### 4.4 Synchrony Models

Consensus protocol designers must assume something about network timing. There are three models:

| Model | Assumption | Real-world analogy |
|---|---|---|
| **Synchronous** | Messages arrive within a known bound Δ | A telephone call — both parties respond immediately |
| **Asynchronous** | Messages eventually arrive but with no bound | Postal mail — may take days, weeks, or never |
| **Partially synchronous** | The network is eventually synchronous (after some Global Stabilisation Time, GST) | The internet — usually fast, occasionally slow |

Most Internet-based blockchain protocols operate in a **partially synchronous** model (Dwork, Lynch, Stockmeyer, 1988). They are safe (do not produce conflicting decisions) even under asynchrony, but live (make progress) only when the network is synchronous. `[Established]`

### 4.5 FLP Impossibility

In 1985, Fischer, Lynch, and Paterson proved the **FLP impossibility theorem**: in a purely asynchronous network, no deterministic consensus protocol can guarantee both **safety** and **liveness** if even one node may fail (by crashing — not even Byzantine). `[Established]`

**What this means in practice:** You cannot build a perfect blockchain under pure asynchrony. Every real protocol makes trade-offs:
- **Bitcoin's approach:** Accepts probabilistic (not deterministic) finality; always makes progress (liveness), but history can theoretically be rewritten.
- **BFT approach (Tendermint, HotStuff):** Assumes partial synchrony; achieves deterministic finality but can halt entirely if the network is too slow (sacrificing liveness during network partitions).

This is not a bug but a fundamental theorem of distributed computing.

### 4.6 The CAP Theorem

Brewer's CAP theorem (formulated 2000, proved by Gilbert and Lynch 2002) states that a distributed system can guarantee at most two of:

- **Consistency (C):** Every read returns the most recent write.
- **Availability (A):** Every request receives a response.
- **Partition tolerance (P):** The system continues to function even when network partitions separate nodes.

Since network partitions are unavoidable in practice, real systems must choose between C and A during a partition:
- **CP systems** (prioritise consistency): stop serving requests if a quorum cannot be reached. BFT blockchains lean CP.
- **AP systems** (prioritise availability): continue serving requests but may return stale data. Nakamoto consensus leans AP (always produces blocks, but different nodes may temporarily see different chains).

`[Note: The original CAP theorem is a binary choice; in practice the trade-off is a spectrum. Variants like PACELC extend it to latency trade-offs even without partitions.]`

### 4.7 Safety vs Liveness

These two properties appear throughout consensus literature:

- **Safety:** "Nothing bad ever happens." The system never produces two conflicting decisions (no two blocks at the same height are both finalised).
- **Liveness:** "Something good eventually happens." New blocks keep getting added; the system does not halt indefinitely.

**The tension:** A protocol that never makes decisions is trivially safe (no decisions, no conflicts). A protocol that always makes decisions no matter what can be forced into inconsistency by an adversary. Every real protocol is a point in this trade-off space.

Nakamoto consensus sacrifices **finality** (a form of safety) for **liveness**: it always adds blocks, but no block is ever provably final. BFT protocols sacrifice **liveness** (halt during network partitions) for **finality**: once a block is committed, it is final and cannot be reversed.

### 4.8 The Nakamoto Approach as One Answer Among Several

Satoshi's specific answer to the consensus problem was:
1. **Replace identity-based voting with work-based voting.** Nodes prove they spent computation; this costs real money and defeats Sybil attacks.
2. **Accept probabilistic finality.** The probability of reverting a block decreases exponentially with the depth of burial (each subsequent block adds more confirmations).
3. **Use the heaviest-chain rule** to resolve temporary forks.
4. **Align incentives:** honest mining is profitable; dishonest mining is costly and likely unprofitable.

This is elegant and elegant in its simplicity, but it is not the only valid approach. Part 5 surveys many alternatives.

**BFT protocols** had existed since the 1980s but required all participants to be known in advance, which limits them to permissioned networks (a small, known validator set). Proof of Work enabled consensus among anonymous participants for the first time — this is Nakamoto's genuinely novel contribution. `[Established]`

---

### Part 4 Review Questions

**Q1. What is the minimum number of nodes required to tolerate f Byzantine faults?**
A: n ≥ 3f + 1. With 3f nodes or fewer, the loyal majority cannot prevent Byzantine nodes from disrupting consensus.

**Q2. Why is a Sybil attack ineffective against Proof of Work?**
A: Creating additional identities is free (no cost to register fake nodes), but mining power is proportional to real hardware and electricity. More Sybil nodes add zero hash power.

**Q3. State the FLP impossibility theorem in one sentence and name its implication for blockchains.**
A: In a purely asynchronous system, no deterministic protocol can guarantee both safety and liveness when even one node may crash. Implication: every real blockchain must sacrifice either finality or liveness under adverse conditions.

**Q4. What is the difference between safety and liveness?**
A: Safety = "nothing bad happens" (no two nodes finalise conflicting blocks). Liveness = "something good eventually happens" (the chain keeps growing).

**Q5. Why was Proof of Work a genuinely novel contribution to distributed systems?**
A: BFT protocols existed before but required all participants to be known in advance (permissioned). PoW enables Byzantine fault-tolerant consensus among anonymous, adversarial, permissionless participants.

### Common Misconceptions

- **"Bitcoin is 100% secure."** Bitcoin is secure under the assumption that no single entity controls ≥50% of hash power and that miners act rationally. Neither is guaranteed forever.
- **"Blockchains are BFT."** Nakamoto consensus is not BFT in the classical sense — it uses probabilistic finality, not quorum-based finality. Only some PoS chains with BFT finality gadgets achieve classical BFT guarantees.
- **"The CAP theorem means blockchains can't be consistent and available."** CAP applies during a network partition. Under normal network conditions, blockchains can be both.

### After Part 4 You Should Be Able To

State the Byzantine Generals Problem and its solution threshold. Distinguish crash faults from Byzantine faults. Explain the FLP impossibility and what it forces protocol designers to compromise. Explain the safety/liveness trade-off and place Bitcoin and BFT protocols on that spectrum.

### Sources — Part 4

| Source | Date | Type |
|---|---|---|
| Lamport, Shostak, Pease, "The Byzantine Generals Problem" — ACM TOPLAS | 1982 | Primary |
| Fischer, Lynch, Paterson, "Impossibility of Distributed Consensus with One Faulty Process" — JACM | 1985 | Primary |
| Gilbert, Lynch, "Brewer's Conjecture and the Feasibility of Consistent, Available, Partition-Tolerant Web Services" — SIGACT News | 2002 | Primary |
| Dwork, Lynch, Stockmeyer, "Consensus in the Presence of Partial Synchrony" — JACM | 1988 | Primary |
| Nakamoto, Bitcoin whitepaper | 2008 | Primary |

---

## Part 5: Consensus Algorithms — A Full Survey

### 5.1 Nakamoto Consensus (Proof of Work + Heaviest-Chain Rule)

**Block proposal:** Any node may mine a block at any time. It hashes the block header repeatedly until the output is below the difficulty target.

**Validation:** Every node independently verifies the block's Proof of Work and all transactions. Invalid blocks are rejected and never forwarded.

**Ordering:** The winning block extends the tip of the heaviest chain. If two blocks arrive simultaneously, both become temporary chain tips. The fork resolves when one side gets the next block — the other becomes an orphan.

**Fork resolution:** Nodes follow the **heaviest-chain rule**: the chain with the most cumulative Proof of Work wins, regardless of block count. (Satoshi's whitepaper says "longest chain"; the precise meaning is "most work." See Part 7.) `[Established]`

**Finality:** Probabilistic. The probability of reverting k-deep block is approximately (q/p)^k, where p is the honest hash fraction and q = 1−p. For p = 0.9 (10% attacker), a 6-deep block has reversal probability < 0.1%. Never strictly zero.

**Fault tolerance:** Tolerates up to <50% Byzantine hash power. Above 50%, an attacker can outpace honest miners and rewrite history.

**Main attacks:** 51% attack (Section 6.3), selfish mining (Section 6.3), eclipse attacks, long-range attacks (not applicable to PoW).

**Used by:** Bitcoin, Bitcoin Cash, Litecoin, Monero, Ethereum Classic, Kaspa, many others.

### 5.2 Proof of Stake — Chain-Based: Ouroboros (Cardano)

**How it works:** Cardano's Ouroboros protocol (Kiayias et al., 2017) is the first PoS protocol with a formal security proof in the synchronous model. Time is divided into **epochs** (5 days) → **slots** (20 seconds). For each slot, a slot leader is selected via cryptographic sortition proportional to stake. The leader proposes a block; other nodes validate.

**Validator selection:** Based on a Verifiable Random Function (VRF) seeded by a secure randomness beacon derived from the previous epoch's blocks. No one can predict future leaders far in advance.

**Fork resolution:** Heaviest-chain rule (most slots won) with an epoch-based finality checkpoint.

**Finality:** Probabilistic within an epoch; Ouroboros Praos (the more recent variant) achieves security under partial synchrony.

**Fault tolerance:** Tolerates up to <50% Byzantine stake.

**Nothing-at-stake problem:** `[Disputed]` In naive PoS, validators can vote on multiple forks at no cost (unlike PoW miners who must commit hardware to one chain). Ouroboros addresses this by tying stake to a specific stake pool; voting on multiple chains is penalised in later variants (Ouroboros Genesis). Whether these mitigations are fully effective is debated.

**Long-range attacks:** An adversary who corrupts old keys from an earlier epoch can rewrite history from that epoch at negligible cost (old stake is worthless but old keys still sign). Ouroboros mitigates this with **checkpointing** and **key-evolving signatures** that make old keys unusable. `[Established mechanism; sufficiency disputed among researchers.]`

**Used by:** Cardano (ADA).

### 5.3 Proof of Stake — BFT Hybrid: Ethereum Gasper (Casper FFG + LMD-GHOST)

Ethereum's post-Merge consensus is a hybrid combining two protocols:

**LMD-GHOST (Latest Message Driven Greedy Heaviest Observed Subtree):** A fork-choice rule. Validators vote on the head of the chain. When picking among forks, nodes count the latest vote from each validator (not all votes) and follow the fork with the most weight. This prevents long-range attacks from influencing the tip through replayed old votes.

**Casper FFG (Friendly Finality Gadget, Buterin & Griffith, 2017):** A BFT overlay providing **deterministic finality in epochs** (~6.4 minutes per epoch, 32 slots). Validators vote on "checkpoint" blocks at epoch boundaries. A block is "justified" after ≥2/3 of validators vote for it; it is "finalised" after the checkpoint following it is also justified.

**Together (Gasper):** LMD-GHOST selects the current chain tip (fork choice), while Casper FFG periodically finalises checkpoints. Once finalised, a block cannot be reverted without slashing ≥1/3 of all staked ETH.

**Slashing:** Validators who sign two conflicting blocks at the same slot ("double voting") or who make contradictory attestations are automatically and irreversibly penalised (slashed), losing a substantial portion of their stake. This provides economic finality.

**Fault tolerance:** Liveness requires ≥2/3 of validators online; finality requires ≤1/3 Byzantine validators.

**Weak subjectivity:** New nodes that sync from scratch cannot determine which of two chains is "real" using only the protocol rules — they must trust a checkpoint from a trusted source. This is philosophically different from Bitcoin's fully objective sync. `[Disputed: Ethereum developers argue this is a minor, manageable issue; critics argue it introduces a subtle trust assumption.]`

**Used by:** Ethereum (ETH).

### 5.4 Classical BFT: PBFT

**PBFT (Practical Byzantine Fault Tolerance, Castro & Liskov, 1999)** is the foundational practical BFT protocol. It operates in three phases per consensus round:

```
  CLIENT → PRIMARY (leader)
                │
                ▼
       PRE-PREPARE: leader sends proposed value to all replicas
                │
                ▼
       PREPARE: each replica broadcasts "I received the same proposal" to all others
                │
                ▼
       COMMIT: once a replica sees 2f+1 PREPARE messages, it broadcasts COMMIT
                │
                ▼
       REPLY: once a replica sees 2f+1 COMMIT messages, it executes and replies to client
```

**Communication complexity:** O(n²) per round (every node messages every other node).

**Fault tolerance:** Tolerates f < n/3 Byzantine faults (needs 3f+1 replicas total).

**Finality:** Deterministic — once committed, a block is final.

**Weakness:** Does not scale beyond ~100 nodes (O(n²) messages becomes prohibitive). Leader rotation is complex; a Byzantine leader can stall the system (solved by view-change protocol).

**Used by:** Hyperledger Fabric, early enterprise DLTs.

### 5.5 Classical BFT: Tendermint / CometBFT

**Tendermint (Buchman, 2016; Kwon, 2014)** is a BFT protocol designed for blockchain use. It is the consensus layer of the Cosmos ecosystem (now branded **CometBFT**).

**Two phases per block:**
1. **Prevote:** Proposer sends block proposal. All validators broadcast PREVOTE.
2. **Precommit:** Once ≥2/3 prevotes received, validators broadcast PRECOMMIT.
3. **Commit:** Once ≥2/3 precommits received, the block is committed (deterministic finality).

**Leader rotation:** Round-robin, with weighted selection by stake. If a proposer is offline, validators wait for a timeout and move to the next round with a new proposer.

**Locking:** A validator that precommits a block is "locked" on it and cannot vote for a competing block in the next round unless it sees ≥2/3 precommits for the competing block. This prevents safety violations during re-proposing.

**Communication complexity:** O(n²) in the common case but with optimisations (gossip instead of all-to-all) in practice.

**Fault tolerance:** <1/3 Byzantine validators.

**Finality:** Deterministic within one round (seconds).

**Used by:** Cosmos Hub, BNB Chain (modified), many Cosmos SDK chains.

### 5.6 Classical BFT: HotStuff and Its Derivatives

**HotStuff (Yin et al., 2018-2019)** is a significant advance over PBFT and Tendermint. Its key innovation: **linear communication complexity** O(n) per round using threshold signatures (or linear signatures with aggregation). It achieves this by routing all messages through a leader, who collects and aggregates votes into a **quorum certificate (QC)**.

**Three-phase pipeline:**
- **Prepare:** Leader proposes, validators respond. Leader aggregates 2f+1 votes into a Prepare QC.
- **Pre-commit:** Leader sends Prepare QC, validators respond. Leader creates Pre-commit QC.
- **Commit:** Leader sends Pre-commit QC, validators respond. Leader creates Commit QC → block is final.

**Pipelining:** Consecutive rounds overlap — the commit phase of round k and the prepare phase of round k+1 happen simultaneously. This achieves 1-block latency amortised.

**Derivatives:**
- **DiemBFT/LibraBFT** (Meta's Diem project, now defunct) — HotStuff variant with leader reputation scoring.
- **Jolteon** (2021) — reduces rounds from 3 to 2 by combining phases, achieving lower latency.
- **Shoal++ / AptosBFT** (Aptos chain) — further latency optimisations, pipelined DAG-based mempool (Quorum Store).
- **FastHotStuff / Sailfish** — further research variants.

**Fault tolerance:** <1/3 Byzantine validators.

**Finality:** Deterministic.

**Used by:** Aptos, Flow (Dapper), and multiple enterprise chains.

### 5.7 Delegated Proof of Stake (DPoS) and Proof of Authority (PoA)

**DPoS:** Token holders vote for a fixed number of **block producers** (21 in EOS, 27 in Tron). These producers take turns proposing blocks in a round-robin schedule. Governance is explicit — producers who misbehave are voted out.

Faster and cheaper than full BFT with large validator sets, but significantly **more centralised**: 21 known producers is a small trust set. `[Disputed: proponents argue alignment through token voting; critics argue it recreates oligarchic governance.]`

**Used by:** EOS (EOSIO), Tron, Steem, some Cosmos chains.

**PoA (Proof of Authority):** Block production rights are granted to a pre-approved, known set of validators, typically identified by real-world identity. No economic stake required; instead, reputation is the stake. Suitable for permissioned chains where decentralisation is not a goal.

**Used by:** Ethereum's Görli and Rinkeby testnets (now deprecated), Binance Smart Chain's genesis validator set, private enterprise chains.

### 5.8 Avalanche Consensus (Snow Family)

**Avalanche (Rocket et al., 2018; "Team Rocket" anonymous whitepaper; formalised by Yin, Guo et al.)** is a family of probabilistic consensus protocols based on **repeated random subsampling** — a form of metastable gossip.

**How Slush (the simplest version) works for a binary decision (A or B):**
1. A node samples k random peers and asks their preference.
2. If more than a threshold α respond with the same value, the node adopts that value.
3. Repeat for r rounds. The process snowballs: once a slight majority prefers A, more and more nodes adopt A.

**Snowflake** extends Slush with a "confidence counter": a node only changes preference after seeing α responses multiple times, making it resistant to network noise.

**Snowball** extends Snowflake with a "conviction counter": the node tracks how many times each value received α responses, adopting the one with the most convictions. Very hard to reverse once confident.

**Avalanche** (the full protocol) applies Snowball to a DAG of transactions rather than a binary decision, allowing concurrent transactions and very high throughput.

**Fault tolerance:** Theoretically tolerates up to ~1/2 Byzantine nodes (probabilistic). `[The exact guarantee depends on parameters and is probabilistic, not deterministic — Disputed whether this meets classical BFT standards.]`

**Finality:** Probabilistic, achievable in sub-second timeframes under good network conditions.

**Throughput:** Avalanche reports 4,500+ TPS in production. `[Uncertain: benchmarks vary by conditions and are often disputed.]`

**Used by:** Avalanche (AVAX) and its subnets.

### 5.9 DAG-Based Designs

#### 5.9.1 GHOSTDAG (Kaspa)

**GHOSTDAG** extends the GHOST protocol to a **block-DAG** (Directed Acyclic Graph). Instead of one block per round, miners can produce multiple blocks simultaneously, all of which are valid and reference multiple parents.

**Key idea:** Rather than discarding orphan blocks (as Bitcoin does), GHOSTDAG includes them in a **k-cluster** ordering. The k-cluster is the largest set of blocks in the DAG that is "well-connected" (every two blocks have at most k blocks outside the cluster separating them). This allows very high block rates (Kaspa targets one block per second or faster) while maintaining security comparable to Bitcoin at equivalent hash rates. `[Established mechanism; theoretical security proofs exist.]`

**Used by:** Kaspa (KAS).

#### 5.9.2 Narwhal and Bullshark

**Narwhal** (Spiegelman et al., 2022) separates **data availability** from **consensus**. It is a reliable broadcast protocol that constructs a DAG of "certificates" — blocks that have been confirmed as available to 2f+1 validators — without itself ordering them.

**Bullshark** (Spiegelman et al., 2022) is an asynchronous BFT protocol that runs on top of Narwhal's DAG to produce a total order. It achieves consensus purely from the DAG structure — no additional message rounds needed in the common case.

**Benefits:** Decoupling data availability from ordering allows very high throughput (100,000+ TPS claimed in benchmarks). `[Uncertain for production environments.]`

**Used by:** Sui, early Aptos (now uses Quorum Store + Jolteon), research systems.

### 5.10 Other Designs

#### Proof of History (Solana)

**PoH is not a consensus mechanism on its own but a cryptographic clock.** It is a **Verifiable Delay Function (VDF)** applied recursively: H(H(H(H(...H(seed)...)))) producing a sequence where each step proves real time elapsed. Validators embed event hashes into the PoH sequence, timestamping them relative to each other without communication.

**Consensus:** Solana's actual consensus is **Tower BFT** — a modified PBFT-style protocol running on top of PoH. PoH reduces the communication overhead by pre-ordering events before the BFT phase runs.

**Used by:** Solana (SOL). Throughput claims reach ~65,000 TPS (peak theoretical); real-world sustained throughput is lower. `[Uncertain: network has experienced multiple outages.]`

#### Proof of Space and Time (Chia)

**PoSpace:** Farmers (miners) allocate hard drive space to store cryptographic "plots." A plot is a large lookup table derived from their keys. When the network broadcasts a challenge, farmers check whether their plot contains a close match.

**PoTime:** A **Verifiable Delay Function** runs in parallel to prevent grinding attacks (computing many challenges rapidly to find a favourable one). The VDF chain also creates a synchronised time reference.

**Used by:** Chia (XCH). Concerns about large-scale hard drive acquisition by well-funded farms. `[Disputed: whether PoSpace achieves the decentralisation PoW farms have not.]`

#### Proof of Elapsed Time (Intel SGX)

**PoET** is used in Hyperledger Sawtooth. Each node requests a random wait time from an **Intel SGX** trusted execution environment (TEE). The node with the shortest wait time wins the right to propose the next block.

**Security limitation:** Relies entirely on Intel's SGX hardware integrity. If SGX is compromised (and SGX has had several significant vulnerabilities), the protocol fails. Not suitable for permissionless environments. `[Established limitation]`

**Used by:** Hyperledger Sawtooth (now less active).

#### Stellar Consensus Protocol (SCP) — Federated Byzantine Agreement

**SCP** (Mazières, 2015) uses **Federated Byzantine Agreement (FBA)**: each node defines its own "quorum slice" — a set of nodes it trusts. Safety and liveness depend on the intersection properties of all quorum slices.

**Used by:** Stellar (XLM), Ripple (XRP uses a similar model).

**Weakness:** If quorum slices are poorly configured, the network can split. `[Established concern, mitigated by default node configurations but not eliminated.]`

#### Algorand (Pure PoS with Cryptographic Sortition)

**Algorand** (Micali et al., 2017) uses a **VRF-based sortition** to randomly and privately select committee members for each block. Only the selected member knows they were selected until they reveal their block, preventing targeted attacks.

**Finality:** Fast (< 5 seconds), deterministic once committee threshold reached.

**Used by:** Algorand (ALGO).

### 5.11 Consensus Algorithm Comparison Table

| Algorithm | Fault Tolerance | Finality | Est. Throughput | Decentralisation | Hardware Needs | Key Weakness |
|---|---|---|---|---|---|---|
| Nakamoto PoW | <50% hash | Probabilistic | 3-7 TPS (Bitcoin) | Very high (permissionless) | ASIC (Bitcoin) / CPU (Monero) | Energy use; 51% attack |
| Ouroboros PoS | <50% stake | Probabilistic + epoch checkpoints | ~250 TPS | High | Standard server | Long-range attack (mitigated) |
| Gasper (Eth) | <1/3 Byz validators | Deterministic (~12.8 min) | ~15 TPS on L1 (high with L2) | High (100K+ validators) | Standard server | Complexity; weak subjectivity |
| PBFT | <1/3 Byz | Deterministic | ~100s TPS | Low (small n) | Standard server | O(n²) comm.; does not scale |
| Tendermint | <1/3 Byz | Deterministic (seconds) | ~1,000s TPS | Medium (100s validators) | Standard server | Halts during partition |
| HotStuff | <1/3 Byz | Deterministic (seconds) | ~1,000s TPS | Medium | Standard server | Leader bottleneck |
| DPoS | Majority stake | Fast (~1 sec) | ~3,000 TPS | Low (21-27 producers) | Standard server | Centralisation |
| PoA | Majority producers | Fast | Very high | Very low (permissioned) | Standard server | Trust in known parties |
| Avalanche (Snow) | ~<50% Byz (prob.) | Probabilistic (sub-second) | 4,500+ TPS (claimed) | High (permissionless) | Standard server | Probabilistic only; cold-start attack |
| GHOSTDAG | <50% hash | Probabilistic | >100 TPS (Kaspa) | High | ASIC / GPU | Less studied long-term |
| Narwhal+Bullshark | <1/3 Byz | Deterministic | 100,000+ TPS (benchmark) | Medium | Standard server | Mostly research/early production |
| PoH + Tower BFT | <1/3 Byz (BFT phase) | Near-deterministic | 65,000 TPS (theoretical) | Medium | High-bandwidth servers | Network outages; centralized validators |
| PoSpace+Time | Majority disk | Probabilistic | ~1-20 TPS | High (in principle) | Hard drives | Farm concentration |
| PoET (SGX) | Intel SGX integrity | Deterministic | High | Low (permissioned) | Intel SGX hardware | Hardware backdoor risk |

*Notes: TPS figures are approximate and depend heavily on transaction size, block time, and network conditions. All PoS/BFT throughput figures refer to consensus overhead; actual application throughput may differ. Treat all throughput figures as estimates.*

### 5.12 Block Ordering, Leader Selection, and MEV

**Leader / proposer selection** determines who gets to build the next block. Unfair selection is an attack vector. Methods include:
- **PoW:** Any miner who solves the puzzle first. Stateless — no one knows who the next proposer will be.
- **VRF (Ouroboros, Algorand):** Cryptographic lottery — private selection, public verification.
- **Round-robin weighted by stake** (Tendermint, HotStuff): Deterministic rotation; predictable but auditable.
- **DPoS:** Fixed producer rotation.

**Transaction ordering inside a block** is chosen by the proposer. This creates **MEV — Maximal Extractable Value** (formerly "Miner Extractable Value"): profit a rational block producer can extract by reordering, inserting, or censoring transactions.

Examples of MEV:
- **Arbitrage:** Insert a trade that profits from a price discrepancy created by a user's transaction.
- **Sandwich attack:** Insert a buy before and a sell after a large user swap, profiting from price impact.
- **Liquidation priority:** Race to liquidate undercollateralised DeFi positions first.

MEV is a systemic issue in EVM chains. It redistributes value from ordinary users to block producers and bots. See Part 11 for scaling and MEV mitigation strategies.

---

### Part 5 Review Questions

**Q1. What is the difference between probabilistic and deterministic finality? Give one example of each.**
A: Probabilistic — the probability of reversal decreases over time but is never zero (Bitcoin PoW). Deterministic — once a quorum commits, the block cannot be reverted (Tendermint, HotStuff, Casper FFG checkpoints).

**Q2. What is the nothing-at-stake problem in PoS, and how do slashing and locking address it?**
A: Validators can costlessly vote on multiple forks (unlike PoW miners who commit hardware to one chain). Slashing penalises signing two conflicting blocks. Locking prevents a validator from changing its vote in the same round without evidence of a 2/3 commit for the competing fork.

**Q3. Why does HotStuff achieve linear communication while PBFT requires quadratic?**
A: HotStuff routes all votes through the leader, who aggregates them into a quorum certificate (a single compact object). Every node sends one message to the leader. PBFT requires all-to-all voting — every node sends a message to every other node.

**Q4. What is weak subjectivity in Ethereum's PoS, and why is it controversial?**
A: A newly syncing node cannot determine which of two chains is canonical using only on-chain data — it must trust a recent checkpoint from a known source. Critics argue this introduces a trust assumption absent from PoW (where the heaviest chain is objective). Ethereum developers argue checkpoints from multiple independent sources mitigate this.

**Q5. What is GHOSTDAG's key innovation over Nakamoto consensus?**
A: It allows multiple blocks to be produced simultaneously and includes all of them in the canonical DAG rather than orphaning all but one. This allows very high block rates without sacrificing security.

### Common Misconceptions

- **"PoS is always faster than PoW."** Speed depends on the specific protocol, not the consensus category. Some PoS chains are slow; some PoW chains (Kaspa) achieve 1 block/second.
- **"BFT protocols are more decentralised than PoW."** Classical BFT requires a known, bounded validator set. PoW allows any anonymous participant to join.
- **"Ethereum's finality means nothing can be reversed."** Ethereum's finality gadget (Casper FFG) requires slashing 1/3 of all staked ETH to reverse a finalised block. This is economically deterring, not cryptographically impossible.

### After Part 5 You Should Be Able To

Explain how at least five different consensus algorithms propose, validate, and finalise blocks. Explain the safety/liveness trade-off in each. Place each algorithm in the BFT/probabilistic/hybrid spectrum. Explain what MEV is and how it arises from ordering power.

### Sources — Part 5

| Source | Date | Type |
|---|---|---|
| Nakamoto, Bitcoin whitepaper | 2008 | Primary |
| Kiayias et al., "Ouroboros: A Provably Secure PoS Blockchain Protocol" — CRYPTO 2017 | 2017 | Primary |
| Buterin & Griffith, "Casper the Friendly Finality Gadget" — arxiv:1710.09437 | 2017 | Primary |
| Castro & Liskov, "Practical Byzantine Fault Tolerance" — OSDI 1999 | 1999 | Primary |
| Yin et al., "HotStuff: BFT Consensus with Linearity and Responsiveness" — PODC 2019 | 2019 | Primary |
| Spiegelman et al., "Narwhal and Tusk: A DAG-based Mempool and Efficient BFT Consensus" — EuroSys 2022 | 2022 | Primary |
| Sompolinsky & Zohar, "PHANTOM GHOSTDAG: A Scalable Generalisation of Nakamoto Consensus" — 2018 | 2018 | Primary |
| Team Rocket, "Snowflake to Avalanche" — ipfs.io/ipfs/QmUy4jh5mGNZvLkjies1RWM4YuvJh5o2FYopNPVYwrRVGV | 2018 | Primary |
| Mazières, "The Stellar Consensus Protocol" — stellar.org/papers/stellar-consensus-protocol | 2015 | Primary |
| Micali et al., "Algorand" — arxiv:1607.01341 | 2017 | Primary |
| Solana Documentation: Proof of History — docs.solana.com | Ongoing | Documentation |
| Chia whitepaper — chia.net/greenpaper | 2019 | Primary |


---

## Part 6: Proof of Work Algorithms in Depth

### 6.1 What Makes an Algorithm Suitable for PoW

A Proof of Work algorithm must satisfy three properties:

1. **Puzzle hardness:** Finding a valid solution (a nonce that produces a hash below the target) must be computationally expensive and not shortsable. The expected number of attempts before success equals 1 / (target / 2^n) — the "difficulty."

2. **Verification asymmetry:** Verifying that a given nonce is a valid solution must be very cheap (a single hash computation), even though finding it was expensive. This asymmetry is what makes the proof meaningful.

3. **Non-amortisability (ideally):** Solving one puzzle should not make solving the next one significantly cheaper. SHA-256d achieves this — each hash is independent. Memory-hard algorithms try to enforce this by making each hash require substantial memory bandwidth.

**What "work" means:** Work is not computation for its own sake. It is a mechanism that makes producing a valid block expensive, so that an attacker who wants to rewrite history must spend as much energy as the entire honest network spent producing the history to be overwritten. `[Established]`

### 6.2 Mining in Detail

**The mining loop (simplified Bitcoin):**

```
Given:
  block_header = {version, prev_hash, merkle_root, timestamp, bits, nonce}
  target T (derived from "bits" field)

Repeat:
  hash = SHA256d(block_header)
  if hash < T:
    broadcast block  ← SUCCESS
  else:
    increment nonce (0 to 2^32 - 1)
    if nonce exhausted: change extra_nonce in coinbase tx, recompute merkle root
```

**Toy example with a 4-bit hash and target 0010:**

```
Attempt 1: SHA_toy("block data" + nonce=0) = 1101  → 1101 > 0010, reject
Attempt 2: SHA_toy("block data" + nonce=1) = 0111  → 0111 > 0010, reject
Attempt 3: SHA_toy("block data" + nonce=2) = 0001  → 0001 < 0010, ACCEPT!
```

In reality, Bitcoin's target has about 70 leading zero bits (varying with difficulty), so the expected number of attempts is around 10^21 per block. `[Established, approximate — changes daily with difficulty; checked November 2024]`

**Block rewards and fees:** The block reward = subsidy + fees. Bitcoin's subsidy is currently 3.125 BTC (after April 2024 halving). The subsidy halves every 210,000 blocks (~4 years). By ~2140, all 21 million BTC will have been issued and miner income will come entirely from fees. Whether fees alone will sustain sufficient security is an open question. `[Disputed; see "fee market long-term" debate in Bitcoin research]`

**Orphan blocks / stale blocks:** Two miners may solve valid blocks at nearly the same time. Both are initially accepted by parts of the network. When the next block arrives, it extends one of the two, and the other becomes an **orphan** (Bitcoin) or **uncle** (Ethereum pre-Merge). Orphan miners lose their reward. High orphan rates incentivise centralisation (large pools propagate blocks faster).

**Reorganisations (reorgs):** When the network discovers a chain with more cumulative work than the current tip, all nodes switch to it. Reorgs of 1-2 blocks occur occasionally due to propagation delays. Deeper reorgs (> 2 blocks) on healthy networks are extremely rare and typically indicate an attack.

### 6.3 Attacks on Proof of Work Networks

#### 51% Attack (Majority Attack)

If an entity controls more than 50% of the network's hash power, it can:
- Build a private chain faster than the honest network.
- Release the private chain to overtake the public chain (reverting recent transactions — double-spend).
- Censor transactions by refusing to include them.

What a 51% attacker **cannot** do: create coins from nothing, change the emission schedule, steal coins from other wallets (would require private keys), or change consensus rules. `[Established]`

**Historical examples:**
- **Bitcoin Gold (BTG):** Suffered multiple 51% attacks in 2018 and 2020. In 2020, over $70,000 USD was double-spent in one attack (depths of 29+ blocks). `[Established, sourced: bitcoin.com, 2020]`
- **Ethereum Classic (ETC):** Suffered three 51% attacks in August 2020. One attack reorganised 7,935 blocks (the deepest reorg ever on a major public chain). Exchanges increased required confirmations to 10,000+ blocks. `[Established, sourced: ETC blog, August 2020]`

The cost of a 51% attack on Bitcoin would require renting or owning more hash power than the entire existing network — currently tens of billions of dollars of ASIC hardware plus enormous electricity. This makes Bitcoin's PoW security economically robust. `[Established; exact cost estimate is Uncertain and fluctuates with hardware prices and hashrate]`

#### Selfish Mining

**Selfish mining** (Eyal & Sirer, 2013) is a strategy where a miner withholds newly found blocks instead of broadcasting immediately. By releasing them strategically, the selfish miner can cause honest miners to waste work on a chain that will be orphaned, gaining a larger share of rewards than proportional to hash power.

A pool with ≥25% hash power can profit from selfish mining (threshold of profitability, assuming γ=0 — no network propagation advantage). With propagation advantage (γ > 0), the threshold is lower. `[Disputed: threshold depends on network topology and γ; some argue real-world thresholds are higher than the original paper suggests]`

No confirmed selfish mining attack has been documented on Bitcoin mainnet. `[Established — none confirmed]`

#### Eclipse Attacks

An eclipse attack isolates a node from the honest network, connecting it only to attacker-controlled nodes. The victim sees a fake blockchain and may accept double-spends or be tricked into accepting invalid blocks.

Bitcoin Core has mitigation (limiting inbound connections from the same /16 subnet, diverse peer selection), but eclipse attacks remain theoretically feasible. `[Established risk, mitigated but not eliminated]`

### 6.4 PoW Algorithm Survey

#### SHA-256d (Bitcoin, Bitcoin Cash)

- **Algorithm:** Double SHA-256: `hash = SHA256(SHA256(data))`
- **Properties:** Purely compute-bound; no memory hardness. Fully deterministic and parallelisable.
- **Hardware:** Dominated by ASICs (Application-Specific Integrated Circuits). CPUs and GPUs are approximately 1000× less efficient than top ASICs. The Bitmain Antminer S21 XP achieves ~270 TH/s at ~20 J/TH. `[Established, checked November 2024]`
- **Why designed this way:** Bitcoin's original PoW was not designed to be ASIC-resistant. Satoshi chose SHA-256 for its simplicity and security properties. ASIC development was initially not anticipated by the community.
- **Used by:** Bitcoin (BTC), Bitcoin Cash (BCH), Bitcoin SV (BSV).

#### Scrypt (Litecoin, Dogecoin)

- **Algorithm:** Sequential memory-hard key derivation function (Colin Percival, 2009), originally designed for password hashing. Requires a large array of pseudorandom data to be held in memory during computation.
- **Properties:** Memory-hard at the time of design; intended to make GPU and ASIC mining much harder than CPU mining.
- **Hardware:** Initially GPU-favourable. By 2013-2014, Scrypt ASICs were developed anyway. `[Established — ASIC resistance failed]`
- **Parameters (Litecoin):** N=1024, r=1, p=1 (small by modern standards). Today's Scrypt ASICs (e.g., Antminer L7) deliver ~9.5 GH/s. CPUs are irrelevant.
- **Used by:** Litecoin (LTC), Dogecoin (DOGE).
- **Lesson:** Memory requirements that seemed large in 2011 are trivial for dedicated ASICs with fast on-chip SRAM. ASIC resistance requires either enormous memory (beyond cheap on-chip SRAM) or regularly changing algorithms.

#### Ethash / Etchash (Ethereum Classic)

- **Algorithm:** Based on Dagger-Hashimoto, a combination of Dagger (directed acyclic graph construction) and Hashimoto (memory-hard hash using a large dataset).
- **Properties:** A large DAG (Directed Acyclic Graph) is generated from the blockchain state. Mining requires random reads across this dataset, which is too large to fit in GPU VRAM efficiently — but just right to fit in a GPU's VRAM.
- **DAG parameters:**
  - Initial size: ~1 GB (at genesis).
  - Growth: 8 MB per epoch (30,000 blocks ≈ 5 days for Ethash; 60,000 blocks for Etchash).
  - By Ethereum's last PoW block (September 2022, block 15,537,393), the DAG was ~5 GB.
- **Etchash** (Ethereum Classic, ECIP-1099, December 2020): epoch length doubled to 60,000 blocks to slow DAG growth and extend GPU viability. Breaks compatibility with Ethash ASICs.
- **Hardware:** GPUs (Nvidia and AMD were dominant). Ethash ASICs (Bitmain E9) emerged but arrived after GPUs were already established. GPU mining produced a large secondary market in consumer graphics cards.
- **Used by (Ethash):** Ethereum (pre-September 2022); **(Etchash):** Ethereum Classic (ETC), still active.

#### X11 (Dash)

- **Algorithm:** A chain of 11 sequential hash functions: Blake → BMW → Groestl → JH → Keccak → Skein → Luffa → CubeHash → SHAvite → SIMD → Echo.
- **Properties:** Each function provides a different form of computation. The idea was that an ASIC would need to implement 11 different circuits, making it prohibitively expensive.
- **Hardware:** X11 ASICs (Bitmain Antminer D3) arrived in 2017. GPU mining was competitive briefly. `[Established — ASIC resistance failed again]`
- **Energy efficiency:** X11 is notably energy-efficient on ASICs. Dash's lower difficulty and ASIC efficiency make its mining cheaper.
- **Used by:** Dash (DASH).

#### Equihash (Zcash, Komodo, Others)

- **Algorithm:** Based on the **Generalized Birthday Problem** (Biryukov & Khovratovich, 2016). Given N bits and k rounds, find 2^k inputs whose XOR is zero (all pairwise XOR of a certain depth).
- **Parameters (Zcash mainnet):** n=200, k=9. Solution requires finding 512 inputs satisfying the XOR constraint. Verification is fast.
- **Memory requirement:** ~700 MB RAM for efficient solution finding.
- **Hardware:** Initially GPU-friendly. Equihash ASICs (Bitmain Z9) arrived in 2018, devastating GPU miners on Zcash. Zcash accepted ASICs (`[Disputed]`; zcash community debated ASIC resistance policy, chose not to fork). Some Equihash variants (Equihash 192,7; BeamPoW) resist ASICs better.
- **Used by:** Zcash (ZEC), Komodo (KMD).
- **Properties:** Verification-efficient (fast to check), memory-hard (expensive to parallelise on ASICs).

#### CryptoNight (Old Monero, Bytecoin, Aeon)

- **Algorithm:** Designed for CryptoNote protocols. Uses multiple sub-algorithms (AES-NI, Scratchpad) with a 2 MB per-thread scratchpad — sized to fit in CPU L3 cache but not in ASIC on-chip SRAM.
- **Why it failed at ASIC resistance:** By 2017-2018, Bitmain and others developed CryptoNight ASICs. Monero's community responded with algorithm changes (CryptoNight v2, CryptoNightR) which broke each ASIC generation, but the arms race exhausted the community.
- **This led directly to RandomX** (see below).
- **Used by:** Old Monero (until 2019), Bytecoin, Aeon (now largely inactive/abandoned).

#### RandomX (Monero)

RandomX (deployed Monero hard fork, November 30, 2019, block 1,978,433) is Monero's current PoW algorithm. It is the most sophisticated attempt at genuine CPU-preference mining. `[Established]`

**Core idea:** Instead of a fixed hash function, RandomX generates and executes a **random program** (derived from the input) on a virtual machine designed to resemble a modern CPU's execution model. Making an ASIC efficient at RandomX would require building a programmable CPU — which is essentially what a CPU already is.

**Virtual machine design:**
- 8 integer registers (R0-R7)
- 8 floating-point registers (F0-F3, E0-E3)
- 2 MB L3-equivalent scratchpad (accessed randomly during execution)
- Programs consist of 256 pseudo-random instructions (integer, floating-point, branch, memory operations)
- IEEE-754 double-precision floating-point arithmetic (tricky to replicate exactly on custom hardware)
- Frequent AES-NI encrypt/decrypt operations (hardware-accelerated on modern CPUs)

**Memory requirements:**
- **Fast mode (mining):** 2,080 MiB (~2 GB) dataset + 256 MiB cache. The dataset must be held in RAM.
- **Light mode (verification):** 256 MiB cache only; slower (useful for nodes not mining).
- **Scratchpad per thread:** 2 MiB L3 + 256 KiB L2 + 16 KiB L1 (simulating CPU cache hierarchy)

**How hashing works:**
1. Initialise the scratchpad from the cache using AES.
2. Execute 8 random programs (called "superscalar programs" seeded by the cache) against the scratchpad.
3. Apply a final hash (Blake2b) to produce the result.

**ASIC resistance analysis:** `[Disputed]`
- CPUs are optimal because they natively support the instruction set, branch prediction, and memory hierarchy RandomX simulates.
- GPUs are roughly 2-3× less efficient than CPUs per unit cost on RandomX (unusual for PoW). `[Established; observed in practice]`
- Dedicated ASICs would need to implement the full general-purpose CPU execution model and 2 GB fast DRAM — approaching the cost and complexity of a CPU. No production Monero ASIC has been confirmed. `[Established as of November 2024]`
- Large-scale server farms with modern Xeon or EPYC processors have an advantage because they can cache the 2 GB dataset in many CPU NUMA sockets. This provides some centralisation pressure toward datacenters. `[Disputed: whether this constitutes meaningful centralisation]`

**Used by:** Monero (XMR) exclusively.

#### kHeavyHash (Kaspa)

- **Algorithm:** A lightweight hash designed for Kaspa's very high block rate. Applies SHA-256 with a matrix-multiplication interlude: `hash = SHA256(M × SHA256(block_header))` where M is a pseudorandom matrix derived from the input.
- **Properties:** Not memory-hard. Compute-bound. Very fast to verify.
- **Hardware:** Initially GPU-friendly. Kaspa ASICs (IceRiver, Bitmain KA3) arrived in 2023, with top-of-line ASICs reaching 10+ TH/s.
- **Why it matters:** Kaspa's protocol (GHOSTDAG) requires very fast block production; a complex memory-hard algorithm would be too slow. kHeavyHash is designed for speed, not ASIC resistance.
- **Used by:** Kaspa (KAS).

#### Autolykos v2 (Ergo)

- **Algorithm:** Memory-hard PoW designed for Ergo. Version 2 (deployed 2021) removed the pool-friendliness restriction of v1 (which required knowledge of the miner's secret key, preventing pool mining).
- **Properties:** Miners build large lookup tables (~2.25 GB) and perform multiple random reads per hash. Verification requires only a few accesses.
- **Hardware:** GPU-friendly, with moderate CPU feasibility. Autolykos v2 ASICs may be emerging. `[Uncertain as of November 2024]`
- **Used by:** Ergo (ERG).

#### Equihash Variants (BeamPoW, ZHash, etc.)

After Zcash accepted ASICs, several Equihash-based chains adopted modified parameters to break ASIC compatibility:
- **BeamHashIII (Beam):** A variant adding a DAG layer to Equihash; GPU-friendly.
- **ZHash / Equihash 192,7 (BitcoinZ, etc.):** Different (n, k) parameters.
- **Cuckatoo/Cuckoo Cycle (Grin):** Not Equihash but similarly based on memory-hard graph problems — finding cycles in a large bipartite graph. ASIC development has occurred.

#### Progpow (Programmatic PoW — Proposed, Not Widely Deployed)

- **Algorithm:** Designed to close the efficiency gap between GPUs and potential Ethash ASICs by using the full range of GPU operations (not just memory).
- **History:** Heavily discussed for Ethereum (2018-2021) but never adopted on mainnet. Controversial for opaque development process. `[Established: rejected by Ethereum community; some smaller chains adopted it]`
- **Used by:** Zano, some smaller coins. Ethereum Classic briefly considered but rejected it.

#### CuckooHash / Cuckoo Cycle (Grin, Cortex)

- **Algorithm:** Finding a cycle of a specified length in a large bipartite graph (Tromp, 2014). The graph is too large for ASIC on-chip memory; miners must access a ~1+ GB structure randomly.
- **Hardware:** GPUs (and later ASICs) have both been used. Cuckatoo (larger N) is more memory-intensive and more resistant to ASICs.
- **Used by:** Grin (GRIN). Grin uses dual algorithms (Cuckaroo for GPU, Cuckatoo for ASIC).

#### KAWPoW (Ravencoin)

- **Algorithm:** Derived from ProgPoW; uses GPU-specific operations to maximise ProgPoW's design goals.
- **Hardware:** GPU-only friendly in practice.
- **Used by:** Ravencoin (RVN).

#### Survey of Other Notable PoW Algorithms

| Algorithm | Chain | Key Feature | ASIC Status |
|---|---|---|---|
| Yescrypt/YescryptR16 | Ycash, some privacy coins | Memory-hard; CPU-optimised | No confirmed ASICs |
| SHA-3 (Keccak) | Various small chains | NIST standard | Theoretically ASIC-amenable |
| BLAKE3 | Alephium, others | Very fast; BLAKE2 successor | Early-stage ASICs possible |
| Lyra2REv3 | Vertcoin | GPU-friendly, changed multiple times | Resisted ASICs via forks |
| MTP (Merkle Tree PoW) | Firo (Zcoin) | Memory-hard; zero-knowledge proofs of work | Less practical for mining |
| Cryptonight variants | Monero (historical), many alts | Various scratchpad sizes | ASICs repeatedly broken by forks |
| SHA-256d | Bitcoin Gold (attempted PoW change) | Initially equihash; ASIC attack | ASICs followed quickly |

**Category completeness note:** The PoW algorithm landscape is large and many small chains use lightly modified versions of these algorithms. A definitive exhaustive list is beyond the scope of this document; the above covers all algorithms in significant or instructive use. The best reference for completeness is the minerstat.com and whattomine.com algorithm lists.

### 6.5 ASIC Resistance: Goals, Methods, and Outcomes

**Goal:** Keep mining accessible to a wide range of hardware (CPUs, GPUs, eventually any device) to prevent the concentration of hash power in a few large mining operations and reduce barriers to entry.

**Methods:**

| Method | Example | Outcome |
|---|---|---|
| Memory-hardness (large scratchpad) | Scrypt, CryptoNight | ASICs with large on-chip SRAM eventually dominated |
| Sequential memory-hardness (Argon2-like) | RandomX dataset generation | Resists ASICs better; large DRAM required |
| Algorithm complexity / full CPU simulation | RandomX | Most successful; no confirmed ASICs as of Nov 2024 |
| Graph problems | Cuckoo Cycle, Equihash | ASICs followed |
| Regular algorithm changes (forks) | Monero (CryptoNight era) | Exhausting arms race; abandoned for RandomX |
| Multi-algorithm | Myriadcoin | Complicates ASIC design but not impossible |

**Has ASIC resistance worked?** `[Disputed]`

- **Against:** Every algorithm except RandomX (and perhaps Autolykos v2) has seen ASICs developed within 2-5 years. The economic incentive for ASIC manufacturers is enormous. Memory-hardness has not reliably prevented ASICs.
- **For:** RandomX represents a genuine qualitative breakthrough — ASIC development would require building a full general-purpose CPU, eliminating the typical cost and speed advantage. No confirmed RandomX ASIC exists. `[Established as of November 2024]`
- **Middle ground:** Even CPU-friendly algorithms can be run efficiently on large server farms. The gap between a commodity desktop CPU and a datacenter's power-optimised server is real. Perfect decentralisation may be unachievable by algorithm design alone.

### 6.6 Difficulty Adjustment Algorithms

Proof of Work security requires that the expected time to find a block stays close to the target interval. If hash power increases, blocks come too fast; if it decreases (miners turn off), blocks slow. The **difficulty adjustment algorithm** compensates.

#### Bitcoin's Retarget Rule

- **Interval:** Every 2,016 blocks.
- **Calculation:** `new_target = old_target × (actual_time_for_2016_blocks / 1,209,600_seconds)`. The result is clamped to [1/4, 4] × old_target to prevent extreme swings.
- **Target block time:** 10 minutes. Expected adjustment period: ~2 weeks.
- **Weakness:** Because it adjusts every 2 weeks, a sudden large drop in hash power (miners leaving, halvings, geopolitical events) causes blocks to slow significantly for up to two weeks before the next adjustment. During these slow periods, the mempool can build up. `[Established]`

#### Monero's Per-Block Difficulty Adjustment (LWMA)

Monero uses the **Linearly Weighted Moving Average (LWMA)** algorithm, adjusting difficulty every block based on the last 720 blocks (approximately 24 hours). Recent blocks are weighted more heavily than older ones. Extreme outliers in timestamps (top/bottom 10% by default) are discarded to prevent manipulation.

**Advantage:** Responds within hours rather than weeks to changes in hash power. Prevents the "hash power crash = slow blocks" problem.

**Disadvantage:** A sophisticated attacker can manipulate block timestamps within the allowed window to game the difficulty, though LWMA's outlier rejection significantly limits this.

This design has been adopted by several other PoW chains. `[Established]`

#### Other Difficulty Algorithms

| Algorithm | Used by | Key Feature |
|---|---|---|
| DigiShield | Digibyte, Zcash | Faster retarget (every block), simpler formula |
| Dark Gravity Wave | Dash | Multi-block moving average, every block |
| ASERT (aserti3-2d) | Bitcoin Cash | Exponential convergence per block; mathematically proven stable |
| Kimoto Gravity Well | Old altcoins | Per-block, inverse polynomial response curve |
| EMA-based | Various | Exponential moving average |

**Why design matters for security:** A slow difficulty algorithm allows a large miner to mine many blocks quickly (before difficulty rises), collect more than their fair share of rewards, and then leave. This is the "pump-and-dump hash power" attack common on small chains with Bitcoin-style 2-week adjustments. Per-block algorithms largely prevent this.

### 6.7 Hashrate, Mining Pools, Centralisation, and Economics

**Hash rate** is the number of hash attempts per second made by the network collectively. Bitcoin's total hashrate was approximately 700+ EH/s (exahashes per second) as of late 2024. `[Uncertain for precise current figure; fluctuates daily. Checked approximately November 2024.]`

**Mining pools:** Because the probability of a solo miner finding a block is tiny (even a large miner with 1% of hashrate finds a block only 1% of the time), miners form **pools** that combine hash power and share rewards proportionally. The pool operator takes a small fee.

**Centralisation concern:** The top 3-5 Bitcoin mining pools routinely control 50-70%+ of block production at any given time. `[Established, based on mining pool distribution data from mempool.space and BTC.com; exact figures fluctuate]`

This is a centralisation concern: `[Disputed whether it constitutes a meaningful security risk]`
- **Concern:** If the top 3 pools collude, they could execute a 51% attack or censor transactions.
- **Counter-argument:** Pool operators do not control individual miners' hardware. Miners can point their hardware at other pools within minutes if a pool misbehaves. Historical evidence shows miners do switch pools when operators are perceived as acting badly.

**Stratum V2** (a newer mining pool communication protocol) further reduces pool operator power by allowing individual miners to construct their own block templates rather than receiving them from the pool.

**Mining economics:** Miners are rational profit-maximisers. Their break-even point depends on hardware cost (capex), electricity cost (opex), and the BTC/fiat exchange rate. At low prices or high energy costs, miners turn off hardware. At high prices, more hardware comes online. This creates a natural equilibrium — if mining is unprofitable, hash rate falls, difficulty drops, and profitability returns.

---

### Part 6 Review Questions

**Q1. What three properties must a PoW algorithm have, and why is each necessary?**
A: (1) Puzzle hardness — finding a solution must be expensive so that creating fake history costs real resources. (2) Verification asymmetry — checking a solution must be cheap so all nodes can validate without mining themselves. (3) Non-amortisability (ideally) — solving one puzzle should not help with the next, preventing economies of scale that would centralise mining.

**Q2. How does RandomX differ from all previous ASIC-resistance strategies?**
A: Instead of using memory-hardness with a fixed hash, it generates and executes a random program on a virtual machine that simulates a full general-purpose CPU. Building an ASIC that efficiently runs arbitrary CPU programs is equivalent to building a CPU — negating the ASIC advantage.

**Q3. Why did Scrypt and CryptoNight fail at ASIC resistance?**
A: Both used memory sizes that seemed large at design time but were manageable with fast on-chip SRAM in custom ASICs. ASIC manufacturers built chips with enough on-chip SRAM to satisfy the memory requirement without needing slow external DRAM.

**Q4. What is selfish mining, and what hash power threshold makes it profitable?**
A: Withholding newly found blocks to cause honest miners to orphan their work, gaining a disproportionate reward share. Theoretically profitable at ≥25% hash power (with zero network advantage), lower with propagation advantage. No confirmed mainnet attack on Bitcoin.

**Q5. Why is Bitcoin's 2-week difficulty adjustment potentially risky for small PoW chains?**
A: Hash power can flood in, mine many blocks cheaply before difficulty rises, then leave. The chain then slows for up to two weeks. Per-block algorithms (like Monero's LWMA) prevent this by adjusting within hours.

### Common Misconceptions

- **"ASIC resistance means your algorithm is ASIC-proof."** It means ASICs are harder to build profitably. The only design proven so far is RandomX (CPU simulation). All previous memory-hard schemes eventually fell to ASICs.
- **"Mining pools control the coins of miners in them."** Pools direct hash power and distribute rewards but do not hold miners' coins or private keys.
- **"Hash rate directly measures security."** Security depends on the *cost* of hash rate, not the number. A cheap-to-mine algorithm with high hash rate may be less secure than Bitcoin's expensive-to-mine rate.

### After Part 6 You Should Be Able To

Explain what a PoW algorithm must do and why. Describe how Bitcoin mining works, including the nonce, target, and difficulty. Explain the ASIC resistance strategies used by at least five different algorithms and whether each succeeded. Explain what a 51% attack is and what it can and cannot do. Explain Bitcoin's and Monero's difficulty adjustment algorithms and their trade-offs.

### Sources — Part 6

| Source | Date | Type |
|---|---|---|
| Nakamoto, Bitcoin whitepaper | 2008 | Primary |
| RandomX specification — github.com/tevador/RandomX/blob/master/doc/specs.md | 2019 | Primary |
| Percival, "Stronger Key Derivation via Sequential Memory-Hard Functions" (Scrypt) — tarsnap.com/scrypt | 2009 | Primary |
| Biryukov & Khovratovich, "Equihash: Asymmetric Proof-of-Work Based on the Generalized Birthday Problem" — NDSS 2017 | 2016 | Primary |
| Eyal & Sirer, "Majority is Not Enough: Bitcoin Mining is Vulnerable" — FC 2014 | 2013 | Primary |
| Ethereum Classic ECIP-1099 (Etchash) — ecips.ethereumclassic.org/ECIPs/ecip-1099 | 2020 | Primary |
| Tromp, "Cuckoo Cycle: a Memory-Hard Proof-of-Work System" — 2014 | 2014 | Primary |
| Zawy, "LWMA difficulty algorithm" — github.com/zawy12/difficulty-algorithms | 2017 | Primary |
| docs.getmonero.org/proof-of-work/random-x/ | Ongoing | Documentation |
| XMRig RandomX optimization guide — xmrig.com/docs/miner/randomx-optimization-guide | Ongoing | Documentation |


---

## Part 7: Consensus Rules in Depth

### 7.1 Consensus Rules vs Policy Rules

This distinction is fundamental and frequently confused.

**Consensus rules** are the rules that every full node enforces, independently, to determine whether a block or transaction is valid. All honest nodes apply the same consensus rules. A block violating any consensus rule is rejected and never extended.

**Policy rules** (also called "mempool policy") are configurable per-node defaults about what transactions to relay, hold in the mempool, and include in blocks. They are not part of consensus — different nodes may apply different policies and they will still agree on the chain.

**Example:**
- **Consensus rule:** A transaction spending a UTXO that does not exist in the UTXO set is **invalid** — no honest node will include it in a block it accepts. Breaking this would be detected by all nodes and the block rejected.
- **Policy rule (Bitcoin):** Bitcoin Core's default minimum relay fee is 1 sat/vbyte. A node may choose 2 sat/vbyte. Transactions below the threshold are not relayed but may still be included in a block by a miner with a different policy — and that block is valid.

**Why this matters:** Changes to consensus rules require coordination (soft or hard forks). Changes to policy rules can be done unilaterally by node operators without consensus risk.

### 7.2 Block Validity Rules

A node receiving a new block must check all of these before accepting it. The list below is for Bitcoin; other chains have analogous rules.

**Header checks (cheap, done first):**
- Block hash < difficulty target (Proof of Work valid)
- `version` field is a known value
- `timestamp` within acceptable range (see below)
- `prev_block_hash` references a known block

**Block body checks:**
- Merkle root in header matches the computed Merkle root of block transactions
- Block weight ≤ maximum (Bitcoin: 4,000,000 weight units; SegWit-era limit)
- First transaction is a valid coinbase transaction
- Coinbase value ≤ block subsidy + sum of all transaction fees
- All other transactions are valid (see below)
- No duplicate TXIDs within the block

**Timestamp rules:**
- **Median Time Past (MTP):** A block's timestamp must be greater than the median timestamp of the previous 11 blocks. This prevents miners from lying about time to manipulate difficulty or time-locked scripts.
- **Future time limit:** A block's timestamp may not be more than 2 hours ahead of the node's local clock. This prevents mining arbitrarily far into the future.

**Coinbase maturity:** A coinbase transaction's outputs cannot be spent until at least 100 blocks have been mined after them. This ensures that if the block is later orphaned (reorganised away), the miner cannot have spent non-existent coins.

### 7.3 Transaction Validity Rules

For each transaction in a block, nodes check:

- All inputs reference existing, unspent outputs (UTXO membership check)
- Input scripts (scriptSig / witness) validly satisfy the corresponding output scripts (scriptPubKey)
- The sum of input values ≥ the sum of output values (no inflation)
- No input is double-spent within the same transaction or against previous transactions in the same block
- All scripts execute successfully without stack errors
- Transaction weight ≤ maximum
- Outputs have positive values
- Scripts do not contain disabled opcodes

**Script validation** (Bitcoin's locking/unlocking mechanism) verifies signatures using ECDSA or Schnorr, depending on the output type (P2PKH, P2WPKH, P2TR, etc.). Monero's transaction validation is more complex — see Part 8.

**Emission schedule check:** The coinbase's output value must not exceed `block_subsidy(height) + sum(fees)`. This is how the 21 million BTC supply cap is enforced: the subsidy function halves every 210,000 blocks and rounds down to satoshis, so at some point the subsidy is 1 satoshi, then 0. `[Established]`

### 7.4 Chain Selection: Heaviest Chain, Not Longest

Bitcoin's whitepaper says nodes follow "the longest chain." This is imprecise. The correct rule, as implemented in Bitcoin Core, is:

> **Follow the chain with the greatest cumulative Proof of Work (chainwork), not the greatest block count.**

**Why the distinction matters:** After a difficulty adjustment, blocks in one epoch may represent more work than blocks in another. Suppose:
- Chain A: 100 blocks all mined at difficulty 100 → cumulative work = 10,000
- Chain B: 99 blocks mined at difficulty 110 → cumulative work = 10,890

Chain B is shorter (99 blocks vs 100) but has more cumulative work. It is the canonical chain. `[Established]`

Each block header encodes its difficulty target (`bits` field). Bitcoin Core computes chainwork by summing the inverse of each block's target: `work = 2^256 / (target + 1)`. The chain with the highest total chainwork wins.

**Contrast with PoS fork choice:** In Gasper (Ethereum), LMD-GHOST weights fork choice by validator votes — each validator's latest attestation counts once. Finality is provided by Casper FFG's 2/3 quorum checkpoints, not by cumulative work. `[Established]`

**Contrast with BFT finality:** In Tendermint/HotStuff, there are no forks after commit — the commitment of a block is final by protocol. No chain-selection rule is needed beyond "follow the committed chain." `[Established]`

### 7.5 Soft Forks vs Hard Forks

**Soft fork:** Tightens the consensus rules (makes some previously valid blocks invalid). Old nodes still accept the new chain because they see it as valid under their (looser) rules. The network does not split if a majority of hash power enforces the new rules — old nodes will follow the heaviest chain even without understanding the new rules.

**Hard fork:** Loosens or incompatibly changes rules. Old nodes reject blocks valid under new rules. If not all nodes upgrade, the chain splits permanently.

**Historical Bitcoin soft forks:**
| Activation | Description | BIP |
|---|---|---|
| March 2012 | P2SH (pay-to-script-hash) enables complex locking conditions | BIP 16 |
| March 2013 | Block height in coinbase | BIP 34 |
| July 2015 | Strict DER signature encoding | BIP 66 |
| December 2015 | CHECKLOCKTIMEVERIFY — time-locked scripts | BIP 65 |
| July 2016 | CHECKSEQUENCEVERIFY — relative timelocks | BIP 68/112/113 |
| August 2017 | Segregated Witness — separates witness data, fixes malleability | BIP 141 |
| November 2021 | Taproot — Schnorr signatures, Tapscript, MAST | BIP 340-342 |

**Bitcoin Cash fork (hard fork, August 1, 2017):** When the community could not agree on increasing the block size limit, Bitcoin Cash forked with an 8 MB block limit (later increased further). Both chains share pre-fork transaction history. Bitcoin Cash added replay protection to prevent transactions valid on one chain from being replayed on the other. `[Established]`

**Activation mechanisms:**
- **BIP9 (Versionbits):** Miners signal readiness in block version bits. Lock-in requires ≥95% signalling over a 2016-block window within a timeout period. Used for SegWit.
- **Speedy Trial (BIP8 variant, Taproot):** 90% miner signalling required within a fixed 3-month window; fails gracefully if not reached. Taproot activated November 14, 2021. `[Established]`
- **User-Activated Soft Fork (UASF, BIP148):** Economic nodes signal intent to reject blocks that do not signal SegWit. Provided pressure alongside miner signalling in 2017.

### 7.6 Historical Consensus Failures and Bugs

Understanding where consensus has broken is as important as understanding how it works.

**2010 — Value overflow bug (CVE-2010-5139):**
- August 15, 2010.
- A bug in transaction validation allowed outputs summing to >2^63 satoshis via integer overflow. Block 74,638 contained a transaction creating 184,467,440,737 BTC.
- Satoshi Nakamoto patched Bitcoin Core within hours. A 0.3.1 release was rushed out. Miners switched to the patched chain, orphaning the overflow block. `[Established]`
- **Lesson:** Even the foundational emission rule can have implementation bugs.

**2013 — Berkeley DB / LevelDB chain split (CVE-2013-3220, BIP 50):**
- March 11, 2013.
- Block 225,430 contained more than 1,000 transactions — a lock limit in the BerkeleyDB database used by older Bitcoin Core versions (0.7 and earlier) caused them to reject the block. Nodes running 0.8 (which used LevelDB) accepted it. The network split into two chains.
- Developers coordinated with miners to roll back to the 0.7-compatible chain. Bitcoin Core 0.8.1 was released hours later.
- Double-spends were possible during the split and at least one is known to have occurred.
- **Lesson:** Even non-cryptographic implementation details (database lock limits) can cause consensus failures. `[Established]`

**2015 — BIP66 SPV mining fork:**
- July 4, 2015. Miners were mining on top of block headers without validating their contents (SPV mining / empty block mining). A BIP66-invalid block was produced, and several miners extended it without validating. This created a 6-block fork.
- **Lesson:** SPV mining (mining on unvalidated block templates) is dangerous for the network. `[Established]`

**2018 — Inflation vulnerability (CVE-2018-17144):**
- September 17, 2018.
- Bitcoin Core 0.14.x to 0.16.2 contained a bug allowing a transaction to spend the same output twice within a block without being rejected. This could have inflated Bitcoin's supply without any mining.
- Responsible disclosure allowed a patch (Bitcoin Core 0.16.3) to be deployed before exploitation. `[Established]`
- **Lesson:** Consensus code must be treated with extreme care. A silent bug almost broke Bitcoin's supply cap.

**Monero blockchain exploit (November 2017):** A bug in the transaction validation code could have allowed an attacker to create transactions with negative output amounts, effectively inflating the supply. Discovered and patched before exploitation. `[Established, sourced: Monero Research Lab post]`

### 7.7 How Full Nodes Enforce Rules — and Why This Matters

Full nodes enforce consensus rules by independently validating every block from the genesis block forward. They do not trust miners, peers, or even the Bitcoin Core development team. If a proposed rule change is embedded in a software update, users must choose to run that software.

This creates a **separation of power** that is fundamental to Bitcoin's governance model: miners produce blocks, but full nodes validate them and choose which software (consensus rules) to run. An unpopular rule change that miners implement can be rejected by full nodes simply by running older software. `[Established]`

**Why this matters:**
- It means no single entity — not Satoshi, not Bitcoin Core developers, not any government — can unilaterally change Bitcoin's consensus rules. All significant participants must voluntarily upgrade.
- It is the reason Bitcoin's 21 million supply cap is considered reliable: changing it would require convincing millions of full nodes to run software that breaks the cap, which they have strong incentives not to do.
- It is also why governance is slow and contentious — unanimous or near-unanimous consent is effectively required for consensus changes.

---

### Part 7 Review Questions

**Q1. What is the difference between a consensus rule and a policy rule? Give one example of each.**
A: Consensus rule: "A transaction input must reference a valid UTXO" — all nodes enforce this, violation invalidates a block. Policy rule: "Minimum fee 1 sat/vbyte for relay" — per-node default, violation doesn't invalidate the block.

**Q2. What is Median Time Past and why is it needed?**
A: MTP is the median timestamp of the previous 11 blocks. A new block's timestamp must be greater than MTP. This prevents miners from claiming an earlier time to manipulate time-locked scripts or difficulty, since the median is hard to fake without controlling many consecutive blocks.

**Q3. Why is Bitcoin's supply cap enforced by full nodes, not miners?**
A: The cap is a consensus rule. Full nodes independently check that the coinbase output ≤ subsidy + fees. A miner who creates more coins would produce a block that all honest full nodes reject. The miner would receive no reward and waste their work.

**Q4. What bug in 2018 could have silently inflated Bitcoin's supply, and why was it dangerous?**
A: CVE-2018-17144 allowed a transaction to double-spend an input within the same block without rejection. If exploited, it could create extra BTC without mining — inflating the supply. It was dangerous because it was silent (not visible on-chain until exploited), it existed in production software, and it threatened the fundamental monetary policy.

**Q5. How does the heaviest-chain rule differ from the longest-chain rule and in what scenario do they differ?**
A: Heaviest chain maximises cumulative Proof of Work (sum of work per block). Longest chain maximises block count. They differ when difficulty changes between blocks — a shorter chain of high-difficulty blocks can represent more work than a longer chain of low-difficulty blocks.

### Common Misconceptions

- **"Soft forks can't split the chain."** They can, if a large faction of economic nodes refuses to upgrade and the minority continues mining. This is unlikely but not impossible.
- **"The Bitcoin Core developers control Bitcoin."** They write reference software, but users must choose to run it. Developers have no power to force changes on anyone.
- **"Once a block has 6 confirmations it cannot be reversed."** 6 confirmations make reversal extremely improbable, not impossible. For large exchanges, the standard is often higher.

### After Part 7 You Should Be Able To

Distinguish consensus rules from policy rules and give examples. List five Bitcoin transaction validity rules. Explain why the heaviest-chain rule is more precise than the longest-chain rule. Explain soft forks, hard forks, and their activation mechanisms. Describe two historical consensus failures and their lessons.

### Sources — Part 7

| Source | Date | Type |
|---|---|---|
| Nakamoto, Bitcoin whitepaper | 2008 | Primary |
| Bitcoin Wiki, "Value overflow incident" — en.bitcoin.it/wiki/Value_overflow_incident | Ongoing | Documentation |
| BIP 50 (March 2013 Chain Fork Post-Mortem) — github.com/bitcoin/bips/blob/master/bip-0050.mediawiki | 2013 | Primary (BIP) |
| Bitcoin Core security advisories — bitcoincore.org/en/security-advisories | Ongoing | Documentation |
| BIP 9 (Version bits), BIP 141 (SegWit), BIP 340-342 (Taproot) — github.com/bitcoin/bips | Various | Primary (BIPs) |
| Bitcoin Wiki, Common Vulnerabilities and Exposures — en.bitcoin.it/wiki/Common_Vulnerabilities_and_Exposures | Ongoing | Documentation |


---

## Part 8: Monero (XMR) in Depth

### 8.1 Design Goals: Privacy and Fungibility

Monero's primary design goal is **fungibility**: every unit of XMR must be interchangeable with every other unit. Bitcoin is not fungible — each coin has a traceable history, and "tainted" coins (associated with theft, ransomware, etc.) can be blacklisted by exchanges and businesses. If some coins are worth more or less than others based on their history, money ceases to function as a neutral medium of exchange.

Fungibility requires **privacy**: if the history of every coin is public (Bitcoin), coins can be distinguished. If transaction history is private, coins are indistinguishable and therefore equivalent.

This is a qualitatively different goal from Bitcoin's pseudonymity. Bitcoin provides pseudonymity — public addresses without necessarily attached real-world identities. Chain-analysis firms (Chainalysis, CipherTrace, Elliptic) can and regularly do trace Bitcoin transactions with high confidence. `[Established]`

### 8.2 CryptoNote Origins and Monero's Divergence

**CryptoNote** (whitepaper published 2013, attributed to "Nicolas van Saberhagen") introduced:
1. **Ring signatures** for sender privacy — a signer proves membership in a group without revealing which member they are.
2. **One-time (stealth) addresses** for recipient privacy — each payment goes to a unique address derived from the recipient's public key.

**Bytecoin (BCN)** was the first CryptoNote-based coin, launched in 2012-2013. When it became public in 2014, it was discovered that ~80% of the supply had already been mined in private, constituting a massive premine. The community rejected it.

**Monero** was forked from Bytecoin's codebase in April 2014 by a Bitcointalk user "thankful_for_today," then immediately taken over by a development community that rejected the premine and the original developer. Key divergences from CryptoNote/Bytecoin:

| Feature | Original CryptoNote | Monero |
|---|---|---|
| Premine | 80% (Bytecoin) | None — fair launch |
| Amount hiding | No (amounts public) | RingCT (amounts hidden since 2017) |
| Ring size | Variable (sometimes 1 — catastrophic) | Mandatory minimum → currently enforced at 16 |
| PoW algorithm | CryptoNight | CryptoNight → variants → RandomX (2019) |
| Hard forks | Rare | Approximately biannual for upgrades |
| Governance | Unclear | Community-based (Monero Research Lab + Core Team) |

### 8.3 Privacy Components

Monero's privacy rests on three cryptographic layers, all mandatory for every transaction:

#### 8.3.1 Ring Signatures and CLSAG

**Ring signature** (original Rivest, Shamir, Tauman 2001): A signature by one member of a group (the "ring") that proves *someone* in the group signed, without revealing *which* member.

**How it works in Monero:**
When Alice spends an output, she creates a ring of **16 outputs** — 1 real (her output) and 15 decoys drawn from the blockchain. She signs the transaction using the ring, producing a signature that proves she controls *one* of the 16 outputs but reveals nothing about which one.

```
Ring signature schematic (ring size 4 for illustration):

Outputs in ring:    PK_A   PK_B   PK_C   PK_D
                     ↑
                   Real spend (Alice's), unknown to observers
                   
Signature: s = Sign(PK_A, PK_B, PK_C, PK_D; privKey_A)
Verifier checks: Sig is valid for *some* key in {PK_A, PK_B, PK_C, PK_D}
```

**Key image:** To prevent double-spending (Alice spending the same output twice with different rings), each output generates a unique **key image** `I = privKey × Hp(pubKey)` where Hp is a hash-to-point function. The key image is deterministic — the same output always generates the same key image — but cannot be linked back to the public key without the private key. All key images ever used are stored and checked for duplicates. `[Established]`

**Ring size history:** Monero's ring size was variable (and sometimes 1 — completely transparent!) in early years. It was progressively increased and made mandatory: 3 → 5 → 7 → 11 → 16. As of the current protocol (checked November 2024), ring size = 16. `[Established]`

**CLSAG (Concise Linkable Spontaneous Anonymous Group signatures):** Replaced MLSAG (Multi-Layered) in Monero in October 2020 hard fork. CLSAG produces the same privacy guarantees with 20% smaller transaction sizes and 10-15% faster verification. `[Established]`

#### 8.3.2 Stealth Addresses (One-Time Addresses)

**Problem:** If Alice always receives at the same address, anyone watching the blockchain can link all her incoming payments together.

**Solution:** Every transaction generates a fresh one-time address for the recipient.

**How it works (using Diffie-Hellman key exchange):**

Bob publishes two keys: a **spend key** (B = b·G) and a **view key** (V = v·G).

When Alice pays Bob:
1. Alice picks random scalar r.
2. Computes a **shared secret** Q = r · V (only Bob can compute this as r·v·G).
3. Derives one-time address: P = H(Q) · G + B.
4. Only Bob, with his view key v, can compute Q from Alice's published R = r·G (because v·R = r·V = Q).
5. Only Bob, with his spend key b, can compute the private key to P.

No two transactions to Bob ever produce the same P, even from the same payer. `[Established]`

**View keys and spend keys:** This separation has an important privacy property: Bob can give someone his view key V and that person can see all incoming transactions but cannot spend them (they lack the spend key b). This enables selective disclosure for auditing.

**Subaddresses:** An improvement on the base stealth address scheme, subaddresses allow Bob to generate multiple "sub-accounts" from a single seed wallet, each appearing as a different public address, without exposing the master view key.

#### 8.3.3 RingCT — Hiding Amounts

Standard ring signatures hide the sender but not the amount. RingCT (Ring Confidential Transactions, Shen Noether, 2015; deployed January 2017) hides transaction amounts using **Pedersen commitments**.

**Pedersen commitment:** A commitment to a value v: `C = v·H + r·G` where H is a hash-to-point, G is the base point, r is a random blinding factor, v is the amount. The commitment hides v (binding: changing v changes C; hiding: C reveals nothing about v to an observer without r).

**Balance proof:** Given input commitments summing to `C_in = ΣC_inputs` and output commitments summing to `C_out = ΣC_outputs + C_fee`, a valid transaction proves `C_in - C_out - C_fee = 0` without revealing any individual value. This is verified via the homomorphic property of Pedersen commitments. `[Established]`

**Range proofs:** Commitments could hide negative values, allowing artificial inflation. Range proofs prove each committed amount is non-negative (in the range [0, 2^64 − 1]).
- Original RingCT used **Borromean ring signatures** for range proofs — O(n) size per output.
- **Bulletproofs** (Bünz et al., 2017; deployed Monero October 2018) reduced range proof size dramatically (from ~13 KB to ~2.5 KB per output for typical transactions) with O(log n) verifier complexity.
- **Bulletproofs+** (deployed Monero August 2022) further reduced size by approximately 5-7% and improved verification speed. `[Established]`

### 8.4 Network-Level Privacy

On-chain privacy is only part of the picture. Network-level analysis can still reveal transaction origins by monitoring which node first broadcasts a transaction.

**Dandelion++** (deployed Monero, September 2020): A transaction relay protocol that hides the transaction's origin IP.
- **Stem phase:** A transaction is forwarded along a randomly selected path of nodes (the "stem"), not broadcast to all peers.
- **Fluff phase:** After a random number of hops, the transaction "fluffs" and is broadcast normally to all peers.
- An observer watching the network sees the transaction originating from an apparently random node, not the sender. `[Established; reduces but does not eliminate IP-based deanonymisation]`

**Tor and I2P support:** Monero Wallet CLI and GUI support routing connections through Tor or the I2P anonymous network, further hiding the sender's IP. `[Established — built into reference wallet]`

### 8.5 Monero's Consensus Rules

**Algorithm:** RandomX (see Part 6.4). Block time target: **120 seconds (2 minutes)**. `[Established, docs.getmonero.org, checked November 2024]`

**Difficulty adjustment:** Every block, using the LWMA (Linearly Weighted Moving Average) algorithm. Window: 720 blocks (approximately 24 hours). Extreme timestamp outliers discarded. `[Established]`

**Emission curve (main emission):** Monero's block reward follows a smooth exponential decay: `reward ≈ (M − A) × 2^-20 × 10^-12 XMR` where M = 2^64 − 1 and A = current supply in atomic units (piconeroes). This produces a steady, predictable decline rather than Bitcoin's sharp halvings. `[Established; exact formula from Monero codebase]`

**Tail emission:** When the block reward dropped below 0.6 XMR (approximately May 2022), Monero switched to a constant tail emission of **0.6 XMR per block indefinitely**. `[Established, checked November 2024]`

Tail emission is approximately:
- 0.6 × 262,800 blocks/year ≈ **157,680 XMR/year** (at 2-minute blocks)
- As of November 2024, with ~18.3+ million XMR in circulation, annual inflation is well below 1% and declining each year.

**Rationale for tail emission:** `[Disputed]`
- **For:** Unlike Bitcoin (where fees must entirely replace subsidies by ~2140), Monero guarantees miners a perpetual minimum reward, maintaining security indefinitely. This avoids Bitcoin's potential "fee market security" uncertainty.
- **Against:** A fixed perpetual issuance means Monero has no hard supply cap. Opponents argue this makes XMR a weaker store of value.
- **Response:** The annual inflation rate approaches zero asymptotically as the supply grows. By 2140, Monero's inflation will be ~0.1%, similar to or below gold's annual mining dilution.

**Dynamic block size and fee penalty:** Unlike Bitcoin's fixed 1 MB block size limit, Monero has a **dynamic block size** governed by a median-based formula.

- The "median block size" is the median size of the last 100 blocks.
- Miners may produce blocks up to 2× the median size without penalty.
- Blocks larger than the median incur a **block reward penalty** proportional to the excess. This prevents spam inflation (a miner producing giant blocks to suppress difficulty) while allowing organic growth.

Minimum block size is 300 KB (to prevent the median from collapsing to zero in low-use periods). `[Established]`

**Output unlock time (coinbase maturity):** Coinbase outputs (mining rewards) cannot be spent for 60 blocks (~2 hours) after the block containing them. Regular outputs have an unlock time of 10 blocks (~20 minutes), after which they can appear in ring signatures as decoys.

### 8.6 Known Attacks and Limitations

#### Decoy Selection and Timing Analysis

The core statistical risk in Monero ring signatures is that the decoys may be distinguishable from the real input. If an observer can assign high probability to one ring member being the real spend, privacy degrades.

**Coin age analysis:** Early decoy selection algorithms chose outputs non-uniformly — recent outputs were chosen less often than their true spend probability warranted. An adversary who knows that freshly minted outputs are rarely selected as decoys could guess that the most recently created ring member was the real spend (since it was likely there because it had to be, not randomly). `[Established; mitigated by improved gamma-distribution-based decoy selection]`

**OSPEAD research (April 2025 — post-dating this document's research cutoff):** Monero Research Lab published research showing that even with the current gamma distribution, statistical analysis can narrow a ring from 1-in-16 (perfect anonymity) to approximately 1-in-4.2 in the worst case for some transactions. This is a significant privacy reduction but still substantially better than Bitcoin's transparency. `[Established as of April 2025 MRL post; mark as recent finding]`

**Timing attacks:** If an observer monitors the network and records when transactions first appeared (using Dandelion++ source obfuscation data before the fluff phase), they can build probabilistic models of real spends.

**Wallet bugs and historical traceability:** Several wallet bugs have been documented that temporarily reduced ring signature effectiveness for affected transactions:
- "Differ By One" bug (pre-2020): certain wallet versions selected decoys in a detectable pattern.
- "10 Block Decoy Bug": a brief period where the wallet did not select the most recent 10 blocks' outputs as decoys, making those outputs identifiable as real spends when they did appear.
Transactions made during bug-active periods have reduced historical privacy that cannot be retroactively repaired. `[Established; arxiv:2408.05332, 2024]`

#### The 51% Attack and Monero

A 51% majority of Monero's hash rate would allow double-spends and block withholding, as with any PoW chain. Because Monero uses RandomX (CPU-friendly) with a relatively smaller hash rate than Bitcoin, the cost of a 51% attack on Monero is much lower than on Bitcoin. `[Established]`

Monero's privacy properties do not protect against consensus attacks. A successful 51% attacker could double-spend their own outputs just as on any PoW chain. What they cannot do is deanonymise past transactions.

#### Limitations Summary

| Limitation | Severity | Status |
|---|---|---|
| Ring size 16 gives probabilistic, not perfect, anonymity | Moderate | FCMP++ will replace rings with full-chain proofs |
| Decoy selection statistical attacks | Moderate | Ongoing MRL research (OSPEAD); improvement proposed |
| Old wallet bugs leave some transactions with reduced privacy | Low-Moderate | Historical; cannot be fixed retroactively |
| IP-level tracing (without Tor/I2P) | Moderate | Dandelion++ + Tor/I2P recommended |
| CPU mining centralisation pressure | Low | RandomX favours CPUs; large server farms have modest advantage |
| Regulatory exchange delistings (see §8.8) | Practical | Ongoing; P2P exchanges and atomic swaps as alternatives |

### 8.7 FCMP++ and Upcoming Protocol Upgrades

**FCMP++ (Full-Chain Membership Proofs++)** is Monero's most significant planned privacy upgrade. Instead of a ring of 16 outputs, FCMP++ proves membership in the **entire set of all outputs on the blockchain** — currently over 100 million outputs. This eliminates decoy selection as a statistical attack surface entirely.

**Technical approach:** Uses **cryptographic membership proofs** (based on Curve Trees with Pasta curves, proposed by Luke Parker / kayabaNerve) to prove that an output belongs to the full UTXO set without revealing which one. The proof size and verification time are sub-linear in the total output count.

**FCMP++ adds Spend Authorization and Linkability** (SAL) to the scheme, enabling it to be deployed before the full Seraphis protocol migration. This decoupled deployment allows privacy gains to reach users earlier.

**Status as of November 2024:** Approximately 76% of core integration milestone issues were closed. Not yet deployed on mainnet. Targeted for deployment in 2025 (tracking: github.com/monero-project/monero/milestone/1). `[Established as of November 2024; check current sources for latest status — may be deployed by the time you read this]`

**Seraphis and Jamtis:** These are longer-term protocol overhauls:
- **Seraphis:** A new transaction protocol that separates membership proofs from spend authorisation, enabling more scalable cryptography. FCMP++ can be deployed as a component of the existing protocol or as a precursor to Seraphis.
- **Jamtis:** A new addressing scheme accompanying Seraphis. Provides more flexible address types (view balance without spend key, forward-secrecy for subaddresses, etc.).

### 8.8 Comparison: Monero vs Bitcoin vs Zcash

| Feature | Bitcoin | Monero | Zcash |
|---|---|---|---|
| Privacy model | Pseudonymous (all transactions public) | Mandatory privacy (default for all txs) | Optional privacy (transparent or shielded) |
| Sender privacy | None | Ring signatures (CLSAG) | zk-SNARK (shielded only) |
| Amount privacy | None | RingCT (Pedersen commitments) | zk-SNARK (shielded only) |
| Recipient privacy | None | Stealth addresses | z-addresses (shielded only) |
| Privacy coverage | 0% of transactions | 100% of transactions | ~30% (shielded) as of 2024 |
| Supply cap | 21M BTC (fixed) | No hard cap (tail emission) | 21M ZEC (fixed) |
| Consensus | PoW SHA-256d | PoW RandomX | PoW Equihash 200,9 |
| Smart contracts | No | No | No |
| Trusted setup | No | No | Yes (Sapling ceremony; Halo2 reduces trust in newer circuits) |
| Chain analysis feasibility | High | Low (but non-zero) | Low for shielded; high for transparent |
| Exchange listings | Universal | Delistings (see §8.8) | Moderate |

**Zcash zk-SNARKs vs Monero RingCT:** Zcash's shielded transactions use **zk-SNARKs** (specifically Groth16) — zero-knowledge proofs that mathematically guarantee the verifier learns nothing about the transaction beyond its validity. This is theoretically stronger than ring signatures. The limitation is that Zcash's privacy is **optional**, so the anonymity set is only the fraction of transactions using shielded addresses (historically ~30%). A Monero user is in an anonymity set of *all* Monero users; a Zcash shielded user is in an anonymity set of only other shielded users. `[Established]`

Zcash's original trusted setup ("Powers of Tau" ceremony) means the system's security relied on at least one participant securely discarding their random contribution. The Sapling upgrade improved this. The Halo2 proof system (used in Orchard) eliminates the trusted setup requirement entirely. `[Established]`

### 8.9 Regulation and Exchange Delistings

This section states facts with dated sources only. It does not advocate for or against any regulatory position.

| Event | Date | Details |
|---|---|---|
| Bittrex XMR delisting | 2021 | Among the first major US exchanges to delist Monero, citing compliance concerns. |
| Binance XMR delisting (Belgium) | September 2023 | Monero restricted for Belgian users, citing upcoming EU MiCA regulation. |
| Binance XMR global delisting | February 20, 2024 | Monero, Zcash, Dash delisted globally from Binance. Official reason: "evolving regulatory requirements." |
| Kraken XMR delisting (Ireland, Belgium) | June 2024 | Monero trading halted for users in Ireland and Belgium, linked explicitly to MiCA. Other regions retain access as of this writing. |

**Context:** EU's Markets in Crypto-Assets (MiCA) regulation, adopted 2023, imposes stricter AML/CFT requirements on crypto service providers. Privacy coins present compliance challenges because transaction history required for AML checks is hidden by design. `[Established regulatory context; legal analysis is a separate matter]`

**Alternatives:** Monero users in affected jurisdictions use peer-to-peer exchanges (LocalMonero, Haveno), atomic swaps (Bitcoin ↔ Monero via COMIT/farcaster protocol), and direct transfers. `[Established; these alternatives exist and are functional]`

---

### Part 8 Review Questions

**Q1. Why is fungibility more important than pseudonymity for digital money?**
A: Pseudonymous coins have traceable histories, allowing some to be "tainted" (blacklisted) while others are "clean." A tainted coin is worth less than a clean coin, destroying interchangeability and making money non-neutral. Fungibility — all units equally acceptable — requires that history be unreadable.

**Q2. What three cryptographic techniques does Monero combine, and what does each hide?**
A: Ring signatures (CLSAG) — hide the sender. Stealth addresses — hide the recipient. RingCT with Pedersen commitments and Bulletproofs+ — hide the amount.

**Q3. What is a key image and why is it necessary?**
A: A key image is a deterministic curve point derived from the output's private key (I = k × Hp(P)). It prevents double-spending: each output can only produce one key image, and the network rejects any transaction whose key image was used in a previous transaction.

**Q4. What is Monero's tail emission, what is the amount, and what is the main argument for it?**
A: 0.6 XMR per block (~157,680 XMR/year). The argument is that it provides a permanent minimum security subsidy for miners, preventing the security model from depending entirely on fees — unlike Bitcoin, whose subsidy reaches zero by ~2140.

**Q5. How does FCMP++ improve on ring signatures, and what is its deployment status as of November 2024?**
A: FCMP++ proves membership in the full output set (100M+ outputs), eliminating the statistical attack surface of small ring sizes. As of November 2024, approximately 76% complete; not yet on mainnet; targeted for 2025 (check current sources).

### Common Misconceptions

- **"Monero transactions are completely untraceable."** Monero provides strong probabilistic anonymity, not mathematical guarantee. Statistical attacks, timing analysis, wallet bugs, and network-level observations can degrade privacy. FCMP++ will substantially strengthen guarantees.
- **"You need Monero to do anything illegal."** Bitcoin is used for far more criminal activity in absolute terms due to its much larger liquidity and adoption. Privacy is a legitimate and normal requirement for law-abiding individuals.
- **"Monero has no supply cap so it will inflate forever."** The annual inflation rate declines as the circulating supply grows. By 2100, it will be below 0.1%. This is comparable to gold.

### After Part 8 You Should Be Able To

Explain what fungibility means and why it requires privacy. Describe ring signatures, stealth addresses, and RingCT at a technical level. Explain what a key image prevents. Describe Monero's emission curve and tail emission. Identify at least three known limitations of Monero's privacy and their current mitigation status. Compare Monero, Bitcoin, and Zcash on privacy approach.

### Sources — Part 8

| Source | Date | Type |
|---|---|---|
| CryptoNote whitepaper ("Nicolas van Saberhagen") — cryptonote.org/whitepaper.pdf | 2013 | Primary |
| Shen Noether et al., "Ring Confidential Transactions" — MRL-0005 — lab.getmonero.org | 2015 | Primary (Monero Research Lab) |
| Bünz et al., "Bulletproofs: Short Proofs for Confidential Transactions and More" — IEEE S&P 2018 | 2017 | Primary |
| Chator, Green, et al., "CLSAG: Concise Linkable Ring Signatures and Forgery Against Adversarially Chosen Messages" — IEEE S&P 2020 | 2020 | Primary |
| getmonero.org blog: "FCMP++ development" — web.getmonero.org/2024/04/27/fcmps.html | April 2024 | Documentation |
| getmonero.org blog: "OSPEAD — Optimal Ring Signature Research" — getmonero.org/2025/04/05/ospead-optimal-ring-signature-research | April 2025 | Primary (MRL) |
| docs.getmonero.org/technical-specs/ | Ongoing | Documentation |
| Möser et al., "An Empirical Analysis of Traceability in the Monero Blockchain" — PoPETs 2018 | 2018 | Secondary (academic) |
| arxiv:2408.05332, "Monero Traceability Heuristics" — 2024 | 2024 | Secondary (academic) |
| Kappos et al., "An Empirical Analysis of Privacy in the Lightning Network" (comparative context) | 2021 | Secondary |
| Zero to Monero (Kurt Alonso, Sarang Noether) — web.getmonero.org/library/Zero-to-Monero-2-0-0.pdf | 2020 | Primary (technical book) |
| Binance delisting announcement — binance.com/en/square/post/3772794283529 | January 2024 | Secondary (news/official) |
| Kraken delisting notice — cryptobriefing.com/kraken-monero-delisting-update/ | April 2024 | Secondary (news) |


---

## Part 9: Virtual Machines

### 9.1 What a Blockchain VM Is and Why It Exists

A **blockchain virtual machine** is a sandboxed, deterministic execution environment that runs programs ("smart contracts") on-chain. Its purpose: allow arbitrary programmable logic to execute trustlessly, with outputs verified by all network nodes rather than any single trusted server.

**Why determinism is essential:** Every node in the network must run the same program on the same input and produce exactly the same output. If execution were non-deterministic (dependent on system time, floating-point rounding quirks, or random data), different nodes would reach different states and consensus would fail. A blockchain VM enforces determinism by restricting operations to a precisely defined, reproducible instruction set.

**Why sandboxing is essential:** Smart contracts are uploaded by untrusted users. A VM must prevent contracts from accessing the host file system, making network calls, or consuming unbounded resources. The **gas mechanism** (Section 9.2.4) bounds resource use.

### 9.2 The Ethereum Virtual Machine (EVM)

The EVM is a stack-based, deterministic virtual machine that runs at the core of every Ethereum full node. All smart contract execution happens inside it. `[Established]`

#### 9.2.1 Architecture

The EVM uses three storage areas, each with different persistence and cost:

| Area | Persistence | Access | Cost |
|---|---|---|---|
| **Stack** | Ephemeral (per call) | Last-in-first-out; 1,024-item max; 256-bit items | Low |
| **Memory** | Ephemeral (per transaction) | Byte-addressable, dynamically expanding | Grows quadratically with size |
| **Storage** | Permanent (in-state) | Key-value (256-bit → 256-bit) | Very high (SSTORE: 20,000 gas new, 2,900 gas modify) |
| **Calldata** | Ephemeral (input only) | Read-only, cheapest data input | Low |
| **Return data** | Ephemeral | Output from subcall | Low |

Additionally, the EVM accesses:
- **Code**: The contract's bytecode (read-only during execution).
- **World state**: The global mapping of addresses → (nonce, balance, storageRoot, codeHash).

#### 9.2.2 Execution Model

The EVM is a **stack machine**: instructions pop arguments from the top of the stack and push results. Execution proceeds instruction by instruction.

**Transaction execution lifecycle:**
1. Caller provides `nonce`, `gasPrice`, `gasLimit`, `to`, `value`, `data`, `v/r/s`.
2. Node deducts `gasLimit × gasPrice` from caller's balance (upfront payment).
3. EVM loads the contract code at address `to` (or deploys if `to` is null).
4. EVM executes bytecode one opcode at a time, consuming gas.
5. If execution completes without running out of gas: state changes are committed; unused gas is refunded.
6. If gas runs out: an `out-of-gas` exception reverts all state changes (but gas is not refunded).
7. If execution reverts (`REVERT` opcode): state changes undone; unused gas refunded; revert reason returned.

#### 9.2.3 Key Opcodes

| Category | Opcodes | Description |
|---|---|---|
| Arithmetic | `ADD(3)`, `MUL(5)`, `SUB(3)`, `DIV(5)`, `MOD(5)`, `EXP(10+)` | Integer arithmetic (256-bit, modular) |
| Comparison | `LT`, `GT`, `EQ`, `ISZERO` | Stack comparisons |
| Bitwise | `AND`, `OR`, `XOR`, `NOT`, `SHL`, `SHR` | Bitwise operations |
| Environment | `CALLER`, `CALLVALUE`, `CALLDATALOAD`, `BLOCKHASH`, `TIMESTAMP` | Access transaction and block context |
| Storage | `SLOAD(100/2100)`, `SSTORE(100-20000)` | Read/write persistent storage |
| Memory | `MLOAD(3)`, `MSTORE(3)` | Read/write call-scoped memory |
| Control | `JUMP(8)`, `JUMPI(10)`, `STOP(0)`, `REVERT(0)`, `RETURN(0)` | Flow control |
| Call | `CALL(100+)`, `DELEGATECALL`, `STATICCALL`, `CREATE`, `CREATE2` | External calls and contract deployment |
| Cryptographic | `SHA3/KECCAK256(30+)` | Hash operation |
| Stack manipulation | `PUSH1`–`PUSH32`, `POP`, `DUP1`–`DUP16`, `SWAP1`–`SWAP16` | Stack manipulation |
| New (recent additions) | `PUSH0`, `TLOAD`, `TSTORE` (EIP-1153), `MCOPY` (EIP-5656), `BLOBHASH` (EIP-4844) | 2024 additions |

Gas costs in parentheses; dynamic costs apply for many operations (memory expansion, cold vs warm storage access).

#### 9.2.4 Gas

**Gas** is the EVM's unit of computational cost. It serves three purposes:
1. **Denial-of-service prevention:** Infinite loops cannot run forever — they exhaust gas and halt.
2. **Fair pricing:** More computationally intensive operations cost more gas.
3. **Resource allocation:** Users bid for block inclusion by paying `gasPrice`. After EIP-1559 (August 2021): `effectiveFee = baseFee + priorityFee`. The `baseFee` is burnt (removed from supply); `priorityFee` goes to the validator. `[Established]`

**Gas worked example:**
```
Solidity: uint256 a = 5 + 3;

EVM bytecode:
  PUSH1 0x05   (3 gas)   → stack: [5]
  PUSH1 0x03   (3 gas)   → stack: [3, 5]
  ADD          (3 gas)   → stack: [8]
Total: 9 gas for this fragment.
```

**Storage is by far the most expensive operation** because it modifies the global state that every node must store permanently. Writing 32 bytes to a new storage slot costs 20,000 gas (≈$0.01–$10 depending on ETH price and baseFee). `[Established]`

#### 9.2.5 Smart Contract Deployment and Interaction

**Deployment:**
1. User sends a transaction with `to = null` and `data = constructor_bytecode + abi_encoded_args`.
2. EVM executes the constructor code, which sets up storage and returns the runtime bytecode.
3. Runtime bytecode is stored on-chain at the new contract address (deterministic: `keccak256(deployer, nonce)`).

**Interaction:**
1. User calls the contract address with `data = abi.encodeWithSelector(functionSig, args)`.
2. EVM loads the runtime bytecode, decodes the first 4 bytes (function selector), and jumps to the matching function.

**ABI (Application Binary Interface):** A convention for encoding function calls and return values as bytes. Not enforced by the EVM — it is for off-chain tooling (web3 libraries, explorers).

#### 9.2.6 State

Ethereum's global state is a **Modified Merkle Patricia Trie** (MMPT): a trie (prefix tree) where each leaf represents an account, and intermediate nodes are keyed by address bytes. The root hash of the state trie is included in each block header, committing to the entire world state.

The state transition function is: `σ' = Υ(σ, T)` — the new state σ' is the result of applying transaction T to the current state σ. Every node replicates this function and must arrive at the same σ'. `[Established; Ethereum Yellow Paper §2]`

### 9.3 Bitcoin Script

Bitcoin does not have a general-purpose VM. It has **Script** — a **stack-based, non-Turing-complete** language for expressing spending conditions.

**Design goals:** Intentional simplicity. No loops (prevents infinite loops without gas). Very limited operations (no arbitrary computation). `[Established — by design]`

**Locking script (scriptPubKey):** Attached to a UTXO; defines the conditions for spending.
**Unlocking script (scriptSig / witness):** Provided by the spender; must satisfy the locking script.

**Example — Pay-to-Public-Key-Hash (P2PKH):**
```
Locking:   OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
Unlocking: <signature> <pubKey>

Combined execution:
  Stack: [<sig>, <pubKey>]
  OP_DUP    → [<sig>, <pubKey>, <pubKey>]
  OP_HASH160 → [<sig>, <pubKey>, HASH160(<pubKey>)]
  <pubKeyHash> → [<sig>, <pubKey>, HASH160(<pubKey>), <pubKeyHash>]
  OP_EQUALVERIFY → verify HASH160(pubKey) == pubKeyHash; stack: [<sig>, <pubKey>]
  OP_CHECKSIG → verify sig over tx with pubKey; push 1 (true) if valid
```

**Taproot (BIP 341, Tapscript):** Significantly increases Bitcoin Script's expressiveness while maintaining privacy. Uses Schnorr signatures and a Merkle tree of scripts (MAST) so that only the executed spending path is revealed on-chain. `[Established]`

**Key limitation:** Bitcoin Script is intentionally restrictive. It cannot implement arbitrary smart contracts (DeFi, DAOs, tokens). This is a deliberate design philosophy ("simplicity = security"), not an oversight.

### 9.4 Monero's Deliberate Lack of a General VM

Monero has no smart contract language and no general-purpose VM. This is an explicit design choice driven by:

1. **Complexity = attack surface.** Every VM feature adds code that can contain bugs. CryptoNote's designers and Monero's developers have consistently prioritised keeping the protocol as simple as possible.
2. **Privacy preservation.** A VM with on-chain state exposes contract interactions to chain analysis. Monero's privacy guarantees rest on the simplicity of its transaction model.
3. **Focus on fungibility as the core use case.** Monero's developers believe a currency with strong fungibility and privacy is more valuable than a smart-contract platform with weak privacy. `[Established — from Monero design principles documentation]`

The tradeoff: Monero cannot host DeFi protocols, tokens, or complex smart contracts. This is accepted as appropriate for its use case.

### 9.5 Other Virtual Machines

#### Solana eBPF

Solana does not use WebAssembly. It compiles contracts (programs) to **Berkeley Packet Filter (BPF)** bytecode — a low-overhead bytecode originally designed for network packet filtering. Solana uses a modified variant (sBPF).

**Key property:** BPF execution is fast and predictable in time. Combined with Solana's parallel transaction execution (multiple programs in one block run concurrently if they access different state), this contributes to Solana's high throughput. `[Established]`

**Used by:** Solana. Programs are written in Rust or C.

#### WebAssembly (WASM) Virtual Machines

WASM is a binary instruction format and VM specification with broad industry support (all major web browsers). It is deterministic (when configured carefully), portable, and supports many source languages.

**Chains using WASM for smart contracts:**
- **Near Protocol (Rust/TypeScript → WASM):** Account-based chain.
- **Polkadot (ink! → WASM via Substrate):** Each parachain can have its own contract environment.
- **CosmWasm (Rust → WASM):** Smart contract standard for Cosmos SDK chains.
- **Dfinity/Internet Computer:** WASM with actor model (canisters).

**Advantage over EVM:** WASM is a standard with existing compiler toolchains. Performance can exceed EVM by 10-100x. **Disadvantage:** Ecosystem and tooling maturity is lower than EVM; gas cost modelling is harder for a full WASM instruction set.

#### Move VM (Aptos, Sui)

**Move** is a programming language and VM developed by Meta for the Diem blockchain (now discontinued) and adopted by Aptos and Sui.

**Key innovation:** Move treats blockchain assets as **linear types** — a resource can only be moved, not copied or accidentally deleted. This eliminates entire classes of smart contract bugs (re-entrancy-like issues, double-spending of on-chain assets) at the language level. `[Established]`

**Aptos:** Uses Move with BlockSTM (parallel optimistic execution engine). `[Established]`
**Sui:** Uses an object-centric Move variant with concurrent execution by design. `[Established]`

#### Cairo VM (StarkNet)

**Cairo** is a Turing-complete language designed to be efficiently provable by **STARK proofs** (Scalable Transparent ARguments of Knowledge).

**Key property:** Cairo programs compile to a VM trace that can be verified with a validity proof exponentially smaller than re-executing the program. This is the foundation for StarkNet's ZK-rollup. `[Established]`

**Used by:** StarkNet (Ethereum L2).

#### EVM-Compatible and EVM-Equivalent VMs

Many Layer 2 networks and alternative L1s run EVM-compatible VMs:
- **Polygon (PoS):** EVM-compatible L1/sidechain.
- **Arbitrum Nitro:** EVM-equivalent (supports same bytecode, translated to WASM for proving).
- **Optimism / OP Stack:** EVM-equivalent.
- **zkSync Era:** zkEVM — proves EVM execution with ZK proofs (Type 1 or Type 2 compatibility levels).
- **Polygon zkEVM:** ZK-proven EVM.
- **Avalanche C-Chain:** EVM-compatible.
- **BNB Smart Chain:** EVM fork.

The **zkEVM** is a VM that is both EVM-compatible (runs existing Ethereum smart contracts) and provable via zero-knowledge proofs, enabling ZK-rollup security. Building a zkEVM is technically very difficult because the EVM was not designed with ZK provability in mind. `[Established]`

#### VM Comparison Table

| VM | Chain(s) | Turing-complete | Gas mechanism | Language(s) | ASIC/ZK provable |
|---|---|---|---|---|---|
| EVM | Ethereum, Polygon, Avalanche C-Chain, many | Yes | Gas per opcode | Solidity, Vyper, Fe | zkEVM efforts ongoing |
| Bitcoin Script | Bitcoin | No | Weight-based fees | Script (stack-based) | N/A |
| Solana eBPF | Solana | Yes | Compute units | Rust, C | No |
| WASM (Near) | Near | Yes | Gas per instruction | Rust, TypeScript | Partially |
| WASM (ink!) | Polkadot | Yes | Gas | Rust | No |
| Move VM | Aptos, Sui | Yes | Gas | Move | No |
| Cairo VM | StarkNet | Yes | Gas (STARK-provable) | Cairo | Yes (STARK) |
| None | Monero | No VM | N/A | N/A | N/A |

---

### Part 9 Review Questions

**Q1. Why must a blockchain VM be deterministic, and what would happen if it weren't?**
A: Every node runs the same contract and must reach the same state. Non-determinism would cause nodes to disagree on the resulting state, making consensus impossible.

**Q2. What is gas and what two purposes does it serve in the EVM?**
A: Gas is the unit of EVM computational cost. It (1) prevents infinite loops (programs run out of gas and halt) and (2) prices computation so users bid for block inclusion and validators are compensated.

**Q3. Why is Bitcoin Script intentionally non-Turing-complete?**
A: To eliminate the possibility of infinite loops (simplifying resource bound analysis), reduce attack surface, and maintain simplicity. The design philosophy is that simplicity = security for a base-layer monetary system.

**Q4. What is Move's key innovation over Solidity/EVM?**
A: Move treats assets as linear types — they can only be moved, not copied or accidentally destroyed. This eliminates whole categories of re-entrancy-like bugs and double-spend vulnerabilities at the language level.

**Q5. Why does Monero deliberately avoid a general-purpose VM?**
A: Complexity adds attack surface and privacy risks. A VM introduces on-chain state that enables chain analysis. Monero's design prioritises simplicity and privacy over programmability.

### Common Misconceptions

- **"Smart contracts are automatically trustworthy because they are code."** Code can have bugs. Smart contracts have lost hundreds of millions of dollars due to bugs (DAO hack 2016, Parity multisig 2017, countless DeFi exploits).
- **"The EVM is Ethereum."** The EVM is one component. Ethereum also has a P2P layer, a consensus mechanism (Gasper), a state model, and a networking protocol.
- **"WebAssembly-based VMs are strictly better than the EVM."** They can be faster and more flexible but lack the EVM's mature tooling, security track record, and developer ecosystem.

### After Part 9 You Should Be Able To

Explain what a blockchain VM is and why it needs to be deterministic and sandboxed. Describe the EVM's three storage areas and their respective costs. Explain what gas is and how EIP-1559 changed fee mechanics. Distinguish Bitcoin Script from the EVM. Name at least three non-EVM blockchain VMs and their key properties.

### Sources — Part 9

| Source | Date | Type |
|---|---|---|
| Ethereum Yellow Paper (Gavin Wood) — ethereum.github.io/yellowpaper/paper.pdf | 2014, updated | Primary |
| ethereum.org/developers/docs/evm/ | Ongoing | Documentation |
| evm.codes — opcode reference and gas costs | Ongoing | Documentation |
| BIP 341: Taproot (Bitcoin) — github.com/bitcoin/bips/blob/master/bip-0341.mediawiki | 2020 | Primary |
| Move language specification — github.com/move-language/move | 2019 | Primary |
| Solana Documentation: "Developing Programs" — docs.solana.com | Ongoing | Documentation |
| StarkNet / Cairo documentation — docs.starknet.io | Ongoing | Documentation |
| CosmWasm documentation — docs.cosmwasm.com | Ongoing | Documentation |
| EIP-1153 (Transient storage), EIP-5656 (MCOPY), EIP-4844 (Blobs) | 2022–2024 | Primary (EIPs) |


---

## Part 10: What a Chain Needs to Function

This section is a complete checklist. Each item is explained and linked to earlier parts. Items marked `[Additional]` are not in the original specification but are identified as necessary through research.

### 10.1 Network Layer

**What it is:** A peer-to-peer (P2P) communications layer that lets nodes find each other, exchange blocks and transactions, and maintain topology.

**Components required:**
- **Discovery:** New nodes need to find initial peers (seed nodes, DNS seeds, peer address databases).
- **Gossip protocol:** New transactions and blocks must propagate quickly to all nodes. Bitcoin uses an `inv/getdata` pull-based gossip; Ethereum uses a libp2p-based gossip pub/sub.
- **Block and transaction propagation:** Compact blocks (BIP 152, Ethereum's SSZ encoding) minimise bandwidth.
- **Network resilience:** Nodes must handle dropped connections, slow peers, and NAT traversal.

**Failure mode without it:** Nodes cannot learn about new transactions or blocks; the chain halts.

### 10.2 Consensus Algorithm

**What it is:** The rule by which honest nodes agree on which block to extend next. See Part 5 in full.

**Required properties:** Sybil resistance, Byzantine fault tolerance, liveness under expected network conditions, deterministic or probabilistic finality (depending on design goals).

**Failure mode without it:** Double-spends are possible; different nodes have different "truths"; the ledger loses value as money.

### 10.3 Consensus Rules

**What it is:** The complete set of rules every full node enforces independently to determine block and transaction validity. See Part 7.

**Required components:** Block header validity, block body validity, transaction validity (signature checks, unspent input checks, emission limits), chain selection rule.

**Failure mode without it:** Miners can produce invalid blocks (create coins, double-spend), and nodes will not reject them.

### 10.4 State Model

**What it is:** How the current state of the ledger is represented and stored. See Part 3.

**Two primary models:**
- UTXO set: set of all unspent transaction outputs.
- Account model: key-value map of addresses to account states.

**Additional state (EVM chains):** Contract storage, code, nonces.

**Required properties:** Deterministic update rule; efficient query and modification; cryptographic commitment (Merkle root) to detect tampering.

**Failure mode without it:** Nodes cannot determine whether an input is unspent; double-spending cannot be detected.

### 10.5 Transaction Format

**What it is:** The serialised binary encoding of a transaction, including all data needed for validation.

**Required fields (Bitcoin):** Version, inputs (prev txid, vout, scriptSig, sequence), outputs (value, scriptPubKey), locktime, (SegWit: witness data).

**Required fields (Ethereum):** Nonce, gas parameters, to, value, data, signature (v/r/s), chain ID.

**Required properties:** Deterministic serialisation (same transaction = same bytes = same TXID); compact enough for block propagation; complete enough for independent validation.

**Failure mode without it:** Nodes cannot validate, relay, or store transactions consistently.

### 10.6 Fee Mechanism

**What it is:** The incentive for miners/validators to include a transaction in a block, and a mechanism for users to express priority.

**Models:**
- **Bitcoin:** Implicit fee (inputs − outputs); fee rate in sat/vbyte; miners maximise fee revenue per byte.
- **Ethereum (post-EIP-1559):** Base fee (algorithmic, burnt) + priority fee (to validator); gas limit per transaction; fee adjusts block by block based on utilisation.
- **Monero:** Dynamic fee schedule based on block weight and fee level tiers; fee is explicit in the transaction.

**Required properties:** Prevents mempool spam (submitting valid but zero-fee transactions to waste nodes' resources); aligns miner/validator incentives with network health; allows user control over inclusion priority.

**Failure mode without it:** Mempool flooding; inclusion becomes random or arbitrary; miners have no incentive beyond the block subsidy.

### 10.7 Incentive Structure

**What it is:** The economic rewards that make honest participation more profitable than dishonest participation.

**Components:**
- Block subsidy (newly minted coins): compensates miners/validators for the cost of securing the chain.
- Transaction fees: compensates for including transactions; grows in importance as subsidies decline.
- Slashing (PoS): penalises provably dishonest behaviour (double-signing).
- Opportunity cost: a rational 51% attacker who also holds the currency loses value by attacking (the attack destroys confidence and price). `[Disputed whether this is a reliable deterrent — depends heavily on the attacker's holdings and goals]`

**Failure mode without it:** Miners/validators have no incentive to participate honestly; the chain becomes insecure.

### 10.8 Validation Rules

**What it is:** The specific algorithms nodes run to validate blocks and transactions. Distinct from "consensus rules" (what must be true) in being the implementation.

**Components:**
- Script interpreter (Bitcoin) / EVM execution engine (Ethereum) / ring signature verifier (Monero)
- Signature verification library
- UTXO set / state trie management
- Difficulty / chainwork calculation

**Failure mode without it:** Validation is impossible; any block is accepted.

### 10.9 Governance and Upgrade Process

**What it is:** The social and technical mechanisms by which the protocol can be changed.

**Bitcoin's approach:** BIPs (Bitcoin Improvement Proposals); community discussion; miner signalling for activation; full node operators as the final arbiter. Extremely conservative; changes are rare.

**Ethereum's approach:** EIPs (Ethereum Improvement Proposals); core developer calls; scheduled hard forks (now coordinated network upgrades); client diversity reduces single-point failure.

**Monero's approach:** Scheduled hard forks approximately every 6-12 months; Monero Research Lab publications; community consensus via GitHub and IRC/Matrix; Core Team for coordination.

**PoS with on-chain governance (Tezos, Cosmos Hub):** Token holders vote on-chain for upgrades; approved upgrades apply automatically.

**Failure mode without it:** Inability to fix bugs, improve privacy, adjust parameters, or respond to discovered vulnerabilities. The chain either stagnates or requires uncoordinated forks.

### 10.10 Security Assumptions

**What it is:** The explicit assumptions under which the protocol is secure. Every protocol has them; not stating them is dangerous.

**Bitcoin's assumptions:**
- SHA-256 and ECDSA/Schnorr are computationally hard to break (pre-image resistance, discrete log).
- No single entity controls ≥50% of hash power.
- Miners are rational profit-maximisers (economic security).
- The peer-to-peer network delivers messages within reasonable time (partial synchrony).
- The 21 million cap is enforced by all full nodes running correct software.

**Ethereum's additional assumptions:**
- ≤1/3 of validators are Byzantine (for finality).
- The secp256k1 curve and BLS12-381 (validator signatures) are secure.
- ETH price stays high enough to make slashing economically deterring.

**Quantum computing caveat:** `[Disputed]` A sufficiently powerful quantum computer running Shor's algorithm could recover private keys from public keys (breaking ECDSA, Schnorr, Ed25519). The timeline for this is debated (consensus among researchers: not within 10-15 years for Bitcoin-relevant key sizes; some argue sooner). Post-quantum signature schemes exist (lattice-based, hash-based) and migration planning is discussed in the Bitcoin and Ethereum communities. Monero uses Ed25519 and similar; it faces the same eventual quantum risk. `[Established risk; uncertain timeline]`

### 10.11 `[Additional]` Cryptographic Primitive Library

A chain requires a battle-hardened, well-audited implementation of its core cryptographic primitives. This includes hash functions, digital signature schemes, elliptic curve implementations, and (for privacy coins) ring signature and commitment scheme implementations.

**Why this matters:** The 2010 Bitcoin overflow bug was not a cryptographic failure but an integer overflow in validation code. Cryptographic libraries have their own bugs (Libsodium's Ed25519 `has had` minor issues; various OpenSSL vulnerabilities affected non-blockchain software). Using audited, tested libraries is essential.

### 10.12 `[Additional]` Peer Discovery and Bootstrapping

Distinct from the ongoing network layer, bootstrapping is the special problem of a brand-new node finding its first peers. Mechanisms:
- **Hardcoded DNS seeds:** Bitcoin Core has hardcoded DNS seed hostnames (dns-seed.bitcoin.dashjr.org, etc.) that return lists of active nodes.
- **Hardcoded IP addresses:** Fallback for DNS failure.
- **Seed nodes:** Monero includes `-seed` nodes (p2pseed.rino.io, etc.) in the default configuration.

A chain without bootstrapping mechanisms forces users to find peers manually, which is impractical for ordinary users.

### 10.13 `[Additional]` Canonical Transaction Ordering and Malleability Resistance

Within a block, the order of transactions must be deterministic or at minimum canonical. Transaction malleability — the ability to change a transaction's bytes (and thus its TXID) without invalidating it — caused major issues for Bitcoin before SegWit. Payment channels (Lightning Network) depend on stable TXIDs. SegWit moved signatures out of the TXID-committed data, fixing this. `[Established]`

### 10.14 `[Additional]` Archive and Pruning Strategy

Over time, the full chain history grows unboundedly. Nodes need:
- A **pruning mode** (Bitcoin) that discards old spent transaction data, retaining only headers and the UTXO set.
- An **archive mode** for applications requiring historical state.
- **Bootstrap acceleration** (snapshot sync, fast sync, checkpoint sync) so new nodes do not spend weeks downloading history.

Without a pruning strategy, storage requirements grow forever and exclude ordinary users from running full nodes.

---

### Part 10 Review Questions

**Q1. What would happen to a blockchain without a fee mechanism?**
A: Mempool spam becomes free — an attacker could flood the network with zero-cost transactions, filling mempools and blocks with junk. Miners would have no incentive to include any particular transaction.

**Q2. Why is the governance mechanism as important as the consensus mechanism?**
A: A perfect initial design cannot anticipate every future problem. Bugs must be patched, parameters adjusted, and improvements deployed. Without a governance mechanism, the chain cannot evolve — either stagnating with known flaws or splitting chaotically.

**Q3. Why does Bitcoin's economic security depend on the assumption that miners are rational, and when does this fail?**
A: A rational miner maximises profit; attacking the chain destroys it and their own investment (coins + hardware). This fails if the attacker's goal is not profit (a nation-state attack, an ideological actor) or if they have heavily shorted the asset.

**Q4. What is transaction malleability and how did SegWit fix it?**
A: Malleability: altering a transaction's witness data (signature) without invalidating it, changing its TXID. SegWit moved witness data outside the TXID-committed transaction bytes, ensuring the TXID is stable.

**Q5. Why can't a chain simply never change its consensus rules?**
A: Bugs exist in all complex software. Security vulnerabilities are discovered. Network parameters (block size, fee floors) need adjustment as the network grows. A chain that cannot change will eventually fail to a known vulnerability or become economically uncompetitive.

### After Part 10 You Should Be Able To

Name all ten-plus components a blockchain needs to function. Explain how each one would cause the chain to fail if absent. Explain Bitcoin's security assumptions explicitly.

### Sources — Part 10

| Source | Date | Type |
|---|---|---|
| Nakamoto, Bitcoin whitepaper | 2008 | Primary |
| Bitcoin Developer Documentation — developer.bitcoin.org | Ongoing | Documentation |
| Ethereum Yellow Paper | 2014, updated | Primary |
| BIP 152 (Compact Blocks) — github.com/bitcoin/bips/blob/master/bip-0152.mediawiki | 2016 | Primary |
| BIP 141 (SegWit) — fix for transaction malleability | 2015 | Primary |
| Monero Technical Specification — docs.getmonero.org/technical-specs/ | Ongoing | Documentation |


---

## Part 11: Advanced Topics

### 11.1 Scaling and Layer-2 Designs

**The scalability trilemma** `[Disputed — a heuristic, not a theorem]`: Buterin's conjecture that it is difficult to simultaneously achieve all three of:
- **Decentralisation:** Many nodes can participate without special hardware.
- **Security:** Resistant to 51% attacks and other adversarial behaviour.
- **Scalability:** High throughput at low cost.

Bitcoin processes ~7 transactions per second (TPS); Visa handles ~1,700 average (~24,000 peak). This gap motivates Layer-2 (L2) solutions.

#### 11.1.1 State Channels (Lightning Network)

**Concept:** Two parties lock funds into a 2-of-2 multisig on-chain, then exchange signed "commitment transactions" off-chain without broadcasting each one. Only the opening and closing of the channel require on-chain transactions.

**Lightning Network (LN):** Bitcoin's dominant L2. Routes payments through a network of channels. Alice → Charlie even if no direct channel exists, as long as there is a path Alice → Bob → Charlie with sufficient liquidity.

**Security:** Each party can unilaterally close the channel by broadcasting the latest commitment transaction. Outdated states are penalised (via revocation keys) if one party tries to broadcast a stale (better) state.

**Limitations:** Requires both parties to occasionally be online (or use a watchtower). Routing large payments is unreliable. Capital efficiency: funds locked in channels cannot be used elsewhere. `[Established]`

#### 11.1.2 Plasma (Ethereum — largely superseded)

Plasma (Buterin & Poon, 2017) proposed hierarchical chains where operators produce blocks, users periodically commit roots to L1, and exit mechanisms allow users to withdraw if the operator cheats.

**Problems:** Complicated exit games; "data withholding" problem (operator can hide block data preventing users from exiting); complex to implement correctly. Mostly superseded by rollups. `[Established]`

#### 11.1.3 Rollups

Rollups execute transactions off-chain but post transaction data (or validity proofs) to L1. L1 serves as the data availability and security layer.

**Optimistic Rollups (Arbitrum, Optimism):**
- Assume all transactions are valid ("optimistic").
- Post all transaction data to L1.
- A fraud proof period (~7 days) allows anyone to challenge an invalid state transition.
- Security: inherits Ethereum's security if at least one honest watcher submits fraud proofs.
- Withdrawal delay: 7 days for funds from L2 → L1 (can be circumvented with liquidity providers).

**ZK-Rollups (zkSync, StarkNet, Polygon zkEVM):**
- Execute transactions off-chain and generate a **validity proof** (ZK-SNARK or ZK-STARK) proving the state transition is correct.
- Post the proof + compressed data to L1.
- Security: cryptographic guarantee — no need for fraud proofs or trust in operators.
- Withdrawal: near-instant (once proof verified).
- Drawback: ZK proof generation is computationally intensive; zkEVM compatibility is technically very hard.

**Proto-Danksharding (EIP-4844, implemented Ethereum March 2024):** Introduces "blobs" — temporary large data objects attached to blocks that rollups can post their data to, without it being part of permanent calldata. Blobs are pruned after ~18 days. This reduced rollup data costs by approximately 10-100× at activation. `[Established, checked November 2024]`

**Full Danksharding (planned):** Full implementation of data availability sampling (DAS), where nodes verify data availability by sampling random chunks rather than downloading entire blobs. Would enable very high rollup throughput. `[Planned, not yet implemented as of November 2024]`

#### 11.1.4 Sidechains

A sidechain is a separate blockchain with a **two-way peg** to the main chain. Users lock coins on L1 and unlock equivalent tokens on the sidechain, then burn them to unlock on L1.

**Security:** Unlike rollups, sidechains have their own consensus — if the sidechain is compromised, the bridge can be drained.

**Examples:** Polygon PoS (originally sidechain; now being upgraded), Liquid Network (Bitcoin), Rootstock (RSK — Bitcoin sidechain with EVM).

#### 11.1.5 Validiums

A hybrid between ZK-rollup and sidechain: ZK validity proofs verified on L1, but transaction data stored off-chain by a data availability committee. Lower cost than ZK-rollup; weaker data availability guarantees. Used by StarkEx (dYdX V3, Immutable X).

### 11.2 MEV (Maximal Extractable Value) in Depth

**Definition:** The maximum value that can be extracted from block production by including, excluding, or reordering transactions within a block — above the standard block reward and fees. `[Established — Daian et al. 2019 "Flash Boys 2.0"]`

**Types of MEV:**

| Type | Description | Who benefits | Who loses |
|---|---|---|---|
| **Arbitrage** | Exploit price differences across DEX pools; fix price discrepancy created by a user's trade | Arbitrageur, validator | Liquidity providers (small) |
| **Sandwich attack** | Front-run + back-run a large AMM swap to profit from price impact | MEV bot | The user being sandwiched (worse execution price) |
| **Liquidation** | Be first to liquidate undercollateralised DeFi positions | Liquidator (priority fee) | Borrower (liquidation bonus paid to searcher) |
| **NFT sniping** | Front-run a low-priced NFT purchase | Sniper | Original buyer |
| **Just-in-time (JIT) liquidity** | Add and remove liquidity around a large trade to capture fees | JIT provider | Existing passive LPs |

**Scale:** MEV extraction on Ethereum has exceeded $1 billion cumulative. `[Established — mev.explore.flashbots.net data, though exact figures fluctuate; order of magnitude confirmed]`

**Proposer-Builder Separation (PBS):** Ethereum's response to MEV centralisation. Under PBS, the **builder** role (constructing blocks to maximise MEV) is separated from the **proposer** role (signing and broadcasting blocks). Builders bid for proposers' slots via MEV-boost relays. This has partially externalised MEV extraction but raised concerns about relay centralisation. `[Established, partially deployed via MEV-boost; full enshrined PBS in Ethereum roadmap]`

**MEV in Bitcoin:** Smaller than Ethereum because Bitcoin has no smart contracts or DEX protocols. However, ordinals and BRC-20 tokens have introduced some MEV dynamics. `[Established]`

### 11.3 Light Clients and Stateless Clients

**SPV light clients** (Bitcoin): Download only headers. Verify own transactions with Merkle proofs. Trust that the most-work chain is valid. See Part 3.

**Neutrino (BIP 157/158):** An improved Bitcoin light client protocol using **compact filters** — probabilistic summaries of block contents. Clients download filters, test locally which blocks contain their transactions, then download only those blocks. Better privacy than SPV (which reveals to peers which transactions you're interested in). `[Established]`

**Ethereum light clients:** More complex due to the account model. The Ethereum Beacon Chain has a **light client sync protocol** (from the Altair upgrade) where a small committee (~512 validators) signs a sync committee header every 27 hours. Light clients trust this committee. `[Established]`

**Stateless clients:** A major Ethereum research direction. Currently, Ethereum full nodes must hold the full state trie (~100s of GB) to validate blocks. Stateless clients would instead require block producers to provide "witnesses" (Merkle/Verkle proofs of all state accessed by the block). Any node could then validate the block without maintaining full state.

**Verkle trees:** A cryptographic structure replacing Merkle Patricia Tries for Ethereum state, using **polynomial commitments (IPA/KZG)** instead of hash-based Merkle proofs. Verkle witnesses are ~100-1000× smaller than equivalent Merkle Patricia proofs, making stateless clients practical. Verkle tree migration is a major ongoing Ethereum engineering effort. `[Established — in development as of 2024]`

### 11.4 Sharding and Data Availability

**Sharding** (database concept applied to blockchains): Partition the network into groups (shards) where each shard processes a subset of transactions. Total throughput = single shard throughput × number of shards.

**The challenge:** Cross-shard transactions are complex. Data availability: if a shard's block data is withheld, nodes processing other shards cannot verify that shard's state.

**Ethereum's approach:** Ethereum has moved away from execution sharding (each shard independently executing smart contracts) toward **data sharding** — a large number of data blobs available on-chain for rollups, with rollups handling execution. Full Danksharding targets ~32 blobs × 128 KB = 4 MB per slot → ~1.3 MB/s raw throughput for rollup data. `[Established — Ethereum roadmap as of 2024]`

**Data Availability Sampling (DAS):** A technique where nodes verify data availability by randomly sampling small chunks of block data and requesting them from the network. If the data is available, any random sample will be found. If the data is withheld, it cannot be found. This allows light nodes to verify data availability without downloading the entire block. `[Established — theoretical foundation by Buterin; implementation in full Danksharding]`

### 11.5 Principal Attack Vectors

#### Long-Range Attacks (PoS)

An attacker acquires old private keys from validators who had stake in the past (through purchase, theft, or social engineering). With these keys, the attacker rewrites history from that epoch forward at negligible cost (old stake is now spent, old keys sign for free).

**Mitigations:** Checkpointing (nodes reject reorgs past a finality point), key-evolving signatures (old key versions cannot sign new epochs), weak subjectivity (clients must sync from a checkpoint within the safety period). `[Established]`

#### Nothing-at-Stake (PoS)

In naive PoS without slashing, a validator can vote on multiple fork tips at zero cost. This can delay convergence. **Solution:** Slashing — provably conflicting signatures trigger automatic stake loss. Ethereum, Cosmos, and other BFT PoS chains implement slashing. `[Established]`

#### Eclipse Attack

An attacker controls all of a victim node's peer connections, isolating it from the honest network. The victim sees only attacker-controlled chain data, enabling double-spend acceptance and other attacks. **Mitigations:** Diverse peer selection, limiting connections from same subnet, authenticated peer addresses. `[Established]`

#### BGP Hijacking

An attacker uses BGP (Border Gateway Protocol) routing attacks to intercept network traffic between mining pools or large nodes, partitioning the network or creating a man-in-the-middle position. Documented against Bitcoin in 2017-2018 (Apostolaki et al., "Hijacking Bitcoin"). `[Established]`

#### Time Warp Attack (PoW)

The attacker manipulates block timestamps to accelerate difficulty retargeting, allowing blocks to be mined much faster. Bitcoin is theoretically vulnerable if an attacker controls 50%+ hash power; mitigated by Median Time Past rule and timestamp bounds. Some alt-chains have suffered time warp exploits. `[Established risk; mitigated in Bitcoin]`

#### Feather Forking (Economic Censorship)

A minority miner threatens to orphan any block containing specific transactions (e.g., from blacklisted addresses). If the minority is large enough (>33% hash power), other miners may comply to avoid having their blocks orphaned. `[Established theoretical attack; not documented in practice on Bitcoin mainnet]`

#### Grinding Attack (PoS)

An attacker iterates over many candidate blocks or VRF inputs to find one that selects them (or their allies) as future validators, gaining disproportionate influence. VRF-based protocols must ensure the seed for future selection is not predictable or malleable. `[Established risk; most PoS designs include mitigation]`

#### Dust Attack (Privacy Attack on Bitcoin)

An attacker sends tiny amounts of Bitcoin (dust) to a large number of addresses, then monitors which addresses consolidate the dust in future transactions. This allows the attacker to link multiple addresses to the same wallet. `[Established — documented attack used by chain analysis firms]`

#### Cryptographic Attack (Quantum Computing)

A large-scale quantum computer running Shor's algorithm could compute discrete logarithms and factor integers, breaking ECDSA, Schnorr, RSA, and similar schemes. All existing cryptocurrency private keys derived from exposed public keys would be at risk. Address types that never expose the public key (hash-of-public-key addresses where coins have never been spent) have an additional layer of protection.

NIST has standardised several post-quantum signature algorithms (CRYSTALS-Dilithium, FALCON, SPHINCS+) as of 2024. Bitcoin and Ethereum migration paths are under discussion but not yet scheduled. `[Established risk; uncertain timeline; NIST standardisation confirmed 2024]`

#### P+ε Attack (PoS Bribery Attack)

An attacker offers to pay validators slightly more than ε (epsilon) to vote dishonestly in a one-time coordination. If enough validators believe others will also defect, they all defect. This game-theoretic attack targets the assumption that PoS validators act honestly. **Mitigations:** Coordination is hard in practice; slashing removes the economic benefit for the attacker since the attack reveals itself. `[Established theoretical risk; not documented in practice]`

---

### Part 11 Review Questions

**Q1. What is the difference between an optimistic rollup and a ZK-rollup in terms of security model and withdrawal time?**
A: Optimistic: assumes validity; relies on fraud proofs during a 7-day challenge period; 7-day withdrawal delay. ZK: cryptographically proves validity; instant finality once proof verified on L1; near-instant withdrawal. Optimistic is easier to build; ZK is more secure and final.

**Q2. What is a sandwich attack in MEV, and who is harmed?**
A: Attacker front-runs a large user swap (buying before) and back-runs it (selling after), profiting from the price impact the user causes. The user receives a worse execution price — effectively taxed by the MEV bot.

**Q3. What is data availability and why is it critical for rollups?**
A: Data availability is the guarantee that all data needed to verify or reconstruct a chain's state is available on-chain (or can be sampled). Rollups post their transaction data to L1 (or DA layer) so that any observer can verify or fraud-prove the rollup state. Without it, an operator could withhold data and prevent users from exiting.

**Q4. What is a long-range attack on PoS and how is it mitigated?**
A: An attacker uses old private keys to rewrite history from a past epoch at zero cost (old stake has no present value). Mitigated by weak subjectivity checkpoints (nodes refuse reorgs past a checkpoint), key-evolving signatures, and social coordination to identify the legitimate chain.

**Q5. How does quantum computing threaten all existing cryptocurrency private keys, and what is the first line of defence?**
A: Shor's algorithm on a sufficiently large quantum computer can recover a private key from a public key. The first line of defence is address types where the public key is never exposed (hash-of-public-key receiving addresses where coins remain unspent). Longer-term: migration to post-quantum signature schemes.

### Common Misconceptions

- **"Rollups are always safer than the L1."** ZK-rollups with well-audited proofs approach L1 security. But rollup code, bridge code, and sequencer choices introduce their own risks. Many rollup bridges have been exploited.
- **"MEV is only a minor inconvenience."** MEV is a systemic economic problem that redistributes tens of millions of dollars monthly from ordinary users to sophisticated bots and validators.
- **"Sharding solves all scalability problems."** Sharding adds throughput but introduces cross-shard complexity, data availability challenges, and potentially weakens per-shard security.

### After Part 11 You Should Be Able To

Explain how Lightning Network, optimistic rollups, and ZK-rollups differ in their security models. Define MEV and name three types. Explain the data availability problem. Describe a long-range attack and its mitigations. Name at least five attack vectors against blockchain systems.

### Sources — Part 11

| Source | Date | Type |
|---|---|---|
| Daian et al., "Flash Boys 2.0: Frontrunning in Decentralized Exchanges" — IEEE S&P 2020 | 2019 | Primary |
| Poon & Dryja, "The Bitcoin Lightning Network" — lightning.network/lightning-network-paper.pdf | 2016 | Primary |
| Buterin & Poon, "Plasma: Scalable Autonomous Smart Contracts" | 2017 | Primary |
| EIP-4844 (Proto-Danksharding) — eips.ethereum.org/EIPS/eip-4844 | 2022 | Primary |
| Apostolaki et al., "Hijacking Bitcoin: Routing Attacks on Cryptocurrencies" — IEEE S&P 2017 | 2017 | Primary |
| Neutrino: BIP-157, BIP-158 — github.com/bitcoin/bips | 2017 | Primary |
| ethereum.org/roadmap/danksharding | Ongoing | Documentation |
| NIST, "Post-Quantum Cryptography Standardization" — csrc.nist.gov/projects/post-quantum-cryptography | 2024 | Primary (standard) |
| Flashbots MEV data — explore.flashbots.net | Ongoing | Secondary |


---

## Glossary

Terms are listed alphabetically. First use in the main text is marked in bold where defined.

**51% Attack** — An attack where a single entity controls more than 50% of a Proof-of-Work network's hash rate, enabling double-spends and transaction censorship. Cannot create coins or steal others' funds.

**Account model** — A ledger representation where each address has an explicit balance. Used by Ethereum. Compare: UTXO model.

**ASIC (Application-Specific Integrated Circuit)** — Microchip designed for a single computation (e.g., SHA-256 hashing). Far more efficient than CPUs or GPUs at the target function.

**ASIC resistance** — Property of a PoW algorithm designed to prevent ASICs from having a large efficiency advantage over consumer hardware. See Part 6.5.

**Autolykos v2** — Memory-hard PoW algorithm used by Ergo (ERG). ~2.25 GB memory requirement. See Part 6.4.

**Avalanche consensus** — A family of probabilistic consensus protocols using repeated random subsampling. Used by the Avalanche (AVAX) network. See Part 5.8.

**Base fee** — In Ethereum post-EIP-1559: the minimum fee per gas unit, set algorithmically by the protocol and burnt (not paid to validators).

**Bitcoin Improvement Proposal (BIP)** — The formal process for proposing changes to Bitcoin's protocol. BIPs are numbered documents; accepted BIPs become part of the protocol.

**Block** — A data structure containing a header and a list of transactions. The header commits (via hash) to the previous block and the Merkle root of all transactions.

**Block explorer** — A website or tool that displays blockchain data: transactions, blocks, addresses, and statistics.

**Block reward** — The total payment to a block producer = block subsidy + transaction fees.

**Block subsidy** — The newly created coins awarded to the miner of a valid block. Bitcoin's current subsidy is 3.125 BTC (after April 2024 halving). Monero's is ~0.6 XMR (tail emission).

**Blockchain** — A data structure of cryptographically linked blocks. Each block header includes a hash of the previous block, making history tamper-evident.

**BLS signature** — Boneh-Lynn-Shacham signature scheme. Allows multiple signatures to be aggregated into one, reducing bandwidth. Used by Ethereum validators.

**Bulletproofs** — Short, non-interactive zero-knowledge range proofs. Used in Monero to prove transaction amounts are non-negative without revealing them.

**Bulletproofs+** — Improved variant of Bulletproofs deployed in Monero August 2022. ~5-7% smaller proofs and faster verification.

**Byzantine fault** — A failure mode where a node sends conflicting or malicious messages (not simply crashes). More general than a crash fault.

**Byzantine Fault Tolerance (BFT)** — Property of a distributed system that continues operating correctly despite Byzantine failures. Requires n ≥ 3f + 1 nodes to tolerate f Byzantine faults.

**Byzantine Generals Problem** — A distributed computing thought experiment illustrating the difficulty of reaching consensus among nodes where some may be malicious. Formalised by Lamport et al., 1982.

**CAP Theorem** — States that a distributed system can guarantee at most two of Consistency, Availability, Partition Tolerance. Proved by Gilbert and Lynch, 2002.

**Casper FFG** — "Friendly Finality Gadget": Ethereum's BFT finality layer. Provides deterministic finality in epochs (~12.8 minutes) via 2/3 validator quorums.

**Chain ID** — A unique identifier for a blockchain, included in Ethereum transaction signatures. Prevents replay attacks across chains with the same transaction history.

**Chainwork** — The sum of Proof of Work across all blocks in a chain, expressed as a 256-bit integer. Used in Bitcoin's fork choice rule (heaviest chain = most chainwork).

**CoinJoin** — A technique for Bitcoin privacy where multiple parties combine their transactions into one, obfuscating which inputs correspond to which outputs.

**Coinbase transaction** — The first transaction in every block. It has no inputs and creates new coins (block subsidy). The coinbase field can contain arbitrary data.

**Coinbase maturity** — The rule that newly minted coins from a coinbase transaction cannot be spent for a specified number of blocks (100 for Bitcoin, 60 for Monero).

**CLSAG (Concise Linkable Spontaneous Anonymous Group)** — The ring signature scheme used in Monero since October 2020. More efficient than its predecessor MLSAG.

**CryptoNote** — The 2013 protocol specification introducing ring signatures and stealth addresses. The foundation of Monero, Bytecoin, and related coins.

**Dandelion++** — A privacy-preserving transaction relay protocol. Transactions propagate through a "stem" of nodes before flooding the network, hiding the originator's IP address.

**DAG (Directed Acyclic Graph)** — A graph where edges have direction and no cycles. Used in some blockchain designs (GHOSTDAG, Narwhal) to process multiple blocks simultaneously.

**Data Availability** — The property that all data needed to verify or reconstruct a chain's state is accessible to network participants. Critical for rollup security.

**DPoS (Delegated Proof of Stake)** — A consensus mechanism where token holders vote for a fixed set of block producers who take turns producing blocks.

**Difficulty** — In PoW: the inverse of the probability that a single hash attempt produces a valid block hash. Adjusted periodically to maintain the target block interval.

**Double-spend** — Spending the same input twice. The fundamental problem digital cash must prevent.

**Dust** — Extremely small amounts of cryptocurrency that are economically unspendable (fee > value).

**ECDSA** — Elliptic Curve Digital Signature Algorithm. Used in Bitcoin (pre-Taproot), Ethereum, and many other chains.

**Ed25519** — A Schnorr-style digital signature scheme on the twisted Edwards curve Curve25519. Used in Monero for spend key signatures.

**EIP (Ethereum Improvement Proposal)** — The formal process for proposing protocol changes to Ethereum, analogous to Bitcoin's BIPs.

**Elliptic curve** — A mathematical curve used as the basis for public-key cryptography. Bitcoin uses secp256k1 (y² = x³ + 7 mod p); Ethereum also uses secp256k1; Monero uses ed25519 and curve25519 variants.

**Emission curve** — The schedule of coin issuance over time. Bitcoin uses discrete halvings; Monero uses a smooth exponential decay then constant tail emission.

**EVM (Ethereum Virtual Machine)** — The sandboxed, deterministic execution engine running smart contract bytecode on Ethereum and EVM-compatible chains.

**FCMP++ (Full-Chain Membership Proofs++)** — Monero's planned upgrade that proves an output belongs to the full blockchain output set rather than a small ring. Massively increases anonymity set.

**Fee** — Payment to block producers for including a transaction. Implicit in Bitcoin (inputs - outputs); explicit in Ethereum (gas × gasPrice).

**Finality** — The irreversibility of a confirmed transaction. Probabilistic (PoW): probability decreases with confirmations. Deterministic (BFT): once committed, cannot be reversed.

**FLP Impossibility** — Fischer, Lynch, Paterson 1985: in a purely asynchronous system, no deterministic consensus protocol can tolerate even one crash fault. Fundamental limit of distributed systems.

**Fork** — (1) Protocol fork: a change to consensus rules. Soft fork: backwards-compatible. Hard fork: incompatible, may split the chain. (2) Chain fork: two valid blocks at the same height; resolved by fork-choice rule.

**Full node** — A node that downloads, validates, and stores all blocks and transactions from genesis, enforcing all consensus rules independently.

**Fungibility** — The property that all units of a currency are interchangeable and of equal value regardless of history.

**Gas** — EVM's unit of computational cost. Each opcode has a gas cost; transactions must specify a gas limit and gas price.

**Gasper** — Ethereum's consensus mechanism: LMD-GHOST fork choice + Casper FFG finality gadget.

**Genesis block** — Block 0: the first block of a blockchain, hardcoded into the client software.

**GHOST (Greedy Heaviest Observed Subtree)** — A fork-choice rule that counts all descendant blocks (including uncles), not just the main chain. The basis for Ethereum's LMD-GHOST.

**GHOSTDAG** — A generalisation of GHOST to a block-DAG. Used in Kaspa for high-throughput concurrent block production.

**Hard fork** — A protocol change that makes previously invalid blocks/transactions valid. Incompatible with old nodes; may split the chain.

**Hash function** — A function mapping arbitrary-length input to a fixed-length output, with properties: determinism, one-wayness, avalanche effect, collision resistance.

**Hashcash** — Adam Back's 1997 PoW system for email spam prevention. The direct precursor to Bitcoin's PoW.

**Hash rate** — The number of hash computations per second performed by a miner or the entire network.

**HotStuff** — A BFT consensus protocol with linear communication complexity, using quorum certificates. Basis for many modern permissioned and PoS systems.

**Key image** — In Monero ring signatures: a unique curve point derived from an output's private key. Used to prevent double-spending by detecting reuse.

**kHeavyHash** — A lightweight PoW algorithm used by Kaspa. See Part 6.4.

**Layer 1 (L1)** — The base blockchain (Bitcoin, Ethereum, Monero). Settlement and security layer.

**Layer 2 (L2)** — A protocol built on top of L1 to increase throughput (Lightning, rollups, state channels).

**Light client / SPV client** — A node that downloads only block headers and uses Merkle proofs to verify specific transactions without full chain validation.

**Lightning Network** — Bitcoin's principal Layer-2 payment channel network. Enables instant, low-fee payments off-chain.

**LMD-GHOST** — Latest-Message Driven GHOST: Ethereum's fork-choice rule. Each validator's latest attestation counts once; the fork with the most attestation weight is chosen.

**Locktime** — A Bitcoin transaction field specifying the earliest block height or time after which the transaction may be included in a block.

**Long-range attack** — A PoS attack using old private keys to rewrite blockchain history from a past epoch. Mitigated by checkpointing and weak subjectivity.

**Masternode** — A special full node that provides extra services (privacy mixing, instant transactions) and requires a collateral deposit. Used in Dash.

**Median Time Past (MTP)** — The median timestamp of the previous 11 blocks. Bitcoin requires a new block's timestamp to exceed MTP.

**Mempool (memory pool)** — Each node's local set of valid but unconfirmed transactions waiting for inclusion in a block.

**Merkle root** — The root hash of a Merkle tree committing to all transactions in a block. Included in the block header.

**Merkle tree** — A binary hash tree where leaf nodes are transaction hashes and parent nodes are hashes of their children. Enables efficient inclusion proofs.

**MEV (Maximal Extractable Value)** — Value extracted by block producers through transaction reordering, insertion, or censorship beyond normal rewards. See Part 11.2.

**Mining** — The process of repeatedly hashing a block header with varying nonces until the output is below the difficulty target. Earns the block reward.

**Mining pool** — A group of miners combining hash power and sharing rewards proportionally, reducing variance.

**MLSAG** — Multi-Layered Linkable Spontaneous Anonymous Group signature. Monero's ring signature predecessor to CLSAG.

**Move VM** — A virtual machine executing Move language programs; used by Aptos and Sui. Features linear types preventing asset duplication.

**Nakamoto consensus** — Bitcoin's consensus mechanism: Proof of Work + heaviest-chain rule + probabilistic finality.

**Nonce (block)** — A 32-bit value in Bitcoin's block header varied by miners to find a valid PoW hash.

**Nonce (transaction)** — In Ethereum: a per-account counter that prevents replay attacks and enforces transaction ordering.

**Nothing-at-stake** — A PoS problem where validators can vote on multiple forks at zero cost (no hardware committed). Addressed by slashing.

**Oracle** — A service that brings off-chain data (prices, events) on-chain via signed reports, used by smart contracts.

**Orphan block / Stale block** — A valid block that was not extended by the network because another block at the same height accumulated more chainwork.

**Ouroboros** — Cardano's PoS consensus protocol. First PoS protocol with formal security proof (Kiayias et al., 2017).

**PBFT (Practical Byzantine Fault Tolerance)** — Classical BFT protocol by Castro & Liskov (1999). Three-phase commit, O(n²) communication.

**Pedersen commitment** — A cryptographic commitment scheme: C = v·H + r·G. Used in Monero to hide transaction amounts while allowing balance verification.

**Plasma** — An early Ethereum L2 design using hierarchical chains and exit games. Largely superseded by rollups.

**PoA (Proof of Authority)** — Consensus where block production rights are granted to known, identity-verified validators. Permissioned; not Sybil-resistant in a permissionless sense.

**PoET (Proof of Elapsed Time)** — Intel SGX-based consensus for Hyperledger Sawtooth. Fairness relies entirely on hardware integrity.

**PoH (Proof of History)** — Solana's cryptographic clock: a recursive VRF that proves real time elapsed. Not consensus itself; combined with Tower BFT.

**PoS (Proof of Stake)** — Consensus mechanism where validator selection probability is proportional to staked coins rather than computational work.

**PoSpace+Time (Proof of Space and Time)** — Chia's consensus. Farmers allocate disk space; a VDF enforces minimum time between blocks.

**PoW (Proof of Work)** — Consensus mechanism requiring miners to solve computationally expensive puzzles. The solution (a nonce) is cheap to verify but hard to find.

**Private key** — A secret random number from which a public key is derived via scalar multiplication on an elliptic curve. Must never be revealed.

**Probabilistic finality** — Finality where the probability of reversal decreases with each additional confirmation but is never strictly zero (Nakamoto consensus).

**Proposer-Builder Separation (PBS)** — Separation of Ethereum's block construction (builder) from block proposal (proposer) roles, designed to externalise MEV.

**Public key** — The result of scalar multiplication: Q = k·G. Can be freely shared; used to verify signatures and derive addresses.

**RandomX** — Monero's current PoW algorithm. Runs random programs on a CPU-like virtual machine. Memory-intensive (2 GB dataset). Resistant to ASICs (no confirmed ASICs as of November 2024).

**Range proof** — A zero-knowledge proof that a committed value lies within a valid range (e.g., [0, 2^64 − 1]). Used in RingCT to prevent negative-value inflation attacks.

**Relay** — (1) In Bitcoin/Monero: a node that propagates transactions and blocks without necessarily mining. (2) In cross-chain: a service that passes messages between chains.

**Reorganisation (reorg)** — When a node switches from one chain to another that has accumulated more work. Transactions in orphaned blocks return to the mempool.

**Replace-by-Fee (RBF)** — Bitcoin mechanism allowing a sender to replace an unconfirmed transaction with a higher-fee version.

**RingCT (Ring Confidential Transactions)** — Monero protocol combining ring signatures with Pedersen commitments and range proofs to hide transaction amounts. Deployed January 2017.

**Ring signature** — A signature by one member of a group (ring) that proves someone in the group signed, without revealing which member.

**Rollup** — An L2 that executes transactions off-chain and posts data (or validity proofs) to L1. Optimistic or ZK variants.

**Schnorr signature** — A digital signature scheme simpler and more efficient than ECDSA. Natively aggregatable. Used in Bitcoin post-Taproot.

**Script (Bitcoin)** — Bitcoin's stack-based, non-Turing-complete transaction scripting language.

**Searcher** — In MEV: a bot that scans the mempool for profitable transaction ordering opportunities and pays builders to include them in advantageous positions.

**Secp256k1** — The elliptic curve y² = x³ + 7 (mod p) used by Bitcoin and Ethereum for key generation and signatures.

**Segregated Witness (SegWit)** — Bitcoin soft fork (2017) that moves signature data to a separate "witness" structure. Fixes transaction malleability; increases effective block capacity.

**Sequencer** — In an L2 rollup: the node(s) that order transactions and post batches to L1. Currently centralised in most deployments.

**SHA-256** — Secure Hash Algorithm producing a 256-bit output. Used in Bitcoin's PoW (double SHA-256) and transaction IDs.

**Slashing** — Automatic, irreversible penalisation of a PoS validator for provably dishonest behaviour (double-signing, surrogate voting).

**Smart contract** — Self-executing code deployed on a blockchain. Runs deterministically on the EVM (or other VM). Cannot be modified after deployment (usually).

**Soft fork** — A backwards-compatible consensus rule tightening. Old nodes accept blocks valid under new rules; new nodes reject blocks violating them.

**Stealth address** — A one-time address derived from the recipient's public key and a random sender nonce. Each payment produces a unique address, preventing recipient linkability.

**Subaddress** — In Monero: a secondary address derived from the master wallet without exposing the master view key. Allows multiple independent-looking receive addresses.

**Tail emission** — Monero's permanent minimum block reward of 0.6 XMR per block after the main emission ended (~May 2022).

**Taproot** — Bitcoin soft fork (November 2021, BIP 340-342) adding Schnorr signatures, MAST (Merkelised Abstract Syntax Tree), and Tapscript.

**Tendermint / CometBFT** — BFT consensus protocol used by Cosmos SDK chains. Two-phase (prevote/precommit) with deterministic per-block finality.

**Transaction** — A signed instruction that changes the state of the ledger. Consists of inputs (or account data), outputs, and a signature.

**Transaction malleability** — The ability to alter a transaction's bytes without invalidating its signature, changing its TXID. Fixed in Bitcoin by SegWit.

**TXID (Transaction Identifier)** — The double-SHA256 hash of a serialised transaction. Uniquely identifies a transaction on the blockchain.

**UTXO (Unspent Transaction Output)** — A discrete output of a previous transaction that has not yet been spent. The primitive unit of value in Bitcoin.

**UTXO set** — The set of all currently unspent transaction outputs. Maintained by full nodes to detect double-spends.

**Validator** — In PoS chains: a node that participates in consensus by proposing and attesting to blocks, backed by staked coins.

**Verkle tree** — A cryptographic data structure using polynomial commitments (IPA/KZG). Smaller inclusion proofs than Merkle Patricia Tries. Planned for Ethereum state to enable stateless clients.

**View key** — In Monero: a key that lets the holder see incoming transactions to a wallet without spending them. Enables selective disclosure for auditing.

**VRF (Verifiable Random Function)** — A pseudorandom function whose output includes a cryptographic proof of correctness. Used for unbiasable, unpredictable leader selection in PoS protocols.

**Weak subjectivity** — The property of PoS systems where a node syncing from scratch cannot determine the canonical chain using only the protocol rules — it must start from a trusted checkpoint.

**Witness** — In Bitcoin SegWit: the signature and other unlocking data stored separately from the main transaction body. In stateless Ethereum: Merkle/Verkle proofs of all state accessed by a block.

**ZK-SNARK (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge)** — A cryptographic proof that a statement is true without revealing why it is true. Used in Zcash (Groth16) and zkEVMs.

**ZK-STARK (Zero-Knowledge Scalable Transparent Argument of Knowledge)** — A ZK proof system without a trusted setup. Larger than SNARKs but transparent. Used by StarkNet.

**Zero-knowledge proof** — A proof that lets a prover convince a verifier of a statement without revealing any information beyond the truth of the statement.

---

## Recommended Reading

Ordered from beginner to research level. Primary sources are listed first within each level; secondary sources follow.

### Level 1: Foundations (No Technical Background Required)

| Title | Author(s) | Type | Note |
|---|---|---|---|
| *Bitcoin: A Peer-to-Peer Electronic Cash System* (bitcoin.org/bitcoin.pdf) | Satoshi Nakamoto | Primary (whitepaper) | 9 pages; deceptively readable; start here |
| *The Internet of Money* (Vols. 1–3) | Andreas Antonopoulos | Secondary (book) | Excellent conceptual foundation; non-technical |
| Hashcash: A Denial of Service Counter-Measure (hashcash.org/papers/hashcash.pdf) | Adam Back | Primary | PoW precursor; short |

### Level 2: Technical Foundations

| Title | Author(s) | Type | Note |
|---|---|---|---|
| *Mastering Bitcoin* (3rd ed., github.com/bitcoinbook) | Andreas Antonopoulos, David Harding | Secondary (book) | Best single reference for Bitcoin technical details |
| *Programming Bitcoin* | Jimmy Song | Secondary (book) | Build Bitcoin from scratch; hands-on |
| Ethereum Yellow Paper (ethereum.github.io/yellowpaper) | Gavin Wood | Primary | Formal Ethereum spec; dense but authoritative |
| *Mastering Ethereum* (github.com/ethereumbook) | Antonopoulos, Wood | Secondary (book) | Ethereum equivalent of Mastering Bitcoin |
| *Zero to Monero* (web.getmonero.org/library/Zero-to-Monero-2-0-0.pdf) | Alonso, Noether | Primary (technical book) | Monero cryptography from first principles; rigorous |

### Level 3: Consensus and Distributed Systems

| Title | Author(s) | Type | Note |
|---|---|---|---|
| "The Byzantine Generals Problem" — ACM TOPLAS | Lamport, Shostak, Pease | Primary | 1982; foundational |
| "Practical Byzantine Fault Tolerance" — OSDI 1999 | Castro, Liskov | Primary | PBFT; still widely cited |
| "HotStuff: BFT Consensus with Linearity and Responsiveness" — PODC 2019 | Yin, Malkhi, Reiter, Gueta, Abraham | Primary | Modern BFT standard |
| "Ouroboros: A Provably Secure PoS Blockchain Protocol" — CRYPTO 2017 | Kiayias, Russell, David, Oliynykov | Primary | First formally secure PoS |
| "Snowflake to Avalanche" (ipfs.io/ipfs/QmUy...) | Team Rocket | Primary | Avalanche consensus |
| Decentralized Thoughts blog (decentralizedthoughts.github.io) | Malkhi, Raichman et al. | Secondary (blog) | Excellent deep-dives on BFT and consensus |

### Level 4: Proof of Work and Mining

| Title | Author(s) | Type | Note |
|---|---|---|---|
| RandomX specification (github.com/tevador/RandomX/blob/master/doc/specs.md) | tevador | Primary | The RandomX VM specification |
| "Equihash" — NDSS 2017 | Biryukov, Khovratovich | Primary | Memory-hard PoW |
| "Majority is Not Enough: Bitcoin Mining is Vulnerable" — FC 2014 | Eyal, Sirer | Primary | Selfish mining |
| "PHANTOM GHOSTDAG" (eprint.iacr.org/2018/104) | Sompolinsky, Zohar | Primary | GHOSTDAG for Kaspa |
| Difficulty algorithm library (github.com/zawy12/difficulty-algorithms) | zawy12 | Primary | Reference for all difficulty algorithms |

### Level 5: Privacy and Cryptography

| Title | Author(s) | Type | Note |
|---|---|---|---|
| CryptoNote whitepaper (cryptonote.org/whitepaper.pdf) | Nicolas van Saberhagen | Primary | Foundation of Monero |
| "Ring Confidential Transactions" — MRL-0005 (lab.getmonero.org) | Noether, Mackenzie, MRL | Primary | RingCT |
| "Bulletproofs" — IEEE S&P 2018 | Bünz, Bootle, Boneh et al. | Primary | Range proofs |
| Monero Research Lab publications (lab.getmonero.org) | Various | Primary | Ongoing research |
| *A Graduate Course in Applied Cryptography* (cryptobook.us) | Boneh, Shoup | Secondary (textbook) | Free; comprehensive cryptography reference |
| "An Empirical Analysis of Traceability in the Monero Blockchain" — PoPETs 2018 | Möser et al. | Secondary (academic) | Seminal traceability analysis |

### Level 6: Advanced Topics and Scaling

| Title | Author(s) | Type | Note |
|---|---|---|---|
| "Flash Boys 2.0" — IEEE S&P 2020 | Daian, Goldfeder et al. | Primary | MEV foundational paper |
| EIP-4844 (eips.ethereum.org/EIPS/eip-4844) | Buterin et al. | Primary | Proto-Danksharding |
| ethereum.org/roadmap | Ethereum Foundation | Documentation | Danksharding and Ethereum future |
| "Hijacking Bitcoin" — IEEE S&P 2017 | Apostolaki, Zohar, Vanbever | Primary | BGP attacks on Bitcoin |
| Flashbots research (writings.flashbots.net) | Flashbots | Secondary | MEV in depth |
| *Principles of Distributed Computing* (disco.ethz.ch/courses/podc) | Wattenhofer | Secondary (textbook) | Foundations of distributed systems |

### Level 7: Primary Standards and Specifications

| Resource | Note |
|---|---|
| Bitcoin BIPs (github.com/bitcoin/bips) | All Bitcoin protocol changes |
| Ethereum EIPs (eips.ethereum.org) | All Ethereum protocol changes |
| NIST FIPS 180-4 (SHA-256), FIPS 186-5 (ECDSA), FIPS 204-206 (Post-quantum) | Cryptographic standards |
| SEC 2: secp256k1 specification (secg.org/SEC2-Ver-1.0.pdf) | Elliptic curve parameters |
| Monero Technical Specification (docs.getmonero.org/technical-specs/) | Monero parameters |
| RandomX specification (github.com/tevador/RandomX/blob/master/doc/specs.md) | RandomX VM |

---

*End of document.*

*Document compiled November 2024. Live network statistics (hashrate, difficulty, exchange listings, protocol upgrade status) change rapidly. Always verify current figures from primary sources before relying on them.*

*All fact labels apply at the date of research. `[Established]` = well-sourced and uncontroversial as of November 2024. `[Disputed]` = contested in the technical community with reasonable arguments on both sides. `[Uncertain]` = limited evidence or the author is not certain — treat with caution and verify independently.*

