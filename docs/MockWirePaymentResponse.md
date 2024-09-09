# MockWirePaymentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tracking_ref** | Option<**String**> | Wire tracking reference that needs to be set in the wire reference to beneficiary field. This field is retrievable through the response during wire creation or via the bank instruction endpoint. | [optional]
**amount** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**beneficiary_bank** | Option<[**models::MockWirePaymentBeneficiaryBankInstruction**](MockWirePaymentBeneficiaryBankInstruction.md)> |  | [optional]
**status** | Option<**String**> | Enumerated status of the wire payment. Status `pending` indicates that the wire payment is in process; `processed` indicates it finished successfully; `failed` indicates it failed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


