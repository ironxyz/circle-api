# ApplePayTokenRsaHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_data** | Option<**String**> | Optional. Hash of the applicationData property of the original PKPaymentRequest object. If the value of that property is null, this key is omitted. | [optional]
**wrapped_key** | **String** | The symmetric key wrapped using your RSA public key. | 
**public_key_hash** | **String** | Hash of the X.509 encoded public key bytes of the merchant’s certificate. | 
**transaction_id** | **String** | Transaction identifier, generated on the device. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


