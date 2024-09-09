# \DepositsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_business_deposits**](DepositsApi.md#list_business_deposits) | **GET** /v1/businessAccount/deposits | List all deposits



## list_business_deposits

> models::ListBusinessDepositsResponse list_business_deposits(r#type, from, to, page_before, page_after, page_size)
List all deposits

Searches for deposits sent to your business account. If the date parameters are omitted, returns the most recent deposits. This endpoint returns up to 50 deposits in descending chronological order or pageSize, if provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Unique identifier for the deposit type. Filters results to fetch deposits made by this specific type. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListBusinessDepositsResponse**](ListBusinessDepositsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

