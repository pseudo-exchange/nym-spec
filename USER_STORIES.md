# User Stories

## Auction House Creation
- can pause entire auction house
- can withdraw funds to auction house owners (multisig?)
- can upgrade (if paused)

**TYPE**: `contract deploy`, `call?`

**ADMIN STORY**:
As an admin, i want to create a sexy environment for blockchain domain name auctions. Giving users a place where they can exchange value for vanity names could not be done without a place like nym.near. As an admin, i want to be able to deploy an auction house that allows any user that owns a name to place it up for auction, in the end allowing for me to earn fees from each auction. An auction house consists of some top level controls, fee structure/settings, and a mapping of auctions. Keep it dead simple so users want to trade names like its 1999.

**FUNCTION**: `Constructor`

```rust
// Logic:
// - creates an auction house with empty mapping of auctions
// - started contract as paused: false (but owner can call to change this!)
// - ensure owner has full access keys
// - ensure contract acts as an escrow owner of each auction name item
// - ensure contract keeps Ⓝ balance to cover all auction rewards
// - ensure contract can forward Ⓝ profit balance to multisig owners (TC & ZK)
// 
// Optional:
// - Could return an upgrade contract account ID, in case this auction house is paused to allow FE to check and redirect new auctions dynamically
```

**PARAMS**:

`None`

**RESULT**:

`Success`

**POSSIBLE EXPLOITS**:

* Needs to keep full access keys for owner
* Should not allow anyone (including owner) to auction off the deployed contract name
* Should only allow deploy to upgrade contract if deployed contract is "paused"
* ensure contract does NOT get deployed over -- as all state is wiped, releasing all names

----

## Auction House Viewing Auction Items

**TYPE**: `view`

**USER STORY**:
As a user, i want to view all available name auctions, so I can see if I want to place a bid for any of them. I do not yet have permission to own, edit, destroy or otherwise mutate these items, I only can view the listing. This list is an array of names & IDs such that I know how to place a bid with the information available without any further calls.
I also want to get a list of closed auctions, so I can see what has historically been traded if possible. The main goal is to get a list of active auctions only, but inactive would be nice too :)
As a bonus, I want to be able to get a single listing item by ID, so that if I link or get linked directly to a specific auction item, I can load that data and proceed with bidding.

**ADMIN STORY**:
As an admin, I want to view all listing items the same way a user would. I also want to be able to get a list of all closed auctions so I can see history about my auction house.

**FUNCTION**: `get_auctions`, `get_auctions_by_id`

```rust
// Logic:
// - 
```

**PARAMS**:

`None`, `id`

**RESULT**:

```rust
// TBD: Array of items, Single Item
```

**FEES/ECONOMICS**:
None, viewing items should be free, fast, friendly.

**POSSIBLE EXPLOITS**:

`None` - Assuming view is safe from DOS

----

## Auction Item Creation

**TYPE**: `call`

**USER STORY**:

As a user, i want to create an auction for my cool near name: `too.near`, so that i can turn a profit for claiming this name first. I want to be able to specify when this auction will close, so that I can give other users time to place their bids. I need to be okay with moving my precious name to be owned by the auction house as escrow until the auction finishes or I cancel this auction. I am okay paying a small listing fee. I need to 

**ADMIN STORY**:

As an admin, I want to allow anyone that owns a near account id to place it up for auction. I take a small listing fee as reward for facilitating the txn, and guarantee safe ownership in escrow during auction.

**FUNCTION**: 

```rust
// Logic:
// - User inputs name (Required)
// - User inputs payment name (beneficiary), that receives all funds associated with auctionable name (if any) (Required)
// - User inputs starting bid amount (Optional, Defaults to TBD: 10T GAS?)
// - User inputs closing block (Optional, Defaults to 7days from now amount of blocks, does not override min/max block ranges)
// - Contract: Confirms this name is not already being auctioned (ok if previously auctioned)
// - Contract: Confirms this name is not the same as transaction signer, this would be bad :D
// - Contract: Confirms this name does not have a contract deployed to this name (is that possible??? Should this be allowed???)
// - Contract: Creates new auction item:
//    - assigns owner to be auction owner, this is also used as auction beneficiary
//    - assigns name as asset
//    - assigns close block
//    - assigns default empty bids
// - Contract: Adds Auction House as full access key
// - Contract: Removes all other access keys
// - Contract: Returns newly created auction item ID
```

**PARAMS**:

`Asset <AccountId>`, `CloseBlock <BlockIndex>`

**RESULT**:

```rust
// Vec<u8> Auction ID
```

**FEES/ECONOMICS**:

Create auction fee: GAS(TBD) (function call gas required to delete/store data for this auction) - taken immediately as part of create auction transaction.

**POSSIBLE EXPLOITS**:

* Confirm an asset is not being auctioned again during an active auction with same asset
* Close block never closes - ensure maximum and minimum range definitions
* Invalid asset, unparse-able or malicious payload in asset
* enforce asset is a near domain, such that `*.near`, and no other format (for now not allowing sub-domains, since this causes root access/ownership issues)
* Minimum auctionable name: 1 chars - for `*.near`, 32 for `*` (See near name definitions)
* Maximum auctionable name: 64 chars (See near name definitions)
* Only allow chars `[A-Za-z0-9]` (See near name definitions)
* Auction payment too little
* Auction payment too much (?? Pretty sure default transaction behaviour returns overpayments)
* Auctioning a name associated to a contract could be bad
* Auctioning the account owned by transaction signer
* Creator can specify starting bid amount
* Cannot be created if auction house is paused

----

## Auction Item Bid

**TYPE**: `call`

**USER STORY**:

As a user, I want to place a bid on a near name that I like, so I can potentially claim `lol.near` because i MUST have it. I am willing to pay a small bid fee, and I am willing to wait until auction closes to claim this name. To bid, I send a fee, bid amount & beneficiary near account id.

**ADMIN STORY**:

As an auction house admin, I want anyone with a valid near account ID to 

**FUNCTION**: 

```rust
// Logic:
// - User inputs auction item ID (Required)
// - User inputs bid amount (Required)
```

**PARAMS**:

`AuctionId <AccountId>`, `Bid <Balance>`

**RESULT**:

`Success`, `Error`

**FEES/ECONOMICS**:

1. Place bid fee: GAS(10T - TBD) (function call gas * 2) - taken immediately as part of place bid transaction.

**POSSIBLE EXPLOITS**:

* Must not be owner of auction
* Must submit bid amount of greater than zero
* Must be an active auction
* Bidder cannot be original name owner
* Bid amount needs to be greater than 0
* Bid must be greater than MIN bid diff amount (Example: if lowest bid is 10T GAS, then next bid must be 10T GAS higher) (Default bid min: 10T GAS - TBD)
* Bidder can update their bid at any time, but fee applies each time
* Cannot bid if auction house is paused

----

## Auction Item Finalization

**TYPE**: `call`

**USER STORY**:

**ADMIN STORY**:

**FUNCTION**: 

```rust
// Logic:
// - 
```

**PARAMS**:

**RESULT**:

**FEES/ECONOMICS**:

1. Auction Finalization Fee: 3-5% total Sale GAS(??) - computed upon resolving final winner bid amount

**POSSIBLE EXPLOITS**:

----

## Auction Item Cancelation

**TYPE**: `call`

**USER STORY**:

**ADMIN STORY**:

**FUNCTION**: 

```rust
// Logic:
// - 
```

**PARAMS**:

**RESULT**:

**FEES/ECONOMICS**:

1. Auction Removal Fee: GAS(10T - TBD) (function call gas * 2 + data deletion gas) - taken in addition to transaction fee

**POSSIBLE EXPLOITS**:
