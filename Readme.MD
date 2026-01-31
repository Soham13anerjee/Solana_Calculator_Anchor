# 🧮 Solana Calci (PDA Version)

A decentralized calculator program built on Solana using the **Anchor Framework**. This project demonstrates the implementation of Program Derived Addresses (PDAs), state management, and custom error handling on-chain.

## 🚀 Overview

The **Calci** program allows users to initialize a personal calculator state on the blockchain. Every calculation is stored in an account owned by the program, specifically derived from the user's wallet address to ensure data isolation.

### Key Features

* **PDA State:** Uses seeds (`[b"calci", fee_payer]`) to derive a unique account for every user.
* **Core Math:** Supports `Addition`, `Subtraction`, and `Division`.
* **Security:** Includes a `require!` macro to prevent **Division by Zero**.
* **Ownership:** Stores the `payer`'s public key within the account to track ownership.

---

## 🧠 What I Learned

Building this project provided a deep dive into the Solana ecosystem. Key takeaways include:

### 1. The Solana Account Model

I learned that in Solana, everything is an account. Unlike Ethereum’s global state, Solana programs are "stateless"—they logic resides in a Program Account, while data is stored in separate Data Accounts.

### 2. Program Derived Addresses (PDAs)

I implemented PDAs to map a user's wallet to a specific calculator account.

* **Seeds:** Used `b"calci"` and the user's `Pubkey` as seeds.
* **Deterministic:** Learned that PDAs do not have a private key; they are "signed" for by the program using the seeds and a `bump`.

### 3. Anchor Framework Essentials

* **Macros:** Used `#[program]`, `#[derive(Accounts)]`, and `#[account]` to reduce boilerplate.
* **InitSpace:** Utilized `#[derive(InitSpace)]` to automatically calculate the space required for accounts, preventing "out of bounds" errors.
* **Error Handling:** Implemented a custom `ErrorCode` enum and the `require!` macro for validation logic.

### 4. Testing with Anchor & TypeScript

* **BigNumber (BN):** Handled 64-bit integers (`i64`) using the `BN` library in TypeScript.
* **PDA Resolution:** Mastered Anchor's ability to automatically resolve PDA accounts in tests.
* **Asynchronous Testing:** Wrote test suites using `Mocha` and `Chai` to verify on-chain state and catch expected errors.

---

## 🛠 Project Structure

```text
.
├── programs/
│   └── calci/
│       └── src/lib.rs        # Main Rust program logic
├── tests/
│   └── calci.ts              # TypeScript integration tests
├── Anchor.toml               # Program configuration & IDs
└── package.json              # Node dependencies

```

---

## 🧪 Running the Tests

1. **Install Dependencies:**
```bash
yarn install

```


2. **Build the Program:**
```bash
anchor build

```


3. **Run the Test Suite:**
```bash
anchor test

```



---

## 📝 Example Code Snippet (Division with Error Handling)

```rust
pub fn div(ctx: Context<Div>, a: i64, b: i64) -> Result<()> {
    // Custom error check
    require!(b != 0, Errorcode::DivisionByZero);
    
    ctx.accounts.calci_acc.calci_result = a / b;
    Ok(())
}

```

---

## 🎓 Acknowledgments

Special thanks to the **Code Eater** Solana Mastery course for the step-by-step guidance on Anchor and the Solana Account Model.

---
