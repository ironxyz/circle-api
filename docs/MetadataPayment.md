# MetadataPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email of the user. | 
**phone_number** | Option<**String**> | Phone number of the user in E.164 format. We recommend using a library such as [libphonenumber](https://github.com/google/libphonenumber) to parse and validate phone numbers. | [optional]
**session_id** | **String** | Hash of the session identifier; typically of the end user. This helps us make risk decisions and prevent fraud. IMPORTANT: Please hash the session identifier to prevent sending us actual session identifiers.  | 
**ip_address** | **String** | Single IPv4 or IPv6 address of user' | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


