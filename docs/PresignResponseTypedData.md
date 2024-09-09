# PresignResponseTypedData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | [**models::PresignDomain**](PresignDomain.md) |  | 
**message** | [**models::PresignMessage**](PresignMessage.md) |  | 
**total_amount** | [**models::FiatMoneyUsd**](FiatMoneyUsd.md) |  | 
**fee_charge_model** | **String** | Who pays for network fee. Can only be endUser or merchant. If it's the endUser, totalAmount includes the network fee and networkFeeQuote isn't null; If it's the merchant, totalAmount doesn't include network fee and networkFeeQuote is null. | 
**network_fee_quote** | Option<[**models::NetworkFeeQuote**](NetworkFeeQuote.md)> |  | [optional]
**types** | [**models::PresignMessageTypes**](PresignMessageTypes.md) |  | 
**primary_type** | **String** | Type of the message | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


