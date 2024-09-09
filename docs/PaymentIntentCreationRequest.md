# PaymentIntentCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**amount** | [**models::CryptoPaymentsMoney**](CryptoPaymentsMoney.md) |  | 
**settlement_currency** | **String** | Desired currency for the payments to settle in. | 
**payment_methods** | [**Vec<models::PaymentMethodBlockchain>**](PaymentMethodBlockchain.md) |  | 
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


