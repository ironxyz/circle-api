# RedemptionFeeCalculation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**fee** | Option<[**models::FiatMoney**](FiatMoney.md)> |  | [optional]
**cumulated_payout_amount** | Option<[**models::BurnFeePayoutAmount**](BurnFeePayoutAmount.md)> |  | [optional]
**cumulated_payment_amount** | Option<[**models::BurnFeePaymentAmount**](BurnFeePaymentAmount.md)> |  | [optional]
**cumulated_net_amount** | Option<[**models::BurnFeeNetAmount**](BurnFeeNetAmount.md)> |  | [optional]
**value_date** | Option<**String**> | A date representing a day for which a fee is calculated. | [optional]
**status** | Option<**String**> | Fee collection status | [optional]
**threshold_reset_timestamp** | Option<**String**> | Fee calculation reset timestamp. | [optional]
**create_date** | Option<**String**> | The create date of the NET burn daily fee calculation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


