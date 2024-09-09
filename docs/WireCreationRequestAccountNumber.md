# WireCreationRequestAccountNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**account_number** | **String** | Account number that identifies the bank account. | 
**routing_number** | **String** | The bank's SWIFT / BIC code. | 
**billing_details** | [**models::BillingDetails**](BillingDetails.md) |  | 
**bank_address** | [**models::BankAddressNonIban**](BankAddressNonIban.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


