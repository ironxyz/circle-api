# PaymentVerificationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avs** | **String** | Status of the AVS check. Raw AVS response, expressed as an upper-case letter. `not_requested` indicates check was not made. `pending` is pending/processing. | 
**cvv** | [**models::CvvResults**](CvvResults.md) |  | 
**three_d_secure** | Option<[**models::ThreeDsResult**](ThreeDsResult.md)> |  | [optional]
**eci** | Option<[**models::Eci**](Eci.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


