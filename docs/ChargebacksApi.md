# \ChargebacksApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mock_chargeback**](ChargebacksApi.md#create_mock_chargeback) | **POST** /v1/mocks/cards/chargebacks | Create a mock chargeback
[**get_chargeback**](ChargebacksApi.md#get_chargeback) | **GET** /v1/chargebacks/{id} | Get a chargeback
[**list_chargebacks**](ChargebacksApi.md#list_chargebacks) | **GET** /v1/chargebacks | List all chargebacks



## create_mock_chargeback

> models::CreateMockChargebackResponse create_mock_chargeback(mock_chargeback_creation_request)
Create a mock chargeback

In the sandbox environment, initiate a mock chargeback of a specified payment.  The entire payment will be charged back for its full value.  The payment must be in the `paid` state (otherwise the endpoint will return a `404`), and each payment can only be charged back once (otherwise the endpoint will return a `409`).  This endpoint is only available in the sandbox environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mock_chargeback_creation_request** | Option<[**MockChargebackCreationRequest**](MockChargebackCreationRequest.md)> |  |  |

### Return type

[**models::CreateMockChargebackResponse**](CreateMockChargebackResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chargeback

> models::GetChargebackResponse get_chargeback(id)
Get a chargeback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetChargebackResponse**](GetChargebackResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_chargebacks

> models::ListChargebacksResponse list_chargebacks(payment_id, from, to, page_before, page_after, page_size)
List all chargebacks

Retrieve list of chargebacks. Results will be sorted by create date descending: more recent chargebacks will be at the beginning of the list. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | Option<**uuid::Uuid**> | The payment ID associated with the chargeback. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListChargebacksResponse**](ListChargebacksResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

