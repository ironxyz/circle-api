# AddressObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address. | [optional]
**address_tag** | Option<**String**> | The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format. | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**chain** | Option<[**models::Chain**](Chain.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


