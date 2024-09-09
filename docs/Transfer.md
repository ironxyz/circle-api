# Transfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**source** | [**models::TransferSourceLocation**](TransferSourceLocation.md) |  | 
**destination** | [**models::TransferDestinationLocation**](TransferDestinationLocation.md) |  | 
**amount** | [**models::Money**](Money.md) |  | 
**fees** | Option<[**Vec<models::Fee>**](Fee.md)> | An array of fees applied to a transaction. This is only available when there is at least one non-zero fee. | [optional]
**transaction_hash** | Option<**String**> | A hash that uniquely identifies the onchain transaction. This is only available where either source or destination are of type blockchain. | [optional]
**status** | **String** | Status of the transfer. Status `pending` indicates that the transfer is in the process of running; `complete` indicates it finished successfully; `failed` indicates it failed. Circle Mint Singapore customers may have transfers in the `pending` status if the recipient addresses are not verified. | 
**error_code** | Option<[**models::TransferErrorCode**](TransferErrorCode.md)> |  | [optional]
**create_date** | Option<**String**> | The create date of the transfer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


