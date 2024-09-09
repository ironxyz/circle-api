# Wallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wallet_id** | Option<**String**> | Wallet identifier. Numeric value but should be treated as a string as format may change in the future' | [optional]
**entity_id** | Option<**String**> | Universally unique identifier (UUID v4) of the entity that owns the wallet. | [optional]
**r#type** | Option<**String**> | Wallet type. | [optional]
**description** | Option<**String**> | A human-friendly, non-unique identifier for a wallet. | [optional]
**balances** | Option<[**Vec<models::Money>**](Money.md)> | A list of balances for currencies owned by the wallet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


