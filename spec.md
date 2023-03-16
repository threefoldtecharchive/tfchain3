# Specifications of tfchain 3

## Introduction

With tfchain we created a blockchain that serves as the backbone of our Threefold grid. This blockchain is a registar for nodes, farms, twins and deployment contract.
Tfchain currently has implemented a proof of authority network, where only authorized nodes can join the network. There are no staking rewards and no traditional inflation,
making it so that there is no incentive for outsiders to run a tfchain node.

## Tfchain 3 will serve as the backbone of first regional internet

The idea is that for a regional internet, a tfchain will be created and will run in that context.

## Native token

The native token on tfchain3 will be called "CHI" (Chi (also spelled qi) refers to the vital life force or energy that runs through all living beings.

## Staking

Nominated proof of stake with traditional inflation will be used to incentive the
community running validators. It is not required to run a validator yourself to
participate in staking, a token owner can also stake his tokens for a validator
run by someone else.

inflation curve with be implemented as:

- 10% inflation (increase of the total amount of tokens on the chain per year).
This is an ideal value. The staking algoirthm, defined below, aims to make sure
a target percentage of tokens available on chain are staked, and adjusts inflation
payouts based on the amount of actual tokens staked. More details below.
- 5% of revenue of deployments (contracts) on the chain. On the current TFChain,
this 5% is also sent to a different account. On a technical level, the existing
NPOS pallet will be forked, and the payout function will be modified to also distribute
rewards accrued by the utilization. To make this fair, instead of paying out all
tokens in this treasury at the end of every `session`, only 1% of tokens in the
treasury will be distributed per session. This will remove the impact of billing
spikes or drops on a single session. Of course, this means it takes 100 sessions
for this reward pool to start giving out the full expected payment, though we can
consider this to be part of the initial boostrapping phase of the chain. Should
it be needed, this account can be funded as well to immediatly have the expected
payouts.

### Staking algorithm

As explained above, there is a configurable _ideal staking rate_. The chain also
tracks the _actual staking rate_. If the _actual staking rate_ is different from
the _ideal staking rate_, the percentage difference between the 2 values as part
of the inflation is not paid to the stakers, but rather depositted into a treasury
account. This allows the staking algorithm to incetivize token owners to stake
tokens such that the _ideal staking rate_ is met. Note that this also means that
if more tokens are staked than the ideal rate, stakers are incentivized to partially
unstake their tokens again.

Only a limited amount of nodes can be validators at any given time (this amount is
configurable). This incentivizes people to stake a sufficient amount of tokens.

The staking algorithm introduces the concept of `sessions`, a predefined amount of
time (expressed as created blocks) during which a certain `validator set` is active.
Every validator in the set is equal when it comes to block prodution, irrespective
of the amount of staked tokens. The size of the stake is only used to select the
authorities, in a simple "top X stakes become validators".

There are 2 types of staking:

- Validator staking: a user stakes tokens for his own node to become a validator
- Nominator staking: a user stakes tokens and then elects some other node, for that
node to become a validator

There is a limit to the amount of _effective_ nominator stakes for a node (e.g. only
the top 100 nominators are counted).

The sum of the stake of a node for election purposes is the sum of the validator
stake and all of it's effective nominator stakes.

When it comes to payout distribution, a node sets a percentage of the rewards which
it claims for "operational reasons". This is first distributed to the node operator.
The remainder is split across the validator and effective nominators, according to
their share of the stake which is active (if a node has more than the maximum amount
of active nominators, nominators which aren't included don't get rewards, and their
stake is not counted toward the active stake).

It is required for a node to explicitly declare that it wants to get elected as validator
(step up). It is always possible to step up, even if insufficient stake is added
to a node (it will simply not get elected in this case). Likewise, to get stop being
a validator, it should explicitly step down, and wait untill the session is over
to be removed from the validator set. It is not possible to remove a stake from
a node (a bond), while it is not stepped down (i.e. it is an active validator or
participating in validator election).

Unbonding can be done at any time, but will take a singifficant amount of time
up to 3 weeks). This is intended to protect the network when there is a price
fluctuation.

If a validator missbehaves, it will get slashed. The size of the slash is proportional
to the offence. A slash is expressed as a percentage of the total stake. The percentage
is applied to all active stakers. Nominators should thus take care to only nominate
nodes they trust to operate as expected. The size of the slash will depend on the
gravity of the offence, and the probability of malicious intent. For instance, an
elected node which goes offline will be slashed for less than a node which is trying
to create duplicate blocks, since the former can be the result of poor operation,
while the latter is very likely the result of an intent to attack the chain.

In any case, if a validator is slashed, it is also forcibly stepped down and removed
form the validator set (if this happens by during a session, it is done by means
of a forced premature session rotation). This means that after a slash, the node
operator **must** take manual action in order to participate in election again,
thus implicitly acknowledging the slash.

A slash leads to the destruction of the affected tokens. There is no redistribution.
This is to make sure a third party cannot benefit from misbehaving of a node (as
this would incentivize a denial of service attack against validators). After a slash
happens, there is a period of 4 weeks before it becomes effective, during which time
the DAO (or any other entity with elevated priviledges) can decide to revert the
slash, thus preventing the destruction of the funds and returning them to the original
owners.

## Farming

Nodes connected to tfchain3 will receive rewards in CHI. Additional rewards for
utilization will be handed out to Farmers when their nodes are being used. For a
node to receive tokens, it must reach a certain amount of uptime over a given period
of time. This uptime is tracked and recorded on chain through the use of `uptime
remports`. A special minting pallet is implemented which aggregates these reports,
and using the node uptime reported in them, calculates when nodes have rebooted.
After a given amount of time (roughly 1 month) has passed, a verdict is applied wether
the node has reached it's uptime requirement, or not. If it has, tokens are minted
for the node as reward for being online. This is the base farming payout.

Additionally, to incentivize farmers to attract workloads on their farm (by having
good hardware), a node is rewarded for being utilized. Utilization is tracked through
node contracts, which have the resources used set on chain. Every time a node reports
the NRU consumption of a contract, it includes a timing window over which the NRU
was consumed. Using this window, the minting pallet can determine how many resources
were used for how long, by checking the given window and the contract resources which
were already set on chain.

Resources are thus tracked as `Resource X time`, where resource is the base value,
and time is the amount of seconds. Every time used resources are counted, every resource
is multiplied by the amont of time passed, and this is added to a counter for every
resource. This then allows calculation of the percentage a resource was used over
time by dividing this counter by the total of a given resource in the node, and then
multiplying this with the duration of a `period`. Note that this requires to keep
track of the resources in the node by the minting code, to avoid getting a skewed
result if the capacity is upgraded.

Rewards for utilization can be paid out even if the node does not meet the uptime
SLA required for its regular rewards, as the described way of tracking utilization
already makes sure the utilization percentage is proportionally lowered if the node
experiences downtime.

Given the complexity of tokenomics as a whole, those will be handled in a different
standalone spec.

### Boosters in line with utilization

In an effort to boost utilization, there is an intent to increase farming rewards
for nodes which reach a treshold in utilization over a `period`. Once again, considering
that this would influence payouts and the behavior of the token, we will consider
this to be a part of the tokenomic spec, as described above.

#### Linked to USD price

The aim is to create a link with the token price in USD, and to express certain
actions as cost/reward in USD. There are 3 large categories:

- Tokens awarded to validators: this is an inflation percentage, so linking to USD
does not matter here.
- Tokens minted by farming: this should be covered by tokenomics
- Tokens spent in deployemnts: this should also be covered by tokenomics.

It should be noted that linking to the price of a token requires an oracle which
can express the price of the token, which by extension means the token must be
listed on some exchange, such that it can be valued.

## Multisig assets

It must be possible to create new tokens on the chain, optionally secured by multisig
wallets. This means there are 2 things here:

- The possibility to create tokens, which are not the base token (CHI)
- The possibility to have multisig

### Multisignature

Multisignature can naively be implemented with a dedicated pallet. This works by
setting up an address, governed by given public keys and requireing a certain
treshold. The address is generated by the chain logic. Some chain storage is needed
for these addresses. In order to perform a multisig call, the call itself must
first be created and saved on chain. After this, participants of the multisig
must call the sign function in the pallet for the call, untill sufficient signatures
are collected to meet the address treshold, at which point the call is submitted.
Since this approach creates a regular address, it can be used for any and all
actions on chain which require an account.

Alternatively, shnorr signatures support `Treshold Signature Systems`. This means
that when `sr25519` keys are used, it is possible to construct a multisig completely
of chain, which avoids additional transaction fees which will be required in the
previously outlined solution. The downside is that these signatures are only available
on `sr25519` keys, and even then, it is possible other caveats apply.

### Assets

Creating assets will work similar to stellar. Anyone can create an `asset` with
certain given parameters (base supply, name, short name, precision). Once an asset
is created, it is owned by the creator, who can perform the following actions:

- Mint more of the asset
- Burn part of the asset
- Freeze/thaw the asset for an owner
- Destroy the asset in its entirety

To perform these functions, the asset is owned by a `team`. These accounts are
set by the owner. Concretely, there is an `issuer`, a `freezer`, and an `admin`.
It is possible for an account to fulfill multiple or all roles at once.

This is very similar to the way assets work on the stellar blockchain, except
that no explicit trustline must be created first.

In a similar fashion to `ERC20` tokens, it is also possible to `approve` a third
party to spend some asset on behalf of the user.

## Treasury

A Treasury should be in place that can spend CHI using Multisig transactions.
Details on creating multisig transactions have been outlined above, and it is
possible to create a genesis state such that a multisig account is set up to
start with.

Alternatively, a dedicated treasury pallet can be used. This has the advantage
of being easier to track the available balance, as well as adding custom behavior.
Operations on the treasury can be restricted to authorized members (council),
which can put forward a proposal to spend funds for some reason, on which others
can then vote.
