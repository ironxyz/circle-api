# WireCreationRequestIban

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**iban** | **String** | International Bank Account Number (IBAN) for the bank account. | 
**billing_details** | [**models::BillingDetails**](BillingDetails.md) |  | 
**bank_address** | [**models::BankAddressIbanSupported**](BankAddressIbanSupported.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


