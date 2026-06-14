# Turbin3 Typescript Assignment

- Keypair Generation: Create a new Solana keypair. (part 1)
  
  [Transaction](https://explorer.solana.com/tx/5CRi9fxX7jwCq2KFQ2N2Hp9DwHV7nBb2L8tCgxPR8oRyQTLzZqiB9XxeD88E57M1mY4yyH7HNy3wV2arZ2n6Ws8n?cluster=devnet )
- Airdrop Request: Claim SOL from the devnet for testing. (part 2)

  [Transaction](https://explorer.solana.com/tx/4syKzhxDEizdvrX9cnQYSZvycXaPeKGEZKe29fdhDRMXpiHqD6SvfW7SyVFsNRhfhnJQtDBuhb4MNpY4UktoZ3b9?cluster=devnet)
- Token Transfer: Transfer SOL to another wallet. (part 3)

  [Transaction](https://explorer.solana.com/tx/WuzzoXAhp7H215dnJy47Nz8xBJgbvetBboVHctLHatzMQoFULKxa9wB9MVpXn1WEmwVaqBbtdwf191TJEp6zyyg?cluster=devnet)
- Program Interaction: Interact with the Anchor IDL program created by WBA on the devnet. (part 4)
  
  [Transaction](https://explorer.solana.com/tx/tDZz54Vhtiq45fijwEmzpFVPZqGj4prDtwhT9JnwCtERdjwiRSo5R33CEnUwXZj381fhvNqbqXsvWyc7JMkceJS?cluster=devnet)



-  Set Up Project:
  
    - **Clone the Project**
      
       ```bash
   
       git clone https://github.com/AvhiMaz/turbin-assignment
   
       ```
    - **Change Directory**
      
       ```bash
   
       cd turbin-assignment && cd airdrop
   
        ```
   - **Install Required Dependencies**
     
       ```bash
   
       yarn init -y
   
       ```

- Run Scripts:
  
   - **Generate Keypair**:
  
     ```bash
     yarn keygen
     ```
   - **Claim Airdrop**:
     
     ```bash
     yarn airdrop
     ```

   - **Transfer SOL**:
     
     ```bash
     yarn transfer
     ```
     
   - **Interact with WBA IDL**:
     
     ```bash
     yarn enroll
     ```
