# TransferCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**source** | [**models::TransferRequestSourceWalletLocation**](TransferRequestSourceWalletLocation.md) |  | 
**destination** | [**models::TransferCreationRequestDestination**](TransferCreationRequest_destination.md) |  | 
**amount** | [**models::Money**](Money.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


