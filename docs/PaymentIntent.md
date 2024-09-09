# PaymentIntent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**amount** | [**models::CryptoPaymentsMoney**](CryptoPaymentsMoney.md) |  | 
**amount_paid** | Option<[**models::CryptoPaymentsMoney**](CryptoPaymentsMoney.md)> |  | [optional]
**amount_refunded** | Option<[**models::CryptoPaymentsMoney**](CryptoPaymentsMoney.md)> |  | [optional]
**settlement_currency** | **String** | Desired currency for the payments to settle in. | 
**payment_methods** | [**Vec<models::PaymentMethodBlockchain>**](PaymentMethodBlockchain.md) |  | 
**fees** | Option<[**Vec<models::PaymentIntentFees>**](PaymentIntentFees.md)> |  | [optional]
**payment_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of associated payments. | [optional]
**refund_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of associated refunds. | [optional]
**timeline** | Option<[**Vec<models::Timeline>**](Timeline.md)> | State management timeline. | [optional]
**expires_on** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


