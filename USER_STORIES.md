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
// - Create back door -- for Auction House owners to be able to create auctions without fees
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

`AuctionId <Vec<u8>>`, `Bid <Balance>`

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

As a user I want to find out if I won an auction item. I want to either send a transaction to finalize auction to get my reward for winning the auction item, or get the auction item directly if someone else finalized, or lastly get my bid amount (minus fees) returned to me if I lost the auction.

**ADMIN STORY**:

As an auction house admin, I want to be able to call the finalize auction to cover the transaction fee for users if I'm feeling nice. I also am okay with anyone that placed bids to call finalize to pay for transaction fees to release auction outcome.

**FUNCTION**: 

```rust
// Logic:
// - User inputs auction item ID
// - User cannot call this function unless they are: A. the auction owner, B. a bidder, C. Admin
// - Contract: If no bids
//    - add auction item owner account id to full access key
//    - mark auction as inactive
//    - return
// - Contract: Adds Winner account ID as full access key
// - Contract: Sends Auction item owner account ID highest bid amount, minus auction percentage fee
// - Contract: Removes Auction House access keys
// - Contract: all bidders get their bid amounts back, minus fees
// - Contract: Marks auction item as inactive
```

**PARAMS**:

`AuctionId <Vec<u8>>`

**RESULT**:

`Success`, `Error`

**FEES/ECONOMICS**:

1. Auction Finalization Fee: 3-5% total Sale GAS(??) - computed upon resolving final winner bid amount
2. Auction without bids only costs txn execution gas.

**POSSIBLE EXPLOITS**:

* too many bids, transaction fee is too high to complete
* too little balance remaining to return to auction bid loser
* finalization doesnt get called, leaving
* Auction never becoming inactive - failure to allow future auctions
* No bidders, revert auction to owner

----

## Auction Item Cancelation

**TYPE**: `call`

**USER STORY**:

As an auction item owner, i want to be able to cancel my auction at any time between an opened auction, so I dont lose my account ID if I no longer want to auction it. I am okay with paying some fees to return any/all bid money and return the account ID back to me.

**ADMIN STORY**:

As an auction house admin, I want to allow any auction item owner to be able to cancel an auction to allow them to feel in control of their account IDs. This will give more trust and is what users expect to be able to do.

**FUNCTION**: 

```rust
// Logic:
// - User inputs auction ID for auction item
// - Contract: Confirms auction is still active
// - Contract: Confirms transaction signer is auction owner OR Auction house admin
// - Contract: Transfers full access keys to auction item owner
// - Contract: Transfers any/all bid amounts back to bidders
```

**PARAMS**:

`AuctionId <Vec<u8>>`

**RESULT**:

`Success`, `Error`

**FEES/ECONOMICS**:

1. Auction Removal Fee: GAS(10T - TBD) (function call gas * 2 + data deletion gas) - taken in addition to transaction fee

**POSSIBLE EXPLOITS**:

* Not the auction owner calling
* Not enough gas to cover transfer fees for account ID
* Not enough gas to cover transfer fees for any bid returns
* Auction inactive

----

# Scratch notes:

Thoughts on escrow:
Since contracts can be easily upgraded, and state is therefore completely obliterated -- an account for only escrow should be used. It would be something that can own all accounts used during an auction, have 0 full access keys for the escrow contract itself, and live within the namespace of "escrow.nym.near" or similar. This escrow can own both names & colleteral, meaning that if the state of "auction.nym.near" was ever completely wiped, the escrow assets were not affected. It also seems easier to make transfers to/from an escrow account rather than keeping within the auction contract/account.

Another thing to consider:
Abandoned auctions -- until cron, someone has to finalize a name auction. There should be a max window to release the name.
This could allow for the auction house to benefit by either:

1. Allowing a finalized auction item to be purchased directly
2. Allowing finalized auction item to be released (deleted) and all bid amounts paid to auction house