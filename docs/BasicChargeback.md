# BasicChargeback

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**payment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the payment that is associated to the chargeback item. | 
**merchant_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the merchant. | 
**reason_code** | **String** | Reason code given by the card network for the chargeback item. | 
**status** | [**models::ChargebackStatus**](ChargebackStatus.md) |  | 
**category** | Option<[**models::ChargebackCategories**](ChargebackCategories.md)> |  | [optional]
**history** | [**Vec<models::BasicChargebackHistory>**](BasicChargebackHistory.md) | The chargeback item's history list will be sorted by create date descending: more recent chargeback statuses will be at the beginning of the list.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


