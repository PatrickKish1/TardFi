# TardFi  
**Where Oil market Meets the Blockchain**

---

## Overview  
**TardFi** is a decentralized platform that bridges oil trading with blockchain technology using Real World Assets (RWA), Zero-Knowledge Proofs (ZK), AI, and smart contracts.

---

## Frontend  
- Clean, user-friendly UI  
- Displays:
  - Market listings  
  - Ownership tracking  
  - AI assistant interaction  
  - ZK verification output  

---

## Smart Contracts  

### Real World Assets (RWA)  
- Implemented using **ERC-1155** standard  
- Asset minting, transfers, and rights management  

### Chainlink Integration  
- **CCIP**: Cross-chain interoperability  
- **Automation**: Task scheduling, ownership checks  

### Marketplace  
- Functions:
  - `list()` - List oil-backed asset for sale  
  - `buy()` - Purchase listed asset  
- Requirements:
  - Zero-buying validation logic  
  - Ownership transfer tracking  
  - Rights transfer or resale logic  
  - Sale types: Drop, Raise  

### ZK Verification  
- Zero-Knowledge logic for user and transaction validation  
- Private verification without exposing sensitive data  

---

## ZK  
- Custom **zk-SNARK Circuit**  
- Verifies ownership or eligibility before interacting with sensitive features  

---

## AI  
- **AI-led features**:
  - Store/manage multiple assets across chains  
  - Trader AI bots for oil market participation  
- Identity: *"My middle name is effort."*  

---

## MVP Features  
- AI functionality  
- ZK verification logic  
- Frontend UI  

---

## Team & Tasks  

| Name   | Role              | Responsibility            |
|--------|-------------------|---------------------------|
| Lydia  | Smart Contracts   | RWA, Marketplace, Logic   |
| Ken    | ZK                | Circuit Design, Integration |
| Ken    | AI                | Asset Management, Trading Bots |
