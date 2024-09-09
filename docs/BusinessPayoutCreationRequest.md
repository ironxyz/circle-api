# BusinessPayoutCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**destination** | [**models::BusinessDestinationRequest**](BusinessDestinationRequest.md) |  | 
**amount** | [**models::FiatMoney**](FiatMoney.md) |  | 
**to_amount** | Option<[**models::FiatPayoutToMoney**](FiatPayoutToMoney.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


