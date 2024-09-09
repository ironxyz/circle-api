# \TransfersApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_business_transfer**](TransfersApi.md#create_business_transfer) | **POST** /v1/businessAccount/transfers | Create a transfer
[**create_transfer**](TransfersApi.md#create_transfer) | **POST** /v1/transfers | Create a transfer
[**get_business_transfer**](TransfersApi.md#get_business_transfer) | **GET** /v1/businessAccount/transfers/{id} | Get a transfer
[**get_transfer**](TransfersApi.md#get_transfer) | **GET** /v1/transfers/{id} | Get a transfer
[**list_business_transfers**](TransfersApi.md#list_business_transfers) | **GET** /v1/businessAccount/transfers | List all transfers
[**list_transfers**](TransfersApi.md#list_transfers) | **GET** /v1/transfers | List all transfers



## create_business_transfer

> models::CreateBusinessTransferResponse create_business_transfer(business_transfer_creation_request)
Create a transfer

A transfer can be made from an existing business account to a blockchain location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_transfer_creation_request** | Option<[**BusinessTransferCreationRequest**](BusinessTransferCreationRequest.md)> |  |  |

### Return type

[**models::CreateBusinessTransferResponse**](CreateBusinessTransferResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transfer

> models::CreateTransferResponse create_transfer(transfer_creation_request)
Create a transfer

A transfer can be made from an existing funded wallet to a blockchain address or another wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_creation_request** | Option<[**TransferCreationRequest**](TransferCreationRequest.md)> |  |  |

### Return type

[**models::CreateTransferResponse**](CreateTransferResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_transfer

> models::GetBusinessTransferResponse get_business_transfer(id)
Get a transfer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetBusinessTransferResponse**](GetBusinessTransferResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer

> models::GetTransferResponse get_transfer(id, return_identities)
Get a transfer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**return_identities** | Option<**bool**> | Specify if you would like to see identities in the response. Restricts maximum returned items to 5. By default returnIdentities is false, resulting in the response not returning `data.source.identities`. |  |[default to false]

### Return type

[**models::GetTransferResponse**](GetTransferResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_transfers

> models::ListBusinessTransfersResponse list_business_transfers(from, to, page_before, page_after, page_size)
List all transfers

Searches for transfers from your business account. If the date parameters are omitted, returns the most recent transfers. This endpoint returns up to 50 transfers in descending chronological order or pageSize, if provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListBusinessTransfersResponse**](ListBusinessTransfersResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transfers

> models::ListTransfersResponse list_transfers(wallet_id, source_wallet_id, destination_wallet_id, return_identities, from, to, page_before, page_after, page_size)
List all transfers

Searches for transfers involving the provided wallets. If no wallet ids are provided, searches all wallets associated with your Circle API account. If the date parameters are omitted, returns the most recent transfers. This endpoint returns up to 50 transfers in descending chronological order or pageSize, if provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_id** | Option<**String**> | Unique identifier for the source or destination wallet of transfers, if any. May not be used in conjunction with destinationWalletId or sourceWalletId. Useful for fetching all transfers related to a wallet. |  |
**source_wallet_id** | Option<**String**> | Unique identifier for the source wallet of transfers, if any. |  |
**destination_wallet_id** | Option<**String**> | Unique identifier for the destination wallet of transfers, if any. |  |
**return_identities** | Option<**bool**> | Specify if you would like to see identities in the response. Restricts maximum returned items to 5. By default returnIdentities is false, resulting in the response not returning `data.source.identities`. |  |[default to false]
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListTransfersResponse**](ListTransfersResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

