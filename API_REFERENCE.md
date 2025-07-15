# Anya Core API Reference

Welcome to the Anya Core API Reference. This document provides a detailed description of the API endpoints available in the Anya Core application.

## Table of Contents

- [Anya Core API Reference](#anya-core-api-reference)
  - [Table of Contents](#table-of-contents)
  - [1. Introduction](#1-introduction)
  - [2. Authentication](#2-authentication)
  - [3. Public Endpoints](#3-public-endpoints)
    - [3.1. Health Check](#31-health-check)
    - [3.2. System Information](#32-system-information)
    - [3.3. Login](#33-login)
  - [4. Wallet Endpoints](#4-wallet-endpoints)
    - [4.1. Create Wallet](#41-create-wallet)
    - [4.2. Get Wallet](#42-get-wallet)
    - [4.3. Get Balance](#43-get-balance)
    - [4.4. Generate Address](#44-generate-address)
    - [4.5. Send Transaction](#45-send-transaction)
    - [4.6. List Transactions](#46-list-transactions)
  - [5. Identity Endpoints](#5-identity-endpoints)
    - [5.1. Create Identity](#51-create-identity)
    - [5.2. Get Identity](#52-get-identity)
    - [5.3. Create Credential](#53-create-credential)
    - [5.4. Get Credential](#54-get-credential)
    - [5.5. Verify Credential](#55-verify-credential)
  - [6. DLC Endpoints](#6-dlc-endpoints)
    - [6.1. Create Contract](#61-create-contract)
    - [6.2. Get Contract](#62-get-contract)
    - [6.3. Accept Contract](#63-accept-contract)
    - [6.4. Finalize Contract](#64-finalize-contract)
    - [6.5. Execute Contract](#65-execute-contract)

## 1. Introduction

The Anya Core API is a RESTful API that allows you to interact with the application's features, including the Bitcoin wallet, decentralized identity management, and Discreet Log Contracts (DLCs).

The base URL for the API is `http://<host>:<port>/api/v1`.

## 2. Authentication

Most of the API endpoints require authentication. To authenticate, you must include a valid JSON Web Token (JWT) in the `Authorization` header of your request:

```
Authorization: Bearer <your-jwt>
```

You can obtain a JWT by calling the `/login` endpoint.

## 3. Public Endpoints

These endpoints are public and do not require authentication.

### 3.1. Health Check

- **Endpoint:** `GET /health`
- **Description:** Checks the health of the application.
- **Response:**
  - `200 OK`: If the application is healthy.

### 3.2. System Information

- **Endpoint:** `GET /info`
- **Description:** Returns information about the system.
- **Response:**
  - `200 OK`: With a JSON object containing system information.

### 3.3. Login

- **Endpoint:** `POST /login`
- **Description:** Authenticates a user and returns a JWT.
- **Request Body:**
  ```json
  {
    "username": "your-username",
    "password": "your-password"
  }
  ```
- **Response:**
  - `200 OK`: With a JSON object containing the JWT.

## 4. Wallet Endpoints

These endpoints require authentication and are used to manage Bitcoin wallets.

### 4.1. Create Wallet

- **Endpoint:** `POST /wallets`
- **Description:** Creates a new Bitcoin wallet.
- **Response:**
  - `201 Created`: With a JSON object containing the new wallet's details.

### 4.2. Get Wallet

- **Endpoint:** `GET /wallets/:id`
- **Description:** Returns the details of a specific wallet.
- **Response:**
  - `200 OK`: With a JSON object containing the wallet's details.

### 4.3. Get Balance

- **Endpoint:** `GET /wallets/:id/balance`
- **Description:** Returns the balance of a specific wallet.
- **Response:**
  - `200 OK`: With a JSON object containing the wallet's balance.

### 4.4. Generate Address

- **Endpoint:** `POST /wallets/:id/address`
- **Description:** Generates a new address for a specific wallet.
- **Response:**
  - `201 Created`: With a JSON object containing the new address.

### 4.5. Send Transaction

- **Endpoint:** `POST /wallets/:id/transactions`
- **Description:** Sends a transaction from a specific wallet.
- **Request Body:**
  ```json
  {
    "recipient": "recipient-address",
    "amount": 100000
  }
  ```
- **Response:**
  - `202 Accepted`: With a JSON object containing the transaction details.

### 4.6. List Transactions

- **Endpoint:** `GET /wallets/:id/transactions`
- **Description:** Returns a list of transactions for a specific wallet.
- **Response:**
  - `200 OK`: With a JSON array of transaction objects.

## 5. Identity Endpoints

These endpoints require authentication and are used to manage decentralized identities.

### 5.1. Create Identity

- **Endpoint:** `POST /identities`
- **Description:** Creates a new decentralized identity.
- **Response:**
  - `201 Created`: With a JSON object containing the new identity's details.

### 5.2. Get Identity

- **Endpoint:** `GET /identities/:id`
- **Description:** Returns the details of a specific identity.
- **Response:**
  - `200 OK`: With a JSON object containing the identity's details.

### 5.3. Create Credential

- **Endpoint:** `POST /credentials`
- **Description:** Creates a new verifiable credential.
- **Response:**
  - `201 Created`: With a JSON object containing the new credential's details.

### 5.4. Get Credential

- **Endpoint:** `GET /credentials/:id`
- **Description:** Returns the details of a specific credential.
- **Response:**
  - `200 OK`: With a JSON object containing the credential's details.

### 5.5. Verify Credential

- **Endpoint:** `POST /credentials/verify`
- **Description:** Verifies a verifiable credential.
- **Response:**
  - `200 OK`: If the credential is valid.

## 6. DLC Endpoints

These endpoints require authentication and are used to manage Discreet Log Contracts.

### 6.1. Create Contract

- **Endpoint:** `POST /dlc`
- **Description:** Creates a new Discreet Log Contract.
- **Response:**
  - `201 Created`: With a JSON object containing the new contract's details.

### 6.2. Get Contract

- **Endpoint:** `GET /dlc/:id`
- **Description:** Returns the details of a specific contract.
- **Response:**
  - `200 OK`: With a JSON object containing the contract's details.

### 6.3. Accept Contract

- **Endpoint:** `POST /dlc/:id/accept`
- **Description:** Accepts a Discreet Log Contract.
- **Response:**
  - `200 OK`: With a JSON object containing the updated contract's details.

### 6.4. Finalize Contract

- **Endpoint:** `POST /dlc/:id/finalize`
- **Description:** Finalizes a Discreet Log Contract.
- **Response:**
  - `200 OK`: With a JSON object containing the updated contract's details.

### 6.5. Execute Contract

- **Endpoint:** `POST /dlc/:id/execute`
- **Description:** Executes a Discreet Log Contract.
- **Response:**
  - `200 OK`: With a JSON object containing the contract's execution details.
