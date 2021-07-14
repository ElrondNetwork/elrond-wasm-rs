# Bonding Curve Contract

This contract enables using a bonding curve for defining the behaviour of the price of the token as its balance changes.

The contract allows issuing of any ESDT token and together with its issue elements such as details about the supply will be stored together in an entity named `BondingCurve` structured as follows:
  - `FunctionSelector` - an enum that contains the functions available for setting
  - `CurveArguments` - containing:
    - supply_type
    - max_supply
    - available_supply
    - balance
  - `TokenIdentifier` - containing the accepted payment token
  
Because of working with different types of ESDT, the entity under which we will make the mapping with the curve function will be called `Token`, containing the `TokenIdentifier` and the `nonce`.

The behaviour however is differend depending on the issued token:
  - Fungible Token (FT):
    * defines one bonding curve
    * the nonce from `Token` should be set to 0
    * the supply and balance are indicated by the amount minted
  - Semi-Fungible Token (SFT):
    * defines multiple bonding curves (one per each nonce)
    * the supply and balance are indicated by the amount of each nonce
  - Non-Fungible Token (NFT):
    * defines one bonding curve
    * the supply and balance are indicated by the number of nonces (reason why we consider the nonce to be 0)
    * the nonce from `Token` should be set to 0


Upon issue the storage setting of the `CurveArguments` is done, reason why the supply details and the token accepted as payment are requested. Notice though that NFTs and SFTs share the same [implementation](docs/nft_and_sft.md), differing in functionality.

# Usage

When using this contract one should do the following process for each issued token:
  - issue the token
  - add quantity
  - set the curve function

# Setting up the Curve Function

The bonding curve function configurations are set in the [function selector](docs/selector.md)
Here is where you would like to set your custom functions if the predefined ones are not what you are looking for.

In the case where the curve function is not set, `FunctionSelector::None` will be the value of it, throwing an error until a proper function is set.