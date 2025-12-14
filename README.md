# Prediction Market Contract

A Solana prediction market smart contract built using the Anchor framework.

This program allows users to:
- Create prediction markets
- Split and merge outcome tokens
- Settle markets based on the winning outcome
- Claim rewards after settlement



---

## Features

- Initialize a prediction market
- Split tokens into outcome-specific positions
- Merge outcome tokens back
- Settle the market with a winning outcome
- Claim rewards after settlement

---

## Instructions

The program exposes the following instructions:

- **init_market**  
  Initializes a new prediction market.

- **split_tokens**  
  Splits base tokens into outcome-specific tokens.

- **merge_tokens**  
  Merges outcome tokens back into base tokens.

- **settle_market**  
  Finalizes the market by declaring the winning outcome.

- **claim_reward**  
  Allows users to claim rewards after market settlement.

---

## Local Setup

### Prerequisites

- Rust
- Solana CLI
- Anchor

---

### Clone the Repository

```bash
git clone https://github.com/ajeeshRS/prediction-market
cd prediction-market
```

### Build the Program

```
anchor build
```

### Deploy the Program

```
anchor deploy
```

## Security Notes

> ⚠️ **Note:** Tests are **not implemented yet**.
- This contract is experimental
- Not audited
- Do not deploy to mainnet without proper testing and auditing

## License

MIT
