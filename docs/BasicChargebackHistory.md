# BasicChargebackHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Enumerated type of the chargeback history event. `1st Chargeback` represents the first stage of the dispute procedure initiated by the cardholder’s issuing bank.   `2nd Chargeback` represents the second stage of the dispute procedure initiated by the cardholder’s issuing bank (This stage is MasterCard only).   `Chargeback Reversal` represents when 1st Chargeback or 2nd Chargeback is withdrawn by the issuer.   `Representment` represents the stage when merchants decided to dispute 1st Chargeback or 2nd Chargeback.   `Chargeback Settlement` can imply one of the two: 1) If merchant or marketplace is taking the lost of the chargeback, money will be debit from the wallet during this stage.   2) If merchant of marketplace successfully dispute the chargeback, money will be credit back to the wallet during this stage.  | 
**chargeback_amount** | [**models::FiatMoneyUsd**](FiatMoneyUsd.md) |  | 
**fee** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**description** | **String** | The reason the chargeback was created. | 
**settlement_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the settlement related to the chargeback history. | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


