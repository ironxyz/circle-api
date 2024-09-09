# UnwithdrawalObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**payout_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Universally unique identifier (UUID v4) of the payout that is associated with the return. | [optional]
**amount** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**reason** | Option<**String**> | Reason for the return. | [optional]
**status** | Option<**String**> | Status of the return. A `pending` status indicates that the return is in process; `complete` indicates it finished successfully; `failed` indicates it failed. | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


