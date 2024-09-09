# PixFiatAccountCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**name** | **String** | Name of the beneficiary. | 
**account_number** | **String** | Beneficiary account number. | 
**ispb** | **String** | Beneficiary ISPB. | 
**branch_code** | Option<**String**> | Beneficiary account branch code. | [optional]
**tax_id** | **String** | Beneficiary Tax ID. | 
**account_type** | [**models::PixAccountType**](PixAccountType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


