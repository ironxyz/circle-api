# ApplePayToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** | ApplePay token version information. | 
**data** | **String** | Encrypted payment data. Base64 encoded as a string. | 
**signature** | **String** | Signature of the payment and header data. The signature includes the signing certificate, its intermediate CA certificate, and information about the signing algorithm. | 
**header** | [**models::ApplePayTokenHeader**](ApplePayToken_header.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


