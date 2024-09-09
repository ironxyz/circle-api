# ChannelResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**default** | Option<**bool**> | Flag to indicate whether the channel is configured as default. At most one of the channels will have this flag set to true and the default channel is used when a payment request does not have the `channel` property set. | [optional]
**card_descriptor** | Option<**String**> | Descriptor that appears on cardholders' bank statements for card payments submitted through this channel. | [optional]
**ach_descriptor** | Option<**String**> | Descriptor that appears on end-users' bank statements for ACH payments submitted through this channel. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


