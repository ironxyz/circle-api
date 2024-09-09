# \CheckoutSessionsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_checkout_session**](CheckoutSessionsApi.md#create_checkout_session) | **POST** /v1/checkoutSessions | Create a checkout session
[**extend_checkout_session**](CheckoutSessionsApi.md#extend_checkout_session) | **POST** /v1/checkoutSessions/{id}/extend | Extend a checkout session
[**get_checkout_session**](CheckoutSessionsApi.md#get_checkout_session) | **GET** /v1/checkoutSessions/{id} | Get a checkout session
[**list_checkout_sessions**](CheckoutSessionsApi.md#list_checkout_sessions) | **GET** /v1/checkoutSessions | List all checkout sessions



## create_checkout_session

> models::CreateCheckoutSessionResponse create_checkout_session(checkout_session_creation_request)
Create a checkout session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checkout_session_creation_request** | Option<[**CheckoutSessionCreationRequest**](CheckoutSessionCreationRequest.md)> |  |  |

### Return type

[**models::CreateCheckoutSessionResponse**](CreateCheckoutSessionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extend_checkout_session

> models::ExtendCheckoutSessionResponse extend_checkout_session(id, extend_checkout_session_request)
Extend a checkout session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**extend_checkout_session_request** | Option<[**ExtendCheckoutSessionRequest**](ExtendCheckoutSessionRequest.md)> |  |  |

### Return type

[**models::ExtendCheckoutSessionResponse**](ExtendCheckoutSessionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_checkout_session

> models::GetCheckoutSessionResponse get_checkout_session(id)
Get a checkout session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetCheckoutSessionResponse**](GetCheckoutSessionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_checkout_sessions

> models::ListCheckoutSessionsResponse list_checkout_sessions(from, to, page_before, page_after, page_size)
List all checkout sessions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListCheckoutSessionsResponse**](ListCheckoutSessionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

